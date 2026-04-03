//! JSON syntax highlighting for ratatui

use ratatui::text::{Line, Span, Text};
use serde_json::Value;

/// Convert a JSON value to syntax-highlighted ratatui Text
pub fn json_to_text(value: &Value, indent_size: usize) -> Text<'static> {
    let mut lines = Vec::new();
    render_value(value, 0, indent_size, &mut lines);
    Text::from(lines)
}

mod style {
    use ratatui::style::{Color, Style};

    pub fn key() -> Style {
        Style::default().fg(Color::Cyan)
    }
    pub fn string() -> Style {
        Style::default().fg(Color::Green)
    }
    pub fn number() -> Style {
        Style::default().fg(Color::Yellow)
    }
    pub fn boolean() -> Style {
        Style::default().fg(Color::Magenta)
    }
    pub fn null() -> Style {
        Style::default().fg(Color::Red)
    }
    pub fn bracket() -> Style {
        Style::default().fg(Color::White)
    }
    pub fn punctuation() -> Style {
        Style::default().fg(Color::DarkGray)
    }
}

/// Recursively render a JSON value with syntax highlighting
fn render_value(
    value: &Value,
    indent_level: usize,
    indent_size: usize,
    lines: &mut Vec<Line<'static>>,
) {
    let indent = " ".repeat(indent_level * indent_size);

    match value {
        Value::Null => {
            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled("null", style::null()),
            ]));
        }
        Value::Bool(b) => {
            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled(b.to_string(), style::boolean()),
            ]));
        }
        Value::Number(n) => {
            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled(n.to_string(), style::number()),
            ]));
        }
        Value::String(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled(format!("\"{}\"", escaped), style::string()),
            ]));
        }
        Value::Array(arr) => {
            if arr.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw(indent),
                    Span::styled("[]", style::bracket()),
                ]));
                return;
            }

            lines.push(Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled("[", style::bracket()),
            ]));

            for (i, item) in arr.iter().enumerate() {
                render_item(
                    item,
                    indent_level + 1,
                    indent_size,
                    i < arr.len() - 1,
                    lines,
                );
            }

            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled("]", style::bracket()),
            ]));
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw(indent),
                    Span::styled("{}", style::bracket()),
                ]));
                return;
            }

            lines.push(Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled("{", style::bracket()),
            ]));

            let len = obj.len();
            for (i, (key, val)) in obj.iter().enumerate() {
                render_key_value(key, val, indent_level + 1, indent_size, i < len - 1, lines);
            }

            lines.push(Line::from(vec![
                Span::raw(indent),
                Span::styled("}", style::bracket()),
            ]));
        }
    }
}

/// Render an array item with proper comma handling
fn render_item(
    value: &Value,
    indent_level: usize,
    indent_size: usize,
    trailing_comma: bool,
    lines: &mut Vec<Line<'static>>,
) {
    let start_idx = lines.len();
    render_value(value, indent_level, indent_size, lines);

    if trailing_comma
        && let Some(last) = lines.get_mut(start_idx..)
        && let Some(line) = last.last_mut()
    {
        line.spans.push(Span::styled(",", style::punctuation()));
    }
}

/// Render a key-value pair in an object
fn render_key_value(
    key: &str,
    value: &Value,
    indent_level: usize,
    indent_size: usize,
    trailing_comma: bool,
    lines: &mut Vec<Line<'static>>,
) {
    let indent = " ".repeat(indent_level * indent_size);

    match value {
        // Primitives: key and value on same line
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            let mut spans = vec![
                Span::raw(indent),
                Span::styled(format!("\"{}\"", key), style::key()),
                Span::styled(": ", style::punctuation()),
            ];

            match value {
                Value::Null => spans.push(Span::styled("null", style::null())),
                Value::Bool(b) => spans.push(Span::styled(b.to_string(), style::boolean())),
                Value::Number(n) => spans.push(Span::styled(n.to_string(), style::number())),
                Value::String(s) => {
                    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
                    spans.push(Span::styled(format!("\"{}\"", escaped), style::string()));
                }
                _ => unreachable!(),
            }

            if trailing_comma {
                spans.push(Span::styled(",", style::punctuation()));
            }

            lines.push(Line::from(spans));
        }
        // Complex types: opening bracket on same line as key
        Value::Array(arr) => {
            if arr.is_empty() {
                let mut spans = vec![
                    Span::raw(indent),
                    Span::styled(format!("\"{}\"", key), style::key()),
                    Span::styled(": ", style::punctuation()),
                    Span::styled("[]", style::bracket()),
                ];
                if trailing_comma {
                    spans.push(Span::styled(",", style::punctuation()));
                }
                lines.push(Line::from(spans));
                return;
            }

            lines.push(Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled(format!("\"{}\"", key), style::key()),
                Span::styled(": ", style::punctuation()),
                Span::styled("[", style::bracket()),
            ]));

            for (i, item) in arr.iter().enumerate() {
                render_item(
                    item,
                    indent_level + 1,
                    indent_size,
                    i < arr.len() - 1,
                    lines,
                );
            }

            let mut closing = vec![Span::raw(indent), Span::styled("]", style::bracket())];
            if trailing_comma {
                closing.push(Span::styled(",", style::punctuation()));
            }
            lines.push(Line::from(closing));
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                let mut spans = vec![
                    Span::raw(indent),
                    Span::styled(format!("\"{}\"", key), style::key()),
                    Span::styled(": ", style::punctuation()),
                    Span::styled("{}", style::bracket()),
                ];
                if trailing_comma {
                    spans.push(Span::styled(",", style::punctuation()));
                }
                lines.push(Line::from(spans));
                return;
            }

            lines.push(Line::from(vec![
                Span::raw(indent.clone()),
                Span::styled(format!("\"{}\"", key), style::key()),
                Span::styled(": ", style::punctuation()),
                Span::styled("{", style::bracket()),
            ]));

            let len = obj.len();
            for (i, (k, v)) in obj.iter().enumerate() {
                render_key_value(k, v, indent_level + 1, indent_size, i < len - 1, lines);
            }

            let mut closing = vec![Span::raw(indent), Span::styled("}", style::bracket())];
            if trailing_comma {
                closing.push(Span::styled(",", style::punctuation()));
            }
            lines.push(Line::from(closing));
        }
    }
}
