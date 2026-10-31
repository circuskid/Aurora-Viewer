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
- **An upstream PR branch is cut from `upstream/main`**, never from
  `circuskid`: `git switch -c fix/<name> upstream/main`. Our docs must never
  reach a PR. Push it to `origin`, open the PR against `Aurora-Viewer:main`.
  A patch we want before upstream takes it is merged into `circuskid` too.

## Upstream's rules (AGENTS.md)

AGENTS.md loads through upstream's `CLAUDE.md`. **Its code rules bind us**:
Rust style, demo-mode testing, `tr!` strings in en/fr/ja, "Legacy Viewer",
never Firestorm, in anything new, English in the repo. 🔴 So do its §1 rules:
never connect to a grid and never handle credentials. G.W. logs in.

**Its team process does not bind us.** We are outside contributors, not on the
Aurora team, so these are overridden:
- No `scripts/new-task.ps1`, no `../work/` worktrees: sessions use our
  `.claude/worktrees` like any other repo.
- 🔴 **We never merge, review-approve, label, triage or move board items
  upstream**, and we never touch TASKS.md for them except inside our own PR's diff.
- 🔴 **Every PR, issue or comment upstream is drafted and shown to G.W. first**
  (global *Safety* rule: a project we don't own).
- 🔴 **Upstream, follow §10: no AI mention anywhere** (G.W., 2026-10-28). No
  `Posted by an AI agent` line on PRs, issues or comments (an exception to the
  global *Safety* rule, which names this repo), and no `Co-Authored-By` or
  `Session:` trailer on upstream PR branches. The draft-first gate above still
  holds. Commits on `circuskid` follow our usual trailers.
- 🔴 **Their PR-shape rules DO bind our PRs**, because the maintainer reviews
  against them: a TASKS.md line we add names its issue, then its PR
  (`#n` then `PR #m`, each linked, as its neighbours do; AGENTS.md §9), plus the PR template
  and `Closes`/`Refs`. Get them right before the first push: a fix pushed after
  review re-runs first-contributor CI, which waits for maintainer approval
  (missed on #1332, 2026-10-28).
- 🔴 **The maintainer removes contributors who skip the PR checks after being
  asked** (a contributor was removed on 2026-10-30, and all of their PRs
  closed). Their stated bar, every PR: `check.ps1` until "All green.", a
  test of your own (demo or grid) said in the PR, a `type(area): summary`
  title with the template filled in, and one PR per change with fixes pushed
  to that same PR.
- **`check.ps1` works on Linux** on a branch whose base has upstream #1675
  and #1681 (2026-10-31; our #1659 report). A step that throws now fails the
  check, the 5.1 parse runs through `pwsh` (#1655), and clippy passes
  `-A dead-code` off Windows, as the Linux CI job does. Run
  `pwsh -NoProfile -File scripts/check.ps1` in the PR's worktree; its "All
  green." was verified real on #1702 (all four steps ran, 6500 tests). Still
  sum the tests over every `test result:` line (28) for the PR body. It
  rewrites `~/Projects/.cargo/config.toml` (sccache) the same each time. On
  an older base, run its steps by hand with the same commands, including
  `-A dead-code`.
- 🔴 **Never tick a Tests box for a check that didn't run as written.** Tick
  the `check.ps1` box only when it printed "All green." on the head as
  pushed; list each step with its real result, and name the date of the
  human's grid test.
  #1630 ticked "check.ps1 green" over filtered tests and had to be corrected
  (2026-10-30).
- **What the maintainer's review checked on #1397** (2026-10-29). Their
  review is a PR comment in the AGENTS.md §11 shape (`## Review`, `Verdict:`),
  not a GitHub review. Get these right in the first draft:
  - **The title's type is the type label's** (`feat` ↔ `feature`, `fix` ↔
    `fix`), with an area (§9). A Linux path that was missing is a `feature`,
    even when its issue reads like a bug. We cannot add labels (`gh pr edit
    --add-label` is refused for our account), so the maintainer sets them;
    the title must already match.
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

- Dev build: `cargo build --release -p aurora-viewer` (the Arch deps are in
  `docs/LINUX.md` *Building from source*). Upstream's scripts are PowerShell;
  `pwsh` is installed (`powershell-bin`, 2026-10-30) and runs `check.ps1`
  (see *Upstream's rules*).
- **Package: `pkgbuilds/aurora-viewer`** (OWNER `aurora-viewer`). It builds a
  tag on `circuskid`. 🔴 **`circuskid-release` can't run here** because it
  requires `main`, and our `main` is upstream's. Release by hand, on `circuskid`:
  1. `stamp=$(date -u +%s)`; base = the X.Y.Z of `git describe --tags --abbrev=0 upstream/main`.
  2. `git tag v<base>.$stamp && git push origin v<base>.$stamp`.
  3. In `~/Projects/pkgbuilds/aurora-viewer/PKGBUILD`: `pkgver=<base>.$stamp`,
     `pkgrel=1`, `_upstream=` the full upstream tag name, then `updpkgsums`,
     commit, push.
- The binary reports `<upstream tag>+circuskid.<stamp>`; the PKGBUILD's
  `check()` holds it to that. The grid sees only `X.Y.Z.0`.
- 🔴 **No CI on the fork** (G.W., 2026-10-28). All eight inherited workflows
  are disabled (`gh workflow list -R circuskid/Aurora-Viewer --all`): `Release`
  would publish on our stamp tags, `Crash triage` fires on every new issue.
  An upstream merge that adds a workflow brings it in active, so check that
  list after each merge and disable newcomers. Never the homelab runner: a
  public repo would run strangers' PR code on it. Our upstream PRs are tested
  by upstream's CI; our builds by `makepkg`.
