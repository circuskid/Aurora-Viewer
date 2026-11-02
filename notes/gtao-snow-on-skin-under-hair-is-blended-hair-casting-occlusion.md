# GTAO "snow" on skin under hair is blended hair casting occlusion

2026-11-02. **Reproduced on the grid by G.W.** (RTX 5090, 3768 × 2055, Deluxe,
GTAO Ultra), installed 0.8.5. No fix yet; the cause is confirmed by a switch
that turns it off and on.

## What broke

Testers on the Aurora Discord (0.8.5) reported cloudy, blotchy "shadows" on
interior walls and grainy speckle on their avatars' bodies, gone with Ambient
Occlusion off. On G.W.'s avatar it shows as a sandy grey speckle, a "snow", on
the skin of the back right beside the hair, both in the lit view and in the
occlusion-only view (Dev Toolbox R7). It is much more visible while the avatar
or camera moves, where it crawls. Lower GTAO quality makes the specks coarser.
Anti-aliasing TAA instead of SMAA does not remove it.

The misleading part: it looks exactly like GTAO sampling noise that the
temporal accumulation fails to quiet in motion, and the session spent hours on
that reading (below) before the band's location gave it away.

## How it was diagnosed

- The speckle sits in the hair's occlusion band on the skin, not on open skin
  (a still screenshot, zoomed).
- R7 with contact shadows off: the band is still speckled, so it is GTAO's
  occlusion, not the contact shadows.
- `AURORA_BLEND_AO=alt` on the installed 0.8.5
  (`env AURORA_BLEND_AO=alt /usr/bin/aurora-viewer`): the speckle switches off
  and on every second. That switch drops the blended faces from the depth the
  occlusion reads, one second in two.

## Why

Since #889 (2026-10-23, in 0.8.4 and 0.8.5) the occlusion reads a copy of the
depth prepass with the blended faces at least half covered drawn over it
(`fs_depth_contact_normal` in object.wgsl, the "occlusion depth" pass in
renderer.rs), so that opaque-looking blended heads and bodies receive the
occlusion. The coverage test is per texel, so a blended hair texture enters
that depth as scattered strand specks with gaps. GTAO's horizon search then
finds each speck as an occluder of the skin behind it: speckled occlusion,
which moves with the hair and the camera. #1778 (in 0.8.5, not in 0.8.4 nor
the 0.8.5 betas before 2026-11-01) computes GTAO at half resolution on screens
of 2560 × 1440 or more, so each speck covers 2 × 2 pixels: that is likely why
it became visible in 0.8.5. #1702's own-motion path (current weight 0.2, clip
0.6 σ) then throws away the history where those specks change the occlusion,
so it crawls in motion.

## Proposed fix (being prototyped)

Two designs were ruled out before any code:

- **Blended faces receive but do not cast** (horizons searched in the prepass
  depth alone): a blended-but-opaque mesh head is all blended faces, so its
  nose and eye sockets would stop occluding it, the regression #889 fixed.
- **Hair kept out of the occlusion depth** (the faces whose texture has real
  transparency, `classify_alpha` in scene/jobs.rs): G.W. watched the hair in
  `AURORA_BLEND_AO=alt`'s off seconds on 2026-11-02, "it loses a lot of its
  shading and depth".

What is left: hair shades hair, but does not cast on anything else. A second
depth holds the prepass and only the blended faces whose texture is opaque;
a pixel on hair (where the two depths differ) searches its horizons in the
full occlusion depth, every other pixel in the one without hair. Hair keeps
its own depth, normal and motion, so #1702's reprojection of it does not
change. The skin loses the hair's soft shadow, the band the snow was in. To
check against: #889's demo
(`AURORA_DEMO_BLEND_AO=1`), and the maintainer's hem measurement on #1702
(`AURORA_DEMO_PHYSICS=1`, `AURORA_AO=gtao:2`, `AURORA_AUTO_EXPOSURE=0`, the
dress region at frames 1322–1338 against `AURORA_GTAO_TEMPORAL=0`). Check the
Legacy Viewer's source before saying its SSAO ignores blended faces. The
contact shadows read the same depth (the grain beside the hair with them on)
but cast from blended faces on purpose (mesh shoes); that belongs to #988.

## What was ruled out

- **#1702's motion vectors as the cause.** A demo with the camera orbiting a
  hopping avatar showed its own-motion settings add grain (fine flicker 1.99
  against 1.65 with weight 0.1 and clip 1.25 σ), but on the grid the loosened
  values gave no reduction G.W. could see, and the tight values are what fixed
  #1702's hem smear (the maintainer: −15.6 levels to −5.6, accepting "a little
  more grain"). Do not loosen them for this. Two traps on the way: a demo of
  our avatar walking away from a following camera never triggers the
  own-motion path (forcing its weight to 1.0 changed nothing), and a grid A/B
  from two clips in different rooms compared different exposures (the R7 view
  divides by it).
- **Half resolution (#1778) as the cause.** A local build forcing full
  resolution made the specks tiny but crawl harder. It scales the symptom.
- **GTAO quality.** Lower levels only make the specks coarser.
- **TAA.** Still there; Aurora's TAA does not use motion vectors yet, so this
  only rules out a temporal filter without them.
- **Earlier, in demo:** the GPU or driver (NVIDIA and RADV), indirect light and
  specular occlusion. The demo's flat walls stay clean, which is why the walls
  in the original report were never reproduced.
