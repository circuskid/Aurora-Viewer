# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-28

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**One patch of ours on `circuskid`:** the Wayland attention fix
(circuskid/Aurora-Viewer#1): on Linux the attention flash is sent only to a
minimized window, so it is never sent on Wayland, where winit's request is
an xdg_activation that made Hyprland switch workspace and steal focus.
Upstream issue Aurora-Viewer/Aurora-Viewer#1330, PR Aurora-Viewer/Aurora-Viewer#1332
(branch `fix/wayland-attention-focus`, head d7c4ad3b). Released as tag
`v0.8.2.1793212147` (pkgbuilds d66c37c), built by `circuskid-repo update`
and installed on this host 2026-10-28 (aurora-viewer is now in
`~/.config/circuskid-repo/packages`, so `paru` picks up later releases).

### FIRST — the two-minute check

`git fetch upstream && git rev-list --count circuskid..upstream/main` prints
how far behind upstream we are (0 = current; upstream merges many PRs a day).
7 behind at the release.

### THEN — what is owed

1. **PR #1332**: maintainer review ✅ ready to merge on d7c4ad3b, CI green
   (2026-10-28; X11 minimized-only change accepted as intended). Nothing
   owed from us; the merge is hers. We never merge it. When upstream merges it, merge `upstream/main` into
   `circuskid`, check `gh workflow list -R circuskid/Aurora-Viewer --all`,
   close our #1, and end the worktree.
2. A rejected alternative, in case review reopens it: skip the flash only on
   Wayland (detected by `RawWindowHandle::Wayland`) and keep X11 flashing
   whenever behind. Dropped 2026-10-28 because the maintainer accepted the
   Legacy Viewer rule (`LLWindowSDL::flashIcon`, minimized only).

### NOT QUEUED

- AGENTS.md twice? Not seen: the loaded instructions in a worktree session
  show AGENTS.md once. The real gap is the other way: a worktree cut from
  `upstream/main` has no `.claude/CLAUDE.md` or STATUS.md of ours, so the
  fork rules do not auto-load there; the session must read them from
  `~/Projects/aurora-viewer`. Not measured with `/context` itself.
- Upstream attribution was ruled 2026-10-28: follow AGENTS.md §10
  (`.claude/CLAUDE.md`).
