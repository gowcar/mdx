use std::sync::OnceLock;

use comrak::nodes::{AstNode, ListType, NodeCode, NodeCodeBlock, NodeHeading, NodeList, NodeValue};
use comrak::{parse_document, Arena, Options};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use unicode_width::UnicodeWidthStr;

use crate::theme::{self, Theme};

fn syntax_set() -> &'static SyntaxSet {
    static SS: OnceLock<SyntaxSet> = OnceLock::new();
    SS.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme_set() -> &'static ThemeSet {
    static TS: OnceLock<ThemeSet> = OnceLock::new();
    TS.get_or_init(ThemeSet::load_defaults)
}

/// Render markdown string into ratatui Text for display
pub fn render_markdown(source: &str, width: u16, theme: &Theme, wrap_code: bool) -> Text<'static> {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;

    let root = parse_document(&arena, source, &options);
    let mut lines: Vec<Line<'static>> = Vec::new();

    render_node(root, &mut lines, width, theme, 0, wrap_code);

    Text::from(lines)
}

fn render_node<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    width: u16,
    theme: &Theme,
    depth: usize,
    wrap_code: bool,
) {
    let val = node.data.borrow().value.clone();

    match &val {
        NodeValue::Document => {
            for child in node.children() {
                render_node(child, lines, width, theme, depth, wrap_code);
            }
        }
        NodeValue::Heading(NodeHeading { level, .. }) => {
            lines.push(Line::default());

            let heading_text = collect_text(node);
            let lvl = *level as usize;

            let text_lines = wrap_plain_text(&heading_text, width as usize);

            match lvl {
                1 => {
                    for t in text_lines {
                        let spans = theme::gradient_spans(
                            &t,
                            theme.h1_gradient.0,
                            theme.h1_gradient.1,
                            true,
                        );
                        lines.push(Line::from(spans));
                    }
                }
                2 => {
                    for t in text_lines {
                        let spans = theme::gradient_spans(
                            &t,
                            theme.h2_gradient.0,
                            theme.h2_gradient.1,
                            true,
                        );
                        lines.push(Line::from(spans));
                    }
                }
                3 => {
                    let style = Style::default()
                        .fg(theme.h3_color)
                        .add_modifier(Modifier::BOLD);
                    for t in text_lines {
                        lines.push(Line::from(Span::styled(t, style)));
                    }
                }
                4 => {
                    let style = Style::default()
                        .fg(theme.h4_color)
                        .add_modifier(Modifier::BOLD);
                    for t in text_lines {
                        lines.push(Line::from(Span::styled(t, style)));
                    }
                }
                _ => {
                    let style = Style::default()
                        .fg(theme.h4_color)
                        .add_modifier(Modifier::ITALIC | Modifier::BOLD);
                    for t in text_lines {
                        lines.push(Line::from(Span::styled(t, style)));
                    }
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
            render_code_block(lines, info, literal, width, theme, wrap_code);
            lines.push(Line::default());
        }
        NodeValue::List(list) => {
            render_list(node, lines, width, theme, list, depth, wrap_code);
            if depth == 0 {
                lines.push(Line::default());
            }
        }
        NodeValue::Item(_) => {}
        NodeValue::BlockQuote => {
            render_blockquote(node, lines, width, theme, depth, wrap_code);
        }
        NodeValue::ThematicBreak => {
            let w = (width as usize).min(60);
            let hr_text = theme.hr_char.repeat(w);
            lines.push(Line::from(Span::styled(
                hr_text,
                Style::default().fg(theme.hr),
            )));
            lines.push(Line::default());
        }
        NodeValue::Table(..) => {
            render_table(node, lines, width, theme);
            lines.push(Line::default());
        }
        NodeValue::SoftBreak | NodeValue::LineBreak => {}
        _ => {
            for child in node.children() {
                render_node(child, lines, width, theme, depth, wrap_code);
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
    wrap_code: bool,
) {
    let lang = info.split_whitespace().next().unwrap_or("");
    let highlighted = highlight_code(literal, lang, theme);

    let content_max_width = highlighted
        .iter()
        .map(|spans| spans.iter().map(|s| s.content.width()).sum::<usize>())
        .max()
        .unwrap_or(0);
    let box_width = if wrap_code {
        (width as usize).max(lang.len() + 10)
    } else {
        (content_max_width + 6).max(lang.len() + 10).min(width as usize)
    };

    let border_style = Style::default().fg(theme.code_border);
    let label_style = Style::default()
        .fg(theme.code_label)
        .add_modifier(Modifier::BOLD);
    let bg = theme.code_bg;

    // Top border: ╭─── lang ───────╮  or  ╭────────────────╮
    if lang.is_empty() {
        let fill = "─".repeat(box_width.saturating_sub(2));
        lines.push(Line::from(Span::styled(format!("  ╭{}╮", fill), border_style)));
    } else {
        let remaining = box_width.saturating_sub(7 + lang.len());
        let top_rest = "─".repeat(remaining);
        let top_spans = vec![
            Span::styled("  ╭─── ", border_style),
            Span::styled(lang.to_string(), label_style),
            Span::styled(format!(" {}╮", top_rest), border_style),
        ];
        lines.push(Line::from(top_spans));
    }

    // Empty line after top border
    let empty_pad = " ".repeat(box_width.saturating_sub(4));
    lines.push(Line::from(vec![
        Span::styled("  │ ", border_style),
        Span::styled(empty_pad.clone(), Style::default().bg(bg)),
        Span::styled(" │", border_style),
    ]));

    let inner_width = box_width.saturating_sub(4); // content area between "│ " and " │"

    let emit_row = |lines: &mut Vec<Line<'static>>,
                    content: Vec<Span<'static>>,
                    used: usize| {
        let mut row: Vec<Span<'static>> = Vec::with_capacity(content.len() + 3);
        row.push(Span::styled("  │ ", border_style));
        row.extend(content);
        let pad = inner_width.saturating_sub(used);
        if pad > 0 {
            row.push(Span::styled(" ".repeat(pad), Style::default().bg(bg)));
        }
        row.push(Span::styled(" │", border_style));
        lines.push(Line::from(row));
    };

    for code_spans in &highlighted {
        if wrap_code {
            // Wrap spans across multiple box rows
            let mut current: Vec<Span<'static>> = Vec::new();
            let mut used: usize = 0;
            for span in code_spans {
                let style = span.style.bg(bg);
                let mut chunk = String::new();
                let mut chunk_w: usize = 0;
                for ch in span.content.chars() {
                    let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
                    if used + chunk_w + cw > inner_width {
                        if !chunk.is_empty() {
                            current.push(Span::styled(std::mem::take(&mut chunk), style));
                            used += chunk_w;
                            chunk_w = 0;
                        }
                        emit_row(lines, std::mem::take(&mut current), used);
                        used = 0;
                    }
                    chunk.push(ch);
                    chunk_w += cw;
                }
                if !chunk.is_empty() {
                    current.push(Span::styled(chunk, style));
                    used += chunk_w;
                }
            }
            emit_row(lines, current, used);
        } else {
            // Truncate: clip content to inner_width with "…"
            let mut content: Vec<Span<'static>> = Vec::new();
            let mut used: usize = 0;
            for span in code_spans {
                let span_w = span.content.width();
                if used + span_w <= inner_width {
                    content.push(Span::styled(span.content.to_string(), span.style.bg(bg)));
                    used += span_w;
                } else {
                    let remaining = inner_width.saturating_sub(used + 1);
                    let mut partial = String::new();
                    let mut pw = 0;
                    for ch in span.content.chars() {
                        let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
                        if pw + cw > remaining {
                            break;
                        }
                        partial.push(ch);
                        pw += cw;
                    }
                    partial.push('…');
                    used += pw + 1;
                    content.push(Span::styled(partial, span.style.bg(bg)));
                    break;
                }
            }
            emit_row(lines, content, used);
        }
    }

    // Empty line before bottom border
    lines.push(Line::from(vec![
        Span::styled("  │ ", border_style),
        Span::styled(empty_pad, Style::default().bg(bg)),
        Span::styled(" │", border_style),
    ]));

    // Bottom border: ╰───────────────╯
    let bottom = format!("  ╰{}╯", "─".repeat(box_width.saturating_sub(2)));
    lines.push(Line::from(Span::styled(bottom, border_style)));
}

fn highlight_code(code: &str, lang: &str, _theme: &Theme) -> Vec<Vec<Span<'static>>> {
    use syntect::easy::HighlightLines;

    let ss = syntax_set();
    let ts = theme_set();
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
    wrap_code: bool,
) {
    let indent = "  ".repeat(depth + 1);
    let mut index = list.start;

    for child in node.children() {
        let child_val = child.data.borrow().value.clone();
        // Handle both Item and TaskItem
        let task_check = match &child_val {
            NodeValue::TaskItem(ch) => *ch,
            _ => None,
        };
        let is_list_item = matches!(&child_val, NodeValue::Item(_) | NodeValue::TaskItem(_));
        if is_list_item {
            let marker = if let Some(ch) = task_check {
                // Task list item: ☑ or ☐
                let (icon, color) = if ch == 'x' || ch == 'X' {
                    ("✓ ", theme.task_done)
                } else {
                    ("○ ", theme.task_pending)
                };
                Span::styled(
                    format!("{}{}", indent, icon),
                    Style::default().fg(color),
                )
            } else if list.list_type == ListType::Ordered {
                let m = format!("{}. ", index);
                index += 1;
                Span::styled(
                    format!("{}{}", indent, m),
                    Style::default().fg(theme.list_marker),
                )
            } else {
                let markers = theme.list_markers;
                let bullet = markers[depth.min(markers.len() - 1)];
                Span::styled(
                    format!("{}{} ", indent, bullet),
                    Style::default().fg(theme.list_marker),
                )
            };

            let marker_text: String = marker.content.to_string();
            let marker_width = marker_text.width();
            let cont_indent = " ".repeat(marker_width);

            let mut item_spans: Vec<Span<'static>> = vec![marker];
            let mut first_indent: String = String::new();
            let mut is_first = true;
            let flush = |spans: Vec<Span<'static>>,
                         lines: &mut Vec<Line<'static>>,
                         first_ind: &str,
                         cont_ind: &str| {
                for line in
                    wrap_spans_indented(spans, width as usize, first_ind, cont_ind)
                {
                    lines.push(line);
                }
            };

            for item_child in child.children() {
                let item_val = item_child.data.borrow().value.clone();
                match &item_val {
                    NodeValue::Paragraph => {
                        let para_spans = collect_inline_spans(item_child, theme);
                        if is_first {
                            item_spans.extend(para_spans);
                        } else {
                            flush(
                                std::mem::take(&mut item_spans),
                                lines,
                                &first_indent,
                                &cont_indent,
                            );
                            item_spans = para_spans;
                            first_indent = cont_indent.clone();
                        }
                        is_first = false;
                    }
                    NodeValue::List(sub_list) => {
                        if !item_spans.is_empty() {
                            flush(
                                std::mem::take(&mut item_spans),
                                lines,
                                &first_indent,
                                &cont_indent,
                            );
                        }
                        render_list(item_child, lines, width, theme, sub_list, depth + 1, wrap_code);
                    }
                    _ => {
                        let inner_spans = collect_inline_spans(item_child, theme);
                        item_spans.extend(inner_spans);
                    }
                }
            }
            if !item_spans.is_empty() {
                flush(item_spans, lines, &first_indent, &cont_indent);
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
    wrap_code: bool,
) {
    let border_char = format!("{} ", theme.blockquote_char);

    let mut inner_lines: Vec<Line<'static>> = Vec::new();
    for child in node.children() {
        render_node(
            child,
            &mut inner_lines,
            width.saturating_sub(3),
            theme,
            depth + 1,
            wrap_code,
        );
    }

    let border_colors = [theme.blockquote_border, theme.h3_color, theme.h4_color];
    let border_color = border_colors.get(depth).copied().unwrap_or(theme.text_dim);

    for inner_line in inner_lines {
        let mut spans: Vec<Span<'static>> = vec![
            Span::styled("  ".repeat(depth), Style::default()),
            Span::styled(border_char.clone(), Style::default().fg(border_color)),
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

/// Pad a string to `target_width` display columns using unicode width.
/// If the string is wider than target_width, truncate with "…".
fn pad_cell(text: &str, target_width: usize) -> String {
    let display_width = text.width();
    if display_width <= target_width {
        // Pad with spaces to fill remaining width
        let padding = target_width - display_width;
        format!("{}{}", text, " ".repeat(padding))
    } else {
        // Truncate: find the longest prefix that fits in (target_width - 1) then add "…"
        if target_width <= 1 {
            return "…".to_string();
        }
        let mut width = 0;
        let mut result = String::new();
        for ch in text.chars() {
            let cw = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if width + cw > target_width - 1 {
                break;
            }
            result.push(ch);
            width += cw;
        }
        // Pad if we stopped at an odd width (CJK char didn't fit)
        while width < target_width - 1 {
            result.push(' ');
            width += 1;
        }
        result.push('…');
        result
    }
}

fn render_table<'a>(
    node: &'a AstNode<'a>,
    lines: &mut Vec<Line<'static>>,
    width: u16,
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

    // Calculate natural column widths using unicode display width
    let mut col_widths: Vec<usize> = vec![0; num_cols];
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            if i < num_cols {
                col_widths[i] = col_widths[i].max(cell.width());
            }
        }
    }

    // Ensure minimum column width of 3 (enough for "…" + padding)
    for w in &mut col_widths {
        *w = (*w).max(3);
    }

    let indent = "  ";
    let indent_width = 2usize;
    // Total width = indent + │ + (padding + content + padding + │) * num_cols
    // = indent_width + 1 + sum(col_width + 3) for each col
    // = indent_width + 1 + num_cols * 3 + sum(col_widths)
    let border_overhead = indent_width + 1 + num_cols * 3; // "  │" + " cell │" per col
    let available_width = width as usize;

    let total_natural = border_overhead + col_widths.iter().sum::<usize>();

    if total_natural > available_width && available_width > border_overhead {
        // Need to shrink columns to fit
        let available_for_content = available_width - border_overhead;
        let total_content: usize = col_widths.iter().sum();

        if total_content > 0 {
            // Proportionally shrink, but keep minimum width of 3
            let mut new_widths: Vec<usize> = col_widths
                .iter()
                .map(|&w| {
                    let scaled = (w * available_for_content) / total_content;
                    scaled.max(3)
                })
                .collect();

            // Adjust if total still exceeds (due to minimum widths)
            let mut new_total: usize = new_widths.iter().sum();
            if new_total > available_for_content {
                // Shrink largest columns first
                let mut sorted_indices: Vec<usize> = (0..num_cols).collect();
                sorted_indices.sort_by(|&a, &b| new_widths[b].cmp(&new_widths[a]));
                for &idx in &sorted_indices {
                    if new_total <= available_for_content {
                        break;
                    }
                    let reduce = (new_total - available_for_content).min(new_widths[idx] - 3);
                    new_widths[idx] -= reduce;
                    new_total -= reduce;
                }
            }

            col_widths = new_widths;
        }
    }

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
            let w = col_widths.get(i).copied().unwrap_or(3);
            let padded = format!(" {} ", pad_cell(cell, w));
            spans.push(Span::styled(padded, style));
            spans.push(Span::styled("│", border_style));
        }
        for i in row.len()..num_cols {
            let w = col_widths.get(i).copied().unwrap_or(3);
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
    wrap_spans_indented(spans, max_width, "  ", "  ")
}

fn wrap_spans_indented(
    spans: Vec<Span<'static>>,
    max_width: usize,
    first_indent: &str,
    cont_indent: &str,
) -> Vec<Line<'static>> {
    if max_width == 0 {
        return vec![Line::from(spans)];
    }

    let first_indent_width = first_indent.width();
    let cont_indent_width = cont_indent.width();

    let mut result: Vec<Line<'static>> = Vec::new();
    let mut current_line: Vec<Span<'static>> = Vec::new();
    let mut current_width: usize = 0;
    let mut on_first_line = true;

    let start_line = |line: &mut Vec<Span<'static>>, width: &mut usize, on_first: bool| {
        line.clear();
        let indent = if on_first { first_indent } else { cont_indent };
        if !indent.is_empty() {
            line.push(Span::raw(indent.to_string()));
        }
        *width = 0;
    };

    let effective = |on_first: bool| {
        let reserved = if on_first {
            first_indent_width
        } else {
            cont_indent_width
        };
        max_width.saturating_sub(reserved)
    };

    start_line(&mut current_line, &mut current_width, on_first_line);

    for span in spans {
        let text = span.content.to_string();
        let style = span.style;

        if current_width + text.width() <= effective(on_first_line) {
            current_width += text.width();
            current_line.push(Span::styled(text, style));
            continue;
        }

        let mut remaining = text.as_str();
        while !remaining.is_empty() {
            let available = effective(on_first_line).saturating_sub(current_width);
            if available == 0 {
                result.push(Line::from(std::mem::take(&mut current_line)));
                on_first_line = false;
                start_line(&mut current_line, &mut current_width, on_first_line);
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
                result.push(Line::from(std::mem::take(&mut current_line)));
                on_first_line = false;
                start_line(&mut current_line, &mut current_width, on_first_line);
                remaining = rest;
            } else if split == 0 {
                result.push(Line::from(std::mem::take(&mut current_line)));
                on_first_line = false;
                start_line(&mut current_line, &mut current_width, on_first_line);
            } else {
                let (first, rest) = remaining.split_at(split);
                current_line.push(Span::styled(first.to_string(), style));
                result.push(Line::from(std::mem::take(&mut current_line)));
                on_first_line = false;
                start_line(&mut current_line, &mut current_width, on_first_line);
                remaining = rest;
            }
        }
    }

    let has_content = current_line
        .iter()
        .any(|s| !s.content.trim().is_empty() || current_line.len() > 1);
    if has_content {
        result.push(Line::from(current_line));
    }

    if result.is_empty() {
        result.push(Line::default());
    }

    result
}

fn wrap_plain_text(text: &str, max_width: usize) -> Vec<String> {
    if max_width == 0 || text.width() <= max_width {
        return vec![text.to_string()];
    }
    let mut result: Vec<String> = Vec::new();
    let mut remaining = text;
    while !remaining.is_empty() {
        if remaining.width() <= max_width {
            result.push(remaining.to_string());
            break;
        }
        let split = find_split_point(remaining, max_width);
        if split == 0 {
            let (first, rest) = split_at_width(remaining, max_width);
            result.push(first.to_string());
            remaining = rest;
        } else {
            let (first, rest) = remaining.split_at(split);
            result.push(first.trim_end().to_string());
            remaining = rest.trim_start();
        }
    }
    if result.is_empty() {
        result.push(String::new());
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
