#![allow(dead_code)] // Mange rep-events/metoder er public API til fremtidige systemer.

//! Reputation — spillerens omdømme i byen.
//!
//! Fire lag (fra GDD afsnit 5):
//! 1. Street Rep — hvor farlig/respekteret du virker på gaden.
//! 2. Faction Trust — hver bande har sin egen holdning til dig.
//! 3. Civilian Fear/Love — civile i et område kan frygte eller elske dig.
//! 4. Police Profile — politiet bygger en profil på dig.
//!
//! Alt er data-drevet og serialiserbart (gemmes i save-filer).

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Én factions holdning til spilleren.
/// Trust er -100 (hadet) til +100 (familie). 0 = neutral.
/// Status er en kvalitativ kategorisering af trust.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FactionTrust {
    pub trust: f32,
    /// Har spilleren lavet jobs for denne faction?
    pub jobs_completed: u32,
    /// Har spilleren svigtet denne faction?
    pub jobs_failed: u32,
}

impl Default for FactionTrust {
    fn default() -> Self {
        Self {
            trust: 0.0,
            jobs_completed: 0,
            jobs_failed: 0,
        }
    }
}

impl FactionTrust {
    /// Kategorisering af trust-niveau.
    pub fn status(&self) -> FactionStatus {
        if self.trust >= 60.0 {
            FactionStatus::Family
        } else if self.trust >= 20.0 {
            FactionStatus::Trusted
        } else if self.trust >= -20.0 {
            FactionStatus::Neutral
        } else if self.trust >= -60.0 {
            FactionStatus::Suspicious
        } else {
            FactionStatus::Hunted
        }
    }
}

/// Kvalitativ faction-holdning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactionStatus {
    Family,      // 60+
    Trusted,     // 20-60
    Neutral,     // -20 to 20
    Suspicious,  // -60 to -20
    Hunted,      // < -60
}

impl FactionStatus {
    pub fn label(&self) -> &'static str {
        match self {
            FactionStatus::Family => "Family",
            FactionStatus::Trusted => "Trusted",
            FactionStatus::Neutral => "Neutral",
            FactionStatus::Suspicious => "Suspicious",
            FactionStatus::Hunted => "Hunted",
        }
    }
}

/// Civilian holdning til spilleren per zone.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct CivilianRep {
    /// Fear: 0.0 (rolig) til 1.0 (terrified).
    pub fear: f32,
    /// Love: 0.0 (hader) til 1.0 (elsker).
    pub love: f32,
}

impl CivilianRep {
    /// Samlet civilian attitude: -1.0 (hader) til +1.0 (elsker).
    pub fn attitude(&self) -> f32 {
        self.love - self.fear
    }
}

/// Politiet's profil på spilleren.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoliceProfile {
    /// Kendte zoner spilleren opererer i.
    pub known_zones: Vec<String>,
    /// Foretrukne bil-typer (fra observationer).
    pub preferred_vehicles: Vec<String>,
    /// Våbentyper brugt.
    pub weapon_types: Vec<String>,
    /// Aggressionsniveau 0.0-1.0 (hvor voldelig er spilleren?).
    pub aggression: f32,
    /// Kendte allierede (faction IDs).
    pub known_allies: Vec<String>,
    /// Flugtmønstre (retninger spilleren typisk flygter).
    pub escape_patterns: Vec<String>,
    /// Efterforskningstatus.
    pub investigation: InvestigationStatus,
}

/// Efterforskningsstatus (fra GDD afsnit 26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum InvestigationStatus {
    #[default]
    Unknown,          // ukendt mistænkt
    PersonOfInterest, // person of interest
    Identified,       // identificeret
    WarrantActive,    // aktiv arrestordre
    Manhunt,          // manhunt
}

impl InvestigationStatus {
    pub fn label(&self) -> &'static str {
        match self {
            InvestigationStatus::Unknown => "Unknown",
            InvestigationStatus::PersonOfInterest => "Person of Interest",
            InvestigationStatus::Identified => "Identified",
            InvestigationStatus::WarrantActive => "Warrant Active",
            InvestigationStatus::Manhunt => "Manhunt",
        }
    }
}

/// Spillerens fulde reputation-state.
/// Gemmes i save-filer. Opdateres af reputation-events.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReputationState {
    /// Street Rep: 0-100. Hvor respekteret/farlig du virker.
    pub street_rep: f32,
    /// Trust per faction (faction_id → FactionTrust).
    pub faction_trust: HashMap<String, FactionTrust>,
    /// Civilian rep per zone (zone_id → CivilianRep).
    pub civilian_rep: HashMap<String, CivilianRep>,
    /// Politiet's profil.
    pub police_profile: PoliceProfile,
}

impl ReputationState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Hent trust for en faction (0 hvis ukendt).
    pub fn trust(&self, faction_id: &str) -> f32 {
        self.faction_trust
            .get(faction_id)
            .map(|t| t.trust)
            .unwrap_or(0.0)
    }

    /// Hent faction status.
    pub fn status(&self, faction_id: &str) -> FactionStatus {
        self.faction_trust
            .get(faction_id)
            .map(|t| t.status())
            .unwrap_or(FactionStatus::Neutral)
    }

    /// Hent civilian rep for en zone (default hvis ukendt).
    pub fn civilian(&self, zone_id: &str) -> CivilianRep {
        self.civilian_rep
            .get(zone_id)
            .copied()
            .unwrap_or_default()
    }
}

