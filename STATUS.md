# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-31

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries no patch of its own.** #1625 (Linux `secondlife://`
links) and #1630 (place link 404) were **merged upstream by 2026-10-31**; the
next sync brings them in, then drop the `slurl-combined` worktree and
re-register links from the packaged build (gotcha below). **Open upstream
(2026-10-31), the Linux GTAO/fps work, worktree `linux-framerate-perf-2a2ce9`:**
#1702 motion vectors for GTAO's history (head `feat/gtao-motion-vectors`,
`main` merged in, `check.ps1` All green, 6500 tests), #1703 GTAO Hilbert index
on NVIDIA/Linux, #1704 egui discarded-pass click, #1705 per-effect profile
timers. All four grid-validated by G.W. 2026-10-30 (except #1705, profiling
only). **#1748** (script editor export 0700/0600) closes fork issue #3 once
synced. **#1752** (menu search keys) is worktree `new-viewer-update-d7fc5f`.
**#1753** (leftover `cef\cef_cache`) and **#1757** (NVIDIA Linux minimums):
worktree `login-profile-pic-pixelated-960617`. **#1760** (Linux CI tests):
`aurora-viewer-contributing-91cc75`. `circuskid` = `upstream/main` 55b3c4e9 + docs.

Latest package: tag `v0.8.4.1793391648` (2026-10-30, `check()` only;
workspace tests not run), installed here as
`aurora-viewer 0.8.4.1793391648-1` (grid sees `0.8.4.0`). Tree clean.

### FIRST — the two-minute check

1. **Upstream drift.** `git fetch --no-tags upstream && git rev-list
   --count circuskid..upstream/main`. Since we carry no code, a sync should
   be conflict-free; ship when G.W. wants a build (steps in
   `.claude/CLAUDE.md`). Upstream now tags betas of the next version
   (`v0.8.4-beta.*`): the package base is that tag's X.Y.Z.
2. **Grid follow-ups G.W. owns:** #1407 Linux voice: initial grid test
   passed 2026-10-30; the full one (mic open under load, device switch
   mid-call) waits until G.W. uses voice (rarely); blocks nothing. The
   maintainer sets its TASKS.md 🟡; nothing for us to push. #1625 + #1630:
   grid-OK on local `test/slurl-plus-place-fix` (worktree `slurl-combined`, drop
   once merged). Test viewers: no `--title` (changes the D-Bus name);
   quit from the menu, never kill (the next login gets no caps).
3. **PRs #1702–#1705, #1748, #1752, #1753, #1757, #1760.** `gh pr view <n> --repo
   Aurora-Viewer/Aurora-Viewer --json state,reviews,comments` for each
   (#1702, #1748, #1752, #1753, #1760 have Auto-fix on; #1748's worktree: `wgpu-performance-benefits-23c091`). CI waits for the maintainer's approval.
   Answer reviews per THEN. #1702's head: worktree `.scratch/motion-merge`
   in this one (pushed as `feat/gtao-motion-vectors`); merge, never rebase.
   All merged → drop it, the worktree, branches `test/motion-plus-hilbert`,
   `backup/linux-framerate-perf-pre-split`. Unqueued next: TAA motion vectors.

### Gotcha — this machine's `secondlife://` links run a worktree build

