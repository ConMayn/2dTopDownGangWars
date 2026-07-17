#![allow(dead_code)] // width/height helpers er public API.

//! Tilemap — grid-baseret map af tiles.
//!
//! Et Tilemap er et 2D grid af tile-ID'er. Hver celle refererer til en tile-type
//! i TileRegistry. Tilemap kan renders som batched sprites og bruges til collision.

use serde::{Deserialize, Serialize};

use heat_core::{Color, RenderContext, Sprite, Vec2};

use super::tiles::{TileId, TileRegistry};

/// Tilemap-data (fra RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilemapDef {
    pub width: usize,
    pub height: usize,
    /// Tile-ID'er i row-major order (y * width + x).
    pub tiles: Vec<TileId>,
}

/// Runtime tilemap: data + dimensioner.
#[derive(Debug, Clone)]
pub struct Tilemap {
    pub def: TilemapDef,
    /// Tile pixel størrelse (kvadratisk).
    pub tile_size: f32,
}

impl Tilemap {
    pub fn new(def: TilemapDef, tile_size: f32) -> Self {
        Self { def, tile_size }
    }

    pub fn width(&self) -> usize {
        self.def.width
    }

    pub fn height(&self) -> usize {
        self.def.height
    }

    pub fn pixel_width(&self) -> f32 {
        self.def.width as f32 * self.tile_size
    }

    pub fn pixel_height(&self) -> f32 {
        self.def.height as f32 * self.tile_size
    }

    /// Hent tile-ID ved (x, y). Returnerer None hvis out of bounds.
    pub fn tile_at(&self, x: usize, y: usize) -> Option<&TileId> {
        if x < self.def.width && y < self.def.height {
            self.def.tiles.get(y * self.def.width + x)
        } else {
            None
        }
    }

    /// Er tile ved (x, y) solid? Out-of-bounds = solid (verden er indelukket).
    pub fn is_solid(&self, x: i32, y: i32, registry: &TileRegistry) -> bool {
        if x < 0 || y < 0 || x as usize >= self.def.width || y as usize >= self.def.height {
            return true; // out of bounds = solid
        }
        let idx = y as usize * self.def.width + x as usize;
        match self.def.tiles.get(idx) {
            Some(id) => registry.is_solid(id),
            None => true,
        }
    }

    /// Render tilemap som sprites. Kun synlige tiles (inden for kamera-view).
    pub fn render(&self, ctx: &mut RenderContext, registry: &TileRegistry) {
        let ts = self.tile_size;
        let half_ts = ts * 0.5;

        // Beregn synlige tile-range baseret på kamera.
        let cam = ctx.camera;
        let view_left = cam.position.x - cam.viewport_w * 0.5 / cam.zoom;
        let view_right = cam.position.x + cam.viewport_w * 0.5 / cam.zoom;
        let view_top = cam.position.y - cam.viewport_h * 0.5 / cam.zoom;
        let view_bottom = cam.position.y + cam.viewport_h * 0.5 / cam.zoom;

        let x_start = ((view_left / ts).floor() as i32).max(0) as usize;
        let x_end = (((view_right / ts).ceil() as i32) + 1).min(self.def.width as i32) as usize;
        let y_start = ((view_top / ts).floor() as i32).max(0) as usize;
        let y_end = (((view_bottom / ts).ceil() as i32) + 1).min(self.def.height as i32) as usize;

        for y in y_start..y_end {
            for x in x_start..x_end {
                let Some(tile_id) = self.tile_at(x, y) else { continue };
                let Some(tile_type) = registry.get(tile_id) else { continue };
                let px = x as f32 * ts + half_ts;
                let py = y as f32 * ts + half_ts;
                let color = Color::rgba(
                    tile_type.def.color[0],
                    tile_type.def.color[1],
                    tile_type.def.color[2],
                    tile_type.def.color[3],
                );
                if let Some(tex) = tile_type.texture {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: Vec2::new(px, py),
                        size: Vec2::new(ts, ts),
                        rotation: 0.0,
                        color,
                        layer: tile_type.def.layer,
                    });
                } else {
                    // Fallback: tegn som farvet quad (brug en null texture — Fase 1 renderer
                    // håndterer dette ved at skippe texture binding).
                    ctx.batch.add(Sprite {
                        texture: heat_core::TextureHandle::null(),
                        position: Vec2::new(px, py),
                        size: Vec2::new(ts, ts),
                        rotation: 0.0,
                        color,
                        layer: tile_type.def.layer,
                    });
                }
            }
        }
    }
}