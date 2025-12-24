use rusqlite::{Connection, Result, Error};

pub fn get_note_db_connection() -> Result<Connection, Error> {
    let conn = Connection::open("./notes.sqlite3")?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    Ok(conn)
}

pub fn setup_schema(conn: &Connection) -> Result<(), Error> {

    println!("Setting up a DB schema");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id           INTEGER PRIMARY KEY,
            name         TEXT NOT NULL,
            description  TEXT
        )",
        (), // empty list of parameters.
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS category_tags (
            id           INTEGER PRIMARY KEY,
            name         TEXT NOT NULL,
            description  TEXT
        )",
        (), // empty list of parameters.
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS applied_note_categories (
            note_id      INTEGER NOT NULL,
            category_id  INTEGER NOT NULL,

            FOREIGN KEY(note_id) REFERENCES notes(id),
            FOREIGN KEY(category_id) REFERENCES category_tags(id),
            UNIQUE(note_id, category_id)
        )",
        (), // empty list of parameters.
    )?;

    Ok(())

}
