use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use rusqlite::{Connection, Error};
use crate::app::App;

pub trait AppScreen {

    fn new() -> Self;
    fn handle_events(&mut self, event: &Option<Event>) {
        match event {
            Some(Event::Key(key_event)) => {self.handle_key_events(key_event)}
            _ => {}
        }
    }

    fn handle_key_events(&mut self, key: &KeyEvent);

    fn render(&self, f: &mut Frame, app: &App);
}

pub trait AppScreenWithDBAccess {

    // TODO: Research to better understand Sized constraint
    fn new(conn: &Connection) -> Result<Self, Error> where Self: Sized;

    fn handle_events(&mut self, event: &Event, conn: &Connection) {
        match event {
            Event::Key(key_event) => {self.handle_key_events(key_event, conn)}
            _ => {}
        }
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection);

    fn render(&self, frame: &mut Frame, app: &App);



}