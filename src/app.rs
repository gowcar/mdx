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
use crate::theme::Theme;
use crate::ui;

pub struct App {
    pub rendered: Text<'static>,
    pub viewport: Viewport,
    pub theme: Theme,
    pub file_path: PathBuf,
    pub should_quit: bool,
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
            pending_g: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
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

            _ => {}
        }

        if key.code != KeyCode::Char('g') {
            self.pending_g = false;
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
