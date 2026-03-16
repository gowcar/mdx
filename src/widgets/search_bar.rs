use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::theme::Theme;

pub struct SearchBar<'a> {
    input: &'a str,
    match_info: &'a str,
    theme: &'a Theme,
}

impl<'a> SearchBar<'a> {
    pub fn new(input: &'a str, match_info: &'a str, theme: &'a Theme) -> Self {
        Self {
            input,
            match_info,
            theme,
        }
    }
}

impl Widget for SearchBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = self.theme.status_bar_accent;
        let bg = self.theme.code_bg;
        let fg = self.theme.text;
        let base_style = Style::default().fg(fg).bg(bg);

        // Fill background
        for x in area.x..area.x + area.width {
            for y in area.y..area.y + area.height {
                buf[(x, y)].set_style(base_style);
                buf[(x, y)].set_char(' ');
            }
        }

        if area.height >= 3 {
            // Top border
            let top_label = " Search ";
            let remaining = area.width as usize - top_label.len() - 4;
            let top = format!(
                "┌─{}{}┐",
                top_label,
                "─".repeat(remaining)
            );
            let top_line = Line::from(Span::styled(
                top,
                Style::default().fg(border_color).bg(bg),
            ));
            buf.set_line(area.x, area.y, &top_line, area.width);

            // Input line
            let input_line = Line::from(vec![
                Span::styled("│ ", Style::default().fg(border_color).bg(bg)),
                Span::styled("> ", Style::default().fg(border_color).bg(bg).add_modifier(Modifier::BOLD)),
                Span::styled(self.input, Style::default().fg(fg).bg(bg)),
                Span::styled("█", Style::default().fg(border_color).bg(bg)),
            ]);
            buf.set_line(area.x, area.y + 1, &input_line, area.width);

            // Right border on input line
            let right_x = area.x + area.width - 1;
            buf[(right_x, area.y + 1)].set_char('│');
            buf[(right_x, area.y + 1)].set_style(Style::default().fg(border_color).bg(bg));

            // Bottom border with match info
            let info_part = if self.match_info.is_empty() {
                String::new()
            } else {
                format!(" {} ", self.match_info)
            };
            let bottom_remaining = area.width as usize - info_part.len() - 2;
            let bottom = format!(
                "└{}{}┘",
                "─".repeat(bottom_remaining),
                info_part
            );
            let bottom_line = Line::from(Span::styled(
                bottom,
                Style::default().fg(border_color).bg(bg),
            ));
            buf.set_line(area.x, area.y + 2, &bottom_line, area.width);
        }
    }
}
