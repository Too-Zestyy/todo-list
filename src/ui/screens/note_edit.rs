use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use rusqlite::{Connection, Error};
use tui_textarea::{Input, Key, TextArea};
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
        return "Editing note.".to_string()
    }

    fn get_hotkey_text(&self) -> String {
        return "TODO".to_string() 
    }

    fn handle_events(&mut self, event: &Event, conn: &Connection) -> Result<(), Error> {

        // let input: Input = Input::from(event.clone());

        match event.clone().into() {
            Input { key: Key::Esc, .. } => {},
            input => {
                self.content_edit.input(input);
            }
        }

        Ok(())
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {
        todo!()
    }

    // fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {
    //
    //
    //
    //     match key {
    //         Input { key: Key::Esc, .. } => {}
    //
    //         input => {
    //             let _ = self.content_edit.input(input);
    //         }
    //     }
    //
    //     Ok(())
    // }

    fn render(&self, rect: Rect, frame: &mut Frame) {
        todo!()
    }
}