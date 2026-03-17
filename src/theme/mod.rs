mod gradient;

pub use gradient::gradient_spans;

use ratatui::style::Color;

#[derive(Clone)]
#[allow(dead_code)]
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
    // Decorative characters (per-theme personality)
    pub h1_sep_char: &'static str,
    pub h2_sep_char: &'static str,
    pub list_markers: &'static [&'static str],
    pub blockquote_char: &'static str,
    pub hr_char: &'static str,
    pub status_icon: &'static str,       // Unicode fallback
    pub status_icon_nerd: &'static str,  // Nerd Font icon
}

/// Detect if a Nerd Font is likely available
pub fn has_nerd_font() -> bool {
    // Check config override first
    if let Ok(val) = std::env::var("MDX_NERD_FONT") {
        return val == "1" || val.eq_ignore_ascii_case("true");
    }
    // Check if TERM contains "nerd" or common nerd-font-capable terminals
    if let Ok(term) = std::env::var("TERM_PROGRAM") {
        let t = term.to_lowercase();
        // These terminals commonly ship with or default to Nerd Fonts
        if t.contains("wezterm") || t.contains("kitty") || t.contains("alacritty") {
            return true;
        }
    }
    false
}

impl Theme {
    /// Get the appropriate status icon based on font availability
    pub fn icon(&self, nerd: bool) -> &str {
        if nerd { self.status_icon_nerd } else { self.status_icon }
    }
}

fn hex(s: &str) -> Color {
    let s = s.trim_start_matches('#');
    let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
    Color::Rgb(r, g, b)
}

pub const THEME_NAMES: &[&str] = &["catppuccin", "dracula", "nord", "tokyo-night", "gruvbox", "solarized", "one-dark", "monokai"];

impl Theme {
    pub fn by_name(name: &str) -> Self {
        match name {
            "catppuccin" => Self::catppuccin(),
            "dracula" => Self::dracula(),
            "nord" => Self::nord(),
            "tokyo-night" => Self::tokyo_night(),
            "gruvbox" => Self::gruvbox(),
            "solarized" => Self::solarized(),
            "one-dark" => Self::one_dark(),
            "monokai" => Self::monokai(),
            _ => Self::dracula(),
        }
    }

    pub fn next_theme(current: &str) -> Self {
        let idx = THEME_NAMES.iter().position(|&n| n == current).unwrap_or(0);
        let next = (idx + 1) % THEME_NAMES.len();
        Self::by_name(THEME_NAMES[next])
    }

    pub fn prev_theme(current: &str) -> Self {
        let idx = THEME_NAMES.iter().position(|&n| n == current).unwrap_or(0);
        let prev = if idx == 0 { THEME_NAMES.len() - 1 } else { idx - 1 };
        Self::by_name(THEME_NAMES[prev])
    }

    /// Return the lowercase key used in THEME_NAMES for matching
    pub fn key(&self) -> &str {
        THEME_NAMES.iter()
            .find(|&&n| Self::by_name(n).name == self.name)
            .unwrap_or(&"dracula")
    }

    // ── 𝗖𝗮𝘁𝗽𝗽𝘂𝗰𝗰𝗶𝗻 ── Warm pastel dreamscape
    pub fn catppuccin() -> Self {
        Self {
            name: "Catppuccin".into(),
            h1_gradient: (hex("#f38ba8"), hex("#fab387")),
            h2_gradient: (hex("#cba6f7"), hex("#f5c2e7")),
            h3_color: hex("#89b4fa"),
            h4_color: hex("#74c7ec"),
            h1_separator: hex("#585b70"),
            code_bg: hex("#1e1e2e"),
            code_border: hex("#585b70"),
            code_label: hex("#a6e3a1"),
            text: hex("#cdd6f4"),
            text_dim: hex("#7f849c"),
            bold: hex("#f5e0dc"),
            italic: hex("#f2cdcd"),
            strikethrough: hex("#6c7086"),
            inline_code_fg: hex("#f5c2e7"),
            inline_code_bg: hex("#313244"),
            link: hex("#89b4fa"),
            link_url: hex("#7f849c"),
            list_marker: hex("#f9e2af"),
            task_done: hex("#a6e3a1"),
            task_pending: hex("#6c7086"),
            blockquote_border: hex("#cba6f7"),
            blockquote_text: hex("#bac2de"),
            table_header_bg: hex("#313244"),
            table_border: hex("#585b70"),
            table_row_alt: hex("#1e1e2e"),
            hr: hex("#585b70"),
            status_bar_bg: hex("#11111b"),
            status_bar_fg: hex("#cdd6f4"),
            status_bar_accent: hex("#f5c2e7"),
            search_match_bg: hex("#7c6f9f"),
            search_current_bg: hex("#f9e2af"),
            h1_sep_char: "━",
            h2_sep_char: "─",
            list_markers: &["●", "◦", "▸", "▹"],
            blockquote_char: "▌",
            hr_char: "━",
            status_icon: "◈",
            status_icon_nerd: "󰍔",
        }
    }

