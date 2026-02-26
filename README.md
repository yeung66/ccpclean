# ccpclean

Scan and clean up orphaned local web service processes — e.g. Node, Python, Deno, Bun servers left running after your dev session ends.

## Install

```bash
cargo install ccpclean
```

## Usage

```bash
# Interactive TUI (strict mode: dev runtimes only)
ccpclean

# Show all listening processes (loose mode)
ccpclean --all

# Filter by port
ccpclean --port 3000

# Non-interactive list
ccpclean --no-tui
```

## TUI Keybindings

| Key | Action |
|-----|--------|
| Up/Down or j/k | Navigate |
| Space | Toggle selection |
| A | Select / deselect all |
| Enter | Kill selected processes |
| Tab | Switch between list and detail view |
| F | Toggle strict / loose filter mode |
| Q / Esc | Quit |

## License

MIT
