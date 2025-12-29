// TODO: Update to encompass all necessary parts of UI workflow

use std::io;
use std::io::Stdout;
use crossterm::event::{self, Event};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Backend;
use rusqlite::{Connection, Error};
use crate::ui;
use crate::ui::modals::exit::UiExitModalDialog;
use crate::ui::modals::interfaces::ModalDialog;

pub fn get_note_db_connection() -> rusqlite::Result<Connection, Error> {
    let conn = Connection::open("./notes.sqlite3")?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    Ok(conn)
}

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}


pub enum CurrentlyExiting {
    Yes,
    No,
}

pub enum CurrentlyEditing {
    NoteSelection,
    Details,
    Tags,
}


pub struct App {
    pub notes_db_conn: Connection,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub exit_dialog: UiExitModalDialog,
}

impl App {
    pub fn new() -> Result<App, Error> {
        Ok(App {
            notes_db_conn: get_note_db_connection()?,
            current_screen: CurrentScreen::Main,
            exit_dialog: UiExitModalDialog::new(),
        })
    }

    pub fn handle_events(&mut self, event: Event) {
        if let Event::Key(key) = event {

            self.exit_dialog.handle_key_events(&key);

        }
    }

    pub fn get_current_ui(&self, frame: &mut Frame) {
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

        self.exit_dialog.render(frame, &self);
    }
}