    // ── 𝗗𝗿𝗮𝗰𝘂𝗹𝗮 ── Vivid neon gothic
    pub fn dracula() -> Self {
        Self {
            name: "Dracula".into(),
            h1_gradient: (hex("#ff79c6"), hex("#bd93f9")),
            h2_gradient: (hex("#50fa7b"), hex("#8be9fd")),
            h3_color: hex("#ffb86c"),
            h4_color: hex("#f1fa8c"),
            h1_separator: hex("#6272a4"),
            code_bg: hex("#21222c"),
            code_border: hex("#6272a4"),
            code_label: hex("#50fa7b"),
            text: hex("#f8f8f2"),
            text_dim: hex("#6272a4"),
            bold: hex("#ff79c6"),
            italic: hex("#8be9fd"),
            strikethrough: hex("#6272a4"),
            inline_code_fg: hex("#ff79c6"),
            inline_code_bg: hex("#44475a"),
            link: hex("#8be9fd"),
            link_url: hex("#6272a4"),
            list_marker: hex("#50fa7b"),
            task_done: hex("#50fa7b"),
            task_pending: hex("#6272a4"),
            blockquote_border: hex("#bd93f9"),
            blockquote_text: hex("#f8f8f2"),
            table_header_bg: hex("#44475a"),
            table_border: hex("#6272a4"),
            table_row_alt: hex("#282a36"),
            hr: hex("#6272a4"),
            status_bar_bg: hex("#191a21"),
            status_bar_fg: hex("#f8f8f2"),
            status_bar_accent: hex("#ff79c6"),
            search_match_bg: hex("#bd93f9"),
            search_current_bg: hex("#f1fa8c"),
            h1_sep_char: "═",
            h2_sep_char: "─",
            list_markers: &["◆", "◇", "▪", "·"],
            blockquote_char: "┃",
            hr_char: "═",
            status_icon: "◆",
            status_icon_nerd: "󰊠",
        }
    }

    // ── 𝗡𝗼𝗿𝗱 ── Arctic frost, calm & clean
    pub fn nord() -> Self {
        Self {
            name: "Nord".into(),
            h1_gradient: (hex("#88c0d0"), hex("#5e81ac")),
            h2_gradient: (hex("#b48ead"), hex("#81a1c1")),
            h3_color: hex("#ebcb8b"),
            h4_color: hex("#d08770"),
            h1_separator: hex("#4c566a"),
            code_bg: hex("#242933"),
            code_border: hex("#4c566a"),
            code_label: hex("#a3be8c"),
            text: hex("#eceff4"),
            text_dim: hex("#616e88"),
            bold: hex("#88c0d0"),
            italic: hex("#b48ead"),
            strikethrough: hex("#4c566a"),
            inline_code_fg: hex("#88c0d0"),
            inline_code_bg: hex("#3b4252"),
            link: hex("#81a1c1"),
            link_url: hex("#616e88"),
            list_marker: hex("#ebcb8b"),
            task_done: hex("#a3be8c"),
            task_pending: hex("#4c566a"),
            blockquote_border: hex("#5e81ac"),
            blockquote_text: hex("#d8dee9"),
            table_header_bg: hex("#3b4252"),
            table_border: hex("#4c566a"),
            table_row_alt: hex("#2e3440"),
            hr: hex("#4c566a"),
            status_bar_bg: hex("#1c2028"),
            status_bar_fg: hex("#eceff4"),
            status_bar_accent: hex("#88c0d0"),
            search_match_bg: hex("#5e81ac"),
            search_current_bg: hex("#ebcb8b"),
            h1_sep_char: "╌",
            h2_sep_char: "╌",
            list_markers: &["◇", "◦", "▫", "·"],
            blockquote_char: "▎",
            hr_char: "╌",
            status_icon: "❖",
            status_icon_nerd: "󰅟",
        }
    }

