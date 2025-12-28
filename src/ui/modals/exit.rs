use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Modifier, Style, Text};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::App;
use crate::ui::modals::interface::ModalDialog;

pub enum CurrentlyExiting {
    Yes,
    No,
}

pub struct UiExitModalDialog {
    exit_state: Option<CurrentlyExiting>,
    pub exit_requested: bool,
}

impl ModalDialog for UiExitModalDialog {

    fn new() -> Self {
        UiExitModalDialog {
            exit_state: None,
            exit_requested: false,
        }
    }
    fn handle_key_events(self: &mut Self, key: &KeyEvent) {
        if key.kind == KeyEventKind::Press {
            match self.exit_state {
                None => {
                    if key.code == event::KeyCode::Esc {
                        self.exit_state = Option::from(CurrentlyExiting::No);
                    }
                }
                Some(CurrentlyExiting::Yes) => {
                    if key.code == event::KeyCode::Enter {
                        return self.exit_requested = true;
                    }
                    if key.code == event::KeyCode::Char('d') || key.code == event::KeyCode::Right {
                        self.exit_state = Option::from(CurrentlyExiting::No);
                    }
                }

                Some(CurrentlyExiting::No) => {
                    if key.code == event::KeyCode::Enter {
                        self.exit_state = None;
                    }
                    if key.code == event::KeyCode::Char('a') || key.code == event::KeyCode::Left {
                        self.exit_state = Option::from(CurrentlyExiting::Yes);
                    }
                }
            }
        }
    }

    fn render(&self, frame: &mut Frame, app: &App) {
        // Only render when the dialog is active (i.e the exit state contains a selected option)
        if self.exit_state.is_none() {
            return;
        }

        let vert_layout_chunks = Layout::default().direction(Direction::Vertical).constraints([
            Constraint::Fill(1),
            Constraint::Percentage(33),
            Constraint::Fill(1),
        ]).split(frame.area());

        let modal_layout_chunks = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ]).split(vert_layout_chunks[1]);

        // Modal takes the centre chunk within the frame
        let modal_chunk = modal_layout_chunks[1];

        let modal_block = Block::default().borders(Borders::ALL).on_black().title("Exit App?");

        let entry_chunks = Layout::default().constraints([Constraint::Fill(1)]).margin(1).split(modal_chunk);
        let entry_rect = entry_chunks[0];
        let entry_block = Block::default().title("Entry");

        let option_chunks = Layout::default().direction(Direction::Horizontal).constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ]).split(entry_rect);

        let yes_rect = option_chunks[0];
        let yes_opt = Block::default().borders(Borders::ALL).title("Yes");
        let mut yes_opt_with_text = Paragraph::new(Text::styled("Exit the program and return to the terminal.",
                                                                Style::default().add_modifier(Modifier::ITALIC))).block(yes_opt).wrap(Wrap { trim: false });

        let no_rect = option_chunks[1];
        let no_opt = Block::default().borders(Borders::ALL).title("No");
        let mut no_opt_with_text = Paragraph::new(Text::styled("Cancel this action and return to the program.",
                                                               Style::default().add_modifier(Modifier::ITALIC))).block(no_opt).wrap(Wrap { trim: false });

        match self.exit_state {
            Some(CurrentlyExiting::Yes) => { yes_opt_with_text = yes_opt_with_text.black().on_light_red();}
            Some(CurrentlyExiting::No) => {no_opt_with_text = no_opt_with_text.black().on_light_yellow();}
            _ => {}
        }

        frame.render_widget(modal_block, modal_chunk);
        frame.render_widget(entry_block, entry_rect);

        frame.render_widget(yes_opt_with_text, yes_rect);
        frame.render_widget(no_opt_with_text, no_rect);
    }
}