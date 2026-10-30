# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-30

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries no patch of its own. One PR of ours is open
upstream: #1603** (`feat/take-to-rez-folder`, in worktree
`status-check-200b7b`; Sugar's Discord request, opened 2026-10-30): Take
puts objects back in their rez folder per `handle_take` (nil FolderID
skipped, conflict → Objects, never Trash/Library; Take Copy → Objects).
6 `build::take_tests` pass; demo-validated by G.W. 2026-10-30; Fable and
codex findings fixed (8464eb07), agy timed out. The eight before it are
merged; `circuskid` = `upstream/main` 4709bed3 (tag
`v0.8.4-beta.20261030.1`) plus `.claude/CLAUDE.md` and this file.

Latest package: tag `v0.8.4.1793365514`, installed here as
`aurora-viewer 0.8.4.1793365514-1` (grid sees `0.8.4.0`). Tree clean.

### FIRST — the two-minute check

1. **Upstream drift.** `git fetch --no-tags upstream && git rev-list
   --count circuskid..upstream/main`. Since we carry no code, a sync should
   be conflict-free; ship when G.W. wants a build (steps in
   `.claude/CLAUDE.md`). Upstream now tags betas of the next version
   (`v0.8.4-beta.*`): the package base is that tag's X.Y.Z.
2. **Grid follow-ups G.W. owns:** the voice test on Linux for #1407
   (planned 2026-10-30). The maintainer sets its TASKS.md line to 🟡 in a
   docs PR of their own; nothing for us to push. Also the grid check of
   #1603 (rez from a folder, Take, it comes back there; its TASKS.md ⬜).
3. **PR #1603.** `gh pr view 1603 --repo Aurora-Viewer/Aurora-Viewer
   --json state,reviews,comments`. CI had not run at opening (waits for the
   maintainer's approval). Answer any review per THEN below; once merged,
   remove the worktree's branch on the next sync.

### Gotcha — dev builds on `circuskid` panic

Our stamp tags (`v0.8.2.<stamp>`, four numbers) match the
`v[0-9]*.[0-9]*.[0-9]*` pattern of `crates/aurora-version/build.rs`, whose
parser rejects them, so any `cargo build`/`cargo test` on a commit that
descends from one stops in that build script. `AURORA_VERSION_TAG` does not
help: `describe` still finds the stamp tag and the build refuses the
mismatch. The PKGBUILD deletes the tags in its clone. To run the tests,
clone with no tags (`git clone --no-tags --shared`, then delete what
`--branch` fetched) and point `CARGO_TARGET_DIR` at the worktree's `target/`.
Fetch the upstream tag with `--no-tags`, or the stamp tags follow it in.
Count the tests over every `test result:` line of the full output (28 of
them): a tail of the output drops crates and undercounts.

**Editing this file from a worktree session:** a hook refuses edits in
`~/Projects/aurora-viewer` (the main checkout, on `circuskid`). Branch from
`circuskid` in the worktree, edit and commit there, then
`git -C ~/Projects/aurora-viewer merge --ff-only <branch>`, push `circuskid`,
switch back and delete the branch.

### THEN — the next upstream PR

- Cut from `upstream/main`, no AI trailers (`.claude/CLAUDE.md`). To answer
  a review: fix on the branch, commit with no trailers, `git push origin
  HEAD`. Draft any PR comment to G.W. first. Never rebase or force-push.
- **Each push re-runs first-contributor CI**, which waits for the
  maintainer to approve it, so batch the fixes for one review into one push.
- **The maintainer merges, not us**; the next sync brings merged PRs in.
  Then remove the PR's worktree and local branch (#1405–#1411 went
  2026-10-29, #1483 and #1497 2026-10-30). Still around, probably done:
  `aurora-linux-improvements-5a6f8e` (#1397's, merged; reused 2026-10-30
  for the sync) and `new-viewer-update-d7fc5f` (old 0.8.2 merge); check
  before removing.
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

- **Merged upstream PRs (eight, through 2026-10-30):** #1397
  (notifications), #1405 (desktop identity), #1406 (jobs at lower
  priority), #1407 (PulseAudio / PipeWire), #1408 (local time), #1411
  (advanced inventory search), #1483 (inventory Left / Right arrows climb
  folders), #1497 (search field in the wolf menu). Worktrees and local
  branches gone; the fork still holds remote branches
  `feat/inventory-arrow-fold` and `feat/menu-search`.
- **2026-10-30 sync:** merged `upstream/main` 4709bed3 (18 commits) into
  `circuskid` with no conflict (e8ba61b9); fork `main` fast-forwarded. No
  new workflow; all eight stay `disabled_manually`. Package
  `v0.8.4.1793365514` from upstream `v0.8.4-beta.20261030.1` plus 16
  commits (pkgbuilds 4b820bb); workspace tests 6260 passed, 0 failed, 31
  ignored; `makepkg` passed `check()`; `--version` gives
  `0.8.4-beta.20261030.1+circuskid.1793365514`.
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
