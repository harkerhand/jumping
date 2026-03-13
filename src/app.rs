use crate::entry::Entry;
use ratatui::widgets::ListState;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// State of the application
pub struct App {
    /// Left (parent directory) list state
    pub left_state: ListState,
    /// Left (parent directory) entries
    pub left_entries: Vec<Entry>,
    /// Center (current directory) list state
    pub center_state: ListState,
    /// Center (current directory) entries
    pub center_entries: Vec<Entry>,
    /// Current directory path
    pub current_path: PathBuf,
    /// Whether to quit the app
    pub should_quit: bool,
    /// Whether to show hidden directories (starting with .)
    pub show_hidden: bool,
}

impl App {
    pub fn new() -> io::Result<App> {
        let current_path = std::env::current_dir()?;
        let mut app = App {
            left_state: ListState::default(),
            left_entries: Vec::new(),
            center_state: ListState::default(),
            center_entries: Vec::new(),
            current_path,
            should_quit: false,
            show_hidden: false,
        };
        app.load_current_dir()?;
        Ok(app)
    }

    /// Load entries from a directory
    /// If `addition` is true, the entries will include the current directory (".") and parent directory ("..") with their paths for display in the center pane. If false, they will be displayed as normal without paths.
    fn read_dir(&self, path: &Path, addition: bool) -> io::Result<Vec<Entry>> {
        if !path.is_dir() {
            return Ok(Vec::new());
        }
        let mut entries = Vec::new();

        entries.push(Entry {
            name: if addition {
                format!(". ({})", path.display())
            } else {
                ".".to_string()
            },
            path: path.to_path_buf(),
            is_dir: true,
            need_preview: false,
        });
        if let Some(parent) = path.parent() {
            entries.push(Entry {
                name: if addition {
                    format!(".. ({})", parent.display())
                } else {
                    "..".to_string()
                },
                path: parent.to_path_buf(),
                is_dir: true,
                need_preview: false,
            });
        }

        let mut read_entries = fs::read_dir(path)?
            .filter_map(|res| res.ok())
            .map(|e| {
                let path = e.path();
                let is_dir = path.is_dir();
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                Entry {
                    name,
                    path,
                    is_dir,
                    need_preview: true,
                }
            })
            .filter(|e| e.is_dir && (self.show_hidden || !e.name.starts_with('.')))
            .collect::<Vec<_>>();

        read_entries.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        entries.append(&mut read_entries);
        Ok(entries)
    }

    /// Load the current directory entries into the center, and update the left (parent) entries accordingly
    fn load_current_dir(&mut self) -> io::Result<()> {
        self.center_entries = self.read_dir(&self.current_path, true)?;
        if !self.center_entries.is_empty() {
            self.center_state.select(Some(0));
        } else {
            self.center_state.select(None);
        }

        if let Some(parent) = self.current_path.parent() {
            self.left_entries = self.read_dir(parent, false)?;
            let current_dir_name = self
                .current_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            let pos = self
                .left_entries
                .iter()
                .position(|e| e.name == current_dir_name);
            self.left_state.select(pos);
        } else {
            self.left_entries = Vec::new();
            self.left_state.select(None);
        }

        Ok(())
    }

    /// Get entries for the right preview pane based on the currently selected entry in the center pane
    pub fn get_right_entries(&self) -> Vec<Entry> {
        if let Some(index) = self.center_state.selected()
            && let Some(entry) = self.center_entries.get(index)
            && entry.need_preview
        {
            return self.read_dir(&entry.path, false).unwrap_or_default();
        }
        Vec::new()
    }

    pub fn move_up(&mut self) {
        let i = match self.center_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.center_entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        if !self.center_entries.is_empty() {
            self.center_state.select(Some(i));
        }
    }

    pub fn move_down(&mut self) {
        let i = match self.center_state.selected() {
            Some(i) => {
                if i >= self.center_entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        if !self.center_entries.is_empty() {
            self.center_state.select(Some(i));
        }
    }

    pub fn move_right(&mut self) -> io::Result<()> {
        if let Some(index) = self.center_state.selected()
            && let Some(entry) = self.center_entries.get(index)
            && entry.is_dir
        {
            self.current_path = entry.path.clone();
            self.load_current_dir()?;
        }
        Ok(())
    }

    pub fn move_left(&mut self) -> io::Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.load_current_dir()?;
        }
        Ok(())
    }

    pub fn toggle_hidden(&mut self) -> io::Result<()> {
        self.show_hidden = !self.show_hidden;
        self.load_current_dir()?;
        Ok(())
    }
}
