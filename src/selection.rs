use std::io::Write;

use ratatui::text::Line;

/// Represents a text selection in the viewport
#[derive(Clone, Default)]
pub struct Selection {
    /// Start position (line, column) in viewport coordinates
    pub start: Option<(u16, u16)>,
    /// End position (line, column) in viewport coordinates
    pub end: Option<(u16, u16)>,
    /// Whether a drag is in progress
    pub dragging: bool,
}

impl Selection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_drag(&mut self, row: u16, col: u16) {
        self.start = Some((row, col));
        self.end = Some((row, col));
        self.dragging = true;
    }

    pub fn update_drag(&mut self, row: u16, col: u16) {
        if self.dragging {
            self.end = Some((row, col));
        }
    }

    pub fn end_drag(&mut self) {
        self.dragging = false;
    }

    pub fn clear(&mut self) {
        self.start = None;
        self.end = None;
        self.dragging = false;
    }

    pub fn has_selection(&self) -> bool {
        self.start.is_some() && self.end.is_some()
    }

    /// Get normalized (start <= end) selection range
    fn normalized(&self) -> Option<((u16, u16), (u16, u16))> {
        let start = self.start?;
        let end = self.end?;

        if start.0 < end.0 || (start.0 == end.0 && start.1 <= end.1) {
            Some((start, end))
        } else {
            Some((end, start))
        }
    }

    /// Check if a cell (row, col) is within the selection
    pub fn contains(&self, row: u16, col: u16) -> bool {
        let Some((start, end)) = self.normalized() else {
            return false;
        };

        if row < start.0 || row > end.0 {
            return false;
        }

        if start.0 == end.0 {
            // Single line selection
            col >= start.1 && col <= end.1
        } else if row == start.0 {
            col >= start.1
        } else if row == end.0 {
            col <= end.1
        } else {
            true // Middle lines are fully selected
        }
    }

    /// Extract selected text from visible lines
    pub fn extract_text(&self, visible_lines: &[Line<'_>], viewport_y: u16) -> String {
        let Some((start, end)) = self.normalized() else {
            return String::new();
        };

        let mut result = String::new();

        for row in start.0..=end.0 {
            let line_idx = (row - viewport_y) as usize;
            if line_idx >= visible_lines.len() {
                continue;
            }

            let line = &visible_lines[line_idx];
            let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            let chars: Vec<char> = line_text.chars().collect();

            let col_start = if row == start.0 {
                start.1 as usize
            } else {
                0
            };
            let col_end = if row == end.0 {
                (end.1 as usize + 1).min(chars.len())
            } else {
                chars.len()
            };

            let selected: String = chars
                .get(col_start..col_end)
                .unwrap_or(&[])
                .iter()
                .collect();
            result.push_str(&selected);

            if row < end.0 {
                result.push('\n');
            }
        }

        result
    }
}

/// Copy text to clipboard using OSC 52 escape sequence
/// This works in tmux, iTerm2, Kitty, WezTerm, Alacritty, etc.
pub fn copy_to_clipboard(text: &str) {
    use std::io::stdout;
    use base64::Engine;

    let encoded = base64::engine::general_purpose::STANDARD.encode(text);

    // OSC 52: \x1b]52;c;BASE64\x07
    // Use \x1b\\ as string terminator (works better in tmux)
    let osc = format!("\x1b]52;c;{}\x1b\\", encoded);

    let mut out = stdout().lock();
    let _ = out.write_all(osc.as_bytes());
    let _ = out.flush();
}
