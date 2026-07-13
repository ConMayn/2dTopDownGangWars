//! Tile definitions — data-drevet tile-typer for Heat City.
//!
//! Hver tile-type har en texture, om den er solid (blokkerer bevægelse),
//! og et visuelt lag. Tile-typer defineres i RON (`assets/data/tiles.ron`).

use serde::{Deserialize, Serialize};

use heat_core::TextureHandle;

/// Unik tile-type ID (string key i tile-registry).
pub type TileId = String;

/// En tile-type definition (fra RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileDef {
    pub id: TileId,
    pub solid: bool,
    pub layer: i32,
    /// Farve tint hvis ingen texture (fallback rendering).
    pub color: [f32; 4],
}

/// Runtime tile-type: definition + loaded texture handle.
#[derive(Debug, Clone)]
pub struct TileType {
    pub def: TileDef,
    pub texture: Option<TextureHandle>,
}

/// Registry af tile-typer. Indlæses fra RON ved init.
#[derive(Debug, Clone, Default)]
pub struct TileRegistry {
    tiles: std::collections::HashMap<TileId, TileType>,
}

impl TileRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, id: TileId, tile: TileType) {
        self.tiles.insert(id, tile);
    }

    pub fn get(&self, id: &str) -> Option<&TileType> {
        self.tiles.get(id)
    }

    pub fn is_solid(&self, id: &str) -> bool {
        self.tiles.get(id).map(|t| t.def.solid).unwrap_or(false)
    }
}