// TODO: Update to encompass all necessary parts of UI workflow

use std::io;
use std::io::Stdout;
use crossterm::event::{self, Event};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::{Alignment, Backend};
use rusqlite::{Connection, Error};
use crate::ui;
use crate::ui::modals::exit::UiExitModalDialog;
use crate::ui::modals::interfaces::ModalDialog;
use crate::ui::screens::interfaces::AppScreenWithDBAccess;
use crate::ui::screens::note_select::NoteSelectScreen;

pub fn get_note_db_connection() -> rusqlite::Result<Connection, Error> {
    let conn = Connection::open("./notes.sqlite3")?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    Ok(conn)
}

pub enum CurrentScreen {
    SelectNote,
}

pub struct AppScreenState {
    current_screen: CurrentScreen,
    note_select_screen: NoteSelectScreen
}

pub struct App {
    pub notes_db_conn: Connection,
    pub exit_dialog: UiExitModalDialog,
    pub screen_state: AppScreenState,
}

impl App {
    pub fn new() -> Result<App, Error> {

        let app_conn = get_note_db_connection()?;
        let note_select_screen = NoteSelectScreen::new(&app_conn)?;

        Ok(App {
            notes_db_conn: app_conn,
            exit_dialog: UiExitModalDialog::new(),
            screen_state: AppScreenState {
                current_screen: CurrentScreen::SelectNote,
                note_select_screen,
            }
        })
    }

    pub fn handle_events(&mut self, event: Event) {

        self.exit_dialog.handle_events(&event);
        match &self.exit_dialog.exit_state {
            None => {}
            Some(..) => {
                // Exit dialogue will use the event as normal
                // while blocking all other elements from using them
                return;
            }
        }

        match self.screen_state.current_screen {
            // TODO: Find a way to get the current screen from a single function,
            //  rather than using a match for both
            CurrentScreen::SelectNote => {
                self.screen_state.note_select_screen.handle_events(
                    &event,
                    &self.notes_db_conn
                ).expect("Error handling events for note selection screen.");
            }
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
        let screen_rect = content_layout_chunks[0];
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
            .white()
            .style(Style::default());

        let status_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let hotkey_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));

        let title_content: &str;
        let status_content: &str;
        let hotkey_content: &str;

        match self.screen_state.current_screen {

            CurrentScreen::SelectNote => {
                self.screen_state.note_select_screen.render(screen_rect, frame);

                title_content = self.screen_state.note_select_screen.get_title();
                status_content = self.screen_state.note_select_screen.get_status();
                hotkey_content = self.screen_state.note_select_screen.get_hotkey_text();
            }

        }
        let title = Paragraph::new(Text::styled(
            title_content,
            Style::default().add_modifier(Modifier::BOLD),
        )).alignment(Alignment::Center).block(title_block);

        let status_text = Paragraph::new(Text::styled(
            status_content,
            Style::default().add_modifier(Modifier::ITALIC),
        )).block(status_block);

        let hotkey_text = Paragraph::new(Text::styled(
            hotkey_content,
            Style::default().add_modifier(Modifier::BOLD).add_modifier(Modifier::ITALIC),
        )).block(hotkey_block);

        frame.render_widget(title, main_layout_chunks[0]);
        frame.render_widget(status_text, footer_layout_chunks[0]);
        frame.render_widget(hotkey_text, footer_layout_chunks[1]);

        self.exit_dialog.render(frame);
    }
}