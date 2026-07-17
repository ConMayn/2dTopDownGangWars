#![allow(dead_code)] // Public API endnu ikke konsumeret overalt.

//! Zone influence — territorie-kontrol system.
//!
//! Hver zone har influence-procenter per faction (sum = 100%).
//! Spillerens handlinger (missioner, kaos, hjælp) ændrer influence.
//! Faction-AI skifter også influence over tid (patruljer, konflikter).
//!
//! Når en faction har > 50% influence, "ejer" de zonen (visuelt + gameplay).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::faction_def::FactionRegistry;

/// Én zones influence-state: faction_id → procent (0-100).
/// Summen bør være ~100, men vi håndhæver det ikke strengt.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZoneInfluence {
    /// faction_id → influence procent.
    pub influences: HashMap<String, f32>,
}

impl ZoneInfluence {
    pub fn new() -> Self {
        Self::default()
    }

    /// Opret med en dominerende faction.
    pub fn dominated_by(faction_id: &str, pct: f32) -> Self {
        let mut inf = HashMap::new();
        inf.insert(faction_id.to_string(), pct);
        inf.insert("civilians".to_string(), 100.0 - pct);
        Self { influences: inf }
    }

    /// Hent influence for en faction (0 hvis fraværende).
    pub fn get(&self, faction_id: &str) -> f32 {
        self.influences.get(faction_id).copied().unwrap_or(0.0)
    }

    /// Hvilken faction dominerer zonen (højest influence)?
    /// Returnerer None hvis zonen er neutral/tom.
    pub fn dominant(&self) -> Option<(&str, f32)> {
        self.influences
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, pct)| (id.as_str(), *pct))
    }

    /// Tilføj influence til en faction (clamped 0-100).
    /// Trækker proportionelt fra andre factions for at holde sum ~100.
    pub fn add_influence(&mut self, faction_id: &str, delta: f32) {
        let current = self.get(faction_id);
        let new_val = (current + delta).clamp(0.0, 100.0);
        let actual_delta = new_val - current;

        // Træk proportionalt fra andre factions.
        let other_total: f32 = self
            .influences
            .iter()
            .filter(|(id, _)| *id != faction_id)
            .map(|(_, v)| *v)
            .sum();

        if other_total > 0.0 {
            let scale = -actual_delta / other_total;
            for (id, v) in self.influences.iter_mut() {
                if id != faction_id {
                    *v = (*v + *v * scale).clamp(0.0, 100.0);
                }
            }
        }

        self.influences.insert(faction_id.to_string(), new_val);
    }

    /// Langsom drift mod ligevægt: hvis en faction har home_zone, stiger deres
    /// influence langsomt mod 60%. Civiles trækkes mod 20%.
    pub fn drift(&mut self, home_factions: &[&str], dt: f32) {
        for faction_id in home_factions {
            let current = self.get(faction_id);
            if current < 60.0 {
                let drift = (60.0 - current) * 0.01 * dt;
                self.add_influence(faction_id, drift);
            }
        }
    }

    /// Sum af alle influences (bør være ~100).
    pub fn total(&self) -> f32 {
        self.influences.values().sum()
    }
}

/// Influence-graf: zone_id → ZoneInfluence.
#[derive(Debug, Clone, Default)]
pub struct InfluenceGraph {
    zones: HashMap<String, ZoneInfluence>,
}

impl InfluenceGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialisér influence for en zone baseret på faction home_zones.
    pub fn init_zone(&mut self, zone_id: &str, owner: &str, police: f32) {
        let mut inf = ZoneInfluence::new();
        inf.influences.insert(owner.to_string(), 60.0);
        inf.influences.insert("civilians".to_string(), 40.0 - police * 0.4);
        inf.influences.insert("police".to_string(), police * 0.4);
        self.zones.insert(zone_id.to_string(), inf);
    }

    pub fn get(&self, zone_id: &str) -> Option<&ZoneInfluence> {
        self.zones.get(zone_id)
    }

    pub fn get_mut(&mut self, zone_id: &str) -> Option<&mut ZoneInfluence> {
        self.zones.get_mut(zone_id)
    }

    /// Opdatér influence for alle zoner (drift, faction-AI).
    pub fn update(&mut self, registry: &FactionRegistry, dt: f32) {
        for (zone_id, influence) in self.zones.iter_mut() {
            // Find hvilke factions der har denne zone som home.
            let home_factions: Vec<&str> = registry
                .defs()
                .filter(|f| f.home_zones.iter().any(|z| z == zone_id))
                .map(|f| f.id.as_str())
                .collect();
            influence.drift(&home_factions, dt);
        }
    }

    pub fn zones(&self) -> impl Iterator<Item = (&String, &ZoneInfluence)> {
        self.zones.iter()
    }
}