use comrak::nodes::{AstNode, ListType, NodeCode, NodeCodeBlock, NodeHeading, NodeList, NodeValue};
use comrak::{parse_document, Arena, Options};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use unicode_width::UnicodeWidthStr;

use crate::theme::{self, Theme};

/// Render markdown string into ratatui Text for display
pub fn render_markdown(source: &str, width: u16, theme: &Theme) -> Text<'static> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;

    let root = parse_document(&arena, source, &options);
    let mut lines: Vec<Line<'static>> = Vec::new();

    render_node(root, &mut lines, width, theme, 0);

    Text::from(lines)
}

fn render_node<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    width: u16,
    theme: &Theme,
    depth: usize,
) {
    let val = node.data.borrow().value.clone();

    match &val {
        NodeValue::Document => {
            for child in node.children() {
                render_node(child, lines, width, theme, depth);
            }
        }
        NodeValue::Heading(NodeHeading { level, .. }) => {
            lines.push(Line::default());

            let heading_text = collect_text(node);
            let lvl = *level as usize;

            match lvl {
                1 => {
                    let spans = theme::gradient_spans(
                        &heading_text,
                        theme.h1_gradient.0,
                        theme.h1_gradient.1,
                        true,
                    );
                    lines.push(Line::from(spans));
                    let sep_width = (width as usize).min(heading_text.width() + 10).max(20);
                    let sep: String = "━".repeat(sep_width);
                    let sep_spans = theme::gradient_spans(
                        &sep,
                        theme.h1_gradient.0,
                        theme.h1_gradient.1,
                        false,
                    );
                    lines.push(Line::from(sep_spans));
                }
                2 => {
                    let spans = theme::gradient_spans(
                        &heading_text,
                        theme.h2_gradient.0,
                        theme.h2_gradient.1,
                        true,
                    );
                    lines.push(Line::from(spans));
                    let sep_width = (width as usize / 2).min(heading_text.width() + 6).max(10);
                    let sep: String = "─ ".repeat(sep_width / 2);
                    lines.push(Line::from(Span::styled(
                        sep,
                        Style::default().fg(theme.h1_separator),
                    )));
                }
                3 => {
                    let prefix = Span::styled(
                        "### ",
                        Style::default()
                            .fg(theme.h3_color)
                            .add_modifier(Modifier::BOLD),
                    );
                    let text = Span::styled(
                        heading_text,
                        Style::default()
                            .fg(theme.h3_color)
                            .add_modifier(Modifier::BOLD),
                    );
                    lines.push(Line::from(vec![prefix, text]));
                }
                _ => {
                    let marker = "#".repeat(lvl);
                    let prefix = Span::styled(
                        format!("{} ", marker),
                        Style::default()
                            .fg(theme.h4_color)
                            .add_modifier(Modifier::BOLD),
                    );
                    let text = Span::styled(
                        heading_text,
                        Style::default()
                            .fg(theme.h4_color)
                            .add_modifier(Modifier::BOLD),
                    );
                    lines.push(Line::from(vec![prefix, text]));
                }
            }
            lines.push(Line::default());
        }
        NodeValue::Paragraph => {
            let spans = collect_inline_spans(node, theme);
            let wrapped = wrap_spans(spans, width as usize);
            for line in wrapped {
                lines.push(line);
            }
            lines.push(Line::default());
        }
        NodeValue::CodeBlock(NodeCodeBlock { info, literal, .. }) => {
            render_code_block(lines, info, literal, width, theme);
            lines.push(Line::default());
        }
        NodeValue::List(list) => {
            render_list(node, lines, width, theme, list, depth);
            if depth == 0 {
                lines.push(Line::default());
            }
        }
        NodeValue::Item(_) => {}
        NodeValue::BlockQuote => {
            render_blockquote(node, lines, width, theme, depth);
        }
        NodeValue::ThematicBreak => {
            let w = (width as usize).min(60);
            let mut hr_text = String::new();
            for i in 0..w {
                let mid = w / 2;
                let dist = if i > mid { i - mid } else { mid - i };
                if dist < w / 6 {
                    hr_text.push('━');
                } else if dist < w / 3 {
                    hr_text.push('─');
                } else if i % 2 == 0 {
                    hr_text.push('─');
                } else {
                    hr_text.push(' ');
                }
            }
            lines.push(Line::from(Span::styled(
                hr_text,
                Style::default().fg(theme.hr),
            )));
            lines.push(Line::default());
        }
        NodeValue::Table(..) => {
            render_table(node, lines, theme);
            lines.push(Line::default());
        }
        NodeValue::SoftBreak | NodeValue::LineBreak => {}
        _ => {
            for child in node.children() {
                render_node(child, lines, width, theme, depth);
            }
        }
    }
}

