# NixOS: the prebuilt Linux viewer will not start without `steam-run`

2026-11-01. **Reported, not reproduced** — a user on the Aurora Discord, running
NixOS; nobody here runs Nix. Recorded because G.W. has never used Nix and the
fix reads like magic without the why.

## What broke

The prebuilt Linux `aurora-viewer` from a release zip does not start on NixOS.
The usual symptom of this class of failure is a misleading
`No such file or directory` for a binary that plainly exists, or a missing
`.so` error. The user's working command:

```bash
NIXPKGS_ALLOW_UNFREE=1 nix-shell -p steam-run --run "steam-run ./aurora-viewer"
```

They added that every Second Life viewer needs this wrapper on Nix, which fits:
all of them ship prebuilt binaries.

## Why

A normal distro keeps shared libraries at fixed paths (`/lib`, `/usr/lib`) and
the program loader at `/lib64/ld-linux-x86-64.so.2`. A binary built on a normal
distro has that loader path and those library names baked in. NixOS has no such
paths: every package lives in its own hashed folder under `/nix/store/…`. So the
kernel cannot find the loader, which is what the confusing "No such file" refers
to. If the loader is found, the next error is a missing library (Vulkan loader,
X11/Wayland, ALSA/PulseAudio…).

## What fixes it

`steam-run` is a nixpkgs wrapper made for Valve's Steam client, which has the
same problem. It has nothing to do with Steam otherwise. It builds a small
sandbox (nixpkgs `buildFHSEnv`, bubblewrap namespaces) where `/lib`, `/usr/lib`
and the loader exist and hold the libraries of Steam's runtime, then runs the
given program inside it.

- `nix-shell -p steam-run` fetches it into a temporary shell, installing nothing.
- `--run "steam-run ./aurora-viewer"` runs the viewer inside the sandbox.
- `NIXPKGS_ALLOW_UNFREE=1`: Steam is marked unfree, and Nix refuses unfree
  packages unless you allow them.

Other ways to get the same result, if a user asks: `nix-ld` (a system-wide
stand-in loader), a `buildFHSEnv` of our own listing only the libraries the
viewer needs, or a real Nix package that patches the binary's loader and library
paths into `/nix/store` (`autoPatchelfHook`).

## Ruled out

Nothing tested here. This is not an Aurora bug and needs no code change: the
binary is correct for every distro with the standard paths. Upstream's
`docs/LINUX.md` does not mention NixOS (checked 2026-11-01); a paragraph there
would be an upstream docs PR, drafted for G.W. first.
