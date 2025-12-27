# Changelog

## v0.1.4 (2025-12-27)

- Now captures full scrollback history instead of only the last 5000 lines

<!-- skipped: v0.1.3 -->

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
