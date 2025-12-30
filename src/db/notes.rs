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

pub fn get_note_page_count(conn: &Connection, note_page_length: u32) -> Result<u32, Error> {
    let count: Result<u32, Error> = conn.query_one("SELECT COUNT(*) FROM notes;", (), |row| {
        row.get(0)
    });

    match count {
        Ok(val) => {
            Ok(val.div_ceil(note_page_length))
        }

        Err(error) => {
            Err(error)
        }
    }
}