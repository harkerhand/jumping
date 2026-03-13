use crate::app::App;
use crate::entry::Entry;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(f.area());

    let selected_style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::BOLD);

    fn entries_to_items(entries: &[Entry]) -> Vec<ListItem<'_>> {
        entries
            .iter()
            .map(|e| {
                let icon = if e.is_dir { " " } else { "󰈔 " };
                let style = if e.is_dir {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!("{}{}", icon, e.name)).style(style)
            })
            .collect()
    }

    let left_items = entries_to_items(&app.left_entries);
    let left_block = Block::default().borders(Borders::ALL).title(" PARENT  ");
    let left_list = List::new(left_items)
        .block(left_block)
        .highlight_style(selected_style)
        .highlight_symbol("");
    f.render_stateful_widget(left_list, chunks[0], &mut app.left_state);

    let center_items = entries_to_items(&app.center_entries);
    let center_title = format!(" CURRENT: {} ↕ ", app.current_path.display());
    let center_block = Block::default()
        .borders(Borders::ALL)
        .title(center_title)
        .border_style(Style::default().fg(Color::Yellow));
    let center_list = List::new(center_items)
        .block(center_block)
        .highlight_style(selected_style)
        .highlight_symbol(">> ");
    f.render_stateful_widget(center_list, chunks[1], &mut app.center_state);

    let right_entries = app.get_right_entries();
    let right_items = entries_to_items(&right_entries);
    let right_block = Block::default().borders(Borders::ALL).title(" PREVIEW  ");
    let right_list = List::new(right_items).block(right_block);
    f.render_widget(right_list, chunks[2]);
}
