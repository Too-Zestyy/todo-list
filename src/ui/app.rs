use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::app::App;


/*
    TODO:
     - Modularise UI function to prevent monolithic structure
     - Add interface to structure rendering (struct with methods?)
     - Add functionality to UI using docs
     - Separate UI code to per 'mode'/'window'
 */
pub fn ui(frame: &mut Frame, app: &App) {
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