//! Render — wgpu wrapper for 2D rendering.
//!
//! Fase 1: surface + sprite batcher + kamera + layers.
//! Sprites tegnes som textured quads i batches per layer.

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use winit::window::Window;

use crate::app::AppError;
use crate::assets::{AssetStore, TextureHandle};
use crate::math::{Color, Rect, Vec2};

/// Kamera — 2D view af verden. Follow + clamp til zone-bounds.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub viewport_w: f32,
    pub viewport_h: f32,
    pub bounds: Option<Rect>,
}

impl Camera {
    pub fn new(viewport_w: f32, viewport_h: f32) -> Self {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport_w,
            viewport_h,
            bounds: None,
        }
    }

    /// Follow et target, clamps til zone bounds hvis sat.
    pub fn follow(&mut self, target: Vec2) {
        self.position = target;
        if let Some(b) = self.bounds {
            let half_w = self.viewport_w * 0.5 / self.zoom;
            let half_h = self.viewport_h * 0.5 / self.zoom;
            // Hvis zonen er mindre end viewport, centrér den (undgå min > max i clamp).
            if (b.w as f32) <= self.viewport_w / self.zoom {
                self.position.x = (b.x + b.w / 2) as f32;
            } else {
                self.position.x = self.position.x.clamp(
                    (b.x as f32) + half_w,
                    (b.x + b.w) as f32 - half_w,
                );
            }
            if (b.h as f32) <= self.viewport_h / self.zoom {
                self.position.y = (b.y + b.h / 2) as f32;
            } else {
                self.position.y = self.position.y.clamp(
                    (b.y as f32) + half_h,
                    (b.y + b.h) as f32 - half_h,
                );
            }
        }
    }

    pub fn set_bounds(&mut self, bounds: Rect) {
        self.bounds = Some(bounds);
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.viewport_w = w;
        self.viewport_h = h;
    }

    /// Byg view-projection matrix (screen-space: 0..w, 0..h, y ned).
    pub fn view_proj(&self) -> [[f32; 4]; 4] {
        let zoom = self.zoom;
        let half_w = self.viewport_w * 0.5 / zoom;
        let half_h = self.viewport_h * 0.5 / zoom;
        let left = self.position.x - half_w;
        let right = self.position.x + half_w;
        let top = self.position.y - half_h;
        let bottom = self.position.y + half_h;
        // Orthographic, y ned (ligner skærm): [0,0] = top-left af kamera-view
        // For 2D top-down: world (0,0) top-left, y vokser nedad.
        // Column-major for wgpu.
        [
            [2.0 / (right - left), 0.0, 0.0, 0.0],
            [0.0, 2.0 / (top - bottom), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [
                -(right + left) / (right - left),
                -(top + bottom) / (top - bottom),
                0.0,
                1.0,
            ],
        ]
    }
}

/// Render-layer: lavere tegnes først (baggrund), højere tegnes øverst (UI).
pub const LAYER_GROUND: i32 = 0;
pub const LAYER_DECALS: i32 = 10;
pub const LAYER_ENTITIES: i32 = 20;
pub const LAYER_OVERHEAD: i32 = 30;
pub const LAYER_EFFECTS: i32 = 40;
pub const LAYER_UI: i32 = 50;

/// Vertex for sprite rendering.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub(crate) struct SpriteVertex {
    pos: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

/// Sprite-instans der skal tegnes.
#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub texture: TextureHandle,
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub color: Color,
    pub layer: i32,
}

impl Sprite {
    pub fn new(texture: TextureHandle, position: Vec2, size: Vec2) -> Self {
        Self {
            texture,
            position,
            size,
            rotation: 0.0,
            color: Color::WHITE,
            layer: LAYER_ENTITIES,
        }
    }
}

/// Sprite batcher — samler sprites per frame, sorterer pr. layer+texture,
/// tegner i batches med wgpu.
pub struct SpriteBatch {
    sprites: Vec<Sprite>,
    vertices: Vec<SpriteVertex>,
    indices: Vec<u16>,
}

