mod db;

use rusqlite::{Connection, Error};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = db::utils::get_note_db_connection()?;
    db::utils::setup_schema(&conn)?;

    // Functions borrow the connection by using & to request a reference to the object
    // rather than taking ownership

    // db::notes::add_note(&conn, "Hello World!", "Description")?;
    // db::notes::add_note(&conn, "Hello World!", "Description")?;

    // db::notes::delete_note_by_id(&conn, &1)?;


    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("To-do Lists")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // TODO: Replace with generic error handling ASAP
    conn.close().expect("Failed to close database connection");

    Ok(())
}