# Releasing aurora-viewer — the fork's deltas

Read when packaging. The convention is `agent-config/packaging-convention.md`;
this file holds only where our fork departs from it. The rule that sends you
here is in `.claude/CLAUDE.md` *Building and releasing*.

## Why by hand

`circuskid-release` requires `main`, and our `main` mirrors upstream's. The
package (`pkgbuilds/aurora-viewer`, OWNER `aurora-viewer`) builds a tag on
`circuskid` instead.

## Steps, on `circuskid`

1. `stamp=$(date -u +%s)`; base = the X.Y.Z of
   `git describe --tags --abbrev=0 upstream/main` (a beta tag such as
   `v0.8.5-beta.*` gives `0.8.5`).
2. `git tag v<base>.$stamp && git push origin v<base>.$stamp`.
3. In `~/Projects/pkgbuilds/aurora-viewer/PKGBUILD`: `pkgver=<base>.$stamp`,
   `pkgrel=1`, `_upstream=` the full upstream tag name, then `updpkgsums`,
   commit, push.

## What the build reports

The binary reports `<upstream tag>+circuskid.<stamp>`; the PKGBUILD's `check()`
holds it to that. The grid sees only `X.Y.Z.0`. The PKGBUILD deletes the tags in
its clone, because our four-number stamp tags break
`crates/aurora-version/build.rs` (STATUS.md, *dev builds on `circuskid` panic*).