    // ── 𝗧𝗼𝗸𝘆𝗼 𝗡𝗶𝗴𝗵𝘁 ── Neon city lights
    pub fn tokyo_night() -> Self {
        Self {
            name: "Tokyo Night".into(),
            h1_gradient: (hex("#ff007c"), hex("#7aa2f7")),
            h2_gradient: (hex("#bb9af7"), hex("#7dcfff")),
            h3_color: hex("#e0af68"),
            h4_color: hex("#ff9e64"),
            h1_separator: hex("#3b4261"),
            code_bg: hex("#16161e"),
            code_border: hex("#3b4261"),
            code_label: hex("#9ece6a"),
            text: hex("#c0caf5"),
            text_dim: hex("#565f89"),
            bold: hex("#7aa2f7"),
            italic: hex("#bb9af7"),
            strikethrough: hex("#565f89"),
            inline_code_fg: hex("#bb9af7"),
            inline_code_bg: hex("#292e42"),
            link: hex("#7dcfff"),
            link_url: hex("#565f89"),
            list_marker: hex("#ff9e64"),
            task_done: hex("#9ece6a"),
            task_pending: hex("#565f89"),
            blockquote_border: hex("#7aa2f7"),
            blockquote_text: hex("#a9b1d6"),
            table_header_bg: hex("#292e42"),
            table_border: hex("#3b4261"),
            table_row_alt: hex("#1a1b26"),
            hr: hex("#3b4261"),
            status_bar_bg: hex("#101014"),
            status_bar_fg: hex("#c0caf5"),
            status_bar_accent: hex("#ff007c"),
            search_match_bg: hex("#7aa2f7"),
            search_current_bg: hex("#ff9e64"),
            h1_sep_char: "━",
            h2_sep_char: "╸",
            list_markers: &["➤", "▸", "▹", "·"],
            blockquote_char: "▐",
            hr_char: "━",
            status_icon: "✦",
            status_icon_nerd: "󰛩",
        }
    }

    // ── 𝗚𝗿𝘂𝘃𝗯𝗼𝘅 ── Retro warm amber
    pub fn gruvbox() -> Self {
        Self {
            name: "Gruvbox".into(),
            h1_gradient: (hex("#fb4934"), hex("#fe8019")),
            h2_gradient: (hex("#fabd2f"), hex("#b8bb26")),
            h3_color: hex("#83a598"),
            h4_color: hex("#d3869b"),
            h1_separator: hex("#504945"),
            code_bg: hex("#1d2021"),
            code_border: hex("#504945"),
            code_label: hex("#b8bb26"),
            text: hex("#ebdbb2"),
            text_dim: hex("#7c6f64"),
            bold: hex("#fabd2f"),
            italic: hex("#d3869b"),
            strikethrough: hex("#665c54"),
            inline_code_fg: hex("#fe8019"),
            inline_code_bg: hex("#3c3836"),
            link: hex("#83a598"),
            link_url: hex("#7c6f64"),
            list_marker: hex("#fe8019"),
            task_done: hex("#b8bb26"),
            task_pending: hex("#665c54"),
            blockquote_border: hex("#d65d0e"),
            blockquote_text: hex("#d5c4a1"),
            table_header_bg: hex("#3c3836"),
            table_border: hex("#665c54"),
            table_row_alt: hex("#1d2021"),
            hr: hex("#665c54"),
            status_bar_bg: hex("#141617"),
            status_bar_fg: hex("#ebdbb2"),
            status_bar_accent: hex("#fe8019"),
            search_match_bg: hex("#d79921"),
            search_current_bg: hex("#fabd2f"),
            h1_sep_char: "▬",
            h2_sep_char: "─",
            list_markers: &["◈", "◆", "◇", "·"],
            blockquote_char: "█",
            hr_char: "▬",
            status_icon: "▣",
            status_icon_nerd: "󰊲",
        }
    }

    // ── 𝗦𝗼𝗹𝗮𝗿𝗶𝘇𝗲𝗱 ── Scientific precision
    pub fn solarized() -> Self {
        Self {
            name: "Solarized".into(),
            h1_gradient: (hex("#dc322f"), hex("#b58900")),
            h2_gradient: (hex("#268bd2"), hex("#2aa198")),
            h3_color: hex("#859900"),
            h4_color: hex("#6c71c4"),
            h1_separator: hex("#586e75"),
            code_bg: hex("#002b36"),
            code_border: hex("#586e75"),
            code_label: hex("#859900"),
            text: hex("#93a1a1"),
            text_dim: hex("#657b83"),
            bold: hex("#cb4b16"),
            italic: hex("#2aa198"),
            strikethrough: hex("#586e75"),
            inline_code_fg: hex("#2aa198"),
            inline_code_bg: hex("#073642"),
            link: hex("#268bd2"),
            link_url: hex("#657b83"),
            list_marker: hex("#b58900"),
            task_done: hex("#859900"),
            task_pending: hex("#586e75"),
            blockquote_border: hex("#268bd2"),
            blockquote_text: hex("#839496"),
            table_header_bg: hex("#073642"),
            table_border: hex("#586e75"),
            table_row_alt: hex("#002b36"),
            hr: hex("#586e75"),
            status_bar_bg: hex("#001e27"),
            status_bar_fg: hex("#93a1a1"),
            status_bar_accent: hex("#b58900"),
            search_match_bg: hex("#268bd2"),
            search_current_bg: hex("#b58900"),
            h1_sep_char: "─",
            h2_sep_char: "╶",
            list_markers: &["◉", "○", "◌", "·"],
            blockquote_char: "▍",
            hr_char: "─",
            status_icon: "◎",
            status_icon_nerd: "󰖨",
        }
    }

