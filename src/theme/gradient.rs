use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Interpolate between two RGB colors in linear space
fn lerp_color(from: Color, to: Color, t: f64) -> Color {
    let (r1, g1, b1) = match from {
        Color::Rgb(r, g, b) => (r as f64, g as f64, b as f64),
        _ => (255.0, 255.0, 255.0),
    };
    let (r2, g2, b2) = match to {
        Color::Rgb(r, g, b) => (r as f64, g as f64, b as f64),
        _ => (255.0, 255.0, 255.0),
    };

    // Use gamma-corrected interpolation for more perceptually uniform gradients
    let gamma = 2.2_f64;
    let r = (lerp(r1.powf(gamma), r2.powf(gamma), t)).powf(1.0 / gamma);
    let g = (lerp(g1.powf(gamma), g2.powf(gamma), t)).powf(1.0 / gamma);
    let b = (lerp(b1.powf(gamma), b2.powf(gamma), t)).powf(1.0 / gamma);

    Color::Rgb(r.clamp(0.0, 255.0) as u8, g.clamp(0.0, 255.0) as u8, b.clamp(0.0, 255.0) as u8)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Create gradient-colored spans from text, each character gets an interpolated color
pub fn gradient_spans<'a>(text: &str, from: Color, to: Color, bold: bool) -> Vec<Span<'a>> {
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    if len == 0 {
        return vec![];
    }
    if len == 1 {
        let mut style = Style::default().fg(from);
        if bold {
            style = style.add_modifier(Modifier::BOLD);
        }
        return vec![Span::styled(chars[0].to_string(), style)];
    }

    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| {
            let t = i as f64 / (len - 1) as f64;
            let color = lerp_color(from, to, t);
            let mut style = Style::default().fg(color);
            if bold {
                style = style.add_modifier(Modifier::BOLD);
            }
            Span::styled(ch.to_string(), style)
        })
        .collect()
}
