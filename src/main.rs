mod db;

use std::error::Error;
use rusqlite::{Connection};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use ratatui::crossterm::{event, execute};
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use todo_list::app;
use todo_list::app::CurrentlyExiting;
use todo_list::app::CurrentlyExiting::No;
use todo_list::ui;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<bool> {

    loop {
        terminal.draw(|f| ui::app::ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }

            match app.currently_exiting {
                None => {
                    if key.code == event::KeyCode::Esc {
                        app.currently_exiting = Option::from(No);
                    }
                }

                // TODO: Update so exit only happens when yes is selected
                Some(CurrentlyExiting::Yes) => {
                    if key.code == event::KeyCode::Enter {
                        return Ok(true);
                    }
                    if key.code == event::KeyCode::Char('d') || key.code == event::KeyCode::Right {
                        app.currently_exiting = Option::from(CurrentlyExiting::No);
                    }
                }

                Some(CurrentlyExiting::No) => {
                    if key.code == event::KeyCode::Enter {
                        app.currently_exiting = None;
                    }
                    if key.code == event::KeyCode::Char('a') || key.code == event::KeyCode::Left {
                        app.currently_exiting = Option::from(CurrentlyExiting::Yes);
                    }
                }

            }
        }

    }
    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = app::get_note_db_connection().expect("Connection to notes DB failed.");
    // db::utils::setup_schema(&conn).expect("Failed to create DB schema.");

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    // let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = app::App::new()?;
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    app.notes_db_conn.close().expect("Failed to close database connection");

    if let Ok(do_print) = res {
        if do_print {
            println!("No Errors on Exit.");
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    // ratatui::restore();
    // TODO: Replace with generic error handling ASAP

    Ok(())
}