fn collect_text<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    collect_text_inner(node, &mut text);
    text
}

fn collect_text_inner<'a>(node: &'a AstNode<'a>, buf: &mut String) {
    let val = node.data.borrow().value.clone();
    match &val {
        NodeValue::Text(t) => buf.push_str(t),
        NodeValue::Code(NodeCode { literal, .. }) => buf.push_str(literal),
        NodeValue::SoftBreak | NodeValue::LineBreak => buf.push(' '),
        _ => {
            for child in node.children() {
                collect_text_inner(child, buf);
            }
        }
    }
}

fn collect_inline_spans<'a>(node: &'a AstNode<'a>, theme: &Theme) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    collect_inline_inner(node, &mut spans, theme, Style::default().fg(theme.text));
    spans
}

fn collect_inline_inner<'a>(
    node: &'a AstNode<'a>,
    spans: &mut Vec<Span<'static>>,
    theme: &Theme,
    parent_style: Style,
) {
    for child in node.children() {
        let val = child.data.borrow().value.clone();
        match &val {
            NodeValue::Text(t) => {
                spans.push(Span::styled(t.clone(), parent_style));
            }
            NodeValue::Code(NodeCode { literal, .. }) => {
                spans.push(Span::styled(
                    format!(" {} ", literal),
                    Style::default()
                        .fg(theme.inline_code_fg)
                        .bg(theme.inline_code_bg),
                ));
            }
            NodeValue::Strong => {
                let style = parent_style.fg(theme.bold).add_modifier(Modifier::BOLD);
                collect_inline_inner(child, spans, theme, style);
            }
            NodeValue::Emph => {
                let style = parent_style.fg(theme.italic).add_modifier(Modifier::ITALIC);
                collect_inline_inner(child, spans, theme, style);
            }
            NodeValue::Strikethrough => {
                let style = parent_style
                    .fg(theme.strikethrough)
                    .add_modifier(Modifier::CROSSED_OUT);
                collect_inline_inner(child, spans, theme, style);
            }
            NodeValue::Link(link) => {
                let link_text = collect_text(child);
                let display = if link_text.is_empty() {
                    link.url.clone()
                } else {
                    link_text
                };
                spans.push(Span::styled(
                    display,
                    Style::default()
                        .fg(theme.link)
                        .add_modifier(Modifier::UNDERLINED),
                ));
                if !link.url.is_empty() {
                    spans.push(Span::styled(
                        format!(" ({})", link.url),
                        Style::default().fg(theme.link_url),
                    ));
                }
            }
            NodeValue::Image(link) => {
                let alt = collect_text(child);
                let label = if !alt.is_empty() {
                    alt
                } else if !link.title.is_empty() {
                    link.title.clone()
                } else {
                    link.url.clone()
                };
                spans.push(Span::styled(
                    format!("[Image: {}]", label),
                    Style::default()
                        .fg(theme.link)
                        .add_modifier(Modifier::ITALIC),
                ));
            }
            NodeValue::SoftBreak => {
                spans.push(Span::raw(" "));
            }
            NodeValue::LineBreak => {
                spans.push(Span::raw(" "));
            }
            _ => {
                collect_inline_inner(child, spans, theme, parent_style);
            }
        }
    }
}

