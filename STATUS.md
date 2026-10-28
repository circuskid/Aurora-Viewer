# aurora-viewer — status

Our fork of Aurora-Viewer/Aurora-Viewer. Rules: [.claude/CLAUDE.md](.claude/CLAUDE.md).

## ▶ RESUME HERE — 2026-10-28

Set up 2026-10-28 from the `~/Projects` session. Fork `circuskid/Aurora-Viewer`,
`main` = upstream, our branch `circuskid` (this file). `pkgbuilds/aurora-viewer`
builds tags on `circuskid`. `~/Projects/phoenix-firestorm` is the shallow reference clone.
No patches of ours yet: `circuskid` is upstream plus docs.

### FIRST — the two-minute check

`git fetch upstream && git rev-list --count circuskid..upstream/main` prints
how far behind upstream we are (0 = current; upstream merges many PRs a day).

### THEN — what is owed

1. Unmeasured: does a worktree session load AGENTS.md twice (once from its
   own `CLAUDE.md`, once from the main checkout's)? That would be ~47 KB per turn.
   Check with `/context` in the first worktree session.

### NOT QUEUED

- Nothing. Upstream attribution was ruled 2026-10-28: follow AGENTS.md §10
  (`.claude/CLAUDE.md`).
