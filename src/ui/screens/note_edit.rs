use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use rusqlite::{Connection, Error};
use tui_textarea::TextArea;
use crate::ui::screens::interfaces::AppScreenWithDBAccess;
use crate::ui::screens::note_view::NoteViewSignals;

pub struct NoteEditSignals {

}

pub struct NoteEditScreen<'a> {
    pub note_id: u32,
    pub title: String,
    pub content: String,
    content_edit: TextArea<'a>,
    pub signals: NoteEditSignals
}


impl AppScreenWithDBAccess for NoteEditScreen<'_> {
    fn get_title(&self) -> String {
        // TODO: Replace if wanted
        return self.title.clone();
    }

    fn get_status(&self) -> String {
        todo!()
    }

    fn get_hotkey_text(&self) -> String {
        todo!()
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {
        todo!()
    }

    fn render(&self, rect: Rect, frame: &mut Frame) {
        todo!()
    }
}