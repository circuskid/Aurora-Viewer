//! NVIDIA DLSS super resolution: a local experiment, never distributed (the
//! DLSS SDK licence does not allow a GPLv3 viewer to ship it).
//!
//! Built with the cargo feature `dlss` (env `DLSS_SDK` = the SDK folder, at
//! build and run time) and turned on with `AURORA_DLSS=dlaa|quality|balanced|
//! performance|ultra|auto`. It takes the place of the reduced-resolution
//! upscale: the world is drawn at DLSS's render size, jittered, then DLSS
//! rebuilds it at the window's size from the composed image, the scene depth
//! and camera-only motion vectors (dlss_motion.wgsl). Without the feature,
//! [`Dlss::new`] gives nothing and the renderer runs as before.

use glam::Mat4;

/// The cameras of the frame, for the motion vectors.
#[cfg_attr(not(feature = "dlss"), allow(dead_code))]
pub(crate) struct Cameras {
    /// this frame's jittered camera
    pub view_proj: Mat4,
    /// this frame's and last frame's cameras, without the jitter
    pub unjittered: Mat4,
    pub prev: Mat4,
}

#[cfg(feature = "dlss")]
mod imp {
    use super::Cameras;
    use dlss_wgpu::super_resolution::{DlssSuperResolution, DlssSuperResolutionExposure, DlssSuperResolutionRenderParameters};
    use dlss_wgpu::{DlssFeatureFlags, DlssPerfQualityMode, DlssSdk, FeatureSupport};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    /// NGX application id of this local build (any fixed UUID).
    const PROJECT_ID: uuid::Uuid = uuid::uuid!("5b0c4a8e-6f1d-4d7a-9a52-3e1f0c2d7b91");
    const MOTION_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rg16Float;
    const OUTPUT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

    /// The device was opened with the NGX extensions and DLSS can run.
    static SUPPORTED: AtomicBool = AtomicBool::new(false);

    fn wanted() -> Option<DlssPerfQualityMode> {
        static MODE: std::sync::OnceLock<Option<DlssPerfQualityMode>> = std::sync::OnceLock::new();
        *MODE.get_or_init(|| {
            let v = std::env::var("AURORA_DLSS").ok()?;
            let mode = match v.trim().to_ascii_lowercase().as_str() {
                "dlaa" => DlssPerfQualityMode::Dlaa,
                "quality" | "1" => DlssPerfQualityMode::Quality,
                "balanced" => DlssPerfQualityMode::Balanced,
                "performance" => DlssPerfQualityMode::Performance,
                "ultra" => DlssPerfQualityMode::UltraPerformance,
                "auto" => DlssPerfQualityMode::Auto,
                "" | "0" | "off" => return None,
                other => {
                    log::warn!("DLSS: unknown AURORA_DLSS mode {other:?}, off");
                    return None;
                }
            };
            Some(mode)
        })
    }

    /// The jitter's sign given to DLSS; `AURORA_DLSS_JITTER=x|y|xy|none`
    /// flips the axes named, to debug it (static scenes must resolve sharp).
    fn sign() -> [f32; 2] {
        static SIGN: std::sync::OnceLock<[f32; 2]> = std::sync::OnceLock::new();
        *SIGN.get_or_init(|| {
            let flip = std::env::var("AURORA_DLSS_JITTER").unwrap_or_default();
            [
                if flip.contains('x') { 1.0 } else { -1.0 },
                if flip.contains('y') { 1.0 } else { -1.0 },
            ]
        })
    }

    /// A Vulkan instance with the NGX extensions, when DLSS is wanted.
    pub(crate) fn instance(desc: &wgpu::InstanceDescriptor) -> Option<wgpu::Instance> {
        wanted()?;
        let mut support = FeatureSupport::default();
        match dlss_wgpu::create_instance(PROJECT_ID, desc, &mut support) {
            Ok(instance) if support.super_resolution_supported => Some(instance),
            Ok(_) => {
                log::warn!("DLSS: super resolution unsupported by this system (instance)");
                None
            }
            Err(e) => {
                log::warn!("DLSS: instance with the NGX extensions failed: {e}");
                None
            }
        }
    }

    /// The device opened with the NGX extensions, when [`instance`] made the
    /// instance; `None` to open it the usual way.
    pub(crate) fn request_device(
        adapter: &wgpu::Adapter,
        desc: &wgpu::DeviceDescriptor,
        limits: wgpu::Limits,
    ) -> Option<Result<(wgpu::Device, wgpu::Queue), String>> {
        wanted()?;
        let mut support = FeatureSupport::default();
        let opened = dlss_wgpu::request_device(PROJECT_ID, adapter, desc, &mut support, Some(limits));
        Some(match opened {
            Ok(pair) => {
                SUPPORTED.store(support.super_resolution_supported, Ordering::Relaxed);
                if !support.super_resolution_supported {
                    log::warn!("DLSS: super resolution unsupported by this GPU or driver");
                }
                Ok(pair)
            }
            Err(e) => Err(format!("DLSS device: {e}")),
        })
    }

