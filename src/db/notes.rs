use rusqlite::{params, Connection, Error};

pub fn add_note(conn: &Connection, name: &str, description: &str) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO notes (name, description) VALUES (?, ?)",
        params![name, description]
    )?;

    Ok(())
}

pub fn delete_note_by_id(conn: &Connection, id: &i32) -> Result<(), Error> {
    conn.execute(
    "DELETE FROM notes WHERE id = ?",
    params![id]
    )?;

    Ok(())
}