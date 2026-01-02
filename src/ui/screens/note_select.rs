use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Modifier, Style, Text};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use rusqlite::{params, Connection, Error};
use crate::app::App;
use crate::db::notes::get_note_page_count;
use crate::ui::screens::interfaces::{AppScreen, AppScreenWithDBAccess};

const NOTE_PAGE_SIZE: usize = 10;
const NOTE_PAGE_SIZE_U32: u32 = NOTE_PAGE_SIZE as u32;

pub struct NoteSelectScreen {
    current_selection_index: usize, // Only needs to go up to a reasonable number to fit in a single screen
    currently_selected_page: u32,
    current_note_page: [Option<NoteOption>; NOTE_PAGE_SIZE],
    note_page_count: u32,

    pub signals: NoteSelectSignals
}

pub struct NoteSelectSignals {
    pub note_view_request: Option<u32>
}

#[derive(Debug, Clone)]
pub struct NoteOption {
    note_id: u32,
    note_name: String,
}

const DEFAULT_NOTE: NoteOption = NoteOption {
    note_id: 0,
    note_name: String::new(),
};

pub fn get_note_title_page(conn: &Connection, page: u32) -> Result<[Option<NoteOption>; NOTE_PAGE_SIZE], Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name FROM NOTES LIMIT ?1 OFFSET ?2"
    )?;

    let note_iter = stmt.query_map(
        params![NOTE_PAGE_SIZE_U32, NOTE_PAGE_SIZE_U32 * (page - 1)], |
            row| {
            Ok(
                NoteOption {
                    note_id: row.get(0)?,
                    note_name: row.get(1)?
                }
            )
        })?;

    let mut note_arr: [Option<NoteOption>; NOTE_PAGE_SIZE] = [const { None }; 10];

    let mut note_count = 0;
    for note in note_iter {
        if note_count >= NOTE_PAGE_SIZE {
            panic!("Row count from DB has exceeded expected number of notes for display. Check that the query LIMIT is functioning correctly.");
        }


        note_arr[note_count] = Option::from(note?);

        note_count += 1;
    }
    // params![NOTE_PAGE_SIZE_U32, NOTE_PAGE_SIZE_U32 * (page - 1)]

    Ok(note_arr)

}

impl AppScreenWithDBAccess for NoteSelectScreen {

    fn get_title(&self) -> String {
        "To-do List - Select a note".to_string()
    }

    fn get_status(&self) -> String {
        "Select a note from the list".to_string()
    }

    fn get_hotkey_text(&self) -> String {
        "Esc: Quit | W/S/↑/↓: Select Note".to_string()
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) -> Result<(), Error> {
        if key.kind != KeyEventKind::Press {
            return Ok(())
        }

        // TODO: Remove page count state and check lazily?
        match key.code {
            KeyCode::Char('w') | KeyCode::Up => {
                if self.current_selection_index > 0 {
                    if self.current_note_page[self.current_selection_index - 1].is_none() {
                        return Ok(());
                    }
                    self.current_selection_index -= 1;
                }
                else if self.currently_selected_page > 1 {
                    self.currently_selected_page -= 1;
                    self.current_selection_index = NOTE_PAGE_SIZE - 1;

                    self.current_note_page = get_note_title_page(conn, self.currently_selected_page)?;
                }
            }

            KeyCode::Char('s') | KeyCode::Down => {
                if self.current_selection_index < NOTE_PAGE_SIZE - 1 {
                    if self.current_note_page[self.current_selection_index + 1].is_none() {
                        return Ok(());
                    }
                    self.current_selection_index += 1;
                }
                else {
                    // Update page count to ensure selection is within bounds
                    self.update_page_count(conn)?;

                    // Return to the last available page if we're now out of bounds
                    if self.currently_selected_page >= self.note_page_count {
                        self.currently_selected_page = self.note_page_count;
                    }
                    else {
                        self.currently_selected_page += 1;
                        // Only move the selected item to the first one
                        // when we're moving to the next page in an expected manner
                        self.current_selection_index = 0;
                    }

                    self.current_note_page = get_note_title_page(conn, self.currently_selected_page)?;
                }
            }

            KeyCode::Enter => {
                match &self.current_note_page[self.current_selection_index] {
                    Some(note_option) => {
                        self.signals.note_view_request = Some(note_option.note_id);
                    },
                    // Ignore attempts to select an invalid note
                    None => {}
                }
                // self.note_selection = self.current_note_page[self.current_selection_index];
            }

            _ => {}
        }

        Ok(())

    }

    fn render(&self, rect: Rect, frame: &mut Frame) {
        // frame.render_widget(content, content_layout_chunks[0]);
        self.render_note_titles(rect, frame);
    }

}

impl NoteSelectScreen {
    pub(crate) fn new(conn: &Connection) -> Result<Self, Error> {
        Ok(
            NoteSelectScreen {
                current_selection_index: 0,
                currently_selected_page: 1,
                current_note_page: get_note_title_page(conn, 1)?,
                note_page_count: get_note_page_count(conn, NOTE_PAGE_SIZE_U32)?,

                signals: NoteSelectSignals {
                    note_view_request: None
                }
            }
        )
    }

    fn render_note_titles(&self, rect: Rect, frame: &mut Frame) {
        let note_rects = Layout::default().direction(Direction::Vertical).constraints(
            // Ensure each note as one character of height to display the title
            [const {Constraint::Min(1)}; NOTE_PAGE_SIZE]
        ).split(rect);

        for i in 0..NOTE_PAGE_SIZE {
            match &self.current_note_page[i] {
                None => {}

                Some(NoteOption { note_name, .. }) => {
                    let note_title_block = Block::default().style(Style::default());
                    let mut note_title_text = Paragraph::new(
                        Text::styled(note_name, Style::default())
                    ).wrap(Wrap { trim: false }).block(note_title_block);

                    if self.current_selection_index == i {
                        note_title_text = note_title_text.black().on_white()
                    }

                    frame.render_widget(note_title_text, note_rects[i]);
                }
            }
        }
    }

    fn update_page_count(&mut self, conn: &Connection) -> Result<(), Error> {
        self.note_page_count = get_note_page_count(conn, NOTE_PAGE_SIZE_U32)?;

        Ok(())
    }
}