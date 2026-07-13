//! Zone — zone-definition format for Heat City.
//!
//! En zone er et område af byen (East Blocks, Downtown etc.).
//! Fase 2: simpel zone med tilemap, bounds og spawn-punkter.
//! Fase 5+: faction ownership, influence, police intensity etc.

use serde::{Deserialize, Serialize};

use heat_core::Rect;

/// Zone-definition (fra RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub bounds: ZoneBounds,
    pub tilemap_path: String,
    /// Spawn-punkter for NPC'er (world coordinates).
    pub npc_spawns: Vec<NpcSpawn>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneBounds {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl ZoneBounds {
    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.w, self.h)
    }
}

/// NPC spawn-punkt i en zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSpawn {
    pub x: f32,
    pub y: f32,
    /// NPC-type: "pedestrian", "shopkeeper", "gang_member" etc.
    pub npc_type: String,
    /// Patrol-rute (waypoints). Tom = stationær.
    pub patrol: Vec<(f32, f32)>,
}