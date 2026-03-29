use ratatui::style::{Color, Modifier};
use ratatui::text::Text;

/// Convert ratatui Text to ANSI-colored string for stdout output.
/// Used by `--raw` mode to serve as a previewer for tools like yazi/fzf.
pub fn text_to_ansi(text: &Text<'_>) -> String {
    let mut out = String::new();

    for line in &text.lines {
        for span in &line.spans {
            let style = span.style;
            let mut codes: Vec<String> = Vec::new();

            if let Some(fg) = style.fg {
                if let Color::Rgb(r, g, b) = fg {
                    codes.push(format!("38;2;{};{};{}", r, g, b));
                }
            }

            if let Some(bg) = style.bg {
                if let Color::Rgb(r, g, b) = bg {
                    codes.push(format!("48;2;{};{};{}", r, g, b));
                }
            }

            if style.add_modifier.contains(Modifier::BOLD) {
                codes.push("1".into());
            }
            if style.add_modifier.contains(Modifier::ITALIC) {
                codes.push("3".into());
            }
            if style.add_modifier.contains(Modifier::UNDERLINED) {
                codes.push("4".into());
            }
            if style.add_modifier.contains(Modifier::CROSSED_OUT) {
                codes.push("9".into());
            }

            if codes.is_empty() {
                out.push_str(&span.content);
            } else {
                out.push_str(&format!("\x1b[{}m{}\x1b[0m", codes.join(";"), span.content));
            }
        }
        out.push('\n');
    }

    out
}
