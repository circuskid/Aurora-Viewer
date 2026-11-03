# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-11-03

Fork `circuskid/Aurora-Viewer`, `main` = upstream, our branch `circuskid` (this file);
`pkgbuilds/aurora-viewer` builds its tags; `~/Projects/phoenix-firestorm`: reference clone.

**State.** `circuskid` = upstream `0caf816d` + our docs, no patch of its own; main checkout
clean; policy reviewed through `0caf816d`. Package `v0.8.5.1793627160` (pkgbuilds e56e23c,
`check()` passed) installed by G.W. 2026-11-02; no package of `0caf816d` built yet (synced
2026-11-03). Nothing is half-done.

**Open upstream: #1921, #1922, #2077, #2078, #2083, #2085, #2086, #2087, #2090, #2091, #2093, #2097, #2149, #2151, #2166, #2177, #2178** (worktree `wgpu-performance-benefits-23c091`; drop it
and its branches once both merge). #1921 (issue #1919): a worn unrigged mesh hidden by the
transparent texture takes the touch (a slapper). #1922 (issue #1920): a worn animesh keeps its
touch (#1796's `own_rigged` had caught it). Both grid-tested by G.W., codex + Fable reviewed,
144-run `worn-*` sweep identical. Whichever merges second merges main (one TESTING.md line).
11-02: #1921 ✅ team review (red macOS = runner abandoned `rest`, not us); #1922 no review yet.
**#2077** (issue #2025; worktree `adapter-choice-match-22edad`, branch `fix/gpu-adapter-name-match` f936367a): a saved GPU name matching several adapters takes the first that can present, an exact name beats a longer one containing it, and the warning tells "no adapter matches" from "cannot present". check.ps1 green, demo ×3, G.W. tested the Preferences path, codex + Fable ×2; Auto-fix on.
#2025's cause is unproven: our comment asks the reporter for J4, the `GPU adapter` log lines and `vulkaninfo --summary`; their reply decides `Refs` vs `Closes`. The TASKS line lacks the PR link (add it with any review push). **#2093** (no issue; worktree `chat-time-switches-66689c`, branch `feat/chat-time-switches` 47df5caa, opened 2026-11-02): Preferences › Chat's one Show time becomes `chat_timestamps_nearby` (Nearby chat group: lines + on-screen cards) and `chat_timestamps_im` (Conversations group: private, group, popped-out), both on (Legacy FSShowTimestampsNearbyChat / FSShowTimestampsIM); a saved `chat_timestamps` is read once into both (`former_chat_timestamps`), never written back. **No transcripts switch (G.W., 2026-11-02)**: don't add one. check.ps1 All green (7,028), G.W. demo-tested, codex + Fable clean, 88-capture A/B vs main (11 scenarios incl. Legacy style, bubbles, legacy lines, popout; ×2, on/off) = noise only; body updated with it. Auto-fix on, CI waits on first-contributor approval. The TASKS line lacks the PR link (add it with any review push). Expect a merge with #2087 (`th.times`) and with #2097 (same chat files). Capture scripts: its `.scratch/runs/` (`capture.sh`, `ab.sh`, `compare.sh`). **#2097** (no issue, a feature; worktree `chat-time-12h-99cbb7`, branch `feat/chat-time-12h` 840dc9ef, opened 2026-11-02): every chat time (all chat styles, on-screen cards, notifications) follows the bars' clock 12/24 h (`clock_format`, Auto default) via a per-frame `clock::set_chat_format`; money tracker and offline "(Saved at …)" stay 24 h (`hhmm_24`, as Legacy); Compact time column 40→62 px in 12 h and scaled with chat text size; notification titles reserve the measured time width; roomy hover time shrinks to fit its gutter. Legacy differs: its `Use24HourClock` is a separate setting defaulting to 24 h (said in the body); transcripts stay 24 h. check.ps1 All green (7,028), G.W. demo-tested 12/24 h switching, codex + Fable reviewed (both findings fixed in 840dc9ef), merges cleanly with #2093. Captures on the fork's `captures` branch `chat-time-12h/`. Auto-fix on, CI waits on first-contributor approval. The TASKS line lacks the PR link (add it with any review push). Unchecked by capture: the roomy hover time at 200 % text (pointer-only). If #2087 merges first, merge main: its notice times go through `hhmm` and follow on their own.
**#2149** (issue #2143; worktree `gtao-pr-lookup-9cc8eb`, branch `fix/toolbar-toggle-buttons` 612e7318 = upstream `ce5bdac1` merged in (conflicts with #2110, #2052, #2126: adjacent additions, both kept), opened 2026-11-03): toolbar Profile, Poser, Report abuse, Marketplace listings toggle their window (minimized expands; closed through the window's own close via new `widgets::close_floater`, so unsaved-changes questions still ask); those plus Build and Teleport history light (`toolbar::Shown`); menus stay open-only; Create landmark unchanged (Legacy never toggles it). Teleport history keeps Legacy toggleTeleportHistory (hides Places on any tab; a minimized one now expands): don't make it switch tabs, Fable caught that. check.ps1 All green on 612e7318 (7,202), G.W. demo-tested 0677ff26 and 49abb496 (merge not demoed, compiles + tests), codex clean + Fable (2 points fixed). TASKS line links the PR. Team review ✅ ready to merge (2026-11-03; maintainer pushed 2de2168e: TASKS ✅→🟡 + ⬜ grid check, pulled into the worktree). Issue reply posted; merge is theirs. Return check: `gh pr view 2149 --repo Aurora-Viewer/Aurora-Viewer --json state,mergeable,reviews`. **#2085** (issue #1823, the media-pack half; worktree `media-pack-in-use-1def9e`, branch `fix/media-pack-in-use` 37af73b5, opened 2026-11-02): on Linux/macOS a viewer installing a newer pack no longer deletes the pack a running viewer uses: per-pack shared flock `media-plugins/.in-use-<v>` held for the viewer's life, prune/replace only under an exclusive one. Lock files are never unlinked (an unlink lets a viewer lock an orphan inode) and a damaged pack found at start is not held (it would block repairs): both from the codex + Fable review, don't undo them. check.ps1 green, two-viewer A/B ×3 each, neighbour demos, G.W. demo-tested; Auto-fix on, CI waits on the maintainer. `Refs #1823` (TASKS 🟡); two ⬜ limits in TASKS (a kept pack goes only at the next newer install; pre-fix viewers take no lock). **#2086** (issue #1780, branch `fix/im-toast-emote` a5598386): an emote IM toasts "Name waves", not "/me waves" (LLToastIMPanel); demo `AURORA_DEMO_TOASTS=emote`. **#2087** (issue #1999, branch `fix/chat-notice-times` fdcaf6ec): the viewer's notice lines show their time in every chat style (FSChatHistory::appendMessage). Both opened 2026-11-02 from worktree `new-viewer-update-d7fc5f`: upstream main merged in, check.ps1 All green (7,029 / 7,027 passed), touched + neighbour demo scenarios ×2, G.W. demo-tested both (Compact with Show time on and off on the final head), codex + Fable reviewed; Auto-fix on, `Refs` in both, CI waits on the maintainer's first-contributor approval. Capture profiles: `XDG_DATA_HOME`/`XDG_CONFIG_HOME` must be absolute (a relative one is ignored and the run uses G.W.'s shared demo profile); script in that worktree's `.scratch/runs/capture.sh`. **#2078 + #2083: native crash stacks on Linux** (refs #2004 #2025 #2051; worktree `linux-crash-handler-d8ba35`, branches `feat/linux-native-crash` f75bf1fd and `feat/linux-freeze-watchdog` b4b1a53c). #2078: fatal-signal handler (`crash/linux.rs`) writes `native.txt` as `module+0xoffset` + ELF build ID; panic/OOM stacks; summary `Native signal SIGSEGV (11)`. #2083 **contains #2078's commits, merge #2078 first**: watchdog on Linux (main-loop heartbeat `crash::pumped`, 24 ns/pass), main/render stacks via `tgkill` SIGRTMAX-2 for `freeze.txt` and `gpu-stall.txt`. Both: check.ps1 All green (7,029 / 7,030), repeated demo runs, G.W. demo-tested native + freeze, codex + Fable reviewed; Auto-fix on, CI waits on first-contributor approval. If #2078 changes after review, merge it into the #2083 branch (never rebase). Dropped (G.W., 2026-11-02): a Linux crash watcher (a non-parent cannot read the exit status; would need a supervisor); its ⬜ TASKS line stays as upstream's item. Separate later PR: Linux symbol files + triage `Native signal` kind. Design notes and gotchas (8 KB alt stack: no big arrays in the handler): `~/.config/Claude/side-session-notes/local_ef298c89-314c-4c84-a219-9b3402a7a9a5/linux-crash-diagnostics.md`. **#2090 + #2091: Wayland gaps** (worktree `wayland-layout-minimize-08871e`, opened 2026-11-02). **#2090** (issue #2088, branch `fix/wayland-active-layout` 320f642c, `Closes #2088`): on Wayland the keyboard layout came from the keymap's first layout; now a `wl_keyboard` of our own on winit's connection (`Backend::from_foreign_display`, private queue, `dispatch_pending` in `about_to_wait`) hears `modifiers.group`, labels follow it live, and a new profile or one saved before settings version 31 (new migration: 0.8.5 placed letters for the first layout) moves its default letter shortcuts once; later switches change labels only. check.ps1 All green on 320f642c (7,030 / 0 / 35); 38-run nested-Hyprland suite ×2 + neighbours identical to main; profile A/B no measurable cost (noisy, load 17); G.W. demo-tested 846bd073 (the migration came after, from codex + Fable review). Non-Linux `Watch` is an uninhabited `GroupWatch(Infallible)` stub: an earlier `match never {}` left unreachable code for Windows/macOS clippy, don't go back. Limits: XWayland unchanged; a keymap changed at run time is not re-read. #2052 (another contributor) also edits `keybinds.rs`/`app.rs`: whoever merges second merges. **#2091** (no issue, `docs/wayland-minimize-mute` 0f6d5ea6): "Mute when minimised" help text (en/fr/ja) + LINUX.md say it cannot work on Wayland (winit 0.30 `is_minimized()` = None, no Occluded, `suspended` not exposed). Both: Auto-fix on, CI waits on first-contributor approval. Notes: `~/.config/Claude/side-session-notes/local_ef298c89-314c-4c84-a219-9b3402a7a9a5/wayland-layout-minimize.md`. **#2177 + #2178: System fonts list** (worktree `font-search-close-177678`, opened 2026-11-03). **#2177** (issue #2136, branch `fix/font-search-close` 108b5ebc, `Closes #2136`): clicking the search no longer closes the list (`CloseOnClickOutside`), it filters, starts empty at each opening, and the list shrinks and grows with the matches (`let_font_list_grow`). Grow only on the frame after rows were cut off: raised every frame the hit area stays inflated (codex) and a list above its button is misplaced (Fable), don't undo it. **#2178** (no issue, a feature; branch `feat/font-list-keys` dd37c6c4, contains #2177's commits, **merge #2177 first**; if #2177 changes, merge it into this branch, never rebase): Up / Down / Enter, keys taken before the field as in #1752, Enter always consumed (else nothing found drops the field's focus, a6aaa776), selection scrolled into view on set (0e74d8a4). check.ps1 All green on both heads (7,140 / 7,141), A/B ×3 + `graphics` = noise only (#2178's run at dd37c6c4, 24 captures; `system-fonts-scroll` differs by a timing-dependent scroll offset, main vs main too), codex + Fable, G.W. demo-tested both. **Not hand-tested: Enter with nothing found** (test only; the body says so by omission). Fable's "repaint every frame on a short screen" was probed from 330 to 1,000 px and not reproduced. Both TASKS lines lack the PR link (add it with any review push). Auto-fix on #2178 (unconfirmed for #2177), both bound here, CI waits on first-contributor approval. Capture scripts: its `.scratch/runs/` (`capture.sh`, `ab.sh`, `compare.sh`). **#2151 + #2166: environment lists** (feedback post 160, no issue). **#2151** (worktree `search-environments-window-c4ac03`, branch `feat/environment-search` 4670e523, opened 2026-11-03): a search field at the top of the sky, water and day cycle drop-downs, in the environment window and the photo window's Light section (`settings_list`, `env_select::matching`); Enter applies the first match, a click in the field keeps the list open, ↑ ↓ go to the names; demo `AURORA_DEMO_ENV_SELECT=search|search-none`. check.ps1 All green (7,184), G.W. demo-tested, codex + Fable reviewed, neighbours 0 px against main. **Team review ✅ ready to merge**; the maintainers set its TASKS line to 🟡 with a ⬜ grid check (4670e523), so G.W. owes that grid test: type in the three lists. Their later idea, not for this PR: `widgets::clearable_search_box` would add a clear button. **#2166** (worktree `worktree-build-time-c540b3`, branch `feat/environment-list-order` 6194dde2, stacked on #2151; G.W. chose environment lists only): an A–Z switch on the environment lists. #2151 merges first.

**Worktrees left** (23 removed 2026-11-02): `font-search-close-177678` (#2177/#2178); `search-environments-window-c4ac03` (#2151); `worktree-build-time-c540b3` (#2166); `wgpu-performance-benefits-23c091` (#1921/#1922), `new-viewer-update-d7fc5f` (haze; #2086/#2087), `media-pack-in-use-1def9e` (#2085; drop it once merged), `linux-crash-handler-d8ba35` (#2078/#2083; drop once both merge), `chat-time-switches-66689c` (#2093; drop once merged), `chat-time-12h-99cbb7` (#2097; drop once merged), `wayland-layout-minimize-08871e` (#2090/#2091; drop once both merge; its `.scratch/bin/av-fix` and `gw*` profiles serve G.W.'s open nested Hyprland test), `transparency-pr-test-viewer-543e74` (the #1963 session), `contact-shadow-investigation-58c69f` (contact-shadow fade; its `target/debug/av-*` test builds, ~3 GB, go with it), `previous-work-continuation-06803e` (GTAO snow, branch `fix/gtao-hair-cast-occlusion`), `linux-framerate-perf-2a2ce9` (the checkpointing session's own; drop it from another session). Unsaved branches are on the fork as `archive/<branch>` (local copies deleted); removed worktrees' `.scratch/` is in `~/.local/share/aurora-viewer-scratch-archive/2026-11-02/` (1.4 GB; G.W. deletes it when unneeded).

### FIRST — the two-minute check

1. **Upstream drift.** `git fetch --no-tags upstream && git rev-list --count circuskid..upstream/main`, then the policy diff (`.claude/CLAUDE.md` *Branches*). Sync and ship when G.W. wants a build (`.claude/RELEASING.md`; a beta `v0.8.5-beta.*` gives base X.Y.Z). After a sync, from the main checkout: `.claude/bin/aurora-sweep` (dry run); 🔴 `--apply` only with G.W.'s go-ahead each time. **Architect, 2026-11-02:** it stays a prototype here until G.W. rules on shared tooling (`agent-config/docs/architect-audit-2026-10-23.md`); the push rule is in agent-config 405ca90 `worktree-sessions.md`. Its traps are in its comments.
2. **#1921 / #1922 / #2077 / #2085 / #2086 / #2087 / #2090 / #2091 / #2093 / #2097 / #2151 / #2166 / #2177 / #2178** (+ #2025 for the reporter): `gh pr view <n> --repo Aurora-Viewer/Aurora-Viewer --json state,reviews,comments`. Answer reviews per THEN below (one batched push: each push re-runs first-contributor CI).

### NEXT — the work, in order

- **GTAO snow on avatars (Discord, 0.8.5)** — worktree `previous-work-continuation-06803e`, fork branch `fix/gtao-hair-cast-occlusion` (cut from upstream f752ea9a; prototype commits carry no trailers, but are prototypes: squash before a PR). Two causes, both in `notes/gtao-snow-on-skin-under-hair-is-blended-hair-casting-occlusion.md`, both grid-tested by G.W. 2026-11-02: (1) #889's occlusion depth lets blended hair cast specks; a1e759f0 gives GTAO a second depth without cut-out faces (`flags::CUTOUT_TEX`), the snow "settles a lot faster", #889's demo identical to main. (2) breathing trips #1702's own-motion path at ½ px, so the avatar shows half-res grain as squares; `AURORA_GTAO_OWN=0.2,0.6,2` (83703d69, own motion only above 2 px) removes them. **Hem check done: a speed threshold undoes #1702's hem fix** (note), so the squares wait for another discriminator. **Hair fix alone (G.W., 2026-11-02): upstream branch `fix/gtao-hair-snow`** (worktree `gtao-continued-5c4ecb`, from upstream ace2f5d6, pushed to the fork, no PR): 6b9d0beb + format 9d61b9ff; switch dropped; hair/skin test made exact with the solid pass on the occlusion depth's own vertex shader (the 0.2 % margin left a dark dot at the nape). check.ps1 All green; demo A/B as main except the hair; cost `g_ssao` 5090 1.17→1.43 ms, 610M 4.55→5.40 ms (half the Rg32 levels + copy, half the extra pass; two R32 chains would save the first half). Drafts in its `.scratch/` (`issue-gtao-hair-snow.md`, `pr-gtao-hair-snow.md`, `pr-hair-*.png`). **Grid-tested by G.W. 2026-11-02 (9d61b9ff):** the hair-band snow is fixed; hair may shade a little less than before; the breathing grain (cause 2) still dominates what he sees, so the two are hard to tell apart by eye. **G.W.: hold the PR until both causes are fixed, then judge them together.** Next: a discriminator for own motion that keeps #1702's hem (ideas in the note). A/B baseline worktree `gtao-ab-main` (detached): remove when done. Test viewers for G.W.: `hl.exec_cmd("cd <wt> && env … ./target/debug/aurora-viewer")`, no workspace, after `pgrep` shows his has quit.
- **Haze sun glow** (worktree `new-viewer-update-d7fc5f`, branch `fix/haze-sun-glow` 909da1e4, pushed to the fork, no issue yet). A tester sees a soft sun through opaque rock (Bloom and Glow off), locked to the sky sun, gone under Midday. Cause: objects and terrain ran calcAtmosphericVars in world space, so its haze glow peaks on whatever is in front of the sun (#1781 fixed only the water); the fix runs it in eye space everywhere, plus demo `AURORA_DEMO_SKY_SUN`. A glow-only probe shows the hotspot on the demo hill (world) and none (eye); a normal demo frame has not shown it yet. **Waits on G.W.** testing the fix build at the reported spot (location in that session, not the repo); then demo A/B, 5090 + 610M timings (`AURORA_GPU=1`), check.ps1, issue + PR drafts.
- **Contact-shadow close-range fade — waits on Aznlilly** (G.W. sent her the note 2026-11-02). Her `codex/contact-shadow-investigation` (snapshot 53a6aaba) lengthened Bend's 60 px search for close-up cutoffs; our A/B (her `shadow-zoom` demo, 5090): the 10 cm bounded reach spreads hard faceted shadow (neck, torso side), her full-view reference costs 5.7–10.5 ms `g_contact` vs 0.12. Real cause: the depth buffer's flat facets cast hard edges where the shadow map (2 cm normal offset, soft filter) already shades. Our fix, fork branch `contact-shadow-close-fade` (a81ed709, one clean commit, no AI trailers, from upstream 87db85b7): fade the mask in over 1.5–2.5 cm of search reach in metres (`AURORA_CONTACT_FADE=<from>,<to>`, `0` off); 2.4 m and 1.2 m identical to Bend (<1 px), 0.6 m spots and jagged edges gone, +0.002 ms. Wider ranges (2–5, 3–6, 5–10 cm) erased the 1.2 m hairline/chin shadows: don't widen it back. Before any PR (only if she wants us to): grid test, 610M timing, photo/tiled-photo check, check.ps1, TESTING.md line, bug issue. `archive/contact-shadow-close-fade` (same change, our trailers) is redundant: delete when done.
- **Chat times (G.W., 2026-11-02): both done**, #2097 (12/24 h) and #2093 (Show time switches), above.
- **#1847 promise:** one or two real `sysrq` resets on btrfs (claim the machine from active peers first; the dm method is in the A/B comment on #1847, 11-01).
- **Next issues:** #1963 (0.8.5 FPS drop in inventory/appearance; the "1963" session is on it), **#1815** (Movement button opens camera controls; `bars.rs` maps `"move"` right, suspect `controls_ui.show` in `app/controls.rs` with classic controls off), #1709; skip #1530. #1702's line continues with TAA motion vectors (610M cost is the Rgba16F width; 8-bit normals and an Rg8 target: ruled out).
- **G.W. owns:** #1407 Linux voice full test (mic under load, device switch mid-call) when he next uses voice; blocks nothing. Test viewers: no `--title` (changes the D-Bus name); quit from the menu, never kill (the next login gets no caps). Four stamp tags remain (`v0.8.5.1793540223` on): tag deletes run in a terminal tab G.W. approves (the classifier refuses them).

### Gotcha — `secondlife://` links and test builds

The link handler `~/.local/share/applications/aurora-viewer-slurl.desktop` runs
`/usr/bin/aurora-viewer` (checked 2026-11-02). Registering links from a worktree build
points it at that build, and removing the worktree then breaks links: re-register from
the installed package. Check: `gio mime x-scheme-handler/secondlife` and the file's `Exec`.

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
switch back and delete the branch. **`.claude/CLAUDE.md` is capped at 9,000 characters**
(`BUDGET` in `claude-status-inject`; it counts characters, and `wc -c` counts bytes, 🔴 = 4;
8,857 on 2026-11-02): trim before adding. Check, in characters:
`python3 -c 'import sys;print(len(open(sys.argv[1]).read()))' .claude/CLAUDE.md`. The
`--rules | grep TRUNCATED` check reads the main checkout's copy, not your branch's.

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
- **`stream_volume.rs` reconnect gap: not reproducible, no PR** (2026-11-02).
  A refused reconnect is retried only on the next request (`run` blocks on
  `recv()`; `StreamVolume::set` skips unchanged volumes), but a
  `systemctl --user restart` of the socket-activated pipewire-pulse never
  refuses it (measured: the worker reconnects in the same second), and CEF
  never recreates its stream after any restart (pages stay silent until
  their plugin restarts). Bites only without socket activation and with a
  new plugin within 10 s. Method (a private pipewire-pulse on its own socket
  via `systemd-run --user --socket-property=ListenStream=…`, so G.W.'s audio
  is untouched) in `~/.config/Claude/side-session-notes/local_ef298c89-314c-4c84-a219-9b3402a7a9a5/stream-volume-retry.md`.

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
