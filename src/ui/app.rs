use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use crate::app::{App, CurrentlyExiting};
use crate::app::CurrentlyExiting::Yes;
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

    render_current_modal(app, frame);
}

fn render_current_modal(app: &App, frame: &mut Frame) {
    match app.currently_exiting {
        None => {return;}
        // Render the exit modal if a value is available
        _ => {
            render_exiting_modal(app, frame);
        }
    }
}

fn render_exiting_modal(app: &App, frame: &mut Frame) {
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

    match app.currently_exiting {
        Some(CurrentlyExiting::Yes) => { yes_opt_with_text = yes_opt_with_text.black().on_light_red();}
        Some(CurrentlyExiting::No) => {no_opt_with_text = no_opt_with_text.black().on_light_yellow();}
        _ => {}
    }



    frame.render_widget(modal_block, modal_chunk);
    frame.render_widget(entry_block, entry_rect);

    frame.render_widget(yes_opt_with_text, yes_rect);
    frame.render_widget(no_opt_with_text, no_rect);
}