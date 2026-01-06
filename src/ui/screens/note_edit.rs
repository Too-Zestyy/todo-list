use std::borrow::Cow;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders};
use rusqlite::{Connection, Error};
use tui_textarea::{CursorMove, Input, Key, TextArea};
use crate::db::notes::get_note_details;
use crate::ui::screens::interfaces::AppScreenWithDBAccess;
use crate::ui::screens::note_view::NoteViewSignals;

pub struct NoteEditSignals {
    pub exit_requested: bool,
}

pub struct NoteEditScreen<'a> {
    pub note_id: u32,
    pub title: String,
    content_edit: TextArea<'a>,
    pub signals: NoteEditSignals
}


impl NoteEditScreen<'_> {
    pub fn new(connection: &Connection, id: u32) -> Result<NoteEditScreen<'static>, Error> {
        let mut screen = NoteEditScreen::default();

        screen.set_note(connection, id)?;
        Ok(screen)
    }

    pub fn default() -> NoteEditScreen<'static> {
        let mut screen = NoteEditScreen {
            note_id: 0,
            title: "".to_string(),
            content_edit: TextArea::default(),
            signals: NoteEditSignals {
                exit_requested: false,
            },
        };
        screen.content_edit.set_block(
            Block::default().borders(Borders::ALL).white().title("Content"),
        );

        screen
    }

    pub fn set_note(&mut self, connection: &Connection, note_id: u32) -> Result<(), Error> {
        // TODO: Replace with function that gets less data?
        let note_details = get_note_details(connection, note_id)?;

        self.note_id = note_id;
        self.title = note_details.name;
        self.set_content(note_details.description);


        Ok(())
    }

    pub fn set_content(&mut self, content: String) {
        self.clear_content();
        self.content_edit.insert_str(content);
    }

    // https://github.com/rhysd/tui-textarea/pull/113/commits/bbc9f0449f9940a935eeec1aa99ca93583be2e8e
    pub fn clear_content(&mut self) -> bool {
        if self.content_edit.is_empty() {
            return false;
        }

        // Get all lines in the text area
        let all_lines = self.content_edit.lines();
        let summed_up_chars_new_lines: usize = all_lines
            // Loop through all lines
            .iter()
            // Assign a value of 1 to each char
            .map(|line| line.chars().map(|_| 1usize).sum::<usize>())
            // Sum all chars counted with assigned value
            .sum();
        // Newlines are represented as separate lines as opposed to a newline char,
        // so are summed separately by adding the number of lines within the text area
        let all_chars = all_lines.len() + summed_up_chars_new_lines;
        self.content_edit.move_cursor(CursorMove::Jump(0, 0));
        self.content_edit.delete_str(all_chars);

        // Internal implementation creates a new history object, so this effectively
        // clears the history to prevent the ability to undo to the contents of a previous note
        self.content_edit.set_max_histories(50);

        true
    }
}


impl AppScreenWithDBAccess for NoteEditScreen<'_> {
    fn get_title(&self) -> String {
        // TODO: Replace if wanted
        self.title.clone()
    }

    fn get_status(&self) -> String {
        return "Editing note.".to_string()
    }

    fn get_hotkey_text(&self) -> String {
        return "TODO".to_string() 
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {

        match key.clone().into() {
            Input { key: Key::Esc, .. } => {self.signals.exit_requested = true;},
            input => {
                self.content_edit.input(input);
            }
        }

        Ok(())
    }

    fn render(&self, rect: Rect, frame: &mut Frame) {
        frame.render_widget(&self.content_edit, rect);
    }
}