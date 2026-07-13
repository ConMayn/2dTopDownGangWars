//! Faction — definitioner for byens grupper.
//!
//! Hver faction har personlighed, økonomi, aggression, hjem-zoner og allierede/fjender.
//! Data-drevet (RON-serialiserbar) — klar til at loade fra assets/data/factions.ron.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Faction-type: bestemmer overordnet adfærd.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FactionKind {
    StreetGang,   // gadebande — territorium, respekt, småkrig
    Mafia,         // rolig, farlig, rig — afpresning, snigmord
    Biker,         // udkant, motorcykler, våbenhandel
    Cartel,        // international, havn, smugling
    Police,        // lov og orden
    Civilian,      // neutrale civile
}

impl FactionKind {
    pub fn label(&self) -> &'static str {
        match self {
            FactionKind::StreetGang => "Street Gang",
            FactionKind::Mafia => "Mafia",
            FactionKind::Biker => "Biker Gang",
            FactionKind::Cartel => "Cartel",
            FactionKind::Police => "Police",
            FactionKind::Civilian => "Civilian",
        }
    }
}

/// Faction-definition (statisk data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionDef {
    pub id: String,
    pub name: String,
    pub kind: FactionKind,
    /// Hvilke zoner de holder til i (zone IDs).
    pub home_zones: Vec<String>,
    /// Allierede faction IDs.
    pub allies: Vec<String>,
    /// Fjendtlige faction IDs.
    pub enemies: Vec<String>,
    /// Indkomst niveau (penge per dag passivt).
    pub income: f32,
    /// Aggression 0.0-1.0 — hvor hurtigt de reagerer voldeligt.
    pub aggression: f32,
    /// Disciplin 0.0-1.0 — hvor organiserede de er.
    pub discipline: f32,
    /// Primær farve (til graffiti, UI).
    pub color: [f32; 4],
}

/// Default factions for Heat City.
/// 4 street gangs + 1 mafia + 1 biker + 1 cartel + police.
impl FactionDef {
    pub fn all_defaults() -> Vec<FactionDef> {
        vec![
            Self {
                id: "southline_kings".into(),
                name: "Southline Kings".into(),
                kind: FactionKind::StreetGang,
                home_zones: vec!["east_blocks".into()],
                allies: vec![],
                enemies: vec!["los_cuervos".into(), "iron_hounds".into()],
                income: 500.0,
                aggression: 0.7,
                discipline: 0.3,
                color: [0.8, 0.2, 0.2, 1.0], // rød
            },
            Self {
                id: "los_cuervos".into(),
                name: "Los Cuervos".into(),
                kind: FactionKind::StreetGang,
                home_zones: vec!["east_blocks".into()],
                allies: vec![],
                enemies: vec!["southline_kings".into()],
                income: 600.0,
                aggression: 0.6,
                discipline: 0.4,
                color: [0.2, 0.6, 0.8, 1.0], // blå
            },
            Self {
                id: "old_harbor_mafia".into(),
                name: "Old Harbor Mafia".into(),
                kind: FactionKind::Mafia,
                home_zones: vec!["old_town".into()],
                allies: vec![],
                enemies: vec![],
                income: 2000.0,
                aggression: 0.3,
                discipline: 0.9,
                color: [0.15, 0.15, 0.2, 1.0], // mørk
            },
            Self {
                id: "iron_hounds".into(),
                name: "Iron Hounds".into(),
                kind: FactionKind::Biker,
                home_zones: vec!["desert_outskirts".into()],
                allies: vec![],
                enemies: vec!["southline_kings".into()],
                income: 800.0,
                aggression: 0.8,
                discipline: 0.2,
                color: [0.3, 0.3, 0.35, 1.0], // stålgrå
            },
            Self {
                id: "harbor_cartel".into(),
                name: "Harbor Cartel".into(),
                kind: FactionKind::Cartel,
                home_zones: vec!["industrial_zone".into()],
                allies: vec![],
                enemies: vec![],
                income: 1500.0,
                aggression: 0.5,
                discipline: 0.7,
                color: [0.1, 0.5, 0.2, 1.0], // mørkegrøn
            },
            Self {
                id: "police".into(),
                name: "Police Department".into(),
                kind: FactionKind::Police,
                home_zones: vec!["government_district".into()],
                allies: vec![],
                enemies: vec![],
                income: 0.0,
                aggression: 0.4,
                discipline: 0.8,
                color: [0.1, 0.2, 0.5, 1.0], // mørkeblå
            },
            Self {
                id: "civilians".into(),
                name: "Civilians".into(),
                kind: FactionKind::Civilian,
                home_zones: vec![],
                allies: vec![],
                enemies: vec![],
                income: 0.0,
                aggression: 0.0,
                discipline: 0.0,
                color: [0.5, 0.5, 0.5, 1.0], // grå
            },
        ]
    }
}

/// Registry af faction-definitioner.
#[derive(Debug, Clone)]
pub struct FactionRegistry {
    defs: HashMap<String, FactionDef>,
}

impl FactionRegistry {
    pub fn from_defaults() -> Self {
        let mut defs = HashMap::new();
        for def in FactionDef::all_defaults() {
            defs.insert(def.id.clone(), def);
        }
        Self { defs }
    }

    pub fn get(&self, id: &str) -> Option<&FactionDef> {
        self.defs.get(id)
    }

    pub fn ids(&self) -> impl Iterator<Item = &str> {
        self.defs.keys().map(|s| s.as_str())
    }

    pub fn defs(&self) -> impl Iterator<Item = &FactionDef> {
        self.defs.values()
    }

    pub fn len(&self) -> usize {
        self.defs.len()
    }
}