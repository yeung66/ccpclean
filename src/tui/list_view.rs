use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph},
    Frame,
};
use crate::filter::{score_display, FilterMode};
use super::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(2)])
        .split(area);

    render_table(f, chunks[0], state);
    render_footer(f, chunks[1], state);
}

fn render_table(f: &mut Frame, area: Rect, state: &AppState) {
    let mode_str = match state.filter_mode {
        FilterMode::Strict => "Strict: dev runtimes only",
        FilterMode::Loose => "Loose: all listening processes",
    };
    let title = format!(" ccpclean  [{}]  Tab=detail view  F=switch filter ", mode_str);

    let header = Row::new(vec![
        Cell::from("  "),
        Cell::from("PID"),
        Cell::from("Name"),
        Cell::from("Ports"),
        Cell::from("Score"),
        Cell::from("Command"),
    ])
    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = state
        .processes
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let is_selected = i == state.selected_index;
            let is_checked = state.checked[i];

            let checkbox = if is_checked { "[x]" } else { "[ ]" };
            let cmd_preview = p.cmd.get(1).map(|s| s.as_str()).unwrap_or("");

            let row_style = if is_selected {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            Row::new(vec![
                Cell::from(checkbox),
                Cell::from(p.pid.to_string()),
                Cell::from(p.name.clone()),
                Cell::from(p.ports_display()),
                Cell::from(score_display(p.score)),
                Cell::from(cmd_preview.to_string()),
            ])
            .style(row_style)
        })
        .collect();

    let widths = [
        Constraint::Length(5),
        Constraint::Length(8),
        Constraint::Length(12),
        Constraint::Length(16),
        Constraint::Length(7),
        Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title(title));

    let mut table_state = TableState::default().with_selected(Some(state.selected_index));
    f.render_stateful_widget(table, area, &mut table_state);
}

fn render_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let msg = if let Some(ref s) = state.status_message {
        s.clone()
    } else {
        " Space=select  A=all  Enter=kill selected  F=switch filter  Tab=detail view  Q=quit".to_string()
    };
    let p = Paragraph::new(msg).style(Style::default().fg(Color::DarkGray));
    f.render_widget(p, area);
}