/// Reputation-event: en handling der ændrer spillerens omdømme.
/// Kaldes af gameplay-systemer (missioner, combat, hjælp etc.).
#[derive(Debug, Clone)]
pub enum RepEvent {
    /// Gennemførte et job for en faction.
    JobCompleted { faction: String, reward: f32 },
    /// Fejlede et job for en faction.
    JobFailed { faction: String, penalty: f32 },
    /// Dræbte et medlem af en faction.
    MemberKilled { faction: String },
    /// Hjalp en NPC i en zone (civilian love stiger).
    HelpedCivilian { zone: String },
    /// Skabte kaos i en zone (civilian fear stiger).
    CausedChaos { zone: String, severity: f32 },
    /// Set med våben i en zone.
    SeenArmed { zone: String },
    /// Blev set køre vildt (politiaggression stiger).
    RecklessDriving { zone: String },
    /// Stjal en bil.
    StoleVehicle { faction: String },
    /// Forrådte en faction til en anden.
    Betrayed { from_faction: String, to_faction: String },
    /// Voldt mod en rival (street rep stiger).
    WonFight { reputation_gain: f32 },
    /// Tabte en fight offentligt (street rep falder).
    LostFight { reputation_loss: f32 },
}

/// Anvend en reputation-event på state.
pub fn apply_event(state: &mut ReputationState, event: &RepEvent) {
    match event {
        RepEvent::JobCompleted { faction, reward } => {
            let t = state.faction_trust.entry(faction.clone()).or_default();
            t.trust = (t.trust + reward).clamp(-100.0, 100.0);
            t.jobs_completed += 1;
            state.street_rep = (state.street_rep + reward * 0.3).clamp(0.0, 100.0);
        }
        RepEvent::JobFailed { faction, penalty } => {
            let t = state.faction_trust.entry(faction.clone()).or_default();
            t.trust = (t.trust - penalty).clamp(-100.0, 100.0);
            t.jobs_failed += 1;
        }
        RepEvent::MemberKilled { faction } => {
            let t = state.faction_trust.entry(faction.clone()).or_default();
            t.trust = (t.trust - 25.0).clamp(-100.0, 100.0);
            state.street_rep = (state.street_rep + 5.0).clamp(0.0, 100.0);
            // Hvis spilleren dræber en gang-medlem, stiger aggression hos politiet.
            state.police_profile.aggression = (state.police_profile.aggression + 0.1).min(1.0);
        }
        RepEvent::HelpedCivilian { zone } => {
            let c = state.civilian_rep.entry(zone.clone()).or_default();
            c.love = (c.love + 0.05).min(1.0);
            c.fear = (c.fear - 0.02).max(0.0);
        }
        RepEvent::CausedChaos { zone, severity } => {
            let c = state.civilian_rep.entry(zone.clone()).or_default();
            c.fear = (c.fear + severity * 0.1).min(1.0);
            c.love = (c.love - severity * 0.03).max(0.0);
            state.street_rep = (state.street_rep + severity * 2.0).clamp(0.0, 100.0);
        }
        RepEvent::SeenArmed { zone } => {
            let c = state.civilian_rep.entry(zone.clone()).or_default();
            c.fear = (c.fear + 0.02).min(1.0);
        }
        RepEvent::RecklessDriving { zone } => {
            let c = state.civilian_rep.entry(zone.clone()).or_default();
            c.fear = (c.fear + 0.03).min(1.0);
            if !state.police_profile.known_zones.contains(zone) {
                state.police_profile.known_zones.push(zone.clone());
            }
        }
        RepEvent::StoleVehicle { faction } => {
            if !faction.is_empty() {
                let t = state.faction_trust.entry(faction.clone()).or_default();
                t.trust = (t.trust - 10.0).clamp(-100.0, 100.0);
            }
            state.street_rep = (state.street_rep + 1.0).clamp(0.0, 100.0);
        }
        RepEvent::Betrayed { from_faction, to_faction } => {
            let from = state.faction_trust.entry(from_faction.clone()).or_default();
            from.trust = (from.trust - 40.0).clamp(-100.0, 100.0);
            let to = state.faction_trust.entry(to_faction.clone()).or_default();
            to.trust = (to.trust + 15.0).clamp(-100.0, 100.0);
        }
        RepEvent::WonFight { reputation_gain } => {
            state.street_rep = (state.street_rep + reputation_gain).clamp(0.0, 100.0);
        }
        RepEvent::LostFight { reputation_loss } => {
            state.street_rep = (state.street_rep - reputation_loss).max(0.0);
        }
    }
}

/// Appliér en liste af events (batch).
pub fn apply_events(state: &mut ReputationState, events: &[RepEvent]) {
    for event in events {
        apply_event(state, event);
    }
}