    pub(crate) struct Dlss {
        sdk: Arc<Mutex<DlssSdk>>,
        mode: DlssPerfQualityMode,
        adapter: wgpu::Adapter,
        context: Option<DlssSuperResolution>,
        window: (u32, u32),
        render: (u32, u32),
        output: Option<(wgpu::Texture, wgpu::TextureView, wgpu::BindGroup)>,
        motion: Option<(wgpu::Texture, wgpu::TextureView)>,
        motion_pipeline: wgpu::RenderPipeline,
        motion_uniforms: wgpu::Buffer,
        blit: wgpu::RenderPipeline,
        frame: u32,
        reset: bool,
    }

    impl Dlss {
        pub fn new(device: &wgpu::Device, adapter: &wgpu::Adapter, surface_format: wgpu::TextureFormat) -> Option<Self> {
            let mode = wanted()?;
            if !SUPPORTED.load(Ordering::Relaxed) {
                return None;
            }
            let sdk = match DlssSdk::new(PROJECT_ID, device.clone()) {
                Ok(sdk) => sdk,
                Err(e) => {
                    log::warn!("DLSS: NGX did not start: {e}");
                    return None;
                }
            };
            let motion_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("DLSS motion vectors"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/dlss_motion.wgsl").into()),
            });
            let motion_pipeline = fullscreen(device, "DLSS motion vectors", &motion_shader, MOTION_FORMAT);
            let blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("DLSS output"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shaders/rlva_resolution.wgsl").into()),
            });
            let blit = fullscreen(device, "DLSS output", &blit_shader, surface_format);
            let motion_uniforms = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("DLSS motion vectors"),
                size: 208,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            log::info!("DLSS: on, mode {mode:?}");
            Some(Self {
                sdk,
                mode,
                adapter: adapter.clone(),
                context: None,
                window: (0, 0),
                render: (0, 0),
                output: None,
                motion: None,
                motion_pipeline,
                motion_uniforms,
                blit,
                frame: 0,
                reset: true,
            })
        }

        /// The size the world is drawn at for this window; the context and
        /// its images are made again when the window changes. `None`: DLSS
        /// failed and must be dropped.
        pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, window: (u32, u32)) -> Option<(u32, u32)> {
            if self.context.is_some() && self.window == window {
                return Some(self.render);
            }
            // (its Drop waits for the GPU: the old one goes first)
            self.context = None;
            let context = match DlssSuperResolution::new(
                [window.0, window.1],
                self.mode,
                DlssFeatureFlags::InvertedDepth | DlssFeatureFlags::LowResolutionMotionVectors,
                Arc::clone(&self.sdk),
                device,
                queue,
            ) {
                Ok(context) => context,
                Err(e) => {
                    log::warn!("DLSS: context for {}x{} failed: {e}", window.0, window.1);
                    return None;
                }
            };
            // dlss_wgpu's render_resolution() is the bottom of the dynamic
            // range (half the window even in Quality): the mode's own scale,
            // the "optimal" size NGX created the feature with, kept in range
            let scale = match self.mode {
                DlssPerfQualityMode::Dlaa => 1.0,
                DlssPerfQualityMode::Quality => 1.0 / 1.5,
                DlssPerfQualityMode::Balanced => 1.0 / 1.724,
                DlssPerfQualityMode::UltraPerformance => 1.0 / 3.0,
                _ => 0.5,
            };
            let range = context.render_resolution_range();
            let fit = |i: usize, w: u32| ((w as f32 * scale).round() as u32).clamp(range.start()[i], range.end()[i]).max(1);
            self.render = (fit(0, window.0), fit(1, window.1));
            self.window = window;
            self.context = Some(context);
            let (tex, view) = crate::renderer::make_view(
                device,
                "DLSS output",
                window.0,
                window.1,
                OUTPUT_FORMAT,
                1,
                wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            );
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("DLSS output"),
                layout: &self.blit.get_bind_group_layout(0),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                }],
            });
            self.output = Some((tex, view, group));
            self.motion = Some(crate::renderer::make_view(
                device,
                "DLSS motion vectors",
                self.render.0,
                self.render.1,
                MOTION_FORMAT,
                1,
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            ));
            self.reset = true;
            log::info!("DLSS: {}x{} drawn, {}x{} shown", self.render.0, self.render.1, window.0, window.1);
            Some(self.render)
        }

        /// This frame's jitter, in render pixels, in [-0.5, 0.5] (x right,
        /// y down, as DLSS reads it).
        pub fn jitter(&self) -> [f32; 2] {
            self.context
                .as_ref()
                .map_or([0.0; 2], |c| c.suggested_jitter(self.frame, [self.render.0, self.render.1]))
        }

        /// Motion vectors, DLSS and its output into `target`. DLSS's own
        /// command buffer runs between what `encoder` holds so far and what
        /// comes after: `encoder` is replaced by a fresh one, and the two
        /// returned buffers are submitted, in order, between the frame's
        /// encoder and that new one.
        #[allow(clippy::too_many_arguments)]
        pub fn encode(
            &mut self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            encoder: &mut wgpu::CommandEncoder,
            color: &wgpu::TextureView,
            depth: &wgpu::TextureView,
            cams: &Cameras,
            target: &wgpu::TextureView,
        ) -> Option<[wgpu::CommandBuffer; 2]> {
            let (Some(context), Some((_, out_view, out_group)), Some((_, mv_view))) =
                (self.context.as_mut(), self.output.as_ref(), self.motion.as_ref())
            else {
                return None;
            };
            let mut u = [0f32; 52];
            u[0..16].copy_from_slice(&cams.view_proj.inverse().to_cols_array());
            u[16..32].copy_from_slice(&cams.unjittered.to_cols_array());
            u[32..48].copy_from_slice(&cams.prev.to_cols_array());
            u[48] = self.render.0 as f32;
            u[49] = self.render.1 as f32;
            queue.write_buffer(&self.motion_uniforms, 0, bytemuck::cast_slice(&u));
            let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("DLSS motion vectors"),
                layout: &self.motion_pipeline.get_bind_group_layout(0),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(depth),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.motion_uniforms.as_entire_binding(),
                    },
                ],
            });
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("DLSS motion vectors"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: mv_view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(&self.motion_pipeline);
                pass.set_bind_group(0, &group, &[]);
                pass.draw(0..3, 0..1);
            }
            let jitter = context.suggested_jitter(self.frame, [self.render.0, self.render.1]);
            let params = DlssSuperResolutionRenderParameters {
                color,
                depth,
                motion_vectors: mv_view,
                exposure: DlssSuperResolutionExposure::Automatic,
                bias: None,
                dlss_output: out_view,
                reset: self.reset,
                // the projection moved the image by +jitter (x right, y
                // down); DLSS's offset is the opposite (as the Unreal
                // integration: M[2][0] -= 2 Jx / w, M[2][1] += 2 Jy / h)
                jitter_offset: [jitter[0] * sign()[0], jitter[1] * sign()[1]],
                partial_texture_size: Some([self.render.0, self.render.1]),
                motion_vector_scale: None,
            };
            let dlss = match context.render(params, encoder, &self.adapter) {
                Ok(cb) => cb,
                Err(e) => {
                    log::warn!("DLSS: evaluate failed: {e}");
                    return None;
                }
            };
            self.reset = false;
            self.frame = self.frame.wrapping_add(1);
            let first = std::mem::replace(
                encoder,
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("frame post, after DLSS"),
                }),
            );
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("DLSS output"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: target,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(&self.blit);
                pass.set_bind_group(0, out_group, &[]);
                pass.draw(0..3, 0..1);
            }
            Some([first.finish(), dlss])
        }
    }

    fn fullscreen(device: &wgpu::Device, label: &str, shader: &wgpu::ShaderModule, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: None,
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: None,
            cache: None,
        })
    }
}

