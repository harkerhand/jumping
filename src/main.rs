#![deny(clippy::unwrap_used, clippy::expect_used)]

mod app;
mod entry;
mod ui;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new()?;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app)).map_err(|e| {
            eprintln!("UI rendering error: {:?}", e);
            io::Error::other("UI rendering failed")
        })?;

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
        {
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
                    if let Some(index) = app.center_state.selected()
                        && let Some(entry) = app.center_entries.get(index)
                    {
                        let path_str = if entry.name == ".." {
                            entry.path.to_string_lossy().into_owned()
                        } else {
                            let abs_path = entry.path.canonicalize().unwrap_or(entry.path.clone());
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
