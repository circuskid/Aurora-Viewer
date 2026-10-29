# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-28 (night)

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries no patch of its own.** Upstream released v0.8.2 stable
(2026-10-28 23:47Z) with our Wayland attention fix
(Aurora-Viewer/Aurora-Viewer#1332). `circuskid` = `upstream/main` (602e3ae8)
+ `.claude/CLAUDE.md` + this file, merge 53f13d84; fork `main` mirrors
602e3ae8. circuskid/Aurora-Viewer#1 closed; the wayland worktree is gone;
all seven fork workflows still `disabled_manually`.

Latest package: tag `v0.8.2.1793232360`, pkgbuilds b4f56ac, built with
`makepkg` and `check()` green (binary says `0.8.2+circuskid.1793232360`).
Installed on this host 2026-10-28 (`pacman -Q` and `--version` agree).

### FIRST — the two-minute check

`git fetch upstream && git rev-list --count circuskid..upstream/main` — 0
at this checkpoint (upstream merges many PRs a day).

### THEN — nothing queued

Ship a new build when G.W. wants one (steps in `.claude/CLAUDE.md`). Any
upstream contribution: branch from `upstream/main`, draft to G.W. first.

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
