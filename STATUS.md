# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-29

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid`
(this file). `pkgbuilds/aurora-viewer` builds tags on `circuskid`.
`~/Projects/phoenix-firestorm` is the shallow reference clone.

**circuskid carries one patch of its own: the advanced inventory search**
(merge 11935b23, 2026-10-29; branch `claude/inventory-regex-filters-0221ca`,
cut from `upstream/main` 602e3ae8, feature commits free of AI trailers so it
went upstream clean as Aurora-Viewer/Aurora-Viewer#1411, opened 2026-10-29). Visual
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

1. **Our six open PRs upstream.** Did the maintainer review or merge any?

   ```
   for n in 1397 1405 1406 1407 1408 1411; do gh pr view $n -R Aurora-Viewer/Aurora-Viewer --json number,state,mergeable,comments --jq '"\(.number) \(.state) \(.mergeable) comments=\(.comments|length)"'; done
   ```

   At this checkpoint (2026-10-29) all six are OPEN (#1411 just opened).
   #1397 has 1 comment, the maintainer's review, which has been addressed;
   the others have 0 comments. A new `## Review` comment is the next work. Read it in
   full, then fix on that PR's branch.
2. **Upstream drift.** `git fetch upstream && git rev-list --count
   circuskid..upstream/main` gave 4 at this checkpoint (tip f629385e).
   `git merge-tree --write-tree --name-only circuskid upstream/main` predicts
   the next sync: only `TASKS.md` conflicts. Keep both sides: our 🟡
   advanced-search line and upstream's new lines. Upstream also edits
   `ui/inventory/aurora.rs` and `aurora/rows.rs`, next to our patch, so
   build and run the inventory tests after any sync.

#1411 = the feature rebased on 0dca8c46 (table below); app Auto-fix is on,
but G.W. sees any reply first. Merged, it supersedes our merge 11935b23.

### THEN — answer reviews on the Linux PRs

None of these PRs is merged into `circuskid`, so our package has none of
them yet. Each PR is cut from `upstream/main` and kept free of AI trailers,
per `.claude/CLAUDE.md`.

| PR | Branch | Worktree (`.claude/worktrees/`) | Head |
|---|---|---|---|
| #1397 notifications | `fix/linux-desktop-notifications` | `aurora-linux-improvements-5a6f8e` | 25b91982 |
| #1405 desktop identity | `feat/linux-desktop-identity` | `linux-app-id-b86aee` | a68637fa |
| #1406 jobs at lower priority | `perf/linux-thread-priority` | `linux-thread-priority-0343b7` | f86b1106 |
| #1407 PulseAudio / PipeWire | `feat/linux-pulseaudio-host` | none: `git worktree add` it | af9ebaad |
| #1408 local time | `fix/linux-local-time` | `linux-local-time-4c73c4` | 8e3883eb |
| #1411 advanced inventory search | `feat/inventory-advanced-search` | `inventory-regex-filters-0221ca` | 6f5ec3a5 |

- **To answer a review**: fix on the branch, then `git commit -m … -- <paths>`
  with no trailers, then `git push origin HEAD`. Draft any PR comment to
  G.W. first. To catch up, merge `upstream/main` into the branch. Never
  rebase it and never force-push it.
- **Each push re-runs first-contributor CI**, which waits for the
  maintainer to approve it. So batch the fixes for one review into a
  single push.
- **The PRs answer each other's nits.** #1397's `Icon=` nit is fixed by
  #1405, so #1397 points to #1405 for it.
- **The maintainer merges, not us**; the next sync brings merged PRs into
  `circuskid`. Merge one in earlier only if G.W. wants it in a build.
- **Verified 2026-10-29** with all four new branches merged on
  upstream/main 0dca8c46: the merge is clean, fmt passes, and 6089 tests
  pass with 0 failures. Clippy reports only dead-code lints, in files none
  of the branches touch. `check.ps1` was not run (no `pwsh`); every PR body
  says so.
- **#1406's gain** is probably nil here (pool = `cores − 2`, 32 threads);
  it could show on 4–8 cores. A measurement (`taskset -c 0-3`,
  `AURORA_DEMO_STREAM=4000`, 10 runs each) was offered, not asked for.

Ship a new build when G.W. wants one (steps in `.claude/CLAUDE.md`).

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

- **Linux notification click-to-raise: unshelved 2026-10-29 by G.W., left to
  the maintainer.** Not implemented; the notifications PR
  (Aurora-Viewer/Aurora-Viewer#1397, opened 2026-10-29) explains it and asks; no answer yet. A click opens the
  right conversation but cannot bring the window forward: winit 0.30.13's
  Wayland `focus_window` is an empty function, and raising needs the
  xdg-activation token a server may send with the click (`ActivationToken`).
  Measured with `dbus-monitor` on a `notify-send` probe: quickshell 0.3.2 sends
  only `ActionInvoked` then `NotificationClosed`, so on this desktop even our
  own activation code would not raise it. Act on the maintainer's answer.

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
