use crossterm::event::KeyEvent;
use ratatui::Frame;
use rusqlite::Connection;
use crate::app::App;
use crate::ui::screens::interfaces::{AppScreen, AppScreenWithEventDBAccess};

pub struct NoteSelectScreen {
    current_selection_index: u8, // Only needs to go up to a reasonable number to fit in a single screen
    current_selection_page: u32,
    current_note_page: [NoteOption; 10],
}

pub struct NoteOption {
    note_id: u32,
    note_title: String,
}

impl AppScreenWithEventDBAccess for NoteSelectScreen {
    fn new() -> Self {
        todo!()
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &mut Connection) {
        todo!()
    }

    fn render(&self, f: &mut Frame, app: &App) {
        todo!()
    }
}