fn render_code_block(
    lines: &mut Vec<Line<'static>>,
    info: &str,
    literal: &str,
    width: u16,
    theme: &Theme,
) {
    let lang = info.split_whitespace().next().unwrap_or("");
    let highlighted = highlight_code(literal, lang, theme);

    let content_max_width = highlighted
        .iter()
        .map(|spans| spans.iter().map(|s| s.content.width()).sum::<usize>())
        .max()
        .unwrap_or(0);
    let box_width = (content_max_width + 6).max(lang.len() + 10).min(width as usize);

    let border_style = Style::default().fg(theme.code_border);
    let label_style = Style::default()
        .fg(theme.code_label)
        .add_modifier(Modifier::BOLD);
    let bg = theme.code_bg;

    // Top border: ╭─── lang ───────╮
    let remaining = box_width.saturating_sub(8 + lang.len());
    let top_rest = "─".repeat(remaining);

    let top_spans = vec![
        Span::styled("  ╭─── ", border_style),
        Span::styled(lang.to_string(), label_style),
        Span::styled(format!(" {}╮", top_rest), border_style),
    ];
    lines.push(Line::from(top_spans));

    // Empty line after top border
    let empty_pad = " ".repeat(box_width.saturating_sub(4));
    lines.push(Line::from(vec![
        Span::styled("  │ ", border_style),
        Span::styled(empty_pad.clone(), Style::default().bg(bg)),
        Span::styled(" │", border_style),
    ]));

    // Code lines
    for code_spans in &highlighted {
        let mut line_spans: Vec<Span<'static>> = vec![Span::styled("  │ ", border_style)];
        let mut content_width: usize = 0;
        for span in code_spans {
            content_width += span.content.width();
            line_spans.push(Span::styled(span.content.to_string(), span.style.bg(bg)));
        }
        let padding = box_width.saturating_sub(content_width + 4);
        if padding > 0 {
            line_spans.push(Span::styled(" ".repeat(padding), Style::default().bg(bg)));
        }
        line_spans.push(Span::styled(" │", border_style));
        lines.push(Line::from(line_spans));
    }

    // Empty line before bottom border
    lines.push(Line::from(vec![
        Span::styled("  │ ", border_style),
        Span::styled(empty_pad, Style::default().bg(bg)),
        Span::styled(" │", border_style),
    ]));

    // Bottom border: ╰───────────────╯
    let bottom = format!("  ╰{}╯", "─".repeat(box_width.saturating_sub(4)));
    lines.push(Line::from(Span::styled(bottom, border_style)));
}

fn highlight_code(code: &str, lang: &str, _theme: &Theme) -> Vec<Vec<Span<'static>>> {
    use syntect::easy::HighlightLines;
    use syntect::highlighting::ThemeSet;
    use syntect::parsing::SyntaxSet;

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());
    let syntect_theme = &ts.themes["base16-ocean.dark"];
    let mut h = HighlightLines::new(syntax, syntect_theme);

    let code = code.trim_end_matches('\n');
    code.lines()
        .map(|line| {
            match h.highlight_line(line, &ss) {
                Ok(ranges) => ranges
                    .iter()
                    .map(|(style, text)| {
                        let fg =
                            Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                        Span::styled(text.to_string(), Style::default().fg(fg))
                    })
                    .collect(),
                Err(_) => vec![Span::raw(line.to_string())],
            }
        })
        .collect()
}

