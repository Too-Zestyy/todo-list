use crossterm::event::{Event, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
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
    // fn new(conn: &Connection) -> Result<Self, Error> where Self: Sized;

    fn get_title(&self) -> &str;
    fn get_status(&self) -> &str;
    fn get_hotkey_text(&self) -> &str;

    fn handle_events(&mut self, event: &Event, conn: &Connection) -> Result<(), Error> {
        match event {
            Event::Key(key_event) => {self.handle_key_events(key_event, conn)}
            _ => {Ok(())}
        }
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error>;

    fn render(&self, rect: Rect, frame: &mut Frame);



}