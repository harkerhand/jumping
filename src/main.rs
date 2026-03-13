#![deny(clippy::unwrap_used, clippy::expect_used)]


use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend}, layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
    Terminal,
};
use std::{
    env, fs,
    io::{self},
    path::{Path, PathBuf},
};

/// An entry in the catalog
#[derive(Clone)]
struct Entry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    need_preview: bool,
}

/// State of the application
struct App {
    /// Left (parent directory) list state
    left_state: ListState,
    /// Left (parent directory) entries
    left_entries: Vec<Entry>,
    /// Center (current directory) list state
    center_state: ListState,
    /// Center (current directory) entries
    center_entries: Vec<Entry>,
    /// Current directory path
    current_path: PathBuf,
    /// Whether to quit the app
    should_quit: bool,
}

impl App {
    fn new() -> io::Result<App> {
        let current_path = env::current_dir()?;
        let mut app = App {
            left_state: ListState::default(),
            left_entries: Vec::new(),
            center_state: ListState::default(),
            center_entries: Vec::new(),
            current_path,
            should_quit: false,
        };
        app.load_current_dir()?;
        Ok(app)
    }

    /// Load entries from a directory
    /// If `addition` is true, the entries will include the current directory (".") and parent directory ("..") with their paths for display in the center pane. If false, they will be displayed as normal without paths.
    fn read_dir(path: &Path, addition: bool) -> io::Result<Vec<Entry>> {
        if !path.is_dir() {
            return Ok(Vec::new());
        }
        let mut entries = Vec::new();

        entries.push(Entry {
            name: if addition { format!(". ({})", path.display()) } else { ".".to_string() },
            path: path.to_path_buf(),
            is_dir: true,
            need_preview: false,
        });
        if let Some(parent) = path.parent() {
            entries.push(Entry {
                name: if addition { format!(".. ({})", parent.display()) } else { "..".to_string() },
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
                Entry { name, path, is_dir, need_preview: true }
            })
            .filter(|e| e.is_dir)
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
        self.center_entries = Self::read_dir(&self.current_path, true)?;
        if !self.center_entries.is_empty() {
            self.center_state.select(Some(0));
        } else {
            self.center_state.select(None);
        }

        if let Some(parent) = self.current_path.parent() {
            self.left_entries = Self::read_dir(parent, false)?;
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
    fn get_right_entries(&self) -> Vec<Entry> {
        if let Some(index) = self.center_state.selected()
            && let Some(entry) = self.center_entries.get(index)
            && entry.need_preview {
            return Self::read_dir(&entry.path, false).unwrap_or_default();
        }
        Vec::new()
    }


    fn move_up(&mut self) {
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

    fn move_down(&mut self) {
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

    fn move_right(&mut self) -> io::Result<()> {
        if let Some(index) = self.center_state.selected()
            && let Some(entry) = self.center_entries.get(index) && entry.is_dir {
            self.current_path = entry.path.clone();
            self.load_current_dir()?;
        }
        Ok(())
    }

    fn move_left(&mut self) -> io::Result<()> {
        if let Some(parent) = self.current_path.parent() {
            self.current_path = parent.to_path_buf();
            self.load_current_dir()?;
        }
        Ok(())
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    let selected_style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::BOLD);

    fn entries_to_items(entries: &[Entry]) -> Vec<ListItem<'_>> {
        entries
            .iter()
            .map(|e| {
                let icon = if e.is_dir { " " } else { "󰈔 " };
                let style = if e.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{}{}", icon, e.name)).style(style)
            })
            .collect()
    }

    let left_items = entries_to_items(&app.left_entries);
    let left_block = Block::default().borders(Borders::ALL).title(" PARENT  ");
    let left_list = List::new(left_items)
        .block(left_block)
        .highlight_style(selected_style)
        .highlight_symbol("");
    f.render_stateful_widget(left_list, chunks[0], &mut app.left_state);

    let center_items = entries_to_items(&app.center_entries);
    let center_title = format!(" CURRENT: {} ↕ ", app.current_path.display());
    let center_block = Block::default()
        .borders(Borders::ALL)
        .title(center_title)
        .border_style(Style::default().fg(Color::Yellow));
    let center_list = List::new(center_items)
        .block(center_block)
        .highlight_style(selected_style)
        .highlight_symbol(">> ");
    f.render_stateful_widget(center_list, chunks[1], &mut app.center_state);

    let right_entries = app.get_right_entries();
    let right_items = entries_to_items(&right_entries);
    let right_block = Block::default().borders(Borders::ALL).title(" PREVIEW  ");
    let right_list = List::new(right_items).block(right_block);
    f.render_widget(right_list, chunks[2]);
}


fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app)).map_err(|e| {
            eprintln!("UI rendering error: {:?}", e);
            io::Error::other("UI rendering failed")
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()? {
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Up | KeyCode::Char('k') => app.move_up(),
                KeyCode::Down | KeyCode::Char('j') => app.move_down(),
                KeyCode::Left | KeyCode::Char('h') => app.move_left()?,
                KeyCode::Right | KeyCode::Char('l') => app.move_right()?,
                KeyCode::Enter => {
                    if let Some(index) = app.center_state.selected() && let Some(entry) = app.center_entries.get(index) {
                        let path_str = if entry.name == ".." {
                            entry.path.to_string_lossy().into_owned()
                        } else {
                            let abs_path =
                                entry.path.canonicalize().unwrap_or(entry.path.clone());
                            abs_path.to_string_lossy().into_owned()
                        };
                        let res = fs::write("/tmp/rjump_result", path_str);
                        if res.is_err() {
                            eprintln!("Unable to write '/tmp/rjump_result': {:?}", res.err());
                        }
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
