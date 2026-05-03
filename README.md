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

## Installation

### Download pre-built binary (no Rust required)

Download the latest binary for your platform from [Releases](https://github.com/muzammilkhattri/tui-filebrowser/releases).

| Platform | Binary |
|----------|--------|
| Linux (x64) | `tui-fm-linux-amd64` |
| Linux (ARM64) | `tui-fm-linux-arm64` |
| macOS (Intel) | `tui-fm-macos-amd64` |
| macOS (Apple Silicon) | `tui-fm-macos-arm64` |
| Windows (x64) | `tui-fm-windows-amd64.exe` |

```bash
# Example: macOS Apple Silicon
chmod +x tui-fm-macos-arm64
mv tui-fm-macos-arm64 /usr/local/bin/tui-fm
```

### Build from source

Requires [Rust](https://www.rust-lang.org/tools/install) (1.74 or later).

```bash
git clone https://github.com/muzammilkhattri/tui-filebrowser.git
cd tui-filebrowser
cargo build --release
cargo install --path .
```

## Usage

```bash
# Build the project
cargo build --release

# Run the file manager
cargo run --release
```

To install system-wide:

```bash
cargo install --path .
```

Then run `tui-fm` from anywhere.

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