The installed package has no Linux link support (#1625 is merged upstream but in no package yet), so
`~/.local/share/applications/aurora-viewer-slurl.desktop` points at
`linux-items-status-afad2d/target/debug/aurora-viewer` (hand-repaired
2026-10-30 from a `(deleted)` path). Removing that worktree breaks links:
turn the Preferences switch off first, or re-register from the build that
replaces it. Check: `gio mime x-scheme-handler/secondlife`.

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
  Then remove its worktree and branch (check `aurora-linux-improvements-5a6f8e`).
- **The fork cannot label PRs or publish captures upstream** (`push: false`;
  `pr-captures.ps1` pushes upstream's `captures` branch). The title's type
  carries the label; for screenshots, push them to the fork's own `captures`
  branch and link the raw URLs.
- 🔴 **Before opening any PR, meet the maintainer's bar** in
  `.claude/CLAUDE.md` (*Upstream's rules*): `check.ps1` until "All
  green." (it works on Linux since #1675/#1681, on a branch cut after them),
  no ticked box for a check that didn't run, the human test stated.

### WATCHING — the third-party AUR package `aurora-viewer-git`

Uploaded 2026-10-30 by `jesusokay` (no project tie; their `lostorm-git` is
ban-evasion HWID spoofing on `git.allthefallen.moe`). First PKGBUILD clean.
G.W., 2026-10-31: watch only; any upstream post is drafted for G.W. first.
Scheduled task `aur-aurora-viewer-watch` (Mon ~09:40, while the app is open)
runs `~/.local/share/aur-aurora-viewer-watch/check.sh` (0 no change, 1 diff →
reviewed + `notify-send`, 2 lookup failed); state in `~/.local/state/` same name.

### DECIDED — do not reopen

- **DLSS: no** (user, 2026-10-28; upstream #209 chose FSR): the RTX SDK licence
  is GPLv3-incompatible. **Frame generation: dropped** (no Vulkan/Linux path).

### NOT QUEUED

- **0.8.3 VRAM / device-lost reports: left to upstream** (G.W., 2026-10-30;
  no AMD card). Fixed by upstream #1592, after `v0.8.4-beta.20261030.1`.
  Unverified: AMD (#1490, #1486) reports budget = usage to the MB — revisit
  only if AMD reports continue once #1592 ships.
- **Linux notification click-to-raise: the maintainer answered 2026-10-29**
  on Aurora-Viewer/Aurora-Viewer#1397 (merged): keep the ⬜ "later" line, and
  a separate PR once testable on a server sending `ActivationToken` (winit
  0.30.13's Wayland `focus_window` is empty; quickshell 0.3.2 sends no token,
  measured with `dbus-monitor`).

---

### Linux review leftovers (2026-10-31, not queued)

From the Linux review that produced #1753 and #1757: Linux-only unit tests
absent from upstream's CI (now PR #1760); keyboard
layout detection is a no-op on Linux (AZERTY gets WASD, #1741-adjacent);
resource monitor and per-thread CPU empty on Linux (`sys_probe.rs`,
`thread_cpu.rs`); `xdg-open` children never reaped; GPU-stall box never
closes; Wayland never flashes the taskbar; fonts ignore `XDG_DATA_HOME`.
Upstream #1659 (`check.ps1` false green) was fixed by #1675.

**#1760 hand-over (2026-10-31).** `ci.yml` job `clippy (Linux)`: `cargo test
--profile ci --workspace --locked` on PRs and the queue, `--no-run` cache build
on main pushes, `shared-key: linux-tests` (macos.yml's pattern). Passed locally
headless (`env -i PATH HOME`, 6549 tests); check.ps1 All green; Fable and codex
reviews clean after fixing the PR body's list of Linux-only tests (only
`url_scheme.rs` `xdg_tests` and `slurl_handoff.rs`). If the maintainer finds
the PR wall time (~11–14 min, estimated) too long: one-line change to
`github.event_name == 'merge_group'`, already offered in the body. A first-run
failure on a missing library goes in the `System libraries` step. Merged →
drop worktree `aurora-viewer-contributing-91cc75` and the branch.

## Finished — read on demand

- **Research for DLSS / FSR (2026-10-28), in case FSR work starts:**
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

- **Researched, not queued: 3D preview of inventory objects** (Discord
  idea, 2026-10-30; no issue, no code). Fetching without rezzing is
  impossible: the Legacy Viewer's asset table marks `AT_OBJECT` neither
  fetchable by ID nor ID-knowable (`indra/llcommon/llassettype.cpp`,
  enforced by `lookupIsAssetFetchByIDAllowed` in
  `lltransfersourceasset.cpp`); mesh IDs arrive only in ObjectUpdate
  ExtraParams once rezzed or attached. No viewer found with such a preview.
  Workable path: "Add" to a HUD point (only the owner receives HUD
  attachments), read the mesh IDs, fetch, detach, render with the existing
  `aurora_render::preview`. Limits: counts against the attachment cap, and
  **the object's scripts run on attach** (RLV, animations, perms), so warn
  or skip scripted items. TPV policy 2.a.i (no circumventing copy limits)
  and 2.b (export creator-only): a view-only preview with no save, export
  or UUID copy reads as allowed — our reading, not LL's; quotes came via a
  summarising fetch, recheck the page. Grid-only, unverified: ViewerAsset
  refusing `object_id=`, FetchInventory2 hiding `asset_id`. Feature
  requests go to feedback.aurora-viewer.org.

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
