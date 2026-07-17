#![allow(dead_code)] // Heist API er public/stub til fremtidig planlægnings-UI.

//! Heists — store planlagte røverier med flere approaches.
//!
//! Fra GDD afsnit 24:
//! - Planlægningsfase: approach, crew, flugtrute, køretøj, våben, disguise, afledning, stash, insider, risiko.
//! - 4 approaches: Loud, Quiet, Social, Dirty.
//! - Samme job kan løses på flere måder.
//! - Konsekvenser: heat, beviser, faction trust, crew-skader.

use serde::{Deserialize, Serialize};

/// Approach — strategi for heist (GDD 24.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Approach {
    /// Hurtigt, voldsomt, meget heat. Lav planlægning, høj risiko.
    Loud,
    /// Mere planlægning, mindre heat, men sværere. Kræver stealth/lockpick.
    Quiet,
    /// Brug relationer, forklædning, adgang, bestikkelse. Kræver reputation.
    Social,
    /// Frame en anden faction. Lav personlig heat, men skaber faction-konflikt.
    Dirty,
}

impl Approach {
    pub fn label(&self) -> &'static str {
        match self {
            Approach::Loud => "Loud",
            Approach::Quiet => "Quiet",
            Approach::Social => "Social",
            Approach::Dirty => "Dirty",
        }
    }

    /// Basal heat der genereres ved udførelse.
    pub fn base_heat(&self) -> f32 {
        match self {
            Approach::Loud => 40.0,
            Approach::Quiet => 10.0,
            Approach::Social => 5.0,
            Approach::Dirty => 15.0,
        }
    }

    /// Hvor meget evidence der typisk efterlades.
    pub fn evidence_weight(&self) -> f32 {
        match self {
            Approach::Loud => 30.0,
            Approach::Quiet => 8.0,
            Approach::Social => 3.0,
            Approach::Dirty => 12.0,
        }
    }

    /// Krævet crew-rolle for approachen.
    pub fn preferred_role(&self) -> Option<crate::crew::CrewRole> {
        use crate::crew::CrewRole;
        match self {
            Approach::Loud => Some(CrewRole::Gunman),
            Approach::Quiet => Some(CrewRole::Ghost),
            Approach::Social => Some(CrewRole::Fixer),
            Approach::Dirty => Some(CrewRole::Fixer),
        }
    }
}

/// Flugtrute — hvordan spilleren undslipper efter heist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EscapeRoute {
    /// Kør direkte ud af zonen — hurtigt, men synligt.
    Direct,
    /// Kør gennem baggårde/smalle gader — langsommere, mindre synligt.
    BackAlleys,
    /// Skjul i safehouse i zonen — kræver ejet safehouse.
    Safehouse,
    /// Skift bil under flugt — kræver stash-bil.
    SwitchCar,
    /// Tag tunnel/underground — kræver insider viden.
    Tunnel,
    /// Flugt via tag — stealth-krav.
    Rooftop,
}

impl EscapeRoute {
    pub fn label(&self) -> &'static str {
        match self {
            EscapeRoute::Direct => "Direct",
            EscapeRoute::BackAlleys => "Back Alleys",
            EscapeRoute::Safehouse => "Safehouse",
            EscapeRoute::SwitchCar => "Switch Car",
            EscapeRoute::Tunnel => "Tunnel",
            EscapeRoute::Rooftop => "Rooftop",
        }
    }

    /// Hvor meget heat der reduceres ved flugt (højere = bedre flugt).
    pub fn heat_reduction(&self) -> f32 {
        match self {
            EscapeRoute::Direct => 2.0,
            EscapeRoute::BackAlleys => 6.0,
            EscapeRoute::Safehouse => 12.0,
            EscapeRoute::SwitchCar => 15.0,
            EscapeRoute::Tunnel => 10.0,
            EscapeRoute::Rooftop => 8.0,
        }
    }
}

/// Aflednings-metode — distraherer politi under heist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Diversion {
    None,
    FakeCall,       // falsk politi-opkald
    FireAlarm,       // brandalarm
    DecoyCar,        // lokke-bil
    GangSkirmish,    // start bande-slåskamp som afledning
    Explosive,       // eksplosion (farligt, meget heat)
}

