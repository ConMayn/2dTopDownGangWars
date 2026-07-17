#![allow(dead_code)] // Rivals API er public/stub til fremtidig NPC-spawning.

//! Rivals — personlige fjender der udvikler sig.
//!
//! Fra GDD afsnit 25:
//! Rivaler kan være: bandeleder, betjent, dusørjæger, tidligere crew-medlem,
//! journalist, mafia-enforcer, gaderacer, korrupt politichef.
//! De udvikler sig: sætte dusør, angribe forretninger, tippe politi,
//! kidnappe crew, udfordre, sprede rygter, sabotere bil.
//! Rivaler kan blive allierede hvis du hjælper dem mod en større fjende.

use serde::{Deserialize, Serialize};

/// Rival-type (fra GDD 25.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RivalKind {
    GangLeader,
    Cop,
    BountyHunter,
    FormerCrew,
    Journalist,
    MafiaEnforcer,
    StreetRacer,
    CorruptChief,
}

impl RivalKind {
    pub fn label(&self) -> &'static str {
        match self {
            RivalKind::GangLeader => "Gang Leader",
            RivalKind::Cop => "Cop",
            RivalKind::BountyHunter => "Bounty Hunter",
            RivalKind::FormerCrew => "Former Crew",
            RivalKind::Journalist => "Journalist",
            RivalKind::MafiaEnforcer => "Mafia Enforcer",
            RivalKind::StreetRacer => "Street Racer",
            RivalKind::CorruptChief => "Corrupt Chief",
        }
    }
}

/// Rivalens aktuelle tilstand over for spilleren.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RivalDisposition {
    /// Aktiv fjende — vil skade spilleren.
    Hostile,
    /// Venter på hævn / planlægger.
    Plotting,
    /// Midlertidig våbenhvile.
    Truce,
    /// Kan blive allieret hvis spilleren hjælper.
    Wavering,
    /// Tidligere fjende, nu allieret.
    Ally,
}

impl RivalDisposition {
    pub fn label(&self) -> &'static str {
        match self {
            RivalDisposition::Hostile => "Hostile",
            RivalDisposition::Plotting => "Plotting",
            RivalDisposition::Truce => "Truce",
            RivalDisposition::Wavering => "Wavering",
            RivalDisposition::Ally => "Ally",
        }
    }
}

/// En rival.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rival {
    pub id: String,
    pub name: String,
    pub kind: RivalKind,
    pub faction: Option<String>,
    pub disposition: RivalDisposition,
    /// Had-niveau 0-100 (hvor meget de vil skade spilleren).
    pub grudge: f32,
    /// Respekt-niveau 0-100 (hvor meget de anerkender spilleren).
    pub respect: f32,
    /// Dusør de har sat på spilleren (0 = ingen).
    pub bounty: u32,
    /// Tællere over handlinger rivalen har udført mod spilleren.
    pub actions_taken: u32,
    /// Tællere over handlinger spilleren har udført mod rivalen.
    pub actions_received: u32,
}

impl Rival {
    pub fn new(id: &str, name: &str, kind: RivalKind) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            kind,
            faction: None,
            disposition: RivalDisposition::Hostile,
            grudge: 50.0,
            respect: 10.0,
            bounty: 0,
            actions_taken: 0,
            actions_received: 0,
        }
    }

    /// Opdatér rival per frame.
    pub fn tick(&mut self, dt: f32) {
        // Grudge falder langsomt hvis ingen nye handlinger.
        self.grudge = (self.grudge - dt * 0.02).max(0.0);
        // Respect kan stige hvis spilleren har lav profil.
        if self.grudge < 20.0 {
            self.respect = (self.respect + dt * 0.01).min(100.0);
        }
        // Opdatér disposition baseret på grudge og respect.
        self.disposition = if self.grudge > 70.0 {
            RivalDisposition::Hostile
        } else if self.grudge > 40.0 {
            RivalDisposition::Plotting
        } else if self.grudge > 15.0 && self.respect < 30.0 {
            RivalDisposition::Truce
        } else if self.respect > 50.0 {
            RivalDisposition::Wavering
        } else {
            RivalDisposition::Truce
        };
    }

    /// Spilleren ydmygede rivalen (kaldes når spilleren vinder en fight).
    pub fn on_humiliated_by_player(&mut self) {
        self.grudge = (self.grudge + 20.0).min(100.0);
        self.actions_received += 1;
        self.bounty = self.bounty.saturating_add(500);
    }

    /// Spilleren hjalp rivalen mod en større fjende.
    pub fn on_helped_by_player(&mut self) {
        self.grudge = (self.grudge - 30.0).max(0.0);
        self.respect = (self.respect + 25.0).min(100.0);
        self.actions_received += 1;
    }

    /// Rivalen udførte en handling mod spilleren.
    pub fn on_action_taken(&mut self) -> RivalAction {
        self.actions_taken += 1;
        if self.disposition == RivalDisposition::Hostile {
            if self.grudge > 80.0 && self.bounty > 1000 {
                RivalAction::AttackBusiness
            } else if self.grudge > 60.0 {
                RivalAction::TipPolice
            } else {
                RivalAction::SpreadRumor
            }
        } else if self.disposition == RivalDisposition::Plotting {
            RivalAction::KidnapCrew
        } else {
            RivalAction::None
        }
    }
}

/// Handling rivalen kan udføre mod spilleren (GDD 25.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RivalAction {
    None,
    TipPolice,
    AttackBusiness,
    KidnapCrew,
    SpreadRumor,
    SabotageVehicle,
    Challenge,
    SetBounty,
}

impl RivalAction {
    pub fn label(&self) -> &'static str {
        match self {
            RivalAction::None => "None",
            RivalAction::TipPolice => "Tip Police",
            RivalAction::AttackBusiness => "Attack Business",
            RivalAction::KidnapCrew => "Kidnap Crew",
            RivalAction::SpreadRumor => "Spread Rumor",
            RivalAction::SabotageVehicle => "Sabotage Vehicle",
            RivalAction::Challenge => "Challenge",
            RivalAction::SetBounty => "Set Bounty",
        }
    }
}

/// Rival-manager.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RivalSystem {
    pub rivals: Vec<Rival>,
    pub id_counter: u32,
}

impl RivalSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, kind: RivalKind, name: &str) -> &Rival {
        self.id_counter += 1;
        let id = format!("rival_{}", self.id_counter);
        self.rivals.push(Rival::new(&id, name, kind));
        self.rivals.last().unwrap()
    }

    pub fn get(&self, id: &str) -> Option<&Rival> {
        self.rivals.iter().find(|r| r.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Rival> {
        self.rivals.iter_mut().find(|r| r.id == id)
    }

    pub fn tick(&mut self, dt: f32) -> Vec<(String, RivalAction)> {
        let actions = Vec::new();
        for r in &mut self.rivals {
            r.tick(dt);
        }
        actions
    }

    pub fn hostile_count(&self) -> usize {
        self.rivals.iter().filter(|r| r.disposition == RivalDisposition::Hostile).count()
    }

    pub fn total_bounty(&self) -> u64 {
        self.rivals.iter().map(|r| r.bounty as u64).sum()
    }
}