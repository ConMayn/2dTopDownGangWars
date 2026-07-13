//! Faction AI — beslutninger, patruljer, konflikter.
//!
//! Fase 5: simpel faction-AI der opdaterer influence over tid, reagerer på
//! spillerens handlinger, og genererer faction-reaktioner (dialog, aggression).
//!
//! Fase 10+: fuld AI Director integration med emergent events.

use super::faction_def::{FactionDef, FactionKind, FactionRegistry};
use super::influence::InfluenceGraph;
use super::reputation::{FactionStatus, RepEvent, ReputationState, apply_event};

/// Faction AI state — kører per frame, opdaterer influence og reputation.
pub struct FactionAi {
    /// Timer for næste faction-konflikt tick (hver ~30 sim-sek).
    conflict_timer: f32,
}

impl FactionAi {
    pub fn new() -> Self {
        Self {
            conflict_timer: 0.0,
        }
    }

    /// Opdatér faction-AI: influence drift, konflikter, reputation decay.
    /// Kaldes per frame med dt.
    pub fn update(
        &mut self,
        registry: &FactionRegistry,
        graph: &mut InfluenceGraph,
        reputation: &mut ReputationState,
        dt: f32,
    ) {
        // 1. Influence drift (factions langsomt overtager home-zones).
        graph.update(registry, dt);

        // 2. Konflikt-timer: hver ~30 sim-sekunders simulér en faction-konflikt.
        self.conflict_timer += dt;
        if self.conflict_timer > 30.0 {
            self.conflict_timer = 0.0;
            self.simulate_conflict(registry, graph);
        }

        // 3. Street rep langsom decay mod et baseline (hvis ingen nye events).
        // Rep falder mod 10 hvis over 10, stiger ikke hvis under.
        if reputation.street_rep > 10.0 {
            reputation.street_rep = (reputation.street_rep - dt * 0.5).max(10.0);
        }
    }

    /// Simulér en tilfældig faction-konflikt: en faction angriber en anden's zone.
    /// Dette ændrer influence uden spillerens involvering.
    fn simulate_conflict(&self, registry: &FactionRegistry, graph: &mut InfluenceGraph) {
        // Find alle street gangs.
        let gangs: Vec<&FactionDef> = registry
            .defs()
            .filter(|f| f.kind == FactionKind::StreetGang)
            .collect();
        if gangs.len() < 2 {
            return;
        }

        // Vælg en tilfældig angribende og forsvarende faction.
        let attacker_idx = (self.conflict_timer as usize * 7) % gangs.len();
        let attacker = gangs[attacker_idx];
        let defender = gangs
            .iter()
            .find(|g| attacker.enemies.contains(&g.id))
            .or_else(|| gangs.iter().find(|g| g.id != attacker.id));

        if let Some(defender) = defender {
            // Angrib en af defender's home zones.
            for zone_id in &defender.home_zones {
                if let Some(inf) = graph.get_mut(zone_id) {
                    let defender_pct = inf.get(&defender.id);
                    let attacker_pct = inf.get(&attacker.id);
                    // Konflikt: angribers influence stiger, defenders falder.
                    let change = if attacker.aggression > 0.6 { 3.0 } else { 1.5 };
                    inf.add_influence(&attacker.id, change);
                    inf.add_influence(&defender.id, -change * 0.5);
                    tracing::debug!(
                        "Faction conflict: {} attacked {} in {} (+{:.1}% influence)",
                        attacker.name,
                        defender.name,
                        zone_id,
                        change
                    );
                    break;
                }
            }
        }
    }

    /// Håndter en reputation-event: opdatér reputation state.
    /// Dette er den API andre systemer (missioner, combat) kalder.
    pub fn handle_event(&self, reputation: &mut ReputationState, event: &RepEvent) {
        apply_event(reputation, event);
    }

    /// Beregn en NPC's reaktion på spilleren baseret på spillerens reputation
    /// hos den faction NPC'en tilhører.
    /// Returnerer en "attitude" string til brug i dialog.
    pub fn npc_attitude(
        &self,
        reputation: &ReputationState,
        faction_id: &str,
        player_armed: bool,
    ) -> &'static str {
        let status = reputation.status(faction_id);
        match status {
            FactionStatus::Family => "My friend! What do you need?",
            FactionStatus::Trusted => "Good to see you. We're cool.",
            FactionStatus::Neutral => {
                if player_armed {
                    "Put that away. Not here."
                } else {
                    "Don't know you. Move along."
                }
            }
            FactionStatus::Suspicious => "I've heard about you. Not good things.",
            FactionStatus::Hunted => {
                if player_armed {
                    "You're dead meat, you hear me?"
                } else {
                    "Get out. Now. Before I call them."
                }
            }
        }
    }
}

impl Default for FactionAi {
    fn default() -> Self {
        Self::new()
    }
}