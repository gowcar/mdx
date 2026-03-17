use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;

use crate::app::{App, AppMode};
use crate::widgets::{HelpPopup, SearchBar, StatusBar};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Min(1),    // content area
        Constraint::Length(1), // status bar
    ])
    .split(f.area());

    let content_area = chunks[0];
    let status_area = chunks[1];

    let viewport_height = content_area.height;

    let text = &app.rendered;
    let total_lines = text.lines.len() as u16;
    let offset = app.viewport.offset as usize;

    let has_search = !app.search.query.is_empty() || app.search.active;

    let visible_lines: Vec<_> = text
        .lines
        .iter()
        .enumerate()
        .skip(offset)
        .take(viewport_height as usize)
        .map(|(idx, line)| {
            if has_search {
                app.search.highlight_line(line, idx, &app.theme)
            } else {
                line.clone()
            }
        })
        .collect();

    let visible_text = ratatui::text::Text::from(visible_lines);
    let paragraph = ratatui::widgets::Paragraph::new(visible_text);
    f.render_widget(paragraph, content_area);

    // Render selection highlight overlay
    if app.selection.has_selection() {
        let buf = f.buffer_mut();
        for row in content_area.y..content_area.y + content_area.height {
            let doc_row = row - content_area.y + app.viewport.offset;
            for col in content_area.x..content_area.x + content_area.width {
                if app.selection.contains(doc_row, col) {
                    let cell = &mut buf[(col, row)];
                    // Apply selection style while preserving foreground
                    let existing = cell.style();
                    cell.set_style(
                        existing
                            .bg(Color::Rgb(100, 100, 180))
                    );
                }
            }
        }
    }

    // Render status bar
    let filename = app
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("stdin");

    let search_info = app.search.match_info();
    let pending = if app.pending_g { "g" } else { "" };

    // Show status message (like "Copied 42 chars") or search info
    let display_info = if let Some(ref msg) = app.status_message {
        msg.as_str()
    } else {
        &search_info
    };

    let status_bar = StatusBar::new(
        filename,
        app.viewport.current_line(),
        total_lines,
        app.viewport.percentage(),
        &app.theme.name,
        &app.theme,
        app.nerd_font,
    )
    .search_info(display_info)
    .pending_key(pending);
    f.render_widget(status_bar, status_area);

    // Overlays
    match app.mode {
        AppMode::Search => {
            let search_area = centered_rect_bottom(60, 3, content_area);
            let match_info = app.search.match_info();
            let search_bar = SearchBar::new(&app.search.input, &match_info, &app.theme);
            f.render_widget(search_bar, search_area);
        }
        AppMode::Help => {
            let help_height = 18;
            let help_area = centered_rect(44, help_height, content_area);
            let help = HelpPopup::new(&app.theme);
            f.render_widget(help, help_area);
        }
        AppMode::Normal => {}
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

fn centered_rect_bottom(percent_width: u16, height: u16, area: Rect) -> Rect {
    let width = (area.width * percent_width / 100).min(area.width);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + area.height.saturating_sub(height + 1);
    Rect::new(x, y, width, height)
}