/// Without the `dlss` feature: never on.
#[cfg(not(feature = "dlss"))]
mod imp {
    use super::Cameras;

    pub(crate) fn instance(_: &wgpu::InstanceDescriptor) -> Option<wgpu::Instance> {
        None
    }

    pub(crate) fn request_device(
        _: &wgpu::Adapter,
        _: &wgpu::DeviceDescriptor,
        _: wgpu::Limits,
    ) -> Option<Result<(wgpu::Device, wgpu::Queue), String>> {
        None
    }

    pub(crate) enum Dlss {}

    impl Dlss {
        pub fn new(_: &wgpu::Device, _: &wgpu::Adapter, _: wgpu::TextureFormat) -> Option<Self> {
            None
        }

        pub fn prepare(&mut self, _: &wgpu::Device, _: &wgpu::Queue, _: (u32, u32)) -> Option<(u32, u32)> {
            match *self {}
        }

        pub fn jitter(&self) -> [f32; 2] {
            match *self {}
        }

        #[allow(clippy::too_many_arguments)]
        pub fn encode(
            &mut self,
            _: &wgpu::Device,
            _: &wgpu::Queue,
            _: &mut wgpu::CommandEncoder,
            _: &wgpu::TextureView,
            _: &wgpu::TextureView,
            _: &Cameras,
            _: &wgpu::TextureView,
        ) -> Option<[wgpu::CommandBuffer; 2]> {
            match *self {}
        }
    }
}

pub(crate) use imp::{Dlss, instance, request_device};
