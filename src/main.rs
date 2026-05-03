use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::{
    env, fs,
    io::{self, stdout},
    path::PathBuf,
};

struct App {
    current_dir: PathBuf,
    entries: Vec<DirEntry>,
    list_state: ListState,
    show_hidden: bool,
    message: String,
}

struct DirEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: u64,
}

impl App {
    fn new() -> io::Result<Self> {
        let current_dir = env::current_dir()?;
        let mut app = App {
            current_dir,
            entries: Vec::new(),
            list_state: ListState::default(),
            show_hidden: false,
            message: String::new(),
        };
        app.refresh()?;
        Ok(app)
    }

    fn refresh(&mut self) -> io::Result<()> {
        self.entries.clear();
        let read_dir = fs::read_dir(&self.current_dir)?;

        for entry in read_dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            let metadata = entry.metadata()?;
            self.entries.push(DirEntry {
                name,
                path: entry.path(),
                is_dir: metadata.is_dir(),
                size: metadata.len(),
            });
        }

        self.entries.sort_by(|a, b| {
            b.is_dir
                .cmp(&a.is_dir)
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });

        if !self.entries.is_empty() {
            if self.list_state.selected().is_none() {
                self.list_state.select(Some(0));
            } else if let Some(i) = self.list_state.selected() {
                if i >= self.entries.len() {
                    self.list_state.select(Some(self.entries.len() - 1));
                }
            }
        } else {
            self.list_state.select(None);
        }

        self.message.clear();
        Ok(())
    }

    fn selected_entry(&self) -> Option<&DirEntry> {
        self.list_state.selected().and_then(|i| self.entries.get(i))
    }

    fn enter_dir(&mut self) -> io::Result<()> {
        if let Some(entry) = self.selected_entry() {
            if entry.is_dir {
                self.current_dir = entry.path.clone();
                self.list_state.select(Some(0));
                self.refresh()?;
            } else {
                let path = entry.path.clone();
                if open::that(&path).is_err() {
                    self.message = "Failed to open file".to_string();
                }
            }
        }
        Ok(())
    }

    fn go_up(&mut self) -> io::Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            let old_dir = self
                .current_dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string());
            self.current_dir = parent.to_path_buf();
            self.refresh()?;
            if let Some(old_name) = old_dir {
                if let Some(pos) = self.entries.iter().position(|e| e.name == old_name) {
                    self.list_state.select(Some(pos));
                }
            }
        }
        Ok(())
    }

    fn move_selection(&mut self, delta: i32) {
        if self.entries.is_empty() {
            return;
        }
        let current = self.list_state.selected().unwrap_or(0) as i32;
        let new = (current + delta).clamp(0, self.entries.len() as i32 - 1) as usize;
        self.list_state.select(Some(new));
    }

    fn delete_selected(&mut self) -> io::Result<()> {
        if let Some(entry) = self.selected_entry() {
            let path = entry.path.clone();
            let is_dir = entry.is_dir;
            if is_dir {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
            self.message = format!("Deleted: {}", path.display());
            self.refresh()?;
        }
        Ok(())
    }

    fn toggle_hidden(&mut self) -> io::Result<()> {
        self.show_hidden = !self.show_hidden;
        self.list_state.select(Some(0));
        self.refresh()
    }
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(f.size());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" 📁 ", Style::default().fg(Color::Yellow)),
        Span::styled(
            app.current_dir.display().to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(Block::default().borders(Borders::ALL).title(" Path "));
    f.render_widget(header, chunks[0]);

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|entry| {
            let icon = if entry.is_dir { "📂" } else { "📄" };
            let size_str = if entry.is_dir {
                "<DIR>".to_string()
            } else {
                format_size(entry.size)
            };
            let style = if entry.is_dir {
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(vec![
                Span::raw(format!(" {} ", icon)),
                Span::styled(format!("{:<40}", entry.name), style),
                Span::styled(
                    format!("{:>10}", size_str),
                    Style::default().fg(Color::DarkGray),
                ),
            ]))
        })
        .collect();

    let file_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Files ({}) ", app.entries.len())),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(file_list, chunks[1], &mut app.list_state);

    let footer_text = if app.message.is_empty() {
        " ↑↓/jk:Navigate | Enter/l:Open | Backspace:Up | h:Hidden | d:Delete | g/G:Top/Bottom | q:Quit "
            .to_string()
    } else {
        app.message.clone()
    };
    let footer = Paragraph::new(Line::from(Span::styled(
        footer_text,
        Style::default().fg(Color::Green),
    )))
    .block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(footer, chunks[2]);
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Up | KeyCode::Char('k') => app.move_selection(-1),
                KeyCode::Down | KeyCode::Char('j') => app.move_selection(1),
                KeyCode::Enter | KeyCode::Char('l') => app.enter_dir()?,
                KeyCode::Backspace => app.go_up()?,
                KeyCode::Char('h') => app.toggle_hidden()?,
                KeyCode::Char('d') => app.delete_selected()?,
                KeyCode::Home | KeyCode::Char('g') => app.list_state.select(Some(0)),
                KeyCode::End | KeyCode::Char('G') => {
                    if !app.entries.is_empty() {
                        app.list_state.select(Some(app.entries.len() - 1));
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
