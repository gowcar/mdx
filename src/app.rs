use std::io::{self, stdout};
use std::path::PathBuf;

use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers, MouseButton,
    MouseEventKind,
};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::ExecutableCommand;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::text::Text;

use crate::config::Config;
use crate::event::{AppEvent, EventHandler};
use crate::render::render_markdown;
use crate::scroll::Viewport;
use crate::search::SearchState;
use crate::selection::{self, Selection};
use crate::theme::Theme;
use crate::ui;

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Search,
    Help,
}

pub struct App {
    pub rendered: Text<'static>,
    pub viewport: Viewport,
    pub theme: Theme,
    pub file_path: PathBuf,
    pub should_quit: bool,
    pub mode: AppMode,
    pub search: SearchState,
    pub selection: Selection,
    pub pending_g: bool,
    pub status_message: Option<String>,
    pub raw_content: String,
    pub render_width: u16,
    pub nerd_font: bool,
    pub wrap_code: bool,
}

impl App {
    pub fn new(content: &str, file_path: PathBuf, theme: Theme, width: u16) -> Self {
        let wrap_code = true;
        let rendered = render_markdown(content, width.saturating_sub(4), &theme, wrap_code);
        let content_height = rendered.lines.len() as u16;

        let mut viewport = Viewport::new();
        viewport.content_height = content_height;

        Self {
            rendered,
            viewport,
            theme,
            file_path,
            should_quit: false,
            mode: AppMode::Normal,
            search: SearchState::new(),
            selection: Selection::new(),
            pending_g: false,
            status_message: None,
            raw_content: content.to_string(),
            render_width: width,
            nerd_font: crate::theme::has_nerd_font(),
            wrap_code,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        // Clear status message on any key
        self.status_message = None;

        match self.mode {
            AppMode::Search => self.handle_search_key(key),
            AppMode::Help => self.handle_help_key(key),
            AppMode::Normal => self.handle_normal_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) {
        // If there's a selection and user presses 'y', copy and clear
        if self.selection.has_selection() {
            match key.code {
                KeyCode::Char('y') => {
                    self.copy_selection();
                    return;
                }
                KeyCode::Esc => {
                    self.selection.clear();
                    return;
                }
                _ => {
                    // Any other key clears selection
                    self.selection.clear();
                }
            }
        }

        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                // If there's a selection, copy instead of quit
                if self.selection.has_selection() {
                    self.copy_selection();
                    return;
                }
                self.should_quit = true;
            }

            // Scroll
            KeyCode::Char('j') | KeyCode::Down => self.viewport.scroll_down(1),
            KeyCode::Char('k') | KeyCode::Up => self.viewport.scroll_up(1),

            // Half page
            KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.viewport.page_down();
            }
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.viewport.page_up();
            }

            // Full page
            KeyCode::Char('f') => self.viewport.full_page_down(),
            KeyCode::Char(' ') => self.viewport.full_page_down(),
            KeyCode::Char('b') => self.viewport.full_page_up(),
            KeyCode::PageDown => self.viewport.full_page_down(),
            KeyCode::PageUp => self.viewport.full_page_up(),

            // Top/bottom
            KeyCode::Char('g') => {
                if self.pending_g {
                    self.viewport.go_top();
                    self.pending_g = false;
                } else {
                    self.pending_g = true;
                    return;
                }
            }
            KeyCode::Char('G') => self.viewport.go_bottom(),
            KeyCode::Home => self.viewport.go_top(),
            KeyCode::End => self.viewport.go_bottom(),

            // Search
            KeyCode::Char('/') => {
                self.mode = AppMode::Search;
                self.search.start_search();
            }
            KeyCode::Char('n') => {
                self.search.next_match();
                self.scroll_to_current_match();
            }
            KeyCode::Char('N') => {
                self.search.prev_match();
                self.scroll_to_current_match();
            }

            // Toggle code block wrap
            KeyCode::Char('w') => {
                self.wrap_code = !self.wrap_code;
                self.status_message = Some(format!(
                    "Code wrap: {}",
                    if self.wrap_code { "on" } else { "off" }
                ));
                self.re_render_current();
            }

            // Theme cycling
            KeyCode::Char('t') => {
                let new_theme = Theme::next_theme(self.theme.key());
                self.status_message = Some(format!("Theme: {}", new_theme.name));
                Config::save_last_theme(new_theme.key());
                self.theme = new_theme;
                self.re_render_current();
            }
            KeyCode::Char('T') => {
                let new_theme = Theme::prev_theme(self.theme.key());
                self.status_message = Some(format!("Theme: {}", new_theme.name));
                Config::save_last_theme(new_theme.key());
                self.theme = new_theme;
                self.re_render_current();
            }

            // Help
            KeyCode::Char('?') | KeyCode::Char('h') => {
                self.mode = AppMode::Help;
            }

            // Clear search
            KeyCode::Esc => {
                self.search.clear_search();
            }

