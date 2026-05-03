# tui-fm

A terminal file manager built with Rust and [ratatui](https://github.com/ratatui/ratatui).

![Rust](https://img.shields.io/badge/rust-1.74%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- Browse directories with vim-style or arrow key navigation
- Open files with system default application
- Toggle hidden files visibility
- Delete files and directories
- File size display with human-readable formatting
- Directories sorted first, then alphabetical

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.74 or later)

## Installation

```bash
git clone https://github.com/muzammilkhattri/tui-filebrowser.git
cd tui-filebrowser
cargo build --release
```

The binary will be at `target/release/tui-fm`.

## Usage

```bash
# Run from project directory
cargo run

# Or run the built binary directly
./target/release/tui-fm
```

## Keybindings

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `Enter` / `l` | Open directory / file |
| `Backspace` | Go to parent directory |
| `h` | Toggle hidden files |
| `d` | Delete selected file/directory |
| `g` | Jump to top |
| `G` | Jump to bottom |
| `q` | Quit |

## Project Structure

```
tui-fm/
├── Cargo.toml
└── src/
    └── main.rs
```

## Dependencies

- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) - Cross-platform terminal manipulation
- [open](https://crates.io/crates/open) - Open files with system default app

## License

MIT
