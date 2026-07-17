#![allow(dead_code)] // hit_x/hit_y er public API til fremtidig reaktions-logik.

//! Collision — AABB vs solid tiles.
//!
//! Bevægelse-resolve: flyt først på X-aksen, check collision, resolve.
//! Så Y-aksen. Dette giver "slide along walls" adfærd.

use heat_core::{Aabb, Vec2};

use super::tilemap::Tilemap;
use super::tiles::TileRegistry;

/// Resultat af et move-and-collide.
#[derive(Debug, Clone, Copy)]
pub struct MoveResult {
    pub new_pos: Vec2,
    pub hit_x: bool,
    pub hit_y: bool,
}

/// Flyt en AABB med (dx, dy) mod tilemap, resolve collision.
/// Returnerer ny position og om der var collision på hver akse.
pub fn move_and_collide(
    pos: Vec2,
    half_extents: Vec2,
    delta: Vec2,
    tilemap: &Tilemap,
    registry: &TileRegistry,
) -> MoveResult {
    let ts = tilemap.tile_size;

    // X-akse
    let mut new_x = pos.x + delta.x;
    let box_x = Aabb::from_center(Vec2::new(new_x, pos.y), half_extents);
    let hit_x = check_aabb_vs_tiles(&box_x, tilemap, registry, ts);
    if hit_x {
        // Resolve: ryk tilbage til kanten af den tile der blokkerer.
        if delta.x > 0.0 {
            // Bevæger højre: find venstre kant af den blokkerende tile.
            let tile_left = (box_x.max.x / ts).floor() as i32;
            new_x = tile_left as f32 * ts - half_extents.x - 0.01;
        } else if delta.x < 0.0 {
            // Bevæger venstre: find højre kant af den blokkerende tile.
            let tile_right = (box_x.min.x / ts).floor() as i32 + 1;
            new_x = tile_right as f32 * ts + half_extents.x + 0.01;
        }
    }

    // Y-akse (brug ny X)
    let mut new_y = pos.y + delta.y;
    let box_y = Aabb::from_center(Vec2::new(new_x, new_y), half_extents);
    let hit_y = check_aabb_vs_tiles(&box_y, tilemap, registry, ts);
    if hit_y {
        if delta.y > 0.0 {
            // Bevæger ned: find top-kanten af blokkerende tile.
            let tile_top = (box_y.max.y / ts).floor() as i32;
            new_y = tile_top as f32 * ts - half_extents.y - 0.01;
        } else if delta.y < 0.0 {
            // Bevæger op: find bund-kanten af blokkerende tile.
            let tile_bottom = (box_y.min.y / ts).floor() as i32 + 1;
            new_y = tile_bottom as f32 * ts + half_extents.y + 0.01;
        }
    }

    MoveResult {
        new_pos: Vec2::new(new_x, new_y),
        hit_x,
        hit_y,
    }
}

/// Check om en AABB overlapper med nogen solid tile.
fn check_aabb_vs_tiles(
    box_: &Aabb,
    tilemap: &Tilemap,
    registry: &TileRegistry,
    ts: f32,
) -> bool {
    let min_tx = (box_.min.x / ts).floor() as i32;
    let max_tx = (box_.max.x / ts).floor() as i32;
    let min_ty = (box_.min.y / ts).floor() as i32;
    let max_ty = (box_.max.y / ts).floor() as i32;

    for ty in min_ty..=max_ty {
        for tx in min_tx..=max_tx {
            if tilemap.is_solid(tx, ty, registry) {
                // Tjek reel overlap (ikke kun grid-cell — præcis AABB).
                let tile_min = Vec2::new(tx as f32 * ts, ty as f32 * ts);
                let tile_max = Vec2::new((tx + 1) as f32 * ts, (ty + 1) as f32 * ts);
                let tile_aabb = Aabb::new(tile_min, tile_max);
                if box_.intersects(&tile_aabb) {
                    return true;
                }
            }
        }
    }
    false
}