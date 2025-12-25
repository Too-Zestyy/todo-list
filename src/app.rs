// TODO: Update to encompass all necessary parts of UI workflow

use rusqlite::{Connection, Error};

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

pub enum CurrentlyEditing {
    Details,
    Tags,
}


pub struct App {
    pub notes_db_conn: Connection,
    pub key_input: String,              // the currently being edited json key.
    pub value_input: String,            // the currently being edited json value.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> Result<App, Error> {
        Ok(App {
            notes_db_conn: get_note_db_connection()?,
            key_input: String::new(),
            value_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        })
    }
}