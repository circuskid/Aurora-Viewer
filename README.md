# Iris Xe start-up crash: `water effects` panic after a GPU device loss

[#1892](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1892) (also [#1900](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1900), [#1903](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1903) and [#1907](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1907)): Intel Iris Xe on Windows crashes before the login screen in 0.8.4 and 0.8.5, while 0.8.3 runs. Reproduced on Linux on an AMD APU by injecting the failure.

## Cause (from the code)

1. **0.8.4 raised the texture page count on Intel.** Up to 0.8.3 it was `min(binding-array limit, max_sampled_textures_per_shader_stage − 17)`. Intel's Windows driver reports a small ordinary sampled-texture limit, which kept Intel at a few hundred pages. [#1622](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1622) dropped that term (`texture_page_limit`, `crates/aurora-render/src/main_thread.rs`), so Intel now gets the 16384 cap like NVIDIA and AMD. Those two were already at 16384, so the PR saw no change on them.
2. **wgpu-hal 30 sizes a descriptor pool for at least 64 sets of a layout** (`POOL_MIN_SETS`, `wgpu-hal/src/vulkan/descriptor.rs`) and never checks the pool against `maxUpdateAfterBindDescriptorsInAllPools`. For the `textures` layout (`TextureTable::new`, the only binding-array layout) that is 64 × (16384 pages + sampler + buffer) = **1,048,704** update-after-bind descriptors in one pool. A common value for that limit is 1,048,576.
3. **wgpu-core treats an out-of-memory error from a layout or bind group as fatal** (`Device::handle_hal_error`) and loses the device with the message "Out of memory". This matches the reports: `GPU device lost (Unknown): Out of memory` with only 195 MB of VRAM in use.
4. **After the loss, the first `create_buffer_init` panics** because its mapped-at-creation buffer is invalid. That happens to be `water effects` (`WaterGpu::new`). The water code is not the cause, and it is also why the potato preset does not help.

## Reproduction

The 610M's driver (RADV) allows 67,108,863 update-after-bind descriptors, so the crash does not happen there on its own. We wrote a small Vulkan layer that refuses update-after-bind descriptor pools above a total (default 1,048,576) with `VK_ERROR_OUT_OF_DEVICE_MEMORY`, and can also report that total as the limit. With the v0.8.5 release commit (`e6aa3eab`), demo mode, `AURORA_GPU=radv`:

| Run | Result |
|---|---|
| v0.8.5, no layer | starts, 16384 pages |
| v0.8.5 + layer at 1,048,576 | layer refuses `maxSets=64 descriptors=1048704`, then `GPU device lost (Unknown): Out of memory`, `wgpu error: Out of Memory`, and the panic `Buffer with 'water effects' label is invalid` at `wgpu-30.0.1/src/util/device.rs:72:18`: the same sequence as the reports |
| test cap, no layer | 16384 pages, unchanged |
| test cap + layer at 1,048,576 | 16382 pages, starts and renders |
| test cap + layer at 262,144 | 4094 pages, starts and renders |

The test cap limits pages to `maxUpdateAfterBindDescriptorsInAllPools / 64 − 2`, read through ash the way `detect_vram_mb` reads Vulkan properties. On NVIDIA and RADV it changes nothing.

Layer source and manifest: [`layer/`](layer/). Test patch, against `e6aa3eab`: [`page-cap-test.patch`](page-cap-test.patch).

```bash
cd layer
cc -shared -fPIC -O2 -fvisibility=hidden -o libVkLayer_poolcap.so poolcap.c -lpthread
VK_LAYER_PATH=$PWD VK_INSTANCE_LAYERS=VK_LAYER_TEST_poolcap POOLCAP_REPORT=1 AURORA_DEMO=1 aurora-viewer
# POOLCAP_LIMIT=<n> tries another total
```

## Not confirmed

That the Intel driver reports 1,048,576 for `maxUpdateAfterBindDescriptorsInAllPools` and refuses the pool there. The layer injects exactly that, so it proves the viewer's side of the chain, not Intel's. A shader-compiler out-of-memory during pipeline creation (0.8.3 already stalled 13 s in `igc64` on the same machine) would end the same way. The reporter's `vulkaninfo` values would settle it: `maxUpdateAfterBindDescriptorsInAllPools`, `maxPerStageDescriptorSampledImages` and `maxPerStageDescriptorUpdateAfterBindSampledImages`. If the limit turns out large, a fixed page ceiling (for example 4096 on Intel) is the fallback.

Separately, a device lost during start-up still ends in this panic instead of the safe-graphics offer: [#1660](https://github.com/Aurora-Viewer/Aurora-Viewer/issues/1660) covers losses during a session only.
