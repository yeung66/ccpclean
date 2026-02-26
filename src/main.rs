mod cli;
mod filter;
mod killer;
mod process_info;
mod scanner;
mod tui;

use clap::Parser;
use cli::Cli;
use filter::{apply_filter, compute_score, FilterMode};
use tui::{AppState, runner};

fn main() {
    let cli = Cli::parse();

    let mode = if cli.all {
        FilterMode::Loose
    } else {
        FilterMode::Strict
    };

    // Scan processes
    let mut processes = scanner::scan();

    // Compute scores
    for p in &mut processes {
        p.score = compute_score(p);
    }

    // Apply filter
    let mut processes = apply_filter(processes, mode);

    // Apply port filter if specified
    if let Some(port) = cli.port {
        processes.retain(|p| p.ports.contains(&port));
    }

    // Sort by score descending
    processes.sort_by(|a, b| b.score.cmp(&a.score));

    if cli.no_tui {
        // Non-interactive output
        println!("{:<8} {:<12} {:<18} {:<7} {}", "PID", "NAME", "PORTS", "SCORE", "COMMAND");
        println!("{}", "-".repeat(70));
        for p in &processes {
            let cmd = p.cmd.get(1).map(|s| s.as_str()).unwrap_or("");
            println!(
                "{:<8} {:<12} {:<18} {:<7} {}",
                p.pid,
                p.name,
                p.ports_display(),
                p.score,
                cmd
            );
        }
        return;
    }

    if processes.is_empty() {
        println!("No matching processes found.");
        return;
    }

    let mut state = AppState::new(processes);
    state.filter_mode = mode;

    if let Err(e) = runner::run(state) {
        eprintln!("TUI error: {}", e);
        std::process::exit(1);
    }
}
