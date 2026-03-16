use std::io::{self, stdout};
use std::path::PathBuf;

use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, KeyCode, KeyEvent, KeyModifiers, MouseEventKind,
};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::ExecutableCommand;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::text::Text;

use crate::event::{AppEvent, EventHandler};
use crate::render::render_markdown;
use crate::scroll::Viewport;
use crate::search::SearchState;
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
    pending_g: bool,
}

impl App {
    pub fn new(content: &str, file_path: PathBuf, theme: Theme, width: u16) -> Self {
        let rendered = render_markdown(content, width.saturating_sub(4), &theme);
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
            pending_g: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            AppMode::Search => self.handle_search_key(key),
            AppMode::Help => self.handle_help_key(key),
            AppMode::Normal => self.handle_normal_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
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

            // Help
            KeyCode::Char('?') => {
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
                // Live search
                self.search.find_matches(&self.rendered);
            }
            KeyCode::Char(c) => {
                self.search.input.push(c);
                // Live search
                self.search.find_matches(&self.rendered);
            }
            _ => {}
        }
    }

    fn handle_help_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') => {
                self.mode = AppMode::Normal;
            }
            _ => {}
        }
    }

    fn scroll_to_current_match(&mut self) {
        if let Some(line) = self.search.current_match_line() {
            let line = line as u16;
            if line < self.viewport.offset || line >= self.viewport.offset + self.viewport.height {
                // Center the match in viewport
                self.viewport.offset = line.saturating_sub(self.viewport.height / 2);
                let max = self.viewport.content_height.saturating_sub(self.viewport.height);
                self.viewport.offset = self.viewport.offset.min(max);
            }
        }
    }

    pub fn handle_mouse(&mut self, event: crossterm::event::MouseEvent) {
        match event.kind {
            MouseEventKind::ScrollDown => self.viewport.scroll_down(3),
            MouseEventKind::ScrollUp => self.viewport.scroll_up(3),
            _ => {}
        }
    }

    pub fn re_render(&mut self, content: &str, width: u16) {
        self.rendered = render_markdown(content, width.saturating_sub(4), &self.theme);
        self.viewport.content_height = self.rendered.lines.len() as u16;
        if !self.search.query.is_empty() {
            self.search.find_matches(&self.rendered);
        }
    }
}

pub fn run(content: String, file_path: PathBuf, theme: Theme) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let size = terminal.size()?;
    let mut app = App::new(&content, file_path, theme, size.width);

    let event_handler = EventHandler::new(50);

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
                app.re_render(&content, w);
            }
            AppEvent::Tick => {}
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;

    Ok(())
}