impl SpriteBatch {
    pub fn new() -> Self {
        Self {
            sprites: Vec::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.sprites.clear();
    }

    pub fn add(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }

    pub fn len(&self) -> usize {
        self.sprites.len()
    }

    /// Render alle queued sprites via givent renderer.
    /// Multi-texture batching: sprites grupperes per texture, én draw per gruppe.
    pub fn render(
        &mut self,
        renderer: &mut Renderer,
        assets: &AssetStore,
        camera: &Camera,
    ) -> Result<(), AppError> {
        if self.sprites.is_empty() {
            renderer.clear_only(camera)?;
            return Ok(());
        }
        // Sorter: layer ascending, så texture (for batching).
        self.sprites.sort_by(|a, b| {
            a.layer
                .cmp(&b.layer)
                .then(a.texture.id().cmp(&b.texture.id()))
        });

        self.vertices.clear();
        self.indices.clear();

        let view_proj = camera.view_proj();
        // Upload view-proj som uniform.
        renderer.queue.write_buffer(
            &renderer.camera_uniform_buf,
            0,
            bytemuck::cast_slice(&[view_proj]),
        );

        // Byg vertices/indices for alle sprites, og track texture-grupper.
        // Hver gruppe: (texture_handle, index_start, index_count).
        let mut groups: Vec<(TextureHandle, u32, u32)> = Vec::new();
        let mut current_tex: Option<TextureHandle> = None;
        let mut group_index_start: u32 = 0;
        let mut group_index_count: u32 = 0;

        for sprite in &self.sprites {
            // Skip sprites med invalid texture — de kan ikke tegnes.
            if assets.get_texture(sprite.texture).is_none() {
                // Hvis vi har en aktiv gruppe, afslut den først.
                if group_index_count > 0 {
                    if let Some(tex) = current_tex {
                        groups.push((tex, group_index_start, group_index_count));
                    }
                    group_index_start += group_index_count;
                    group_index_count = 0;
                }
                current_tex = None;
                continue;
            }

            // Ny texture-gruppe?
            if current_tex != Some(sprite.texture) {
                if group_index_count > 0 {
                    if let Some(tex) = current_tex {
                        groups.push((tex, group_index_start, group_index_count));
                    }
                    group_index_start += group_index_count;
                    group_index_count = 0;
                }
                current_tex = Some(sprite.texture);
            }

            let hx = sprite.size.x * 0.5;
            let hy = sprite.size.y * 0.5;
            let cos = sprite.rotation.cos();
            let sin = sprite.rotation.sin();
            let c = sprite.color.0;

            let corners = [
                (-hx, -hy, 0.0, 0.0),
                (hx, -hy, 1.0, 0.0),
                (hx, hy, 1.0, 1.0),
                (-hx, hy, 0.0, 1.0),
            ];
            let base = self.vertices.len() as u16;
            for (lx, ly, u, v) in corners {
                let wx = sprite.position.x + (lx * cos - ly * sin);
                let wy = sprite.position.y + (lx * sin + ly * cos);
                self.vertices.push(SpriteVertex {
                    pos: [wx, wy],
                    uv: [u, v],
                    color: c,
                });
            }
            self.indices.extend_from_slice(&[
                base,
                base + 1,
                base + 2,
                base,
                base + 2,
                base + 3,
            ]);
            group_index_count += 6;
        }
        // Afslut sidste gruppe.
        if group_index_count > 0 {
            if let Some(tex) = current_tex {
                groups.push((tex, group_index_start, group_index_count));
            }
        }

        if groups.is_empty() || self.indices.is_empty() {
            renderer.clear_only(camera)?;
            return Ok(());
        }

        // Upload vertices og indices.
        let vertex_size = self.vertices.len() * std::mem::size_of::<SpriteVertex>();
        let index_size = self.indices.len() * std::mem::size_of::<u16>();

        // Grow buffers hvis nødvendigt.
        if renderer.vertex_buf_size < vertex_size {
            renderer.vertex_buf_size = vertex_size.next_power_of_two().max(4096);
            renderer.vertex_buf = renderer
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("sprite vertex buf"),
                    size: renderer.vertex_buf_size as u64,
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }
        if renderer.index_buf_size < index_size {
            renderer.index_buf_size = index_size.next_power_of_two().max(4096);
            renderer.index_buf = renderer
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some("sprite index buf"),
                    size: renderer.index_buf_size as u64,
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
        }

        renderer.queue.write_buffer(
            &renderer.vertex_buf,
            0,
            bytemuck::cast_slice(&self.vertices),
        );
        renderer
            .queue
            .write_buffer(&renderer.index_buf, 0, bytemuck::cast_slice(&self.indices));

        renderer.render_sprites_multi(camera, assets, &groups)?;
        Ok(())
    }
}

impl Default for SpriteBatch {
    fn default() -> Self {
        Self::new()
    }
}

/// Renderer ejer wgpu-surface, device, queue, sprite-pipeline og buffers.
pub struct Renderer {
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
    pub(crate) camera_uniform_buf: wgpu::Buffer,
    pub(crate) sprite_pipeline: wgpu::RenderPipeline,
    pub(crate) vertex_buf: wgpu::Buffer,
    pub(crate) index_buf: wgpu::Buffer,
    pub(crate) vertex_buf_size: usize,
    pub(crate) index_buf_size: usize,
    pub(crate) bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, AppError> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance
            .create_surface(window.clone())
            .map_err(|e| AppError::Gpu(e.to_string()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| AppError::Gpu("ingen suitable GPU adapter fundet".into()))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("heat_core device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: wgpu::MemoryHints::Performance,
            }, None)
            .await
            .map_err(|e| AppError::Gpu(e.to_string()))?;

