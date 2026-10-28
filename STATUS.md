# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-28 (evening)

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**Our one patch is now upstream.** The Wayland attention fix (flash only a
minimized window on Linux; on Wayland winit's xdg_activation made Hyprland
switch workspace and steal focus) merged upstream as
Aurora-Viewer/Aurora-Viewer#1332 on 2026-10-28 19:33Z (merge 32f1b57a);
upstream issue #1330 is closed. Installed build is still our tag
`v0.8.2.1793212147` (pkgbuilds d66c37c); aurora-viewer is in
`~/.config/circuskid-repo/packages`, so `paru` picks up later releases.

### FIRST — the two-minute check

`git fetch upstream && git rev-list --count circuskid..upstream/main` — 21
at this checkpoint (upstream merges many PRs a day), and
`gh issue view 1 -R circuskid/Aurora-Viewer --json state` — OPEN.

### THEN — the post-merge cleanup, not started

1. Merge `upstream/main` into `circuskid` (it now contains our fix, so
   `circuskid` = upstream + STATUS/`.claude` only), push.
2. `gh workflow list -R circuskid/Aurora-Viewer --all` — check none of
   upstream's workflows got enabled on the fork.
3. Close circuskid/Aurora-Viewer#1, naming #1332.
4. Remove the worktree `.claude/worktrees/wayland-attention-focus-d6e595`
   (branch `fix/wayland-attention-focus`, d7c4ad3b, fully merged).
5. Packaging: ship the next build from `circuskid` per
   `circuskid-release` when the user wants one.

If PR review is ever reopened, the rejected alternative was: skip the flash
only on Wayland (`RawWindowHandle::Wayland`), keep X11 flashing whenever
behind. Dropped because the maintainer accepted the Legacy Viewer rule
(`LLWindowSDL::flashIcon`, minimized only).

### DECIDED — do not reopen

- **DLSS: no.** Ruled 2026-10-28 by the user, matching upstream
  Aurora-Viewer/Aurora-Viewer#209 (merged 2026-10-13, plan switched to FSR).
  NVIDIA's RTX SDK licence (§4(e), §2(c), logo/notice terms) is
  incompatible with GPLv3. **Frame generation: dropped too** (no Vulkan/Linux
  path from NVIDIA or AMD). Research findings, in case FSR work starts:
  the real prerequisite is per-pixel motion vectors (`crates/aurora-render`
  has none; TAA reprojects camera motion only); FSR 2/3.1 is MIT, candidate
  crate `sp-fidelity` (unreviewed); the Legacy Viewer has no upscaler.
  None of it is queued.

### NOT QUEUED

- AGENTS.md twice? Not seen: the loaded instructions in a worktree session
  show AGENTS.md once. The real gap is the other way: a worktree cut from
  `upstream/main` has no `.claude/CLAUDE.md` or STATUS.md of ours, so the
  fork rules do not auto-load there; the session must read them from
  `~/Projects/aurora-viewer`. Not measured with `/context` itself.
- Upstream attribution was ruled 2026-10-28: follow AGENTS.md §10
  (`.claude/CLAUDE.md`).