    // ── 𝗢𝗻𝗲 𝗗𝗮𝗿𝗸 ── Clean code elegance
    pub fn one_dark() -> Self {
        Self {
            name: "One Dark".into(),
            h1_gradient: (hex("#e06c75"), hex("#d19a66")),
            h2_gradient: (hex("#61afef"), hex("#c678dd")),
            h3_color: hex("#56b6c2"),
            h4_color: hex("#98c379"),
            h1_separator: hex("#5c6370"),
            code_bg: hex("#1b1d23"),
            code_border: hex("#5c6370"),
            code_label: hex("#98c379"),
            text: hex("#abb2bf"),
            text_dim: hex("#5c6370"),
            bold: hex("#e5c07b"),
            italic: hex("#c678dd"),
            strikethrough: hex("#5c6370"),
            inline_code_fg: hex("#c678dd"),
            inline_code_bg: hex("#3e4452"),
            link: hex("#61afef"),
            link_url: hex("#5c6370"),
            list_marker: hex("#e5c07b"),
            task_done: hex("#98c379"),
            task_pending: hex("#5c6370"),
            blockquote_border: hex("#61afef"),
            blockquote_text: hex("#9da5b4"),
            table_header_bg: hex("#3e4452"),
            table_border: hex("#5c6370"),
            table_row_alt: hex("#21252b"),
            hr: hex("#5c6370"),
            status_bar_bg: hex("#14161a"),
            status_bar_fg: hex("#abb2bf"),
            status_bar_accent: hex("#61afef"),
            search_match_bg: hex("#61afef"),
            search_current_bg: hex("#e5c07b"),
            h1_sep_char: "━",
            h2_sep_char: "╺",
            list_markers: &["●", "○", "■", "□"],
            blockquote_char: "▌",
            hr_char: "━",
            status_icon: "◉",
            status_icon_nerd: "󰅩",
        }
    }

    // ── 𝗠𝗼𝗻𝗼𝗸𝗮𝗶 ── High-voltage neon
    pub fn monokai() -> Self {
        Self {
            name: "Monokai".into(),
            h1_gradient: (hex("#f92672"), hex("#e6db74")),
            h2_gradient: (hex("#66d9ef"), hex("#a6e22e")),
            h3_color: hex("#fd971f"),
            h4_color: hex("#ae81ff"),
            h1_separator: hex("#75715e"),
            code_bg: hex("#1a1a17"),
            code_border: hex("#75715e"),
            code_label: hex("#a6e22e"),
            text: hex("#f8f8f2"),
            text_dim: hex("#75715e"),
            bold: hex("#f92672"),
            italic: hex("#66d9ef"),
            strikethrough: hex("#75715e"),
            inline_code_fg: hex("#ae81ff"),
            inline_code_bg: hex("#3e3d32"),
            link: hex("#66d9ef"),
            link_url: hex("#75715e"),
            list_marker: hex("#a6e22e"),
            task_done: hex("#a6e22e"),
            task_pending: hex("#75715e"),
            blockquote_border: hex("#f92672"),
            blockquote_text: hex("#e8e8e2"),
            table_header_bg: hex("#3e3d32"),
            table_border: hex("#75715e"),
            table_row_alt: hex("#1e1f1c"),
            hr: hex("#75715e"),
            status_bar_bg: hex("#131411"),
            status_bar_fg: hex("#f8f8f2"),
            status_bar_accent: hex("#f92672"),
            search_match_bg: hex("#a6e22e"),
            search_current_bg: hex("#e6db74"),
            h1_sep_char: "━",
            h2_sep_char: "╸",
            list_markers: &["▶", "▷", "▸", "▹"],
            blockquote_char: "┃",
            hr_char: "━",
            status_icon: "★",
            status_icon_nerd: "󱓻",
        }
    }
}
