//! Shared utility functions

/// Strip ANSI escape codes from a string
pub fn strip_ansi(s: &str) -> String {
    let bytes = strip_ansi_escapes::strip(s);
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Escape special characters for debug display
pub fn escape_debug(s: &str) -> String {
    s.replace('\x1b', "\\e")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}
