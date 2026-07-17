#![allow(dead_code)] // Mission API er public/stub til fremtidig failure/UI integration.

//! Missions — quest-system med objectives, rewards og konsekvenser.
//!
//! Fase 7: data-drevne missioner (RON), tracker i WorldPlugin, rewards
//! (cash, items, reputation) og konsekvenser.

use serde::{Deserialize, Serialize};

/// Mission-status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MissionStatus {
    Inactive,
    Active,
    Completed,
    Failed,
}

/// Objective-type for en mission.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Objective {
    GoToZone { zone: String },
    StealVehicle { def_id: String },
    DeliverItem { item_id: String, count: u32, zone: String },
    KillOrNeutralize { target: String },
    TalkTo { npc_id: String },
    EscapePolice { heat_max: u8 },
    SurviveTime { seconds: f32 },
}

/// Mission-definition (loades fra RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDef {
    pub id: String,
    pub title: String,
    pub description: String,
    pub giver_faction: String,
    pub required_trust: f32,
    pub objectives: Vec<Objective>,
    pub rewards: Vec<Reward>,
    pub consequences: Vec<Consequence>,
}

/// Reward for at fuldføre en mission.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Reward {
    Cash { amount: u32, clean: bool },
    Item { item_id: String, count: u32 },
    FactionTrust { faction: String, delta: f32 },
    StreetRep { delta: f32 },
    Influence { zone: String, faction: String, delta: f32 },
}

/// Konsekvens af en mission (anvendes ved failure eller success).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Consequence {
    FactionTrust { faction: String, delta: f32 },
    StreetRep { delta: f32 },
    Heat { points: f32 },
}

/// Instans af en mission i spillet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub def: MissionDef,
    pub status: MissionStatus,
    pub current_objective: usize,
    pub timer: f32,
}

impl Mission {
    pub fn from_def(def: MissionDef) -> Self {
        Self {
            status: MissionStatus::Active,
            current_objective: 0,
            timer: 0.0,
            def,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.status == MissionStatus::Completed
            || self.current_objective >= self.def.objectives.len()
    }

    pub fn current_objective(&self) -> Option<&Objective> {
        self.def.objectives.get(self.current_objective)
    }

    pub fn advance(&mut self) {
        if self.status == MissionStatus::Active {
            self.current_objective += 1;
            if self.current_objective >= self.def.objectives.len() {
                self.status = MissionStatus::Completed;
            }
        }
    }

    pub fn fail(&mut self) {
        self.status = MissionStatus::Failed;
    }
}

/// Mission tracker: alle kendte/aktive missioner.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MissionTracker {
    pub missions: Vec<Mission>,
    pub completed: Vec<String>,
    pub failed: Vec<String>,
}

impl MissionTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&mut self, def: MissionDef) {
        if self.is_known(&def.id) {
            return;
        }
        self.missions.push(Mission::from_def(def));
    }

    pub fn is_known(&self, id: &str) -> bool {
        self.missions.iter().any(|m| m.def.id == id)
            || self.completed.iter().any(|s| s == id)
            || self.failed.iter().any(|s| s == id)
    }

    pub fn active(&self) -> Vec<usize> {
        self.missions
            .iter()
            .enumerate()
            .filter(|(_, m)| m.status == MissionStatus::Active)
            .map(|(i, _)| i)
            .collect()
    }

    pub fn complete(&mut self, idx: usize) {
        if let Some(m) = self.missions.get_mut(idx) {
            m.status = MissionStatus::Completed;
            self.completed.push(m.def.id.clone());
        }
    }

    pub fn fail(&mut self, idx: usize) {
        if let Some(m) = self.missions.get_mut(idx) {
            m.status = MissionStatus::Failed;
            self.failed.push(m.def.id.clone());
        }
    }

    pub fn get_active_mut(&mut self, id: &str) -> Option<&mut Mission> {
        self.missions
            .iter_mut()
            .find(|m| m.def.id == id && m.status == MissionStatus::Active)
    }
}

/// Default mission-opslagsværk med et par proof-of-concept missioner.
pub fn default_missions() -> Vec<MissionDef> {
    vec![
        MissionDef {
            id: "steal_rival_car".into(),
            title: "Wrong Car, Wrong Block".into(),
            description: "Stjæl en rival-bandes bil fra garagen og kør den tilbage.".into(),
            giver_faction: "southline_kings".into(),
            required_trust: -20.0,
            objectives: vec![
                Objective::GoToZone { zone: "east_blocks".into() },
                Objective::StealVehicle { def_id: "lowrider".into() },
                Objective::GoToZone { zone: "east_blocks".into() },
            ],
            rewards: vec![
                Reward::Cash { amount: 500, clean: false },
                Reward::FactionTrust { faction: "southline_kings".into(), delta: 10.0 },
            ],
            consequences: vec![
                Consequence::FactionTrust { faction: "los_cuervos".into(), delta: -15.0 },
            ],
        },
        MissionDef {
            id: "deliver_package".into(),
            title: "Dead Drop".into(),
            description: "Aflever en pakke i den angivne zone uden at tiltrække politi.".into(),
            giver_faction: "harbor_cartel".into(),
            required_trust: -10.0,
            objectives: vec![
                Objective::TalkTo { npc_id: "cartel_contact".into() },
                Objective::DeliverItem { item_id: "package".into(), count: 1, zone: "industrial_zone".into() },
                Objective::EscapePolice { heat_max: 1 },
            ],
            rewards: vec![
                Reward::Cash { amount: 300, clean: false },
                Reward::Item { item_id: "pistol_ammo".into(), count: 20 },
            ],
            consequences: vec![
                Consequence::Heat { points: 5.0 },
            ],
        },
    ]
}
