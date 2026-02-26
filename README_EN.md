# ccpclean

[中文](./README.md)

A terminal tool to find and kill orphaned local dev servers — Node, Python, Deno, Bun, Ruby processes left running after your dev session (Claude Code, VS Code, etc.) has ended.

Built with Rust. Cross-platform (Windows, macOS, Linux). Interactive TUI with two views.

## The Problem

You run `npm run dev` or `python manage.py runserver` inside Claude Code, a terminal, or your IDE. You close the session, but the process keeps running in the background, holding onto ports like 3000, 5173, 8000...

`ccpclean` scans your system for these orphaned services, scores how likely they are to be leftover dev servers, and lets you kill them interactively.

## Install

```bash
cargo install ccpclean
```

Or build from source:

```bash
git clone https://github.com/yeung66/ccpclean.git
cd ccpclean
cargo install --path .
```

## Usage

```bash
# Launch interactive TUI (default: strict mode, dev runtimes only)
ccpclean

# Show all processes listening on local ports
ccpclean --all

# Filter by a specific port
ccpclean --port 3000

# Non-interactive mode (for scripts or quick checks)
ccpclean --no-tui
```

## TUI Views

### List View (default)

Select multiple processes with checkboxes, then kill them in batch.

```
 ccpclean  [Strict: dev runtimes only]  Tab=detail view  F=switch filter
+------+-------+----------+------------+-------+------------+
|      | PID   | Name     | Ports      | Score | Command    |
+------+-------+----------+------------+-------+------------+
| [x]  | 12345 | node     | 3000, 3001 | ****- | server.js  |
| [ ]  | 23456 | python   | 8000       | ***-- | manage.py  |
| [ ]  | 34567 | node     | 5173       | ***** | vite.js    |
+------+-------+----------+------------+-------+------------+
 Space=select  A=all  Enter=kill  F=switch filter  Tab=detail  Q=quit
```

### Detail View (Tab to switch)

Browse processes one by one with full details: PID, command, uptime, memory, parent process, and confidence score.

```
 Process List          Process Detail
+--------------------+----------------------------------+
| > node    :3000    | PID:     12345                   |
|   python  :8000    | Name:    node                    |
|   node    :5173    | Ports:   3000, 3001              |
|                    | Command: node server.js --watch  |
|                    | Started: 2h 13m ago              |
|                    | Memory:  87.4 MB                 |
|                    | Parent:  bash (PID 11111)        |
|                    | Score:   ****- High              |
+--------------------+----------------------------------+
```

## Keybindings

| Key | Action |
|-----|--------|
| `↑` / `↓` or `j` / `k` | Navigate process list |
| `Space` | Select / deselect a process |
| `A` | Select / deselect all |
| `Enter` | Kill selected processes (list view) or current process (detail view) |
| `Tab` | Switch between list view and detail view |
| `F` | Switch filter: **Strict** (dev runtimes only) ↔ **Loose** (all listening processes) |
| `Q` / `Esc` | Quit |

## Filter Modes

| Mode | What it shows |
|------|---------------|
| **Strict** (default) | Only dev runtimes: `node`, `python`, `deno`, `bun`, `ruby`, `java`, etc. |
| **Loose** (`--all` or `F`) | All processes listening on any local port, including system services |

## Confidence Score

Each process gets a 0–100 score indicating how likely it is to be an orphaned dev server:

| Condition | Points |
|-----------|--------|
| Process is a dev runtime (node, python, deno, bun, ruby...) | +30 |
| Listening on port 1024–9999 | +20 |
| Command contains dev keywords (server, dev, start, watch...) | +20 |
| Parent process is a shell (bash, zsh, sh, pwsh, claude...) | +20 |
| Running for more than 30 minutes | +10 |

The score is displayed as filled dots: `****-` = 80/100.

## CLI Reference

```
ccpclean [OPTIONS]

Options:
  -a, --all          Loose mode: show all processes listening on local ports
  -p, --port <PORT>  Filter by specific port
      --no-tui       Non-interactive: print list and exit
  -h, --help         Show help
  -V, --version      Show version
```

## Requirements

- Rust 1.70+ (for building)
- On Windows: may need administrator privileges to see all port-to-process mappings and to kill certain processes

## License

MIT
