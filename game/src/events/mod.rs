#![allow(dead_code)] // Events API er public/stub til fremtidig spawning.

//! Random events — gade-events der sker af sig selv.
//!
//! Fra GDD afsnit 22:
//! Gade-events: to bander skændes, politi stopper bil, ambulance til ulykke,
//! butik røvet, bilulykke spærrer vej, ulovligt race, NPC løber fra politi,
//! rivaler leder efter dig, informant jagtes, demonstration, brand,
//! mafia-begravelse, biker-konvoj, blackout, stormvejr.
//!
//! Spilleren kan blande sig eller ignorere. Ikke alt er en mission marker.

use serde::{Deserialize, Serialize};

/// Event-type (fra GDD 22).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventKind {
    GangSkirmish,       // to bander skændes
    TrafficStop,        // politi stopper bil
    AmbulanceRun,       // ambulance til ulykke
    StoreRobbery,       // butik røvet
    CarAccident,        // bilulykke spærrer vej
    StreetRace,         // ulovligt race
    FootChase,          // NPC løber fra politi
    RivalHunt,          // rivaler leder efter dig
    InformantChased,    // informant jagtes
    Demonstration,      // demonstration
    BuildingFire,        // brand
    MafiaFuneral,       // mafia-begravelse
    BikerConvoy,        // biker-konvoj
    Blackout,            // blackout
    StormWeather,        // stormvejr
}

impl EventKind {
    pub fn label(&self) -> &'static str {
        match self {
            EventKind::GangSkirmish => "Gang Skirmish",
            EventKind::TrafficStop => "Traffic Stop",
            EventKind::AmbulanceRun => "Ambulance Run",
            EventKind::StoreRobbery => "Store Robbery",
            EventKind::CarAccident => "Car Accident",
            EventKind::StreetRace => "Street Race",
            EventKind::FootChase => "Foot Chase",
            EventKind::RivalHunt => "Rival Hunt",
            EventKind::InformantChased => "Informant Chased",
            EventKind::Demonstration => "Demonstration",
            EventKind::BuildingFire => "Building Fire",
            EventKind::MafiaFuneral => "Mafia Funeral",
            EventKind::BikerConvoy => "Biker Convoy",
            EventKind::Blackout => "Blackout",
            EventKind::StormWeather => "Storm Weather",
        }
    }

    /// Hvor meget heat eventet genererer for spilleren hvis de blander sig.
    pub fn heat_on_intervention(&self) -> f32 {
        match self {
            EventKind::GangSkirmish => 10.0,
            EventKind::TrafficStop => 5.0,
            EventKind::AmbulanceRun => 2.0,
            EventKind::StoreRobbery => 8.0,
            EventKind::CarAccident => 3.0,
            EventKind::StreetRace => 15.0,
            EventKind::FootChase => 5.0,
            EventKind::RivalHunt => 20.0,
            EventKind::InformantChased => 12.0,
            EventKind::Demonstration => 8.0,
            EventKind::BuildingFire => 5.0,
            EventKind::MafiaFuneral => 10.0,
            EventKind::BikerConvoy => 7.0,
            EventKind::Blackout => 15.0,
            EventKind::StormWeather => 0.0,
        }
    }

    /// Varighed i sim-sek (hvor længe eventet varer).
    pub fn duration(&self) -> f32 {
        match self {
            EventKind::GangSkirmish => 30.0,
            EventKind::TrafficStop => 20.0,
            EventKind::AmbulanceRun => 15.0,
            EventKind::StoreRobbery => 25.0,
            EventKind::CarAccident => 40.0,
            EventKind::StreetRace => 60.0,
            EventKind::FootChase => 20.0,
            EventKind::RivalHunt => 120.0,
            EventKind::InformantChased => 30.0,
            EventKind::Demonstration => 300.0,
            EventKind::BuildingFire => 180.0,
            EventKind::MafiaFuneral => 200.0,
            EventKind::BikerConvoy => 90.0,
            EventKind::Blackout => 600.0,
            EventKind::StormWeather => 900.0,
        }
    }
}

/// En aktiv event-instans i verden.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub kind: EventKind,
    pub zone: String,
    /// Position (x, y) hvor eventet sker.
    pub pos: [f32; 2],
    /// Tid tilbage (sim-sek).
    pub time_left: f32,
    /// Har spilleren blandet sig?
    pub intervened: bool,
}

impl WorldEvent {
    pub fn new(id: &str, kind: EventKind, zone: &str, pos: [f32; 2]) -> Self {
        Self {
            id: id.to_string(),
            kind,
            zone: zone.to_string(),
            pos,
            time_left: kind.duration(),
            intervened: false,
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        self.time_left -= dt;
        self.time_left <= 0.0
    }
}

/// Event-manager: genererer og trackere aktive events.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventManager {
    pub active: Vec<WorldEvent>,
    /// Total events spawned.
    pub total_spawned: u32,
    /// Counter for unikt ID.
    pub id_counter: u32,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawn et event.
    pub fn spawn(&mut self, kind: EventKind, zone: &str, pos: [f32; 2]) -> &WorldEvent {
        self.id_counter += 1;
        let id = format!("event_{}", self.id_counter);
        self.active.push(WorldEvent::new(&id, kind, zone, pos));
        self.total_spawned += 1;
        self.active.last().unwrap()
    }

    /// Opdatér alle aktive events; fjern udløbne.
    pub fn tick(&mut self, dt: f32) -> Vec<WorldEvent> {
        let mut expired = Vec::new();
        let mut keep = Vec::new();
        for mut e in self.active.drain(..) {
            let done = e.tick(dt);
            if done {
                expired.push(e);
            } else {
                keep.push(e);
            }
        }
        self.active = keep;
        expired
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }

    /// Find nærmeste event til en position.
    pub fn nearest(&self, pos: [f32; 2]) -> Option<&WorldEvent> {
        self.active.iter().min_by(|a, b| {
            let da = (a.pos[0] - pos[0]).powi(2) + (a.pos[1] - pos[1]).powi(2);
            let db = (b.pos[0] - pos[0]).powi(2) + (b.pos[1] - pos[1]).powi(2);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Markér at spilleren har blandet sig i et event.
    pub fn intervene(&mut self, event_id: &str) -> bool {
        if let Some(e) = self.active.iter_mut().find(|e| e.id == event_id) {
            e.intervened = true;
            true
        } else {
            false
        }
    }
}