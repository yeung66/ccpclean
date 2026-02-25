# ccpclean Design Document

**Date:** 2026-02-25
**Status:** Approved

## Overview

`ccpclean` is a Rust CLI tool that scans running processes to find orphaned local web services (started by Claude Code, dev tools, or any other source) and lets the user interactively select and kill them via a TUI.

## Goals

- Detect local web services that are still running after their parent session (e.g. Claude Code) has exited
- Support both strict mode (dev runtimes only) and loose mode (all processes listening on local ports)
- Provide an interactive TUI with two views: list-select and two-panel detail
- Support `cargo install` from local and publishing to crates.io (cross-platform)

## Architecture

```
ccpclean/
├── src/
│   ├── main.rs             # Entry point, CLI argument parsing (clap)
│   ├── scanner.rs          # Process scanning and port association
│   ├── filter.rs           # Strict / loose filter logic + confidence scoring
│   ├── tui/
│   │   ├── mod.rs          # TUI init and event loop (ratatui + crossterm)
│   │   ├── list_view.rs    # Checkbox list view
│   │   └── detail_view.rs  # Two-panel detail view
│   └── killer.rs           # Process termination (SIGTERM / Windows TerminateProcess)
├── Cargo.toml
└── README.md
```

### Data Flow

```
System process table + port table
        ↓
    scanner.rs  →  Vec<ProcessInfo>
        ↓
    filter.rs   →  strict / loose filter + scoring
        ↓
    tui/        →  user selection
        ↓
    killer.rs   →  terminate selected processes
```

## Core Data Structure

```rust
struct ProcessInfo {
    pid: u32,
    name: String,
    cmd: Vec<String>,         // Full command line
    ports: Vec<u16>,          // Listening local ports
    start_time: u64,
    memory_kb: u64,
    parent_pid: Option<u32>,
    parent_name: Option<String>,
    is_dev_runtime: bool,     // node / python / deno / bun / ruby / etc.
    score: u8,                // Confidence score 0–100
}
```

## Confidence Scoring

| Condition | Points |
|-----------|--------|
| Process name is node / python / deno / bun / ruby | +30 |
| Port in range 1024–9999 | +20 |
| Command line contains server / dev / run / start / manage | +20 |
| Parent process is claude / bash / zsh / sh | +20 |
| Process uptime > 30 minutes | +10 |

## TUI Design

### List Select View (default)

```
ccpclean v0.1.0  [Strict]  F=toggle loose  Tab=switch view

  PID    Name      Ports          Score  Command
  ─────────────────────────────────────────────────────
  [x] 12345  node      3000, 3001     ●●●●○  node server.js
  [ ] 23456  python    8000           ●●●○○  python manage.py runserver
  [ ] 34567  node      5173           ●●●●●  vite --port 5173
  [ ] 45678  bun       4321           ●●●○○  bun run dev

  Space=toggle  A=select all  Enter=kill selected  Q=quit
```

### Two-Panel Detail View (Tab to switch)

```
ccpclean v0.1.0  [Strict]                           [Detail View]

  Process List          │  Process Detail
  ──────────────────    │  ─────────────────────────────────
  ▶ node    :3000       │  PID:         12345
    python  :8000       │  Name:        node
    node    :5173       │  Ports:       3000, 3001
    bun     :4321       │  Command:     node server.js --watch
                        │  Started:     2h 13m ago
                        │  Memory:      87.4 MB
                        │  Parent:      bash (PID 11111)
                        │  Confidence:  ●●●●○ High
                        │  ─────────────────────────────────
                        │  [Enter] Kill   [Q] Quit
```

## CLI Options

```
ccpclean [OPTIONS]

OPTIONS:
  -s, --strict      Strict mode: dev runtimes only (default)
  -a, --all         Loose mode: all processes listening on local ports
  -p, --port <PORT> Filter by specific port
  --no-tui          Non-interactive mode, print list and exit
  -h, --help        Show help
  -V, --version     Show version
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `sysinfo` | Cross-platform process info (PID, name, cmd, memory, parent, start time) |
| `ratatui` | TUI framework |
| `crossterm` | Cross-platform terminal backend for ratatui |
| `clap` | CLI argument parsing |

### Cross-Platform Port Scanning

- **Windows:** `sysinfo` + Windows API `GetExtendedTcpTable` via `windows` or `winapi` crate
- **Linux/macOS:** Parse `/proc/net/tcp` or use `sysinfo`

## Error Handling

- Cannot read process info (permission denied) → skip process, show notice at bottom
- Kill fails (permission denied) → show red error in TUI, do not exit
- Windows: prompt user to run as administrator if needed

## Installation

```bash
# Local build
cargo install --path .

# From crates.io (after publishing)
cargo install ccpclean
```
