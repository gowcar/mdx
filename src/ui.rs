use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};

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

    // Render markdown content with scroll offset and search highlights
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

    // Render status bar
    let filename = app
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("stdin");

    let search_info = app.search.match_info();
    let pending = if app.pending_g { "g" } else { "" };

    let status_bar = StatusBar::new(
        filename,
        app.viewport.current_line(),
        total_lines,
        app.viewport.percentage(),
        &app.theme.name,
        &app.theme,
    )
    .search_info(&search_info)
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
            let help_height = 17; // entries + borders
            let help_area = centered_rect(44, help_height, content_area);
            let help = HelpPopup::new(&app.theme);
            f.render_widget(help, help_area);
        }
        AppMode::Normal => {
            // Show search info in status area if search is active
            if !app.search.query.is_empty() && !app.search.matches.is_empty() {
                // Match info is shown via the search state
            }
        }
    }
}

fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(
        x,
        y,
        width.min(area.width),
        height.min(area.height),
    )
}

fn centered_rect_bottom(percent_width: u16, height: u16, area: Rect) -> Rect {
    let width = (area.width * percent_width / 100).min(area.width);
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + area.height.saturating_sub(height + 1);
    Rect::new(x, y, width, height)
}
