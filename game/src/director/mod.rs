#![allow(dead_code)] // Director API er public/stub til fremtidig tuning.

//! AI Director — skjult system der styrer spænding og drama.
//!
//! Fra GDD afsnit 15:
//! Overvåger: ro-længde, heat, vrede factions, penge-mangel, gentagne ruter,
//! gentagne strategier, byens tomhed.
//! Kan trigge: politi-spot, rival drive-by, kontakt-opkald, butik-røveri,
//! street race, NPC-genkendelse, informant-tilbud, bande-angreb, bil-tyveri,
//! safehouse-overvågning, nyhedsrapport.
//!
//! Princip: Byen føles levende uden at blive irriterende.

use serde::{Deserialize, Serialize};

/// Spændings-niveau director forsøger at holde.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TensionLevel {
    /// Meget stille — director vil snart trigge noget.
    Calm,
    /// Normal aktivitet.
    Medium,
    /// Høj spænding — director holder lav profil.
    High,
    /// Kaos — director lader ting køre ud.
    Chaos,
}

impl TensionLevel {
    pub fn label(&self) -> &'static str {
        match self {
            TensionLevel::Calm => "Calm",
            TensionLevel::Medium => "Medium",
            TensionLevel::High => "High",
            TensionLevel::Chaos => "Chaos",
        }
    }
}

/// Director-state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorState {
    /// Tid siden sidste event (sim-sek).
    pub time_since_event: f32,
    /// Tid spilleren har haft ro (ingen heat over 0).
    pub calm_time: f32,
    /// Nuværende target tension.
    pub tension: TensionLevel,
    /// Heat budget: hvor meget heat director må generere per tidsenhed.
    pub heat_budget: f32,
    /// Cooldown før næste event kan trigge.
    pub event_cooldown: f32,
    /// Total events triggered (statistik).
    pub events_triggered: u32,
    /// Total events suppressed (director holdt sig i ro).
    pub events_suppressed: u32,
}

impl DirectorState {
    pub fn new() -> Self {
        Self {
            time_since_event: 0.0,
            calm_time: 0.0,
            tension: TensionLevel::Medium,
            heat_budget: 50.0,
            event_cooldown: 30.0,
            events_triggered: 0,
            events_suppressed: 0,
        }
    }

    /// Opdatér director state.
    /// `current_heat`: aktuel heat level (0-100).
    /// `player_cash`: spillerens cash (til penge-mangel check).
    /// `dt`: sim-delta.
    pub fn update(&mut self, current_heat: f32, player_cash: u32, dt: f32) {
        self.time_since_event += dt;
        self.event_cooldown = (self.event_cooldown - dt).max(0.0);

        // Ro-tid: akkumuleres når heat er lav.
        if current_heat < 5.0 {
            self.calm_time += dt;
        } else {
            self.calm_time = 0.0;
        }

        // Bestem target tension baseret på heat og ro-tid.
        let new_tension = if current_heat > 60.0 {
            TensionLevel::Chaos
        } else if current_heat > 30.0 {
            TensionLevel::High
        } else if self.calm_time > 120.0 {
            // Mere end 2 min ro → director vil skabe drama.
            TensionLevel::Calm
        } else {
            TensionLevel::Medium
        };

        if new_tension != self.tension {
            tracing::debug!(
                "Director tension: {} → {} (calm {:.0}s, heat {:.0})",
                self.tension.label(),
                new_tension.label(),
                self.calm_time,
                current_heat,
            );
            self.tension = new_tension;
        }

        // Heat budget regenereres langsomt.
        self.heat_budget = (self.heat_budget + dt * 2.0).min(100.0);

        // Hvis spilleren mangler penge, director vil hellere trigge penge-relaterede events.
        let _ = player_cash;
    }

    /// Skal director trigge et event nu?
    /// Returnerer event-type hvis ja, None hvis ej.
    pub fn should_trigger(&self) -> Option<DirectorEvent> {
        if self.event_cooldown > 0.0 {
            return None;
        }
        match self.tension {
            TensionLevel::Calm => {
                // Lang ro → director vil skabe drama.
                if self.time_since_event > 60.0 {
                    Some(DirectorEvent::RandomStreetEvent)
                } else {
                    None
                }
            }
            TensionLevel::Medium => {
                // Periodisk activity.
                if self.time_since_event > 90.0 {
                    Some(DirectorEvent::AmbientFlavor)
                } else {
                    None
                }
            }
            TensionLevel::High => {
                // Høj spænding — lad ting køre, trigget sjældent.
                if self.time_since_event > 180.0 {
                    Some(DirectorEvent::PolicePressure)
                } else {
                    None
                }
            }
            TensionLevel::Chaos => {
                // Kaos — director trækker sig.
                None
            }
        }
    }

    /// Notér at et event blev triggeret.
    pub fn on_event_triggered(&mut self) {
        self.time_since_event = 0.0;
        self.event_cooldown = match self.tension {
            TensionLevel::Calm => 45.0,
            TensionLevel::Medium => 60.0,
            TensionLevel::High => 120.0,
            TensionLevel::Chaos => 180.0,
        };
        self.events_triggered += 1;
    }

    /// Notér at director undertrykte et event.
    pub fn on_event_suppressed(&mut self) {
        self.events_suppressed += 1;
    }
}

/// Director-triggede event-typer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectorEvent {
    /// Random gade-event (bande-skænderi, bilulykke, røveri).
    RandomStreetEvent,
    /// Ambient flavor (demonstration, konvoj, begravelse).
    AmbientFlavor,
    /// Politi-tryk (patrulje rykker ind, checkpoint).
    PolicePressure,
    /// Rival-angreb.
    RivalAttack,
    /// Kontakt-opkald.
    ContactCall,
    /// Nyhedsrapport om spilleren.
    NewsReport,
}

impl DirectorEvent {
    pub fn label(&self) -> &'static str {
        match self {
            DirectorEvent::RandomStreetEvent => "Random Street Event",
            DirectorEvent::AmbientFlavor => "Ambient Flavor",
            DirectorEvent::PolicePressure => "Police Pressure",
            DirectorEvent::RivalAttack => "Rival Attack",
            DirectorEvent::ContactCall => "Contact Call",
            DirectorEvent::NewsReport => "News Report",
        }
    }
}