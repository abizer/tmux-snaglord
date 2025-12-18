//! Application state management

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;

use crate::parser::CommandBlock;

/// Main application state
pub struct App {
    /// Parsed command blocks
    pub blocks: Vec<CommandBlock>,
    /// State for the command list widget
    pub list_state: ListState,
    /// Vertical scroll offset for the output pane
    pub scroll_offset: u16,
    /// Current search query
    pub search_query: String,
    /// Whether we're in search mode
    pub is_searching: bool,
    /// Indices of blocks that match the current filter
    pub filtered_indices: Vec<usize>,
}

impl App {
    /// Create a new App with the given command blocks
    pub fn new(blocks: Vec<CommandBlock>) -> Self {
        let mut list_state = ListState::default();
        // Select first item if available
        if !blocks.is_empty() {
            list_state.select(Some(0));
        }

        // Initialize with all indices
        let filtered_indices = (0..blocks.len()).collect();

        Self {
            blocks,
            list_state,
            scroll_offset: 0,
            search_query: String::new(),
            is_searching: false,
            filtered_indices,
        }
    }

    /// Get the actual data index from the visual list selection
    fn get_current_data_index(&self) -> Option<usize> {
        self.list_state
            .selected()
            .and_then(|i| self.filtered_indices.get(i).copied())
    }

    /// Move selection to the next item
    pub fn next(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.filtered_indices.len() - 1 {
                    0 // Wrap to beginning
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_offset = 0; // Reset scroll when changing selection
    }

    /// Move selection to the previous item
    pub fn previous(&mut self) {
        if self.filtered_indices.is_empty() {
            return;
        }

        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.filtered_indices.len() - 1 // Wrap to end
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        self.scroll_offset = 0; // Reset scroll when changing selection
    }

    /// Scroll the output pane down
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scroll the output pane up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Handle character input during search
    pub fn on_search_input(&mut self, c: char) {
        self.search_query.push(c);
        self.update_search_results();
    }

    /// Handle backspace during search
    pub fn on_search_backspace(&mut self) {
        self.search_query.pop();
        self.update_search_results();
    }

    /// Update filtered results based on current search query
    pub fn update_search_results(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_indices = (0..self.blocks.len()).collect();
        } else {
            let matcher = SkimMatcherV2::default();
            let mut matches: Vec<(i64, usize)> = self
                .blocks
                .iter()
                .enumerate()
                .filter_map(|(idx, block)| {
                    // Match against clean command and ANSI-stripped output
                    let cmd_score = matcher.fuzzy_match(&block.clean_command, &self.search_query);
                    let clean_output = strip_ansi(&block.output);
                    let out_score = matcher.fuzzy_match(&clean_output, &self.search_query);
                    // Take best score from either
                    match (cmd_score, out_score) {
                        (Some(c), Some(o)) => Some((c.max(o), idx)),
                        (Some(c), None) => Some((c, idx)),
                        (None, Some(o)) => Some((o, idx)),
                        (None, None) => None,
                    }
                })
                .collect();

            // Sort by score descending, use index as tie-breaker to preserve order
            matches.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));

            self.filtered_indices = matches.into_iter().map(|(_, idx)| idx).collect();
        }

        // Reset selection to top of results
        if !self.filtered_indices.is_empty() {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
        self.scroll_offset = 0;
    }

    /// Clear search and restore full list
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.is_searching = false;
        self.update_search_results();
    }

    /// Get the output of the currently selected block (ANSI stripped for copying)
    pub fn get_selected_output(&self) -> Option<String> {
        self.get_current_data_index()
            .and_then(|i| self.blocks.get(i))
            .map(|b| strip_ansi(&b.output))
    }

    /// Get the command text only (prompt removed) for copying
    pub fn get_selected_command(&self) -> Option<String> {
        self.get_current_data_index()
            .and_then(|i| self.blocks.get(i))
            .map(|b| b.command_text.clone())
    }

    /// Get the full content (command + output) of the currently selected block (ANSI stripped)
    pub fn get_selected_full(&self) -> Option<String> {
        self.get_current_data_index()
            .and_then(|i| self.blocks.get(i))
            .map(|b| format!("{}\n{}", strip_ansi(&b.command), strip_ansi(&b.output)))
    }

    /// Get debug-formatted output for diagnosing parsing issues
    pub fn get_selected_debug(&self) -> Option<String> {
        self.get_current_data_index()
            .and_then(|i| self.blocks.get(i))
            .map(|b| {
                let mut out = String::new();
                out.push_str("=== COMMAND (raw) ===\n");
                for (i, line) in b.command.lines().enumerate() {
                    out.push_str(&format!("{:3}| {}\n", i + 1, escape_debug(line)));
                }
                out.push_str("\n=== COMMAND (clean) ===\n");
                for (i, line) in b.clean_command.lines().enumerate() {
                    out.push_str(&format!("{:3}| {}\n", i + 1, line));
                }
                out.push_str("\n=== OUTPUT (raw) ===\n");
                if b.output.is_empty() {
                    out.push_str("(empty)\n");
                } else {
                    for (i, line) in b.output.lines().enumerate() {
                        out.push_str(&format!("{:3}| {}\n", i + 1, escape_debug(line)));
                    }
                }
                out
            })
    }

    /// Get the currently selected block for display
    pub fn get_selected_block(&self) -> Option<&CommandBlock> {
        self.get_current_data_index()
            .and_then(|i| self.blocks.get(i))
    }
}

/// Strip ANSI escape codes from a string
fn strip_ansi(s: &str) -> String {
    let bytes = strip_ansi_escapes::strip(s);
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Escape special characters for debug display
fn escape_debug(s: &str) -> String {
    s.replace('\x1b', "\\e")
        .replace('\t', "\\t")
        .replace('\r', "\\r")
}
