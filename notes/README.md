# Notes

One-off findings: things that took real work to diagnose and would cost the same
work again if forgotten. Here: how the viewer behaves on Linux setups and grids
that upstream does not cover.

**This index is the entry point.** Scan the table, open only what's relevant.
Conventions — what earns a place, the four parts, the 180-character cap on the
`What` cell — are canonical in
[agent-config/notes-convention.md](https://github.com/circuskid/agent-config/blob/main/notes-convention.md).
Read that before adding a note; this file says only what is local to this repo.

## Local to this repo

- Running state is [STATUS.md](../STATUS.md); our rules are
  [.claude/CLAUDE.md](../.claude/CLAUDE.md).
- This folder lives on the `circuskid` branch only, like `STATUS.md`. It must
  never reach an upstream PR (those branches are cut from `upstream/main`).

## Index

Newest first.

| Note | Date | What |
|---|---|---|
| [nixos-prebuilt-viewer-will-not-start-without-steam-run.md](nixos-prebuilt-viewer-will-not-start-without-steam-run.md) | 2026-11-01 | NixOS has no /lib or standard loader, so prebuilt viewers fail; `steam-run` fakes them. A user report, never tested here. |
