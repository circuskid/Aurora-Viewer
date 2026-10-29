# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-29

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries one patch of its own: the advanced inventory search**
(merge 11935b23, 2026-10-29; branch `claude/inventory-regex-filters-0221ca`,
cut from `upstream/main` 602e3ae8, feature commits free of AI trailers so it
can go upstream as a clean PR later — not drafted, G.W.'s call). Visual
boolean rules beside quick search: chip bar, rule builder, per-account saved
searches (`inventory-smart-searches.json`), Find similar. Reviewed by Fable,
codex and agy; all confirmed findings fixed (671a6630). G.W. validated it in
demo mode 2026-10-29. Otherwise `circuskid` = `upstream/main` (602e3ae8) +
`.claude/CLAUDE.md` + this file; upstream v0.8.2 already holds our Wayland
fix (Aurora-Viewer/Aurora-Viewer#1332). All seven fork workflows still
`disabled_manually`.

Latest package: tag `v0.8.2.1793242821`, pkgbuilds 69e87fd, installed on
this host 2026-10-29 (`aurora-viewer --version` → `0.8.2+circuskid.1793242821`).

### FIRST — the two-minute check

`git fetch upstream && git rev-list --count circuskid..upstream/main` — 1
at this checkpoint (d555fb67, inventory hover actions; upstream merges many
PRs a day).

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
  None of it is queued. **FSR SDK v2.4.0** (2026-10-28) changes little:
  FSR 4.1.2 (ML) now runs on RX 6000/7000/9000 but stays AMD-only,
  Windows-only, shipped as signed `.dll`s with no source. Its
  `license.md` lists those DLLs under the MIT section, not the
  binary-only one (read at the tag); MIT on a binary still gives a GPLv3
  build no Corresponding Source, so FSR 4 could only ever be an optional
  Windows runtime load, which is upstream's call. The vendor-neutral path
  is unchanged: FSR 3.1.6 source, MIT. **Crate `wgpu-ffx` 0.1.0**
  (FSR 3 upscaler for wgpu, by wgpu maintainer cwfitzgerald; reviewed
  2026-10-28): MIT, GPLv3-compatible, a credible base but not usable
  as-is. Pins wgpu 29 (Aurora is on 30; migration PR #24 unmerged),
  ghosting fix PR #28 unmerged, `dispatch()` blocks on an error scope and
  `panic!`s every frame, allocates a staging buffer per frame, no
  reactive-mask generation, reference shader permutations only, output
  Rgba16Float only, "experimental". Fork-and-fix or wait; motion vectors
  come first either way.

### NOT QUEUED

- **Linux desktop notifications: shelved 2026-10-29 by G.W.** ("not worth
  it"). Branch `fix/linux-desktop-notifications` on `origin` (681ce4f6, cut
  from upstream 98024d73; upstream-clean, no AI trailers), never PR'd. It sends
  notices through `org.freedesktop.Notifications` via zbus (`system_notify.rs`)
  in place of the Windows-only path; demo-checked on Hyprland/quickshell, and a
  click opens the right conversation. Why shelved: a click cannot raise the
  window. winit 0.30.13's Wayland `focus_window` is an empty function, and
  raising needs the xdg-activation token a server may send with the click
  (`ActivationToken`). Measured with `dbus-monitor` on a `notify-send` probe:
  quickshell 0.3.2 sends only `ActionInvoked` then `NotificationClosed`, so
  even our own activation code would not raise it on this desktop. Reopen only
  if quickshell starts sending the token (or atoll makes it), or winit gains a
  Wayland activate-with-token. The rest of the TASKS.md limit is on the branch.
- AGENTS.md twice? Not seen: the loaded instructions in a worktree session
  show AGENTS.md once. The real gap is the other way: a worktree cut from
  `upstream/main` has no `.claude/CLAUDE.md` or STATUS.md of ours, so the
  fork rules do not auto-load there; the session must read them from
  `~/Projects/aurora-viewer`. Not measured with `/context` itself.
- Upstream attribution was ruled 2026-10-28: follow AGENTS.md §10
  (`.claude/CLAUDE.md`).
- **Login-screen portrait pixelated** (2026-10-28): solved, nothing to do.
  The login screen shows `~/.config/auroraviewer/accounts/<agent>/login_portrait.png`,
  rewritten at every login from our own profile picture (`App::picture_pixels`).
  The beta saved the first coarse level of the progressive download; upstream
  Aurora-Viewer/Aurora-Viewer#1364 (in v0.8.2) rewrites it as finer levels
  arrive. G.W. confirmed sharp after a 0.8.2 login. Not a texture-cache bug.
  Leftover, not filed: each login still writes the coarse copy first, so
  quitting before refinement leaves it blurry until the next full login.
  Fixing that is an upstream PR (draft to G.W. first); offered, not asked for.
