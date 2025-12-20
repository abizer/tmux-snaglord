# Changelog

## v0.1.2 (2025-12-20)

- Added all-panes search mode to search across all visible panes in the current
  tmux window (press `a` or cycle with `;`, or start with `--all` flag)
- Matched characters are now highlighted during fuzzy search
- Improved clipboard support across platforms (macOS, Linux with Wayland/X11,
  Windows)

## v0.1.1 (2025-12-20)

- Changed Enter key to copy only the command output instead of command + output

## v0.1.0 (2025-12-19)

Initial release of tmux-snaglord, a TUI tool for browsing and copying from your
tmux scrollback history.
