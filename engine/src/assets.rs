//! Assets — async asset loading for textures og data.
//!
//! Fase 1: PNG textures loades med `image` crate, uploades til wgpu Textures.
//! RON data parses med `ron` + `serde`. Handles er typeløse refs.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use image::ImageReader;
use serde::de::DeserializeOwned;

use crate::app::AppError;

/// Typeløs handle til en loaded asset. Index ind i en registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Handle {
    pub id: u32,
}

impl Handle {
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Opret en null/invalid handle (brugt som fallback).
    pub fn null() -> Self {
        Self { id: 0 }
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}
/// Handle til en GPU texture specifikt.
pub type TextureHandle = Handle;

/// GPU texture bundle — texture + view + sampler.
pub struct GpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub width: u32,
    pub height: u32,
}

/// Asset store ejer loadede textures og data.
pub struct AssetStore {
    textures: HashMap<u32, GpuTexture>,
    texture_paths: HashMap<PathBuf, TextureHandle>,
    next_handle: u32,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
}

impl AssetStore {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            texture_paths: HashMap::new(),
            next_handle: 1,
            device: None,
            queue: None,
        }
    }

    /// Bind GPU device/queue — kræves før textures kan uploades.
    pub fn bind_gpu(&mut self, device: wgpu::Device, queue: wgpu::Queue) {
        self.device = Some(device);
        self.queue = Some(queue);
    }

    pub fn is_gpu_bound(&self) -> bool {
        self.device.is_some()
    }

    /// Load en PNG fra sti. Returnerer handle. Caches pr. sti.
    pub fn load_texture(&mut self, path: &Path) -> Result<TextureHandle, AppError> {
        if let Some(&h) = self.texture_paths.get(path) {
            return Ok(h);
        }
        let device = self
            .device
            .take()
            .ok_or_else(|| AppError::Gpu("AssetStore: GPU ikke bound".into()))?;
        let queue = self
            .queue
            .take()
            .ok_or_else(|| AppError::Gpu("AssetStore: GPU ikke bound".into()))?;
        // restore
        self.device = Some(device.clone());
        self.queue = Some(queue.clone());

        let img = ImageReader::open(path)
            .map_err(|e| AppError::Window(format!("Asset load fejl {}: {e}", path.display())))?
            .decode()
            .map_err(|e| AppError::Window(format!("Asset decode {}: {e}", path.display())))?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("asset {}", path.display())),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("asset sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: 0.0,
            lod_max_clamp: 1.0,
            ..Default::default()
        });

        let handle_id = self.next_handle;
        self.next_handle += 1;
        let handle = Handle { id: handle_id };
        self.textures.insert(
            handle_id,
            GpuTexture {
                texture,
                view,
                sampler,
                width,
                height,
            },
        );
        self.texture_paths.insert(path.to_path_buf(), handle);
        tracing::debug!("Asset loadet: {} ({}x{})", path.display(), width, height);
        Ok(handle)
    }

    pub fn get_texture(&self, handle: TextureHandle) -> Option<&GpuTexture> {
        self.textures.get(&handle.id)
    }

    /// Hent texture handle via sti (hvis allerede loadet).
    pub fn get_texture_by_path(&self, path: &Path) -> Option<&TextureHandle> {
        self.texture_paths.get(path)
    }

    /// Load og parse en RON-fil ind i en typed struct.
    pub fn load_data<T: DeserializeOwned>(&self, path: &Path) -> Result<T, AppError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::Window(format!("Data load {}: {e}", path.display())))?;
        ron::from_str(&content)
            .map_err(|e| AppError::Window(format!("RON parse {}: {e}", path.display())))
    }
}

impl Default for AssetStore {
    fn default() -> Self {
        Self::new()
    }
}