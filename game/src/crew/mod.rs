#![allow(dead_code)] // Crew API er public/stub til fremtidig AI/UI.

//! Crew — spillerens allierede.
//!
//! Hver crew-medlem har personlighed: loyalitet, frygt, moralgrænser.
//! Medlemmer kan bruges i missioner (før heist/job), kan dø, forråde,
//! eller blive arresteret. Rekrutteres via dialog eller redning.

use serde::{Deserialize, Serialize};

/// Disposition/rolle for et crew-medlem.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrewRole {
    /// Alsidig, tager små jobs.
    Runner,
    /// Specialist i køretøjer, flugt, racing.
    Driver,
    /// Specialist i våben, dødbringende konflikt.
    Gunman,
    /// Specialist i stealth, lockpicking, infiltration.
    Ghost,
    /// Forhandler, kontakter, information.
    Fixer,
    /// Healer / medic.
    Medic,
}

impl CrewRole {
    pub fn label(&self) -> &'static str {
        match self {
            CrewRole::Runner => "Runner",
            CrewRole::Driver => "Driver",
            CrewRole::Gunman => "Gunman",
            CrewRole::Ghost => "Ghost",
            CrewRole::Fixer => "Fixer",
            CrewRole::Medic => "Medic",
        }
    }

    /// Hvor meget de koster at hyre (engangsbeløb, cash).
    pub fn hire_cost(&self) -> u32 {
        match self {
            CrewRole::Runner => 500,
            CrewRole::Driver => 2_000,
            CrewRole::Gunman => 2_500,
            CrewRole::Ghost => 3_000,
            CrewRole::Fixer => 4_000,
            CrewRole::Medic => 1_500,
        }
    }
}

/// Et crew-medlem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub id: String,
    pub name: String,
    pub role: CrewRole,
    /// 0-100: hvor loyalt medlemmet er. Lav = risiko for forræderi/afgang.
    pub loyalty: f32,
    /// 0-100: hvor bange medlemmet er. Høj = risiko for flugt under job.
    pub fear: f32,
    /// 0-100: hvor moralsk medlemmet er. Høj = nægter voldelige/amoralske jobs.
    pub morals: f32,
    /// Tilknyttet safehouse (hvis de er i "standby").
    pub home_safehouse: Option<String>,
    /// Klar til mission?
    pub ready: bool,
    /// Kompromitteret (kan afpresse spilleren).
    pub compromised: bool,
    /// Level (fremtidigt skill-system).
    pub level: u32,
}

impl CrewMember {
    pub fn new(id: &str, name: &str, role: CrewRole) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            role,
            loyalty: 50.0,
            fear: 20.0,
            morals: 30.0,
            home_safehouse: None,
            ready: true,
            compromised: false,
            level: 1,
        }
    }

    /// Opdatér medlem per frame: frygt falder, loyalitet kan falde hvis forhold dårligt.
    pub fn tick(&mut self, dt: f32) {
        self.fear = (self.fear - dt * 0.2).max(0.0);
        if self.loyalty < 20.0 {
            self.loyalty = (self.loyalty - dt * 0.05).max(0.0);
        }
    }

    /// Loyalitets-kategori.
    pub fn loyalty_status(&self) -> &'static str {
        if self.loyalty >= 80.0 { "Family" }
        else if self.loyalty >= 60.0 { "Loyal" }
        else if self.loyalty >= 40.0 { "Reliable" }
        else if self.loyalty >= 20.0 { "Shaky" }
        else { "Risky" }
    }

    /// Tjek om medlem vil påtage en voldelig opgave.
    pub fn will_do_violence(&self) -> bool {
        self.morals < 70.0 && self.fear < 60.0
    }

    /// Justér loyalitet (kaldes ved job-success, gaver, beskyttelse).
    pub fn adjust_loyalty(&mut self, delta: f32) {
        self.loyalty = (self.loyalty + delta).clamp(0.0, 100.0);
    }

    /// Justér frygt (kaldes ved farlige events, tab).
    pub fn adjust_fear(&mut self, delta: f32) {
        self.fear = (self.fear + delta).clamp(0.0, 100.0);
    }
}

/// Spillerens crew (alle hyrede medlemmer).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Crew {
    pub members: Vec<CrewMember>,
    pub max_size: u32,
}

impl Crew {
    pub fn new() -> Self {
        Self { members: Vec::new(), max_size: 6 }
    }

    pub fn hire(&mut self, m: CrewMember) -> bool {
        if self.members.len() as u32 >= self.max_size {
            return false;
        }
        self.members.push(m);
        true
    }

    pub fn fire(&mut self, id: &str) {
        self.members.retain(|m| m.id != id);
    }

    pub fn get(&self, id: &str) -> Option<&CrewMember> {
        self.members.iter().find(|m| m.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut CrewMember> {
        self.members.iter_mut().find(|m| m.id == id)
    }

    pub fn ready(&self) -> Vec<&CrewMember> {
        self.members.iter().filter(|m| m.ready).collect()
    }

    pub fn tick_all(&mut self, dt: f32) {
        for m in &mut self.members {
            m.tick(dt);
        }
    }

    /// Starter crew med et par standard-medlemmer (proof).
    pub fn with_starter() -> Self {
        let mut crew = Self::new();
        crew.hire(CrewMember::new("crew_vito", "Vito", CrewRole::Driver));
        crew.hire(CrewMember::new("crew_dana", "Dana", CrewRole::Ghost));
        crew
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loyal_member_does_violence_if_low_morals() {
        let m = CrewMember::new("x", "X", CrewRole::Gunman);
        assert!(m.will_do_violence());
    }

    #[test]
    fn high_morals_rejects_violence() {
        let mut m = CrewMember::new("x", "X", CrewRole::Medic);
        m.morals = 80.0;
        assert!(!m.will_do_violence());
    }
}