# Fork notes

Private reminders for the abizer/tmux-snaglord fork.

## Stack structure

`main` is ordered so upstream-mergeable commits sit at the bottom:

```
486e1fe  ci: upstream-check (fork-only)
1a0a5c7  ci: cache to abizer.cachix.org (fork-only)
70b5e26  add nix flake (maybe upstream-mergeable)
02af8c3  parser: multiline prompts (clean PR candidate)
46a9b34  (upstream/main)
```

The `pr-multiline-prompt` branch pins the parser commit for upstream PR.

## Submit the parser PR upstream

```sh
gh pr create \
  --repo raine/tmux-snaglord \
  --base main \
  --head abizer:pr-multiline-prompt \
  --title "parser: support multiline shell prompts via prompt_lines config" \
  --body-file -  # paste/write description, or use --fill
```

After it merges (squashed upstream):

```sh
fork-sync                       # rebase main; parser commit auto-drops
git push origin :pr-multiline-prompt
git branch -D pr-multiline-prompt
```

## Keep main in sync with upstream

```sh
fork-sync   # fetch upstream, rebase main, force-push
```

The `upstream-check` workflow runs daily and opens a labelled issue
(`upstream-sync`) when upstream has new commits. It auto-closes the
issue when we catch up.