fn render_list<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    width: u16,
    theme: &Theme,
    list: &NodeList,
    depth: usize,
) {
    let indent = "  ".repeat(depth + 1);
    let mut index = list.start;

    for child in node.children() {
        let child_val = child.data.borrow().value.clone();
        if let NodeValue::Item(_) = &child_val {
            let marker = if list.list_type == ListType::Ordered {
                let m = format!("{}. ", index);
                index += 1;
                Span::styled(
                    format!("{}{}", indent, m),
                    Style::default().fg(theme.list_marker),
                )
            } else {
                let bullet = match depth {
                    0 => "●",
                    1 => "○",
                    _ => "■",
                };
                Span::styled(
                    format!("{}{} ", indent, bullet),
                    Style::default().fg(theme.list_marker),
                )
            };

            let mut item_spans: Vec<Span<'static>> = vec![marker];
            let mut is_first = true;
            for item_child in child.children() {
                let item_val = item_child.data.borrow().value.clone();
                match &item_val {
                    NodeValue::Paragraph => {
                        let para_spans = collect_inline_spans(item_child, theme);
                        if is_first {
                            item_spans.extend(para_spans);
                        } else {
                            let cont_indent = "  ".repeat(depth + 2);
                            let mut indented: Vec<Span<'static>> = vec![Span::raw(cont_indent)];
                            indented.extend(para_spans);
                            lines.push(Line::from(item_spans));
                            item_spans = indented;
                        }
                        is_first = false;
                    }
                    NodeValue::List(sub_list) => {
                        if !item_spans.is_empty() {
                            lines.push(Line::from(item_spans));
                            item_spans = Vec::new();
                        }
                        render_list(item_child, lines, width, theme, sub_list, depth + 1);
                    }
                    _ => {
                        let inner_spans = collect_inline_spans(item_child, theme);
                        item_spans.extend(inner_spans);
                    }
                }
            }
            if !item_spans.is_empty() {
                lines.push(Line::from(item_spans));
            }
        }
    }
}

fn render_blockquote<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    width: u16,
    theme: &Theme,
    depth: usize,
) {
    let border_char = "▌ ";

    let mut inner_lines: Vec<Line<'static>> = Vec::new();
    for child in node.children() {
        render_node(
            child,
            &mut inner_lines,
            width.saturating_sub(3),
            theme,
            depth + 1,
        );
    }

    let border_colors = [theme.blockquote_border, theme.h3_color, theme.h4_color];
    let border_color = border_colors.get(depth).copied().unwrap_or(theme.text_dim);

    for inner_line in inner_lines {
        let mut spans: Vec<Span<'static>> = vec![
            Span::styled("  ".repeat(depth), Style::default()),
            Span::styled(border_char, Style::default().fg(border_color)),
        ];
        spans.extend(inner_line.spans.into_iter().map(|s| {
            if depth == 0 {
                Span::styled(
                    s.content.into_owned(),
                    s.style.fg(theme.blockquote_text),
                )
            } else {
                Span::styled(s.content.into_owned(), s.style)
            }
        }));
        lines.push(Line::from(spans));
    }
}

fn render_table<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    theme: &Theme,
) {
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut header_row = 0;

    for child in node.children() {
        let child_val = child.data.borrow().value.clone();
        if let NodeValue::TableRow(is_h) = &child_val {
            let mut row: Vec<String> = Vec::new();
            for cell in child.children() {
                row.push(collect_text(cell));
            }
            if *is_h {
                header_row = rows.len();
            }
            rows.push(row);
        }
    }

    if rows.is_empty() {
        return;
    }

    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let mut col_widths: Vec<usize> = vec![0; num_cols];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                col_widths[i] = col_widths[i].max(cell.width());
            }
        }
    }

    let indent = "  ";
    let border_style = Style::default().fg(theme.table_border);
    let header_style = Style::default()
        .fg(theme.text)
        .bg(theme.table_header_bg)
        .add_modifier(Modifier::BOLD);

    // Top border
    let top: String = col_widths
        .iter()
        .map(|w| "─".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("┬");
    lines.push(Line::from(Span::styled(
        format!("{}┌{}┐", indent, top),
        border_style,
    )));

    for (row_idx, row) in rows.iter().enumerate() {
        let is_header_row = row_idx == header_row && row_idx == 0;
        let style = if is_header_row {
            header_style
        } else if row_idx % 2 == 0 {
            Style::default().fg(theme.text).bg(theme.table_row_alt)
        } else {
            Style::default().fg(theme.text)
        };

        let mut spans: Vec<Span<'static>> =
            vec![Span::styled(format!("{}│", indent), border_style)];
        for (i, cell) in row.iter().enumerate() {
            let w = col_widths.get(i).copied().unwrap_or(0);
            let padded = format!(" {:width$} ", cell, width = w);
            spans.push(Span::styled(padded, style));
            spans.push(Span::styled("│", border_style));
        }
        for i in row.len()..num_cols {
            let w = col_widths.get(i).copied().unwrap_or(0);
            spans.push(Span::styled(" ".repeat(w + 2), style));
            spans.push(Span::styled("│", border_style));
        }
        lines.push(Line::from(spans));

        if is_header_row {
            let sep: String = col_widths
                .iter()
                .map(|w| "─".repeat(w + 2))
                .collect::<Vec<_>>()
                .join("┼");
            lines.push(Line::from(Span::styled(
                format!("{}├{}┤", indent, sep),
                border_style,
            )));
        }
    }

    // Bottom border
    let bottom: String = col_widths
        .iter()
        .map(|w| "─".repeat(w + 2))
        .collect::<Vec<_>>()
        .join("┴");
    lines.push(Line::from(Span::styled(
        format!("{}└{}┘", indent, bottom),
        border_style,
    )));
}

