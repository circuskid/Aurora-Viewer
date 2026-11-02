# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-11-01

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid` (this file);
`pkgbuilds/aurora-viewer` builds its tags; `~/Projects/phoenix-firestorm`: reference clone.

**circuskid carries no patch of its own.** Synced 2026-11-01 to `upstream/main` 6ee98032
(packaged, see below), bringing in our merged #1702, #1760, #1768, #1796, #1797. Drop worktrees
once they hold nothing else: `slurl-combined` (re-register links first, gotcha below),
`wgpu-performance-benefits-23c091`, #1752's, `login-profile-pic-pixelated-960617`,
`awesome-dhawan-42ba61`, `reported-issue-review-14cf6d`, `aurora-viewer-contributing-91cc75`,
`post-feedback-3b6d89`, `linux-framerate-perf-2a2ce9` (+ branches `test/motion-plus-hilbert`,
`backup/linux-framerate-perf-pre-split`).
**Open upstream.** #1796 and #1797 merged 11-01 (drop worktrees `festive-maxwell-3bee95`,
`status-check-b00dfc`). **#1792** (`attachment-interaction-issue-fe6583`, drop old branch of that name
too): main with #1796 merged in 86ee2135, overlap resolved as the maintainer asked; waits on re-review. From the triple-audit sessions: **#1841** (`linux-small-fixes-d9b801`): changes requested 11-01 (merge main for ARCHITECTURE.md, `aurora-open-wait` in monitor `GROUPS`, TASKS PR link), handed to its session; **#1838** green, unreviewed; **#1821**, **#1830**, **#1832** wait on CI approval. **#1861** merged 11-01 (drop worktree `aurora-linux-crash-test-7caa4d`, branch `fix/heartbeat-off-frame-thread`). Power-cut A/B of #1847 posted on #1847 11-01 (loop image + dm, ext4 and btrfs, native NVIDIA: logs lost before 11/12, after 0/12). Still promised there: one or two real `sysrq` resets on btrfs (claim the machine from active peers first). **#1865** (worktree `open-issues-review-bbd228`, branch `fix/remove-outfit-folder`, issues #1849 + #1417, opened 2026-11-01): inventory Remove from current outfit on any folder (dropped `require_outfit` from the take-off, like `takeOffOutfit`); new demo `AURORA_DEMO_APPEARANCE=take-off-folder`, 26 A/B runs, grid-tested by G.W.; CI waits on the maintainer. Captures on the fork's `captures` branch (`remove-outfit-folder/`). **Haze sun glow** (worktree `new-viewer-update-d7fc5f`, local branch `fix/haze-sun-glow` 909da1e4, not pushed, no issue yet): a tester sees a soft sun through opaque rock (Bloom and Glow off), locked to the sky sun, gone under Midday. Cause: objects and terrain ran calcAtmosphericVars in world space, so its haze glow peaks on whatever is in front of the sun (#1781 fixed only the water); the fix runs it in eye space everywhere, plus demo `AURORA_DEMO_SKY_SUN`. A glow-only probe build shows the hotspot on the demo hill (world) and none (eye); a normal demo frame has not shown it yet (sunrise light too weak). Next: G.W. tests the fix build at the reported spot (location in this session, not in the repo), then demo A/B, 5090 + 610M timings (`AURORA_GPU=1`), check.ps1, issue + PR drafts. Next issue candidates: **#1815** (Movement button opens camera controls; `bars.rs` maps `"move"` right, suspect `controls_ui.show` in `app/controls.rs` with classic controls off), #1780, #1709; skip #1530. #1865's test method and a captures-branch trap: *#1865 notes* below the rule.

Latest package: tag `v0.8.5.1793564791` (2026-11-01, base `v0.8.5-beta.20261101.2`
plus 11, `check()` only), installed as `0.8.5.1793564791-1` (grid sees `0.8.5.0`). Tree clean, `circuskid`
pushed. Stamp tags trimmed to the newest three.

### FIRST — the two-minute check

1. **Upstream drift.** `git fetch --no-tags upstream && git rev-list
   --count circuskid..upstream/main`, then the policy diff (`.claude/CLAUDE.md`
   *Branches*). A sync should be conflict-free (no code of ours); ship when
   G.W. wants a build. Upstream now tags betas of the next version
   (`v0.8.5-beta.*`): the package base is that tag's X.Y.Z.
2. **Grid follow-ups G.W. owns:** #1407 Linux voice: initial grid test
   passed 2026-10-30; the full one (mic open under load, device switch
   mid-call) waits until G.W. uses voice (rarely); blocks nothing. The
   maintainer sets its TASKS.md 🟡; nothing for us to push. #1625 + #1630:
   grid-OK on local `test/slurl-plus-place-fix` (worktree `slurl-combined`, drop
   once merged). Test viewers: no `--title` (changes the D-Bus name);
   quit from the menu, never kill (the next login gets no caps).
3. **PRs #1792, #1796, #1797.** `gh pr view <n> --repo
   Aurora-Viewer/Aurora-Viewer --json state,reviews,comments` for each
   (Auto-fix on all three). CI waits for the maintainer. Answer reviews per
   THEN. #1702 merged 2026-11-01; next in that line: TAA motion vectors.
   #1702's 610M cost is the Rgba16F width; 8-bit normals and an Rg8 target:
   ruled out.

### Gotcha — this machine's `secondlife://` links run a worktree build

