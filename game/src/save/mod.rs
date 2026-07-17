#![allow(dead_code)] // Save API er public/stub til fremtidig migration.

//! Save — versions-støttet save-system.
//!
//! Fra TDD afsnit 12:
//! - 5 save slots.
//! - Bincode (binært), versions-støttet.
//! - Gemmes: spiller, factions, zones, missions, crew, safehouses, world time, news, evidence.
//! - Gemmes KUN i safehouses (design-regel).
//! - IKKE gemmes: NPC-positioner (respawner), midlertidige effekter.

use serde::{Deserialize, Serialize};

/// Save-version — bruges til migration.
pub const SAVE_VERSION: u32 = 1;

/// Player-state der gemmes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSaveState {
    pub pos: [f32; 2],
    pub cash: u32,
    pub clean: u32,
    pub armed: bool,
    /// Inventory items (id, count).
    pub inventory: Vec<(String, u32)>,
}

/// Complete save-state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveState {
    pub version: u32,
    pub slot: u32,
    pub timestamp: f32,
    pub player: PlayerSaveState,
    /// Faction trust (faction_id → trust).
    pub faction_trust: Vec<(String, f32)>,
    /// Street rep.
    pub street_rep: f32,
    /// Zone influence (zone_id → (faction_id, pct)).
    pub zone_influence: Vec<(String, Vec<(String, f32)>)>,
    /// Completed mission IDs.
    pub missions_completed: Vec<String>,
    /// Failed mission IDs.
    pub missions_failed: Vec<String>,
    /// World time (sim-time).
    pub world_time: f32,
    /// Police profile (aggression, known zones).
    pub police_aggression: f32,
    /// Evidence identification score.
    pub evidence_identification: f32,
    /// Evidence items (kind, detail).
    pub evidence: Vec<(String, String)>,
    /// Crew member IDs (hirede).
    pub crew_ids: Vec<String>,
    /// Safehouse IDs (ejede).
    pub safehouse_ids: Vec<String>,
    /// Business IDs (ejede).
    pub business_ids: Vec<String>,
    /// Wanted heat points.
    pub heat_points: f32,
    /// Rival IDs.
    pub rival_ids: Vec<String>,
}

impl SaveState {
    pub fn new(slot: u32) -> Self {
        Self {
            version: SAVE_VERSION,
            slot,
            timestamp: 0.0,
            player: PlayerSaveState {
                pos: [0.0, 0.0],
                cash: 0,
                clean: 0,
                armed: false,
                inventory: Vec::new(),
            },
            faction_trust: Vec::new(),
            street_rep: 0.0,
            zone_influence: Vec::new(),
            missions_completed: Vec::new(),
            missions_failed: Vec::new(),
            world_time: 0.0,
            police_aggression: 0.0,
            evidence_identification: 0.0,
            evidence: Vec::new(),
            crew_ids: Vec::new(),
            safehouse_ids: Vec::new(),
            business_ids: Vec::new(),
            heat_points: 0.0,
            rival_ids: Vec::new(),
        }
    }

    /// Serialiser til bincode bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    /// Deserialiser fra bincode bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        bincode::deserialize(bytes).ok()
    }

    /// Tjek version; kør migration hvis nødvendigt.
    pub fn migrate(&mut self) -> Result<(), String> {
        if self.version > SAVE_VERSION {
            return Err(format!(
                "Save version {} er nyere end understøttet {} — opdater spillet.",
                self.version, SAVE_VERSION
            ));
        }
        while self.version < SAVE_VERSION {
            match self.version {
                0 => self.version = 1,
                v => return Err(format!("Ukendt save version {}", v)),
            }
        }
        Ok(())
    }

    /// Gem til fil.
    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let bytes = self.to_bytes();
        std::fs::write(path, bytes)
    }

    /// Load fra fil.
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, SaveError> {
        let bytes = std::fs::read(path).map_err(SaveError::Io)?;
        let mut state = Self::from_bytes(&bytes).ok_or(SaveError::Corrupt)?;
        state.migrate().map_err(SaveError::Migration)?;
        Ok(state)
    }
}

/// Save-fejl.
#[derive(Debug)]
pub enum SaveError {
    Io(std::io::Error),
    Corrupt,
    Migration(String),
}

impl std::fmt::Display for SaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveError::Io(e) => write!(f, "Save IO error: {}", e),
            SaveError::Corrupt => write!(f, "Save file corrupt"),
            SaveError::Migration(e) => write!(f, "Save migration error: {}", e),
        }
    }
}

impl std::error::Error for SaveError {}

/// Save-slot manager (5 slots).
pub struct SaveSlots {
    pub slots: [Option<SaveState>; 5],
}

impl SaveSlots {
    pub fn new() -> Self {
        Self {
            slots: Default::default(),
        }
    }

    pub fn save(&mut self, slot: usize, state: SaveState) -> bool {
        if slot >= 5 {
            return false;
        }
        self.slots[slot] = Some(state);
        true
    }

    pub fn load(&self, slot: usize) -> Option<&SaveState> {
        self.slots.get(slot).and_then(|s| s.as_ref())
    }

    pub fn delete(&mut self, slot: usize) -> bool {
        if slot >= 5 {
            return false;
        }
        self.slots[slot] = None;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_serializes_and_deserializes() {
        let mut state = SaveState::new(0);
        state.player.cash = 5000;
        state.heat_points = 25.0;
        let bytes = state.to_bytes();
        let loaded = SaveState::from_bytes(&bytes).unwrap();
        assert_eq!(loaded.player.cash, 5000);
        assert_eq!(loaded.heat_points, 25.0);
    }

    #[test]
    fn migration_rejects_future_version() {
        let mut state = SaveState::new(0);
        state.version = 999;
        assert!(state.migrate().is_err());
    }
}