// TODO: Update to encompass all necessary parts of UI workflow

use rusqlite::{Connection, Error};
use crate::ui::modals::exit::UiExitModalDialog;
use crate::ui::modals::interface::ModalDialog;

pub fn get_note_db_connection() -> rusqlite::Result<Connection, Error> {
    let conn = Connection::open("./notes.sqlite3")?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    Ok(conn)
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}


pub enum CurrentlyExiting {
    Yes,
    No,
}

pub enum CurrentlyEditing {
    Details,
    Tags,
}


pub struct App {
    pub notes_db_conn: Connection,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub exit_dialog: UiExitModalDialog,
}

impl App {
    pub fn new() -> Result<App, Error> {
        Ok(App {
            notes_db_conn: get_note_db_connection()?,
            current_screen: CurrentScreen::Main,
            exit_dialog: UiExitModalDialog::new(),
        })
    }
}