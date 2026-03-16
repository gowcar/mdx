use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::App;
use crate::widgets::StatusBar;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Min(1),      // content area
        Constraint::Length(1),   // status bar
    ])
    .split(f.area());

    let content_area = chunks[0];
    let status_area = chunks[1];

    // Update viewport height
    let viewport_height = content_area.height;

    // Render markdown content with scroll offset
    let text = &app.rendered;
    let total_lines = text.lines.len() as u16;
    let offset = app.viewport.offset as usize;

    // Create a slice of visible lines
    let visible_lines: Vec<_> = text
        .lines
        .iter()
        .skip(offset)
        .take(viewport_height as usize)
        .cloned()
        .collect();

    let visible_text = ratatui::text::Text::from(visible_lines);
    let paragraph = ratatui::widgets::Paragraph::new(visible_text);
    f.render_widget(paragraph, content_area);

    // Render status bar
    let filename = app
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("stdin");

    let status_bar = StatusBar::new(
        filename,
        app.viewport.current_line(),
        total_lines,
        app.viewport.percentage(),
        &app.theme.name,
        &app.theme,
    );
    f.render_widget(status_bar, status_area);
}
