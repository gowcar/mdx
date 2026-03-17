use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Widget;

use crate::theme::Theme;

pub struct HelpPopup<'a> {
    theme: &'a Theme,
}

impl<'a> HelpPopup<'a> {
    pub fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }
}

const HELP_ENTRIES: &[(&str, &str)] = &[
    ("j / ↓", "Scroll down"),
    ("k / ↑", "Scroll up"),
    ("Ctrl-d", "Half page down"),
    ("Ctrl-u", "Half page up"),
    ("f / Space", "Full page down"),
    ("b", "Full page up"),
    ("g g", "Go to top"),
    ("G", "Go to bottom"),
    ("/", "Search"),
    ("n / N", "Next / prev match"),
    ("t / T", "Next / prev theme"),
    ("Esc", "Clear search / selection"),
    ("Mouse drag", "Select text"),
    ("y", "Copy selection"),
    ("q / Ctrl-c", "Quit"),
    ("? / h", "Toggle help"),
];

impl Widget for HelpPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_color = self.theme.status_bar_accent;
        let bg = self.theme.code_bg;
        let fg = self.theme.text;
        let dim = self.theme.text_dim;
        let accent = self.theme.h3_color;

        let base_style = Style::default().fg(fg).bg(bg);

        // Clear area
        for x in area.x..area.x + area.width {
            for y in area.y..area.y + area.height {
                buf[(x, y)].set_style(base_style);
                buf[(x, y)].set_char(' ');
            }
        }

        let inner_width = area.width.saturating_sub(2) as usize;

        // Top border
        let title = " Keybindings ";
        let rem = inner_width.saturating_sub(title.len());
        let left_pad = rem / 2;
        let right_pad = rem - left_pad;
        let top = format!(
            "╭{}{}{}╮",
            "─".repeat(left_pad),
            title,
            "─".repeat(right_pad)
        );
        buf.set_line(
            area.x,
            area.y,
            &Line::from(Span::styled(top, Style::default().fg(border_color).bg(bg))),
            area.width,
        );

        // Help entries
        for (i, (key, desc)) in HELP_ENTRIES.iter().enumerate() {
            let y = area.y + 1 + i as u16;
            if y >= area.y + area.height - 1 {
                break;
            }

            let key_width = 14;
            let padded_key = format!("{:>width$}", key, width = key_width);
            let line = Line::from(vec![
                Span::styled("│ ", Style::default().fg(border_color).bg(bg)),
                Span::styled(
                    padded_key,
                    Style::default()
                        .fg(accent)
                        .bg(bg)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ", base_style),
                Span::styled(*desc, Style::default().fg(dim).bg(bg)),
            ]);
            buf.set_line(area.x, y, &line, area.width);

            // Right border
            let rx = area.x + area.width - 1;
            if rx < buf.area().width {
                buf[(rx, y)].set_char('│');
                buf[(rx, y)].set_style(Style::default().fg(border_color).bg(bg));
            }
        }

        // Bottom border with version
        let bottom_y = area.y + area.height - 1;
        let version = format!(" mdx v{} ", env!("CARGO_PKG_VERSION"));
        let bottom_remaining = inner_width.saturating_sub(version.len());
        let bottom = format!("╰{}{}╯", "─".repeat(bottom_remaining), version);
        buf.set_line(
            area.x,
            bottom_y,
            &Line::from(Span::styled(
                bottom,
                Style::default().fg(border_color).bg(bg),
            )),
            area.width,
        );
    }
}
