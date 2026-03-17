use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};

use crate::theme::Theme;

pub struct SearchState {
    pub query: String,
    pub input: String,
    pub active: bool,
    pub matches: Vec<SearchMatch>,
    pub current_match: usize,
}

#[derive(Clone)]
pub struct SearchMatch {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            input: String::new(),
            active: false,
            matches: Vec::new(),
            current_match: 0,
        }
    }

    pub fn start_search(&mut self) {
        self.active = true;
        self.input.clear();
    }

    pub fn cancel_search(&mut self) {
        self.active = false;
        self.input.clear();
    }

    pub fn confirm_search(&mut self) {
        self.active = false;
        self.query = self.input.clone();
        if !self.matches.is_empty() {
            self.current_match = 0;
        }
    }

    pub fn clear_search(&mut self) {
        self.query.clear();
        self.input.clear();
        self.matches.clear();
        self.current_match = 0;
        self.active = false;
    }

    pub fn next_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.matches.len();
        }
    }

    pub fn prev_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = if self.current_match == 0 {
                self.matches.len() - 1
            } else {
                self.current_match - 1
            };
        }
    }

    pub fn current_match_line(&self) -> Option<usize> {
        self.matches.get(self.current_match).map(|m| m.line)
    }

    pub fn match_info(&self) -> String {
        if self.matches.is_empty() {
            if !self.query.is_empty() {
                return "No matches".to_string();
            }
            return String::new();
        }
        format!("{}/{}", self.current_match + 1, self.matches.len())
    }

    /// Find all matches in the rendered text
    pub fn find_matches(&mut self, text: &Text<'static>) {
        self.matches.clear();
        let query = if self.active {
            &self.input
        } else {
            &self.query
        };

        if query.is_empty() {
            return;
        }

        let query_lower = query.to_lowercase();

        for (line_idx, line) in text.lines.iter().enumerate() {
            let line_text: String = line.spans.iter().map(|s| s.content.as_ref()).collect();
            let line_lower = line_text.to_lowercase();
            let query_char_len = query_lower.chars().count();

            // Build byte-offset to char-index mapping
            let byte_to_char: Vec<usize> = line_lower
                .char_indices()
                .map(|(_, _)| 0)
                .collect::<Vec<_>>();
            let _ = byte_to_char;

            let chars: Vec<char> = line_lower.chars().collect();
            let char_count = chars.len();

            // Search by char index to avoid byte/char mismatch
            for i in 0..char_count {
                if i + query_char_len > char_count {
                    break;
                }
                let window: String = chars[i..i + query_char_len].iter().collect();
                if window == query_lower {
                    self.matches.push(SearchMatch {
                        line: line_idx,
                        start: i,
                        end: i + query_char_len,
                    });
                }
            }
        }
    }

    /// Apply search highlights to a line
    pub fn highlight_line(
        &self,
        line: &Line<'static>,
        line_idx: usize,
        theme: &Theme,
    ) -> Line<'static> {
        let relevant_matches: Vec<&SearchMatch> = self
            .matches
            .iter()
            .filter(|m| m.line == line_idx)
            .collect();

        if relevant_matches.is_empty() {
            return line.clone();
        }

        // Flatten all spans into a single string with style info
        let mut char_styles: Vec<(char, Style)> = Vec::new();
        for span in &line.spans {
            for ch in span.content.chars() {
                char_styles.push((ch, span.style));
            }
        }

        // Apply highlights
        for (match_idx, search_match) in self.matches.iter().enumerate() {
            if search_match.line != line_idx {
                continue;
            }
            let is_current = match_idx == self.current_match;
            let bg = if is_current {
                theme.search_current_bg
            } else {
                theme.search_match_bg
            };
            let fg = Color::Black;

            for i in search_match.start..search_match.end.min(char_styles.len()) {
                char_styles[i].1 = Style::default()
                    .fg(fg)
                    .bg(bg)
                    .add_modifier(Modifier::BOLD);
            }
        }

        // Rebuild spans (group consecutive chars with same style)
        let mut new_spans: Vec<Span<'static>> = Vec::new();
        let mut current_text = String::new();
        let mut current_style = if char_styles.is_empty() {
            Style::default()
        } else {
            char_styles[0].1
        };

        for (ch, style) in &char_styles {
            if *style == current_style {
                current_text.push(*ch);
            } else {
                if !current_text.is_empty() {
                    new_spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                }
                current_style = *style;
                current_text.push(*ch);
            }
        }
        if !current_text.is_empty() {
            new_spans.push(Span::styled(current_text, current_style));
        }

        Line::from(new_spans)
    }
}
