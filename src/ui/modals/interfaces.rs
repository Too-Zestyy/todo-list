use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use crate::app::App;

pub trait ModalDialog {

    fn new() -> Self;
    fn handle_events(self: &mut Self, event: &Option<Event>) {
        match event {
            Some(Event::Key(key_event)) => {self.handle_key_events(key_event)}
            _ => {}
        }
    }

    fn handle_key_events(self: &mut Self, key: &KeyEvent);

    fn render(&self, f: &mut Frame, app: &App);
}