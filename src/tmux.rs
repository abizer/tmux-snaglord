//! Interface for tmux commands

use anyhow::{Context, Result};
use std::io::Write;
use std::process::{Command, Stdio};

/// Capture the content of a tmux pane
///
/// Uses `tmux capture-pane` with:
/// - `-e`: preserve escape sequences (ANSI colors)
/// - `-J`: join wrapped lines
/// - `-p`: output to stdout
/// - `-S -`: capture full history
pub fn capture_pane(pane_id: Option<&str>) -> Result<String> {
    let mut args = vec!["capture-pane", "-e", "-J", "-p", "-S", "-"];

    if let Some(id) = pane_id {
        args.push("-t");
        args.push(id);
    }

    let output = Command::new("tmux")
        .args(&args)
        .output()
        .context("Failed to execute tmux capture-pane. Are you running inside tmux?")?;

    if !output.status.success() {
        anyhow::bail!(
            "tmux capture-pane failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).context("tmux output contained invalid UTF-8")
}

/// Load content into tmux paste buffer
///
/// This allows pasting with `prefix + ]` or `tmux paste-buffer`
pub fn load_buffer(content: &str) -> Result<()> {
    let mut child = Command::new("tmux")
        .args(["load-buffer", "-"])
        .stdin(Stdio::piped())
        .spawn()
        .context("Failed to spawn tmux load-buffer")?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(content.as_bytes())
            .context("Failed to write to tmux load-buffer stdin")?;
    }

    let status = child
        .wait()
        .context("Failed to wait for tmux load-buffer")?;
    if !status.success() {
        anyhow::bail!("tmux load-buffer exited with non-zero status");
    }

    Ok(())
}
