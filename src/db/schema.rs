use rusqlite::{Connection, Error};

pub fn create_note_db_schema(conn: &Connection) -> Result<(), Error> {

    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS notes
        (
            id               INTEGER PRIMARY KEY,
            name             TEXT NOT NULL,
            description      TEXT,
            date_created_utc TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            last_updated_utc TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        CREATE TRIGGER IF NOT EXISTS update_notes_updated_at
        AFTER UPDATE ON notes
        WHEN old.last_updated_utc <> current_timestamp
        BEGIN
            UPDATE notes
            SET last_updated_utc = CURRENT_TIMESTAMP
            WHERE id = old.id;
        END;

        CREATE TABLE IF NOT EXISTS category_tags
        (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            description TEXT
        );

        CREATE TABLE IF NOT EXISTS applied_note_tags
        (
            note_id      INTEGER NOT NULL,
            category_id  INTEGER NOT NULL,

            FOREIGN KEY(note_id) REFERENCES notes(id),
            FOREIGN KEY(category_id) REFERENCES category_tags(id)
        );
        "#,
    )?;



    Ok(())
}
