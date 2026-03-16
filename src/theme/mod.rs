mod gradient;

pub use gradient::gradient_spans;

use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub name: String,
    pub h1_gradient: (Color, Color),
    pub h2_gradient: (Color, Color),
    pub h3_color: Color,
    pub h4_color: Color,
    pub h1_separator: Color,
    pub code_bg: Color,
    pub code_border: Color,
    pub code_label: Color,
    pub text: Color,
    pub text_dim: Color,
    pub bold: Color,
    pub italic: Color,
    pub strikethrough: Color,
    pub inline_code_fg: Color,
    pub inline_code_bg: Color,
    pub link: Color,
    pub link_url: Color,
    pub list_marker: Color,
    pub task_done: Color,
    pub task_pending: Color,
    pub blockquote_border: Color,
    pub blockquote_text: Color,
    pub table_header_bg: Color,
    pub table_border: Color,
    pub table_row_alt: Color,
    pub hr: Color,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub status_bar_accent: Color,
    pub search_match_bg: Color,
    pub search_current_bg: Color,
}

fn hex(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
    Color::Rgb(r, g, b)
}

impl Theme {
    pub fn by_name(name: &str) -> Self {
        match name {
            "dracula" => Self::dracula(),
            "nord" => Self::nord(),
            "tokyo-night" => Self::tokyo_night(),
            _ => Self::catppuccin(),
        }
    }

    pub fn catppuccin() -> Self {
        Self {
            name: "Catppuccin".into(),
            h1_gradient: (hex("#f38ba8"), hex("#f9e2af")),
            h2_gradient: (hex("#cba6f7"), hex("#f5c2e7")),
            h3_color: hex("#89b4fa"),
            h4_color: hex("#74c7ec"),
            h1_separator: hex("#45475a"),
            code_bg: hex("#1e1e2e"),
            code_border: hex("#45475a"),
            code_label: hex("#a6e3a1"),
            text: hex("#cdd6f4"),
            text_dim: hex("#6c7086"),
            bold: hex("#cdd6f4"),
            italic: hex("#cdd6f4"),
            strikethrough: hex("#6c7086"),
            inline_code_fg: hex("#f5c2e7"),
            inline_code_bg: hex("#313244"),
            link: hex("#89b4fa"),
            link_url: hex("#6c7086"),
            list_marker: hex("#f9e2af"),
            task_done: hex("#a6e3a1"),
            task_pending: hex("#6c7086"),
            blockquote_border: hex("#cba6f7"),
            blockquote_text: hex("#a6adc8"),
            table_header_bg: hex("#313244"),
            table_border: hex("#45475a"),
            table_row_alt: hex("#1e1e2e"),
            hr: hex("#45475a"),
            status_bar_bg: hex("#181825"),
            status_bar_fg: hex("#cdd6f4"),
            status_bar_accent: hex("#cba6f7"),
            search_match_bg: hex("#585b70"),
            search_current_bg: hex("#f9e2af"),
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "Dracula".into(),
            h1_gradient: (hex("#bd93f9"), hex("#ff79c6")),
            h2_gradient: (hex("#8be9fd"), hex("#50fa7b")),
            h3_color: hex("#ffb86c"),
            h4_color: hex("#f1fa8c"),
            h1_separator: hex("#44475a"),
            code_bg: hex("#282a36"),
            code_border: hex("#44475a"),
            code_label: hex("#50fa7b"),
            text: hex("#f8f8f2"),
            text_dim: hex("#6272a4"),
            bold: hex("#f8f8f2"),
            italic: hex("#f8f8f2"),
            strikethrough: hex("#6272a4"),
            inline_code_fg: hex("#ff79c6"),
            inline_code_bg: hex("#44475a"),
            link: hex("#8be9fd"),
            link_url: hex("#6272a4"),
            list_marker: hex("#f1fa8c"),
            task_done: hex("#50fa7b"),
            task_pending: hex("#6272a4"),
            blockquote_border: hex("#bd93f9"),
            blockquote_text: hex("#bfbfbf"),
            table_header_bg: hex("#44475a"),
            table_border: hex("#6272a4"),
            table_row_alt: hex("#282a36"),
            hr: hex("#44475a"),
            status_bar_bg: hex("#21222c"),
            status_bar_fg: hex("#f8f8f2"),
            status_bar_accent: hex("#bd93f9"),
            search_match_bg: hex("#44475a"),
            search_current_bg: hex("#f1fa8c"),
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord".into(),
            h1_gradient: (hex("#88c0d0"), hex("#a3be8c")),
            h2_gradient: (hex("#81a1c1"), hex("#b48ead")),
            h3_color: hex("#ebcb8b"),
            h4_color: hex("#d08770"),
            h1_separator: hex("#3b4252"),
            code_bg: hex("#2e3440"),
            code_border: hex("#3b4252"),
            code_label: hex("#a3be8c"),
            text: hex("#eceff4"),
            text_dim: hex("#4c566a"),
            bold: hex("#eceff4"),
            italic: hex("#eceff4"),
            strikethrough: hex("#4c566a"),
            inline_code_fg: hex("#88c0d0"),
            inline_code_bg: hex("#3b4252"),
            link: hex("#88c0d0"),
            link_url: hex("#4c566a"),
            list_marker: hex("#ebcb8b"),
            task_done: hex("#a3be8c"),
            task_pending: hex("#4c566a"),
            blockquote_border: hex("#b48ead"),
            blockquote_text: hex("#d8dee9"),
            table_header_bg: hex("#3b4252"),
            table_border: hex("#4c566a"),
            table_row_alt: hex("#2e3440"),
            hr: hex("#3b4252"),
            status_bar_bg: hex("#242933"),
            status_bar_fg: hex("#eceff4"),
            status_bar_accent: hex("#88c0d0"),
            search_match_bg: hex("#434c5e"),
            search_current_bg: hex("#ebcb8b"),
        }
    }

    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            h1_gradient: (hex("#7aa2f7"), hex("#bb9af7")),
            h2_gradient: (hex("#7dcfff"), hex("#73daca")),
            h3_color: hex("#e0af68"),
            h4_color: hex("#ff9e64"),
            h1_separator: hex("#292e42"),
            code_bg: hex("#1a1b26"),
            code_border: hex("#292e42"),
            code_label: hex("#9ece6a"),
            text: hex("#c0caf5"),
            text_dim: hex("#565f89"),
            bold: hex("#c0caf5"),
            italic: hex("#c0caf5"),
            strikethrough: hex("#565f89"),
            inline_code_fg: hex("#bb9af7"),
            inline_code_bg: hex("#292e42"),
            link: hex("#7aa2f7"),
            link_url: hex("#565f89"),
            list_marker: hex("#e0af68"),
            task_done: hex("#9ece6a"),
            task_pending: hex("#565f89"),
            blockquote_border: hex("#bb9af7"),
            blockquote_text: hex("#a9b1d6"),
            table_header_bg: hex("#292e42"),
            table_border: hex("#3b4261"),
            table_row_alt: hex("#1a1b26"),
            hr: hex("#292e42"),
            status_bar_bg: hex("#16161e"),
            status_bar_fg: hex("#c0caf5"),
            status_bar_accent: hex("#7aa2f7"),
            search_match_bg: hex("#3b4261"),
            search_current_bg: hex("#e0af68"),
        }
    }
}
