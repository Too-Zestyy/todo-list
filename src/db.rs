use rusqlite::{Connection, Result, Error};


#[derive(Debug)]
struct Note {
    id: i32,
    title: String,
    description: String
}

pub fn setup_schema() -> Result<(), Error> {

    let conn = Connection::open("./db.sqlite3")?;

    println!("Setting up a DB schema");

    conn.execute(
        "CREATE TABLE note (
            id    INTEGER PRIMARY KEY,
            title  TEXT NOT NULL,
            description  TEXT
        )",
        (), // empty list of parameters.
    )?;

    Ok(())

}
