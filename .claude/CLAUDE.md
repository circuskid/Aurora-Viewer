# aurora-viewer — our fork of Aurora-Viewer/Aurora-Viewer

Not our project. Upstream is `Aurora-Viewer/Aurora-Viewer` (GPLv3, a Rust/Vulkan
Second Life viewer); our fork is `circuskid/Aurora-Viewer`, personal account,
public (G.W., 2026-10-28). We build our own packages from it and send PRs
upstream. This file sits beside upstream's `CLAUDE.md` (`@AGENTS.md`) and never
edits it, so merging upstream never conflicts with us.

## Branches (G.W., 2026-10-28)

- **`main` mirrors `upstream/main`, nothing else.** Sync: `git fetch upstream`,
  `git switch main && git merge --ff-only upstream/main && git push origin main`.
  Never commit to it.
- **`circuskid` is what we build**: upstream plus our patches plus our docs
  (this file, `STATUS.md`). The main checkout stays on it. Bring upstream in
  with `git merge upstream/main`, not a rebase: the release tags sit on it.
- 🔴 **Every sync reviews upstream's policy diff** before merging:
  `git diff <reviewed> upstream/main -- AGENTS.md HUMANS.md CONTRIBUTING.md
  .github/ docs/BRANDING.md docs/TRANSLATING.md`. A rule that contradicts an
  override below, or a process change that touches our PRs, is told to G.W.
  and reflected here; then move the mark. A commit count says nothing about
  rules: they land as `chore(tooling)` among ~80 commits a day.
  **Reviewed through `035776f5` (2026-11-01).**
- **An upstream PR branch is cut from `upstream/main`**, never from
  `circuskid`: `git switch -c fix/<name> upstream/main`. Our docs must never
  reach a PR. Push it to `origin`, open the PR against `Aurora-Viewer:main`.
  A patch we want before upstream takes it is merged into `circuskid` too.

## Upstream's rules (AGENTS.md)

🔴 **AGENTS.md binds us in full, current version, except the overrides
below.** It loads through upstream's `CLAUDE.md` in every worktree, so read it
there; this file never restates it (a copy drifts: the feedback board, #1543,
and the `on-hold` PR label, #1772, went unnoticed for days). This section holds
only our overrides and what the maintainer said outside the repo (Discord,
reviews), which nothing else records.

**Overrides**, because we are outside contributors, not on the Aurora team:
- No `scripts/new-task.ps1`, no `../work/` worktrees: sessions use our
  `.claude/worktrees` like any other repo.
- 🔴 **A viewer you start to test or capture goes on workspace 4**, not
  §4's bare `cargo run` (it put capture loops on G.W.'s workspace 1,
  2026-11-01): `hyprctl eval 'hl.exec_cmd("cd $PWD && <cmd>", { workspace =
  "4 silent" })'`, then wait on the capture file, not the process. Only a
  viewer G.W. will use himself opens where he is (no workspace). `--title`
  is for demo and capture runs; a grid test viewer runs without it (STATUS).
- 🔴 **We never merge, review-approve, label, triage or move board items
  upstream** (§5, §8 steps 8–10, §11's posting, §15's board), and we never touch
  TASKS.md for them except inside our own PR's diff. An `on-hold` label on our
  PR also means no push and no comment until it is removed.
