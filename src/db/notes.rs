use rusqlite::{params, Connection, Error};

pub fn add_note(conn: &Connection, name: &str, description: &str) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO notes (name, description) VALUES (?, ?)",
        params![name, description])?;;

    Ok(())
}