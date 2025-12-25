mod db;

use std::error::Error;
use rusqlite::{Connection};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::Terminal;
use todo_list::app;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut app::App) -> io::Result<bool> {
    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let conn = app::get_note_db_connection().expect("Connection to notes DB failed.");
    db::utils::setup_schema(&conn).expect("Failed to create DB schema.");

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

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    // ratatui::restore();
    // TODO: Replace with generic error handling ASAP
    conn.close().expect("Failed to close database connection");

    Ok(())




    // Ok(())
}