mod db;


use std::io;
use rusqlite::{Connection, Error};
// use std::{io, thread, time::Duration};
// use tui::{
//     backend::CrosstermBackend,
//     widgets::{Widget, Block, Borders},
//     layout::{Layout, Constraint, Direction},
//     Terminal
// };
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };

fn main() -> Result<(), Error> {
    let conn = db::utils::get_note_db_connection()?;

    // Functions borrow the connection by using & to request a reference to the object
    // rather than taking ownership
    db::utils::setup_schema(&conn)?;

    db::notes::add_note(&conn, "Hello World!", "Description")?;

    // TODO: Replace with generic error handling ASAP
    conn.close().expect("Failed to close database connection");


    // setup terminal
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    //
    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Block")
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;
    //
    // thread::sleep(Duration::from_millis(5000));
    //
    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    Ok(())
}