impl Diversion {
    pub fn label(&self) -> &'static str {
        match self {
            Diversion::None => "None",
            Diversion::FakeCall => "Fake Call",
            Diversion::FireAlarm => "Fire Alarm",
            Diversion::DecoyCar => "Decoy Car",
            Diversion::GangSkirmish => "Gang Skirmish",
            Diversion::Explosive => "Explosive",
        }
    }

    pub fn cost(&self) -> u32 {
        match self {
            Diversion::None => 0,
            Diversion::FakeCall => 100,
            Diversion::FireAlarm => 50,
            Diversion::DecoyCar => 500,
            Diversion::GangSkirmish => 1_000,
            Diversion::Explosive => 2_000,
        }
    }

    pub fn heat_reduction(&self) -> f32 {
        match self {
            Diversion::None => 0.0,
            Diversion::FakeCall => 8.0,
            Diversion::FireAlarm => 5.0,
            Diversion::DecoyCar => 12.0,
            Diversion::GangSkirmish => 15.0,
            Diversion::Explosive => 20.0,
        }
    }
}

/// Heist-definition (loades fra RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeistDef {
    pub id: String,
    pub title: String,
    pub description: String,
    pub zone: String,
    /// Tilladte approaches.
    pub approaches: Vec<Approach>,
    /// Anbefalet crew-størrelse.
    pub recommended_crew: u32,
    /// Minimum crew-loyalty for at undgå forræderi.
    pub min_loyalty: f32,
    /// Belønning ved success (cash).
    pub reward_cash: u32,
    /// Belønning clean money.
    pub reward_clean: u32,
    /// Hvor meget evidence der genereres ved failure.
    pub evidence_on_fail: f32,
    /// Faction der påvirkes (target).
    pub target_faction: String,
    /// Faction der drager fordel (hvis Dirty).
    pub beneficiary_faction: Option<String>,
}

/// Planlagt heist — spillerens valg.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeistPlan {
    pub def_id: String,
    pub approach: Approach,
    pub escape: EscapeRoute,
    pub diversion: Diversion,
    /// Crew-medlem IDs der tages med.
    pub crew_ids: Vec<String>,
    /// Køretøj def_id til flugt.
    pub vehicle: String,
    /// Disguise item-id eller None.
    pub disguise: Option<String>,
    /// Risiko-niveau spilleren tager (0-100). Højere = større reward, større fare.
    pub risk_appetite: f32,
}

impl HeistPlan {
    pub fn new(def_id: &str, approach: Approach) -> Self {
        Self {
            def_id: def_id.to_string(),
            approach,
            escape: EscapeRoute::Direct,
            diversion: Diversion::None,
            crew_ids: Vec::new(),
            vehicle: String::new(),
            disguise: None,
            risk_appetite: 30.0,
        }
    }
}

/// Heist-status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeistStatus {
    Planned,
    InProgress,
    Success,
    Failed,
    Aborted,
}

/// En heist-instans i verden.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heist {
    pub def: HeistDef,
    pub plan: HeistPlan,
    pub status: HeistStatus,
    /// Fremgang 0-100.
    pub progress: f32,
    /// Heat akkumuleret under heist.
    pub heat_generated: f32,
    /// Evidence akkumuleret.
    pub evidence_generated: f32,
}

impl Heist {
    pub fn new(def: HeistDef, plan: HeistPlan) -> Self {
        Self {
            def,
            plan,
            status: HeistStatus::Planned,
            progress: 0.0,
            heat_generated: 0.0,
            evidence_generated: 0.0,
        }
    }

    /// Start heist.
    pub fn start(&mut self) {
        self.status = HeistStatus::InProgress;
        self.progress = 0.0;
    }

    /// Opdatér heist per frame. Returnerer true hvis heist er færdig.
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.status != HeistStatus::InProgress {
            return false;
        }
        // Fremskridt baseret på approach og risiko.
        let speed = match self.plan.approach {
            Approach::Loud => 25.0,
            Approach::Quiet => 10.0,
            Approach::Social => 15.0,
            Approach::Dirty => 12.0,
        };
        self.progress = (self.progress + speed * dt).min(100.0);
        // Heat stiger over tid.
        self.heat_generated += self.plan.approach.base_heat() * dt * 0.1;
        // Evidence stiger baseret på approach.
        self.evidence_generated += self.plan.approach.evidence_weight() * dt * 0.05;
        if self.progress >= 100.0 {
            self.status = HeistStatus::Success;
            return true;
        }
        false
    }

    /// Fuldfør heist og returnér konsekvenser.
    pub fn complete(&mut self) -> HeistOutcome {
        self.status = HeistStatus::Success;
        let net_heat = (self.heat_generated - self.plan.escape.heat_reduction() - self.plan.diversion.heat_reduction()).max(0.0);
        let net_evidence = (self.evidence_generated - self.plan.escape.heat_reduction() * 0.3).max(0.0);
        HeistOutcome {
            reward_cash: self.def.reward_cash,
            reward_clean: self.def.reward_clean,
            heat: net_heat,
            evidence: net_evidence,
            faction_trust_delta: -10.0, // target faction vred
            crew_injured: Vec::new(),
            success: true,
        }
    }

    /// Fejl heist.
    pub fn fail(&mut self) -> HeistOutcome {
        self.status = HeistStatus::Failed;
        HeistOutcome {
            reward_cash: 0,
            reward_clean: 0,
            heat: self.heat_generated + 20.0,
            evidence: self.evidence_generated + self.def.evidence_on_fail,
            faction_trust_delta: -20.0,
            crew_injured: self.plan.crew_ids.clone(),
            success: false,
        }
    }
}

