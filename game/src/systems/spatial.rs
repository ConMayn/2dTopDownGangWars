//! Spatial grid — grid-baseret spatial partitioning.
//!
//! Deler verdenen op i celler. Bruges til at finde entiteter i nærheden
//! uden at iterere over alle. O(n) → O(1) for "hvem er i nærheden" queries.
//!
//! Fase 4: statisk grid der rebuildes per frame (simpelt).
//! Fase 5+: dynamisk opdatering ved entity movement.

use std::collections::HashMap;

use heat_core::Vec2;

/// Cell-key i grid (integer).
type CellKey = (i32, i32);

/// Spatial grid — map fra celle → entiteter i den celle.
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<CellKey, Vec<hecs::Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    /// Ryd grid (kaldes hver frame før rebuild).
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Indsæt en entitet i grid baseret på dens position.
    pub fn insert(&mut self, entity: hecs::Entity, pos: Vec2) {
        let key = self.cell_key(pos);
        self.cells.entry(key).or_default().push(entity);
    }

    /// Hent alle entiteter inden for radius af en position.
    /// Returnerer en deduplikeret liste af entiteter.
    pub fn query_radius(&self, pos: Vec2, radius: f32) -> Vec<hecs::Entity> {
        let cell_radius = (radius / self.cell_size).ceil() as i32 + 1;
        let (cx, cy) = self.cell_key(pos);
        let mut result = Vec::new();
        for dy in -cell_radius..=cell_radius {
            for dx in -cell_radius..=cell_radius {
                if let Some(entities) = self.cells.get(&(cx + dx, cy + dy)) {
                    result.extend_from_slice(entities);
                }
            }
        }
        result
    }

    fn cell_key(&self, pos: Vec2) -> CellKey {
        let x = (pos.x / self.cell_size).floor() as i32;
        let y = (pos.y / self.cell_size).floor() as i32;
        (x, y)
    }
}