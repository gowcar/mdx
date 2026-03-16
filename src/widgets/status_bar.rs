use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::theme::Theme;

pub struct StatusBar<'a> {
    filename: &'a str,
    current_line: u16,
    total_lines: u16,
    percentage: u16,
    theme_name: &'a str,
    theme: &'a Theme,
}

impl<'a> StatusBar<'a> {
    pub fn new(
        filename: &'a str,
        current_line: u16,
        total_lines: u16,
        percentage: u16,
        theme_name: &'a str,
        theme: &'a Theme,
    ) -> Self {
        Self {
            filename,
            current_line,
            total_lines,
            percentage,
            theme_name,
            theme,
        }
    }

    fn render_progress_bar(&self, width: usize) -> String {
        let filled = (width as u32 * self.percentage as u32 / 100) as usize;
        let empty = width.saturating_sub(filled);
        format!("{}{}", "▓".repeat(filled), "░".repeat(empty))
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let bg = self.theme.status_bar_bg;
        let fg = self.theme.status_bar_fg;
        let accent = self.theme.status_bar_accent;
        let base_style = Style::default().fg(fg).bg(bg);
        let accent_style = Style::default().fg(accent).bg(bg);
        let dim_style = Style::default()
            .fg(self.theme.text_dim)
            .bg(bg);
        let sep = Span::styled(" │ ", dim_style);

        // Get current time
        let now = chrono_lite_time();

        let progress_bar = self.render_progress_bar(10);

        let spans = vec![
            Span::styled(" 📄 ", accent_style),
            Span::styled(self.filename, Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)),
            sep.clone(),
            Span::styled("📍 ", accent_style),
            Span::styled(
                format!("{}/{}", self.current_line, self.total_lines),
                base_style,
            ),
            sep.clone(),
            Span::styled(format!("{} ", progress_bar), accent_style),
            Span::styled(format!("{}%", self.percentage), base_style),
            sep.clone(),
            Span::styled(format!("🕐 {}", now), base_style),
            sep.clone(),
            Span::styled("🎨 ", accent_style),
            Span::styled(self.theme_name, base_style),
            sep.clone(),
            Span::styled("↑↓", accent_style),
            Span::styled(" j/k  ", dim_style),
            Span::styled("/", accent_style),
            Span::styled("🔍  ", dim_style),
            Span::styled("?", accent_style),
            Span::styled("❓ ", dim_style),
        ];

        // Fill background
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(base_style);
            buf[(x, area.y)].set_char(' ');
        }

        // Render spans
        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

/// Simple time without chrono dependency
fn chrono_lite_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // UTC+8 for convenience (can be made configurable later)
    let local_secs = secs + 8 * 3600;
    let hours = (local_secs / 3600) % 24;
    let minutes = (local_secs / 60) % 60;
    format!("{:02}:{:02}", hours, minutes)
}