fn wrap_spans(spans: Vec<Span<'static>>, max_width: usize) -> Vec<Line<'static>> {
    if max_width == 0 {
        return vec![Line::from(spans)];
    }

    let mut result: Vec<Line<'static>> = Vec::new();
    let mut current_line: Vec<Span<'static>> = Vec::new();
    let mut current_width: usize = 0;

    let padding = "  ";
    let effective_width = max_width.saturating_sub(2);

    for span in spans {
        let text = span.content.to_string();
        let style = span.style;

        if current_width + text.width() <= effective_width {
            if current_line.is_empty() {
                current_line.push(Span::raw(padding.to_string()));
            }
            current_width += text.width();
            current_line.push(Span::styled(text, style));
        } else {
            let mut remaining = text.as_str();
            while !remaining.is_empty() {
                let available = effective_width.saturating_sub(current_width);
                if available == 0 {
                    result.push(Line::from(current_line));
                    current_line = vec![Span::raw(padding.to_string())];
                    current_width = 0;
                    continue;
                }

                if remaining.width() <= available {
                    current_line.push(Span::styled(remaining.to_string(), style));
                    current_width += remaining.width();
                    break;
                }

                let split = find_split_point(remaining, available);
                if split == 0 && current_width == 0 {
                    let (first, rest) = split_at_width(remaining, available);
                    current_line.push(Span::styled(first.to_string(), style));
                    result.push(Line::from(current_line));
                    current_line = vec![Span::raw(padding.to_string())];
                    current_width = 0;
                    remaining = rest;
                } else if split == 0 {
                    result.push(Line::from(current_line));
                    current_line = vec![Span::raw(padding.to_string())];
                    current_width = 0;
                } else {
                    let (first, rest) = remaining.split_at(split);
                    current_line.push(Span::styled(first.to_string(), style));
                    result.push(Line::from(current_line));
                    current_line = vec![Span::raw(padding.to_string())];
                    current_width = 0;
                    remaining = rest;
                }
            }
        }
    }

    if !current_line.is_empty() {
        result.push(Line::from(current_line));
    }

    if result.is_empty() {
        result.push(Line::default());
    }

    result
}

fn find_split_point(s: &str, max_width: usize) -> usize {
    let mut last_space = 0;
    let mut width = 0;
    for (i, ch) in s.char_indices() {
        width += UnicodeWidthStr::width(ch.to_string().as_str());
        if width > max_width {
            return last_space;
        }
        if ch == ' ' {
            last_space = i + 1;
        }
    }
    s.len()
}

fn split_at_width(s: &str, max_width: usize) -> (&str, &str) {
    let mut width = 0;
    for (i, ch) in s.char_indices() {
        width += UnicodeWidthStr::width(ch.to_string().as_str());
        if width > max_width {
            return (&s[..i], &s[i..]);
        }
    }
    (s, "")
}