- 🔴 **Every PR, issue or comment upstream is drafted and shown to G.W. first**
  (global *Safety* rule: a project we don't own).
- 🔴 **Upstream, follow §10: no AI mention anywhere** (G.W., 2026-10-28). No
  `Posted by an AI agent` line on PRs, issues or comments (an exception to the
  global *Safety* rule, which names this repo), and no `Co-Authored-By` or
  `Session:` trailer on upstream PR branches. The draft-first gate above still
  holds. Commits on `circuskid` follow our usual trailers.
- 🔴 **Get §9's PR shape right before the first push**: a fix pushed after
  review re-runs first-contributor CI, which waits for maintainer approval
  (TASKS.md line format missed on #1332, 2026-10-28).
- 🔴 **The maintainer removes contributors who skip the PR checks after being
  asked** (a contributor was removed on 2026-10-30, and all of their PRs
  closed). Their stated bar, every PR: `check.ps1` until "All green.", a
  test of your own (demo or grid) said in the PR, a `type(area): summary`
  title with the template filled in, and one PR per change with fixes pushed
  to that same PR.
- 🔴 **A visual or rendering PR shows it regresses nothing and costs little**
  (the maintainer to contributors, aimed mostly at us, 2026-10-31). Every PR
  touching 3D or rendering carries A/B evidence: demo captures before/after at
  the same settings and `AURORA_PROFILE=1` numbers. A new visual feature (fog,
  sun rays…) is a user option, **off by default**. A feature that costs a
  real share of the frame rate (their example: 30 fps) is refused, so measure
  before proposing one, and say the cost in the PR.
- 🔴 **Pre-PR gate for anything touching rendering, run before the first push.**
  `check.ps1` is the process bar; the maintainer reviews against AGENTS.md §11's
  table, and every row we leave empty they fill themselves, then send back
  (#1702, 2026-10-31: all five below were missed). The PR body carries each
  result:
  1. **Pixel A/B against `upstream/main`**, same settings, still **and**
     moving scenes (camera orbit; avatars moving on their own,
     `AURORA_DEMO_PHYSICS=1`), with the feature's debug view if it has one.
     "Motion doesn't show in a still frame" is not an exemption: diff frame
     sequences. Report what differs beyond main-vs-main noise, and look for
     what the change could break (#1702's lifting dress dragged the ground's
     occlusion for 15 frames, found by the maintainer).
  2. **Cost on the Radeon 610M as well as the RTX 5090**, `AURORA_PROFILE=1`,
     alternated runs, medians, before → after, as fps and ms. The 5090 alone
     hides any cost.
  3. **Cost with the feature off is measured, not argued.** A claim like
     "off is unchanged" needs a number with it off (#1702's copies ran with
     GTAO off).
  4. **New visual behaviour is a setting, off by default** (rule above),
     unless it only fixes a defect in existing output.
  5. **Fresh overlap check** right before pushing: `git fetch --no-tags
     upstream` and `git log upstream/main -- <files touched>`; merge main if
     someone landed near us (#1753 duplicated #1749). Ask what the change
     costs *them* too: CI minutes, review time (#1760 took their PR CI from
     4 to 13-15 min).

  TASKS.md and the body never say "validated" for the maintainer's own
  validation step; we state our own test and its date only.
- **`check.ps1` runs on Linux**: `pwsh -NoProfile -File scripts/check.ps1` in
  the PR's worktree (its "All green." verified real on #1702). Sum the tests
  over every `test result:` line (28) for the PR body.
- 🔴 **Never tick a Tests box for a check that didn't run as written.** Tick
  the `check.ps1` box only when it printed "All green." on the head as
  pushed; list each step with its real result, and name the date of the
  human's grid test.
  #1630 ticked "check.ps1 green" over filtered tests and had to be corrected
  (2026-10-30).
- **What the maintainer's review checked on #1397** (2026-10-29; their review
  is a §11-shaped PR comment). Get these right in the first draft:
  - **The title's type must match the label they will set** (§9): we cannot
    add labels. A Linux path that was missing is a `feature`, even when its
    issue reads like a bug.
  - **Legacy Viewer claims go field by field.** Say which behaviours follow
    the Legacy Viewer source and which are Aurora additions, in the PR body
    and in the TASKS.md line. A blanket "as in the Legacy Viewer" gets
    checked against the `.cpp` and corrected.
  - **Re-wrap any docs paragraph you edit** to the file's width.
  - **Read every `cfg(windows)` arm you touch**: Linux clippy cannot see
    them, and Windows CI fails on what it would flag (an unreachable match
    arm failed #1397).
- **Our PRs that touch the same area answer each other's nits.** When a
  review nit is fixed by another of our open PRs (the `.desktop` `Icon=` line,
  fixed by the desktop-identity PR), leave it out of this one and point to
  the other instead of making the two PRs conflict.
- Their reference sources: `../phoenix-firestorm` = `~/Projects/phoenix-firestorm`,
  a `--depth 1` clone, read-only, not one of our projects. Refresh it with
  `git -C ~/Projects/phoenix-firestorm pull --depth 1`.

## Building and releasing

- Dev build: `cargo build --release -p aurora-viewer` (Arch deps:
  `docs/LINUX.md` *Building from source*).
- 🔴 **`circuskid-release` can't run here** (it requires `main`); release by
  hand per [RELEASING.md](RELEASING.md).
- 🔴 **No CI on the fork** (G.W., 2026-10-28). All inherited workflows are
  disabled (`gh workflow list -R circuskid/Aurora-Viewer --all`): `Release`
  would publish on our stamp tags, `Crash triage` fires on every new issue.
  After each upstream merge, disable any new workflow. Never the homelab
  runner: a public repo would run strangers' PR code on it.
