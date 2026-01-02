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
use crate::db::schema::create_note_db_schema;
use crate::ui;
use crate::ui::modals::exit::UiExitModalDialog;
use crate::ui::modals::interfaces::ModalDialog;
use crate::ui::screens::interfaces::AppScreenWithDBAccess;
use crate::ui::screens::note_select::NoteSelectScreen;
use crate::ui::screens::note_view::{NoteViewScreen, NoteViewSignals};

pub fn get_note_db_connection() -> rusqlite::Result<Connection, Error> {
    let conn = Connection::open("./notes.sqlite3")?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    Ok(conn)
}

pub enum CurrentScreen {
    SelectNote,
    ViewNote
}

pub struct AppScreenState {
    current_screen: CurrentScreen,
    note_select_screen: NoteSelectScreen,
    note_view_screen: NoteViewScreen
}

pub struct App {
    pub notes_db_conn: Connection,
    pub exit_dialog: UiExitModalDialog,
    pub screen_state: AppScreenState,
}

impl App {
    pub fn new() -> Result<App, Error> {

        let app_conn = get_note_db_connection()?;
        create_note_db_schema(&app_conn)?;

        let note_select_screen = NoteSelectScreen::new(&app_conn)?;
        // Only load from DB when explicitly selected
        let note_view_screen = NoteViewScreen {
            note_id: 0,
            title: "".to_string(),
            content: "".to_string(),
            signals: NoteViewSignals {
                exit_requested: false,
            },
        };

        Ok(App {
            notes_db_conn: app_conn,
            exit_dialog: UiExitModalDialog::new(),
            screen_state: AppScreenState {
                current_screen: CurrentScreen::SelectNote,
                note_select_screen,
                note_view_screen,
            }
        })
    }

    pub fn handle_events(&mut self, event: Event) {

        match self.screen_state.current_screen {
            // TODO: Find a way to get the current screen from a single function,
            //  rather than using a match for both
            CurrentScreen::SelectNote => {

                let exit_dialog_open = self.exit_dialog.exit_state.is_some();
                self.exit_dialog.handle_events(&event);
                match &self.exit_dialog.exit_state {
                    None => {}
                    Some(..) => {
                        // Exit dialogue will use the event as normal
                        // while blocking all other elements from using them
                        return;
                    }
                }
                // Prevent the enter from saying no to the dialog pass to the main screen
                if exit_dialog_open {
                    return;
                }

                self.screen_state.note_select_screen.handle_events(
                    &event,
                    &self.notes_db_conn
                ).expect("Error handling events for note selection screen.");

                match self.screen_state.note_select_screen.signals.note_view_request {
                    Some(requested_note_id) => {
                        self.screen_state.note_view_screen = NoteViewScreen::new(
                            &self.notes_db_conn,
                            requested_note_id
                        ).unwrap_or(NoteViewScreen {
                            note_id: 0,
                            title: "Invalid Note".to_string(),
                            content: "The note could not be opened. Please try again or restart the app.".to_string(),
                            signals: NoteViewSignals {
                                exit_requested: false,
                            },
                        });

                        self.screen_state.current_screen = CurrentScreen::ViewNote;
                        self.screen_state.note_select_screen.signals.note_view_request = None;
                    },

                    None => {}
                }
            },
            // TODO: Add graceful error handling
            CurrentScreen::ViewNote => {
                self.screen_state.note_view_screen
                    .handle_events(&event, &self.notes_db_conn)
                    .expect("Error handling events for note view.");


                if self.screen_state.note_view_screen.signals.exit_requested {
                    self.screen_state.current_screen = CurrentScreen::SelectNote;
                    self.screen_state.note_view_screen.signals.exit_requested = false;
                }
            },
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
            },
            CurrentScreen::ViewNote => {
                self.screen_state.note_view_screen.render(screen_rect, frame);

                title_content = self.screen_state.note_view_screen.get_title();
                status_content = self.screen_state.note_view_screen.get_status();
                hotkey_content = self.screen_state.note_view_screen.get_hotkey_text();
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
            Style::default().add_modifier(Modifier::BOLD),
        )).block(hotkey_block);

        frame.render_widget(title, main_layout_chunks[0]);
        frame.render_widget(status_text, footer_layout_chunks[0]);
        frame.render_widget(hotkey_text, footer_layout_chunks[1]);

        self.exit_dialog.render(frame);
    }
}