Packages before `0.8.5.1793480285` had no Linux link support; until the links are
re-registered from the installed `0.8.5` build (Preferences switch off, then on), still
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

**Editing this file from a worktree session:** a hook refuses edits in
`~/Projects/aurora-viewer` (the main checkout, on `circuskid`). Branch from
`circuskid` in the worktree, edit and commit there, then
`git -C ~/Projects/aurora-viewer merge --ff-only <branch>`, push `circuskid`,
switch back and delete the branch. **`.claude/CLAUDE.md` is capped at ~8,925 chars**
(the worktree rules hook truncates past it; 8,811 on 2026-11-01): trim before adding. Check:
`echo '{"cwd":"'$PWD'"}' | ~/.local/bin/claude-status-inject --rules | grep -c TRUNCATED` = 0.

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

### #1865 notes — test method and the captures-branch trap (2026-11-01)

- A/B = the branch vs. the branch with the old check put back (both carry the
  new demo); a per-mode capture loop on workspace 4 and a pixel diff of the
  window, with same-build runs as the noise floor. A scenario that only opens
  a window cannot fail: drive the action at a frame. `AURORA_DEMO=1` alone has
  no wearables (`AURORA_DEMO_APPEARANCE=<mode>` seeds them). Logs:
  `~/.local/share/auroraviewer/logs/`.
- Fork `captures` branch: other sessions push it, so fetch with
  `+refs/heads/captures:refs/remotes/origin/captures` before building on it
  (a plain fetch left it stale); in zsh write `"${c}:refs/heads/captures"`,
  `$c:r` is a modifier.

### Windlight colors — #1797's background and leftovers (2026-11-01)

- **Why:** friends found Aurora's skies grey next to Alchemy 7.1.9. Aurora matches
  Firestorm (checked on the grid); 7.1.9 runs LL's pre-nerf renderer: sky ambient
  `srgb_to_linear(tmpAmbient * 0.7)`, colored, where FS/LL soften it (pow 0.9, x0.57)
  and keep only its luminance on linear/HDR skies. #1797 restores that as an
  experimental option, off. Probe blend is the same in both (not the cause).
- **Decided, do not add:** a `RenderSkyAutoAdjustLegacy` switch (a contributor
  named it on Discord): on, it HDR-converts old skies, the opposite look;
  Aurora already keeps them classic like FS (setting off).
- **Pending:** G.W.'s Discord question to the maintainers (was the grey look a
  choice?) — #1797 lets them decide. The PR's cost runs ran at 1226x2055 (workspace
  4 tiles the window; `AURORA_WINDOW_SIZE` loses), stated in the body.
- **Leftovers, remove on G.W.'s say:** Alchemy 7.1.9 at `~/.local/opt/alchemy-7.1.9`
  (wine; launcher was a scratchpad script); a friend's Alchemy `settings.xml` in
  `~/Downloads` (personal data: never copy). Branch
  `claude/aurora-windlight-colors-cf28bd` (local, unpushed) holds the shelved
  "Fuller probe colors" prototype (linear probe-ambiance blend; barely visible on
  real skies) plus pre-PR copies of #1797's commits.

### Linux review leftovers (2026-10-31, not queued)

The Linux review (session "Linux review + CEF cache path fix") produced #1753,
#1757, #1760 (Linux CI tests), #1762 (XKB layout) and #1768 (resource monitor).
Still open, each small: `xdg-open` children never reaped (`logging.rs`,
`snapshot/gallery.rs`, `bug_report/files.rs`, `crash/window.rs`, script
editor); GPU-stall box never closes on Linux (`app/gpu_stall.rs`); Wayland never
flashes the taskbar (`app/attention.rs`); fonts ignore `XDG_DATA_HOME`
(`ui/fonts.rs`). Not worth a PR alone: NVIDIA minimums on FreeBSD / Wine (#1757
limits); shm without `O_EXCL` (as the Legacy Viewer). TASKS.md line ~2318 (Linux
YouTube white squares) is stale: fixed per a tester on #848; the maintainer's line.
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

**#1765 / #1766 hand-over (2026-10-31), black photos on slow GPUs.** Cause:
a photo drawn in one pass past the driver's ~2 s job limit (amdgpu ring reset,
Windows TDR) read back black or lost the device; 610M threshold ~6400x3600.
#1766 (`fix/photo-gpu-timeout`) caps a pass at 500 ms of measured GPU time
(1500 ms with interface/HUD), redraws a black pass smaller, keeps a black
part of window size. #1765 (`fix/photo-busy-frame`) waits for a busy render
thread (30 s cap), never re-sends a capture still on the GPU. Both branches in
worktree `reported-issue-review-14cf6d` (on `fix/photo-busy-frame`; `git
switch` to answer #1766). Three-way review (Fable, codex, agy) done, all
confirmed findings fixed; G.W. validated on the Radeon. Test without a real
slow GPU: `AURORA_DEMO_GPU_STALL=700:15` holds the demo photo frame (stall
notices pop up, nothing crashes). Not filed: RADV reports no memory budget in
the log though `VK_EXT_memory_budget` is present (`vram.rs`). Merged → drop
the worktree and both branches.

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