            _ => {}
        }

        if key.code != KeyCode::Char('g') {
            self.pending_g = false;
        }
    }

    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.search.cancel_search();
                self.mode = AppMode::Normal;
            }
            KeyCode::Enter => {
                self.search.confirm_search();
                self.search.find_matches(&self.rendered);
                self.mode = AppMode::Normal;
                self.scroll_to_current_match();
            }
            KeyCode::Backspace => {
                self.search.input.pop();
                self.search.find_matches(&self.rendered);
            }
            KeyCode::Char(c) => {
                self.search.input.push(c);
                self.search.find_matches(&self.rendered);
            }
            _ => {}
        }
    }

    fn handle_help_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::Char('h') => {
                self.mode = AppMode::Normal;
            }
            _ => {}
        }
    }

    fn scroll_to_current_match(&mut self) {
        if let Some(line) = self.search.current_match_line() {
            let line = line as u16;
            if line < self.viewport.offset || line >= self.viewport.offset + self.viewport.height {
                self.viewport.offset = line.saturating_sub(self.viewport.height / 2);
                let max = self
                    .viewport
                    .content_height
                    .saturating_sub(self.viewport.height);
                self.viewport.offset = self.viewport.offset.min(max);
            }
        }
    }

    fn copy_selection(&mut self) {
        let offset = self.viewport.offset as usize;
        let height = self.viewport.height as usize;
        let visible_lines: Vec<_> = self
            .rendered
            .lines
            .iter()
            .skip(offset)
            .take(height)
            .cloned()
            .collect();

        let text = self
            .selection
            .extract_text(&visible_lines, self.viewport.offset);
        if !text.is_empty() {
            selection::copy_to_clipboard(&text);
            let char_count = text.chars().count();
            self.status_message = Some(format!("Copied {} chars", char_count));
        }
        self.selection.clear();
    }

    pub fn handle_mouse(&mut self, event: crossterm::event::MouseEvent) {
        match event.kind {
            MouseEventKind::ScrollDown => {
                self.selection.clear();
                self.viewport.scroll_down(3);
            }
            MouseEventKind::ScrollUp => {
                self.selection.clear();
                self.viewport.scroll_up(3);
            }
            MouseEventKind::Down(MouseButton::Left) => {
                // Start text selection
                self.selection
                    .start_drag(event.row + self.viewport.offset, event.column);
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                // Update selection during drag
                self.selection
                    .update_drag(event.row + self.viewport.offset, event.column);
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.selection.dragging {
                    self.selection
                        .update_drag(event.row + self.viewport.offset, event.column);
                    self.selection.end_drag();

                    // If it's a click (start == end), clear selection
                    if let (Some(start), Some(end)) =
                        (self.selection.start, self.selection.end)
                    {
                        if start == end {
                            self.selection.clear();
                        }
                    }
                }
            }
            MouseEventKind::Down(MouseButton::Right) => {
                if self.selection.has_selection() {
                    self.copy_selection();
                }
            }
            _ => {}
        }
    }

    fn re_render_current(&mut self) {
        let content = self.raw_content.clone();
        self.re_render(&content, self.render_width);
    }

    pub fn re_render(&mut self, content: &str, width: u16) {
        self.raw_content = content.to_string();
        self.render_width = width;
        self.rendered = render_markdown(content, width.saturating_sub(4), &self.theme, self.wrap_code);
        self.viewport.content_height = self.rendered.lines.len() as u16;
        if !self.search.query.is_empty() {
            self.search.find_matches(&self.rendered);
        }
    }
}

/// Restore terminal to normal state. Safe to call multiple times.
fn restore_terminal() {
    let _ = disable_raw_mode();
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = stdout().execute(DisableMouseCapture);
}

pub fn run(content: String, file_path: PathBuf, theme: Theme) -> io::Result<()> {
    // Install panic hook to always restore terminal state
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        restore_terminal();
        default_panic(info);
    }));

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let size = terminal.size()?;
    let mut app = App::new(&content, file_path.clone(), theme, size.width);

    let event_handler = EventHandler::new(50);

    // File watcher for hot reload (only for real files, not stdin)
    let watcher = if file_path.exists() && file_path != PathBuf::from("stdin") {
        crate::watcher::FileWatcher::new(&file_path)
    } else {
        None
    };

    let result = (|| -> io::Result<()> {
        loop {
            terminal.draw(|f| {
                app.viewport.height = f.area().height.saturating_sub(1);
                ui::draw(f, &app);
            })?;

            if app.should_quit {
                break;
            }

            match event_handler.next()? {
                AppEvent::Key(key) => app.handle_key(key),
                AppEvent::Mouse(mouse) => app.handle_mouse(mouse),
                AppEvent::Resize(w, _h) => {
                    let content = app.raw_content.clone();
                    app.re_render(&content, w);
                }
                AppEvent::Tick => {
                    // Check for file changes
                    if let Some(ref w) = watcher {
                        if w.has_changed() {
                            if let Ok(new_content) = std::fs::read_to_string(&file_path) {
                                let width = app.render_width;
                                app.re_render(&new_content, width);
                                app.status_message = Some("File reloaded".to_string());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    })();

    // Always restore, even if the loop errored
    restore_terminal();

    result
}