/// Resultat af en heist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeistOutcome {
    pub reward_cash: u32,
    pub reward_clean: u32,
    pub heat: f32,
    pub evidence: f32,
    pub faction_trust_delta: f32,
    pub crew_injured: Vec<String>,
    pub success: bool,
}

/// Heist-manager: alle kendte heists + aktive.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HeistManager {
    pub available: Vec<HeistDef>,
    pub active: Option<Heist>,
    pub completed: Vec<String>,
}

impl HeistManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_available(&mut self, def: HeistDef) {
        self.available.push(def);
    }

    pub fn start(&mut self, def_id: &str, plan: HeistPlan) -> bool {
        if self.active.is_some() {
            return false;
        }
        let def = match self.available.iter().find(|d| d.id == def_id) {
            Some(d) => d.clone(),
            None => return false,
        };
        let mut heist = Heist::new(def, plan);
        heist.start();
        self.active = Some(heist);
        true
    }

    pub fn tick_active(&mut self, dt: f32) -> Option<HeistOutcome> {
        let heist = self.active.as_mut()?;
        let done = heist.tick(dt);
        if done {
            let outcome = heist.complete();
            self.completed.push(heist.def.id.clone());
            self.active = None;
            Some(outcome)
        } else {
            None
        }
    }

    pub fn abort(&mut self) -> Option<HeistOutcome> {
        let mut heist = self.active.take()?;
        Some(heist.fail())
    }
}

/// Default heists (fra GDD 24.3 + flere).
pub fn default_heists() -> Vec<HeistDef> {
    vec![
        HeistDef {
            id: "armored_van".into(),
            title: "Armored Van Heist".into(),
            description: "Ram en pansret værditransport. Kan gøres loud eller via insider-rute.".into(),
            zone: "downtown".into(),
            approaches: vec![Approach::Loud, Approach::Quiet, Approach::Social, Approach::Dirty],
            recommended_crew: 3,
            min_loyalty: 40.0,
            reward_cash: 25_000,
            reward_clean: 0,
            evidence_on_fail: 40.0,
            target_faction: "police".into(),
            beneficiary_faction: None,
        },
        HeistDef {
            id: "container_heist".into(),
            title: "Container Heist".into(),
            description: "Stjæl en container fra havnen med ukendt indhold.".into(),
            zone: "industrial_zone".into(),
            approaches: vec![Approach::Quiet, Approach::Loud, Approach::Dirty],
            recommended_crew: 2,
            min_loyalty: 30.0,
            reward_cash: 15_000,
            reward_clean: 0,
            evidence_on_fail: 25.0,
            target_faction: "harbor_cartel".into(),
            beneficiary_faction: Some("southline_kings".into()),
        },
        HeistDef {
            id: "bank_job".into(),
            title: "Bank Job".into(),
            description: "Ran Downtown Bank. Høj reward, høj risiko, kræver planlægning.".into(),
            zone: "downtown".into(),
            approaches: vec![Approach::Loud, Approach::Quiet, Approach::Social],
            recommended_crew: 4,
            min_loyalty: 60.0,
            reward_cash: 100_000,
            reward_clean: 5_000,
            evidence_on_fail: 60.0,
            target_faction: "police".into(),
            beneficiary_faction: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loud_generates_more_heat_than_quiet() {
        assert!(Approach::Loud.base_heat() > Approach::Quiet.base_heat());
    }

    #[test]
    fn heist_completes_when_progress_maxed() {
        let def = default_heists()[0].clone();
        let plan = HeistPlan::new(&def.id, Approach::Loud);
        let mut h = Heist::new(def, plan);
        h.start();
        // Tick indtil done (100 progress).
        for _ in 0..1000 {
            if h.tick(1.0) {
                break;
            }
        }
        assert_eq!(h.status, HeistStatus::Success);
    }
}