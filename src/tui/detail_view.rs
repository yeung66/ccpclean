use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use crate::filter::{score_display, FilterMode};
use super::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(40)])
        .split(area);

    render_process_list(f, chunks[0], state);
    render_detail_panel(f, chunks[1], state);
}

fn render_process_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .processes
        .iter()
        .map(|p| {
            let label = format!("{:<10} :{}", p.name, p.ports_display());
            ListItem::new(label)
        })
        .collect();

    let mode_str = match state.filter_mode {
        FilterMode::Strict => "Strict: dev only",
        FilterMode::Loose => "Loose: all",
    };

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" [{}] Tab=list ", mode_str)))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");

    let mut list_state = ListState::default().with_selected(Some(state.selected_index));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_detail_panel(f: &mut Frame, area: Rect, state: &AppState) {
    let content = match state.current_process() {
        None => "No process selected".to_string(),
        Some(p) => {
            let parent_str = match (&p.parent_name, p.parent_pid) {
                (Some(name), Some(pid)) => format!("{} (PID {})", name, pid),
                _ => "unknown".to_string(),
            };
            format!(
                "PID:        {}\nName:       {}\nPorts:      {}\nCommand:    {}\nStarted:    {}\nMemory:     {}\nParent:     {}\nConfidence: {} {}\n\n[Enter] Kill   [Q] Quit",
                p.pid,
                p.name,
                p.ports_display(),
                p.cmd.join(" "),
                p.uptime_display(),
                p.memory_display(),
                parent_str,
                score_display(p.score),
                if p.score >= 70 { "High" } else if p.score >= 40 { "Medium" } else { "Low" },
            )
        }
    };

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title(" Process Detail "));
    f.render_widget(paragraph, area);
}
