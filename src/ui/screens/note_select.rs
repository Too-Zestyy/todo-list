use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use rusqlite::{params, Connection, Error};
use crate::app::App;
use crate::ui::screens::interfaces::{AppScreen, AppScreenWithDBAccess};

const NOTE_PAGE_SIZE: usize = 10;
const NOTE_PAGE_SIZE_U32: u32 = NOTE_PAGE_SIZE as u32;

pub struct NoteSelectScreen {
    current_selection_index: u8, // Only needs to go up to a reasonable number to fit in a single screen
    current_selection_page: u32,
    current_note_page: [Option<NoteOption>; NOTE_PAGE_SIZE],
}

#[derive(Debug)]
pub struct NoteOption {
    note_id: u32,
    note_name: String,
}

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
    fn new(conn: &Connection) -> Result<Self, Error> {

        Ok(
            NoteSelectScreen {
                current_selection_index: 0,
                current_selection_page: 1,
                current_note_page: get_note_title_page(conn, 1)?,
            }
        )
    }

    fn handle_key_events(&mut self, key: &KeyEvent, conn: &Connection) {
        
    }

    fn render(&self, frame: &mut Frame, app: &App) {
        let main_layout_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let content_layout_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
            ]).split(main_layout_chunks[1]);
        // ANCHOR_END: ui_layout

        let footer_layout_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(main_layout_chunks[2]);

        // ANCHOR: title_paragraph
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "To-do List",
            Style::default().add_modifier(Modifier::BOLD),
        )).block(title_block);

        let content = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let status_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        let status_text = Paragraph::new(Text::styled(
            "App Status/Input",
            Style::default().add_modifier(Modifier::ITALIC),
        )).block(status_block);

        let hotkey_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        let hotkey_text = Paragraph::new(Text::styled(
            "Esc: Quit Program",
            Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC),
        )).block(hotkey_block);

        frame.render_widget(title, main_layout_chunks[0]);
        frame.render_widget(content, content_layout_chunks[0]);
        frame.render_widget(status_text, footer_layout_chunks[0]);
        frame.render_widget(hotkey_text, footer_layout_chunks[1]);
    }
}