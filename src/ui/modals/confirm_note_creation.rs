use ratatui::crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::ui::modals::interfaces::ModalDialog;

pub enum CurrentlyCreatingNote {
    Yes,
    No
}

pub struct UiNoteCreationDialog {
    pub creation_state: Option<CurrentlyCreatingNote>,
    pub creation_requested: bool
}

impl ModalDialog for UiNoteCreationDialog {
    fn new() -> Self {
        UiNoteCreationDialog { 
            creation_state: None, 
            creation_requested: false 
        }
    }
    
    fn handle_key_events(&mut self, key: &KeyEvent) {
    }

    fn render(&self, frame: &mut Frame) {
    }
}