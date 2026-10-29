# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-29

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries no patch of its own.** Every PR we sent upstream is
merged: #1397 (notifications), #1405 (desktop identity), #1406 (jobs at
lower priority), #1407 (PulseAudio / PipeWire), #1408 (local time), #1411
(advanced inventory search). The 2026-10-29 sync merged `upstream/main`
c5a428a7 into `circuskid`, taking upstream's side of every conflict (its
#1411 supersedes our merge 11935b23). The tree is `upstream/main` plus
`.claude/CLAUDE.md` and this file. All seven fork workflows are still
`disabled_manually`, and the sync added no new workflow.

**One PR open upstream: Aurora-Viewer/Aurora-Viewer#1483** (2026-10-29),
inventory Left / Right arrows open, close and climb folders (Legacy
`LLFolderView::handleKeyHere`). A tester request with no issue; G.W.
validated it in demo. Fork branch `feat/inventory-arrow-fold` = local
`claude/elastic-lederberg-6a602f`, worktree
`.claude/worktrees/elastic-lederberg-6a602f`. At checkpoint its CI waited on
the maintainer's first-contributor approval; no labels, no screenshots
(G.W. dropped them).

Latest package: tag `v0.8.3.1793280767`, built from upstream
`v0.8.3-beta.20261029.1` plus 30 commits (`_upstream=0.8.3-beta.20261029.1`).
The binary reports `0.8.3-beta.20261029.1+circuskid.1793280767`, and the grid
sees `0.8.3.0`. The workspace tests passed before the tag (6139 passed,
0 failed, 30 ignored). Installed here as `aurora-viewer 0.8.3.1793280767-2`
(pkgrel 2 fixed `package()` for upstream's desktop file and logo, #1398);
`aurora-viewer --version` confirmed. Tree clean, nothing half-done.
At checkpoint `upstream/main` was 3 commits ahead (not merged).

### FIRST — the two-minute check

1. **Upstream drift.** `git fetch upstream && git rev-list --count
   circuskid..upstream/main`. Since we carry no code, a sync should be
   conflict-free; ship when G.W. wants a build (steps in `.claude/CLAUDE.md`).
2. **#1483 state.** `gh pr view 1483 --repo Aurora-Viewer/Aurora-Viewer
   --json state,mergeable,statusCheckRollup,comments`. A review: fix in the
   worktree above (one push per review). Merged: remove that worktree and
   its branch.
3. **Grid follow-ups G.W. owns:** the voice test on Linux for #1407
   (planned 2026-10-30). The maintainer sets its TASKS.md line to 🟡 in a
   docs PR of their own; nothing for us to push.

### Gotcha — dev builds on `circuskid` panic

Our stamp tags (`v0.8.2.<stamp>`, four numbers) match the
`v[0-9]*.[0-9]*.[0-9]*` pattern of `crates/aurora-version/build.rs`, whose
parser rejects them, so any `cargo build`/`cargo test` on a commit that
descends from one stops in that build script. `AURORA_VERSION_TAG` does not
help: `describe` still finds the stamp tag and the build refuses the
mismatch. The PKGBUILD deletes the tags in its clone. To run the tests,
clone with no tags (`git clone --no-tags --shared`, then delete what
`--branch` fetched) and point `CARGO_TARGET_DIR` at the worktree's `target/`.

### THEN — the next upstream PR

- Cut from `upstream/main`, no AI trailers (`.claude/CLAUDE.md`). To answer
  a review: fix on the branch, commit with no trailers, `git push origin
  HEAD`. Draft any PR comment to G.W. first. Never rebase or force-push.
- **Each push re-runs first-contributor CI**, which waits for the
  maintainer to approve it, so batch the fixes for one review into one push.
- **The maintainer merges, not us**; the next sync brings merged PRs in.
  Then remove the PR's worktree and local branch (the four of #1405–#1411
  went 2026-10-29). Still around, probably done: `aurora-linux-improvements-5a6f8e`
  (#1397's branch, merged) and `new-viewer-update-d7fc5f` (old 0.8.2 merge);
  check before removing.
- **The fork cannot label PRs or publish captures upstream** (`push: false`;
  `pr-captures.ps1` pushes upstream's `captures` branch). The title's type
  carries the label; for screenshots, push them to the fork's own `captures`
  branch and link the raw URLs.
- **Clippy `-D warnings` on Linux fails on existing Windows-only dead code**
  (`crash/`, `raw_mouse.rs`, `url_scheme.rs`…). Not ours (AGENTS.md rule 3):
  check only that the touched files are clean, and say so in the PR.

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

- **Linux notification click-to-raise: the maintainer answered 2026-10-29**
  on Aurora-Viewer/Aurora-Viewer#1397 (merged): keep the ⬜ "later" line, and
  make it a separate PR once it can be tested on a server that sends
  `ActivationToken`. A click opens the
  right conversation but cannot bring the window forward: winit 0.30.13's
  Wayland `focus_window` is an empty function, and raising needs the
  xdg-activation token a server may send with the click (`ActivationToken`).
  Measured with `dbus-monitor` on a `notify-send` probe: quickshell 0.3.2 sends
  only `ActionInvoked` then `NotificationClosed`, so on this desktop even our
  own activation code would not raise it. Blocked on such a server.

---

## Finished — read on demand

- If the review of the attention flash is ever reopened, this alternative was
  rejected: skip the flash only on Wayland (`RawWindowHandle::Wayland`) and
  keep X11 flashing whenever the window is behind. It was dropped because the
  maintainer accepted the Legacy Viewer rule (`LLWindowSDL::flashIcon`,
  minimized only).
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
