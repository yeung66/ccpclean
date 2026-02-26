use std::io;
use std::time::Duration;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use crate::killer;
use super::{AppState, View, list_view, detail_view};

pub fn run(mut state: AppState) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, &mut state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.area();
            match state.view {
                View::List => list_view::render(f, area, state),
                View::Detail => detail_view::render(f, area, state),
            }
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                state.status_message = None;

                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        state.should_quit = true;
                        break;
                    }
                    KeyCode::Tab => state.switch_view(),
                    KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                    KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                    KeyCode::Char(' ') => state.toggle_checked(),
                    KeyCode::Char('a') | KeyCode::Char('A') => state.select_all(),
                    KeyCode::Char('f') | KeyCode::Char('F') => {
                        use crate::filter::FilterMode;
                        state.filter_mode = match state.filter_mode {
                            FilterMode::Strict => FilterMode::Loose,
                            FilterMode::Loose => FilterMode::Strict,
                        };
                    }
                    KeyCode::Enter => {
                        let pids = state.checked_pids();
                        if pids.is_empty() {
                            if state.view == View::Detail {
                                if let Some(p) = state.current_process() {
                                    let pid = p.pid;
                                    handle_kill(state, pid);
                                }
                            } else {
                                state.status_message = Some(" No processes selected (use Space to check)".to_string());
                            }
                        } else {
                            let mut errors = vec![];
                            for pid in pids {
                                if let Err(e) = killer::kill(pid) {
                                    errors.push(e.to_string());
                                }
                            }
                            if errors.is_empty() {
                                state.status_message = Some(" Killed selected processes.".to_string());
                                state.checked.iter_mut().for_each(|c| *c = false);
                            } else {
                                state.status_message = Some(format!(" Errors: {}", errors.join("; ")));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn handle_kill(state: &mut AppState, pid: u32) {
    match killer::kill(pid) {
        Ok(_) => {
            state.status_message = Some(format!(" Killed PID {}.", pid));
            state.checked.iter_mut().for_each(|c| *c = false);
        }
        Err(e) => {
            state.status_message = Some(format!(" {}", e));
        }
    }
}