        let size = window.inner_size();
        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let present_mode = caps
            .present_modes
            .iter()
            .copied()
            .find(|m| *m == wgpu::PresentMode::Mailbox)
            .unwrap_or(wgpu::PresentMode::Fifo);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // Camera uniform buffer (4x4 matrix).
        let camera_uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera uniform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Sprite shader (WGSL).
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/sprite.wgsl").into()),
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite bind layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(std::num::NonZero::new(64).unwrap()),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sprite vertex buf"),
            size: 4096,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let index_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sprite index buf"),
            size: 4096,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("sprite pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<SpriteVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: 8,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: 16,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        tracing::info!(
            "Renderer Fase 1: {}x{}, format {:?}, present {:?}",
            surface_config.width,
            surface_config.height,
            format,
            present_mode,
        );

        Ok(Self {
            device,
            queue,
            surface,
            surface_config,
            camera_uniform_buf,
            sprite_pipeline,
            vertex_buf,
            index_buf,
            vertex_buf_size: 4096,
            index_buf_size: 4096,
            bind_group_layout,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }

    /// Surface dimensions i pixels.
    pub fn surface_size(&self) -> (u32, u32) {
        (self.surface_config.width, self.surface_config.height)
    }

    /// Tegn kun clear (ingen sprites).
    pub fn clear_only(&mut self, camera: &Camera) -> Result<(), AppError> {
        let view_proj = camera.view_proj();
        self.queue
            .write_buffer(&self.camera_uniform_buf, 0, bytemuck::cast_slice(&[view_proj]));
        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| AppError::Render(e.to_string()))?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("clear encoder"),
            });
        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("clear pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    /// Tegn sprites med multi-texture batching.
    /// `groups`: liste af (texture_handle, index_start, index_count).
    pub(crate) fn render_sprites_multi(
        &mut self,
        camera: &Camera,
        assets: &AssetStore,
        groups: &[(TextureHandle, u32, u32)],
    ) -> Result<(), AppError> {
        let view_proj = camera.view_proj();
        self.queue
            .write_buffer(&self.camera_uniform_buf, 0, bytemuck::cast_slice(&[view_proj]));

        let output = self
            .surface
            .get_current_texture()
            .map_err(|e| AppError::Render(e.to_string()))?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("sprite render encoder"),
            });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("sprite pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.sprite_pipeline);
            pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
            pass.set_index_buffer(
                self.index_buf.slice(..),
                wgpu::IndexFormat::Uint16,
            );

            // Ét draw_indexed per texture-gruppe.
            for (tex_handle, index_start, index_count) in groups {
                if *index_count == 0 {
                    continue;
                }
                if let Some(tex) = assets.get_texture(*tex_handle) {
                    let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: Some("sprite bind group"),
                        layout: &self.bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: self.camera_uniform_buf.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(&tex.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::Sampler(&tex.sampler),
                            },
                        ],
                    });
                    pass.set_bind_group(0, &bind_group, &[]);
                    pass.draw_indexed(*index_start..(*index_start + *index_count), 0, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }

    /// Raw device/queue access (til asset uploads).
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}