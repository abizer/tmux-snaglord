//! TUI rendering logic

use ansi_to_tui::IntoText;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::App;

/// Render the application UI
pub fn render(frame: &mut Frame, app: &mut App) {
    // Split into left (30%) and right (70%) panes
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    render_command_list(frame, app, chunks[0]);
    render_output_pane(frame, app, chunks[1]);
}

/// Render the command list in the left pane
fn render_command_list(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect) {
    let items: Vec<ListItem> = app
        .blocks
        .iter()
        .enumerate()
        .map(|(i, block)| {
            // Use pre-computed clean command (ANSI stripped at parse time)
            let clean_cmd = &block.clean_command;

            // Truncate long commands
            let display = if clean_cmd.len() > 40 {
                format!("{}…", &clean_cmd[..39])
            } else {
                clean_cmd.clone()
            };

            ListItem::new(Line::from(format!("{:3} {}", i + 1, display)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Commands [j/k] "),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

/// Render the output pane on the right
fn render_output_pane(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let title = if let Some(idx) = app.list_state.selected() {
        format!(" Output ({}/{}) [y: copy] ", idx + 1, app.blocks.len())
    } else {
        " Output ".to_string()
    };

    // Convert ANSI escape codes to ratatui styled Text
    let content = if let Some(idx) = app.list_state.selected() {
        if let Some(block) = app.blocks.get(idx) {
            block
                .output
                .as_bytes()
                .into_text()
                .unwrap_or_else(|_| block.output.as_str().into())
        } else {
            "No selection".into()
        }
    } else {
        "Select a command with j/k...".into()
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(title))
        .wrap(Wrap { trim: false })
        .scroll((app.scroll_offset, 0));

    frame.render_widget(paragraph, area);
}
