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
    nerd_font: bool,
    search_info: Option<&'a str>,
    pending_key: Option<&'a str>,
    wrap_code: bool,
}

impl<'a> StatusBar<'a> {
    pub fn new(
        filename: &'a str,
        current_line: u16,
        total_lines: u16,
        percentage: u16,
        theme_name: &'a str,
        theme: &'a Theme,
        nerd_font: bool,
    ) -> Self {
        Self {
            filename,
            current_line,
            total_lines,
            percentage,
            theme_name,
            theme,
            nerd_font,
            search_info: None,
            pending_key: None,
            wrap_code: false,
        }
    }

    pub fn search_info(mut self, info: &'a str) -> Self {
        if !info.is_empty() {
            self.search_info = Some(info);
        }
        self
    }

    pub fn pending_key(mut self, key: &'a str) -> Self {
        if !key.is_empty() {
            self.pending_key = Some(key);
        }
        self
    }

    pub fn wrap_code(mut self, on: bool) -> Self {
        self.wrap_code = on;
        self
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
        let dim_style = Style::default().fg(self.theme.text_dim).bg(bg);
        let sep = Span::styled(" │ ", dim_style);

        let now = local_time();
        let progress_bar = self.render_progress_bar(10);

        let icon = self.theme.icon(self.nerd_font);

        let mut spans = vec![
            Span::styled(format!(" {} ", icon), accent_style),
            Span::styled(
                self.filename,
                Style::default()
                    .fg(fg)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            ),
            sep.clone(),
            Span::styled(" ", accent_style),
            Span::styled(
                format!("{}/{}", self.current_line, self.total_lines),
                base_style,
            ),
            sep.clone(),
            Span::styled(format!("{} ", progress_bar), accent_style),
            Span::styled(format!("{}%", self.percentage), base_style),
            sep.clone(),
            Span::styled(format!(" {}", now), base_style),
            sep.clone(),
            Span::styled(" ", accent_style),
            Span::styled(self.theme_name, base_style),
        ];

        if self.wrap_code {
            spans.push(sep.clone());
            spans.push(Span::styled(
                "WRAP",
                Style::default()
                    .fg(self.theme.search_current_bg)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        // Show search info or key hints
        if let Some(info) = self.search_info {
            spans.push(sep.clone());
            spans.push(Span::styled(" ", accent_style));
            spans.push(Span::styled(info, base_style));
        } else if let Some(key) = self.pending_key {
            spans.push(sep.clone());
            spans.push(Span::styled(
                format!("{}_ ", key),
                Style::default()
                    .fg(self.theme.search_current_bg)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(sep.clone());
            spans.push(Span::styled(" j/k  ", dim_style));
            spans.push(Span::styled(" /  ", accent_style));
            spans.push(Span::styled(" ?", accent_style));
        }

        // Fill background
        for x in area.x..area.x + area.width {
            buf[(x, area.y)].set_style(base_style);
            buf[(x, area.y)].set_char(' ');
        }

        let line = Line::from(spans);
        buf.set_line(area.x, area.y, &line, area.width);
    }
}

fn local_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Use local timezone offset
    // Simple approach: get offset from environment or default to system
    let offset_secs = local_utc_offset_secs();
    let local_secs = (secs as i64 + offset_secs) as u64;
    let hours = (local_secs / 3600) % 24;
    let minutes = (local_secs / 60) % 60;
    format!("{:02}:{:02}", hours, minutes)
}

fn local_utc_offset_secs() -> i64 {
    // Try to detect local timezone offset
    // On Unix, we can check TZ or use libc localtime
    // Simple fallback: check common env vars
    if let Ok(tz) = std::env::var("TZ_OFFSET") {
        if let Ok(hours) = tz.parse::<i64>() {
            return hours * 3600;
        }
    }
    // Default: try to detect from system
    // Use a simple heuristic based on current time comparison
    #[cfg(unix)]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        // Use libc to get local time offset
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        unsafe {
            let time_t = now as libc::time_t;
            let mut tm: libc::tm = std::mem::zeroed();
            libc::localtime_r(&time_t, &mut tm);
            tm.tm_gmtoff
        }
    }
    #[cfg(not(unix))]
    {
        0 // UTC fallback
    }
}
