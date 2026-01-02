use ratatui::widgets::{Block, Wrap};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use rusqlite::{Connection, Error};
use crate::db::notes::get_note_details;
use crate::ui::screens::interfaces::AppScreenWithDBAccess;

pub struct NoteViewSignals {
    pub exit_requested: bool,
}

// TODO: Add tags
pub struct NoteViewScreen {
    pub note_id: u32,
    pub title: String,
    pub content: String,
    pub date_created: String,
    pub last_updated: String,
    pub signals: NoteViewSignals
}

impl AppScreenWithDBAccess for NoteViewScreen {
    fn get_title(&self) -> String {
        "Note View".to_string()
    }

    fn get_status(&self) -> String {
        format!("Created: {} | Last Updated: {}", self.date_created, self.last_updated)
    }

    fn get_hotkey_text(&self) -> String {
        "Esc: Return to selection menu".to_string()
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {
        if key.kind != KeyEventKind::Press {
            return Ok(())
        }

        match key.code {
            KeyCode::Esc => {
                self.signals.exit_requested = true;
            }

            _ => {}
        }

        Ok(())
    }

    fn render(&self, rect: Rect, frame: &mut Frame) {
        let content_block = Block::default().on_black();

        let note_content = Paragraph::new(
            Text::styled(&self.content, Style::default().white())
        ).wrap(Wrap { trim: false }).block(content_block);

        frame.render_widget(note_content, rect);
    }
}

impl NoteViewScreen {
    pub fn new(conn: &Connection, note_id: u32) -> Result<Self, Error> {

        let note_details = get_note_details(conn, note_id)?;

        Ok(NoteViewScreen {
            note_id: note_details.id,
            title: note_details.name,
            content: note_details.description,
            date_created: note_details.date_created_utc,
            last_updated: note_details.last_updated_utc,
            signals: NoteViewSignals {
                exit_requested: false,
            }
        })
    }
}