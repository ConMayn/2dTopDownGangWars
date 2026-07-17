//! Heat — wanted-level system (Heat 1-6).
//!
//! Fra GDD afsnit 6:
//! Heat 1: Mistanke — patruljer holder øje.
//! Heat 2: Lokal jagt — nærliggende patruljer leder, sirener.
//! Heat 3: Aktiv eftersøgning — flere biler, helikopter, kender bil-type.
//! Heat 4: Task force — særlige enheder, spike strips, koordinering.
//! Heat 5: Lockdown — zonen lukkes, checkpoints, civil trafik ændres.
//! Heat 6: Manhunt — specialstyrker, dusørjægere, nyhedshelikoptere.

use serde::{Deserialize, Serialize};

/// Heat-level 0 (ingen) til 6 (manhunt).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HeatLevel {
    None = 0,
    Suspicion = 1,
    LocalPursuit = 2,
    ActiveSearch = 3,
    TaskForce = 4,
    Lockdown = 5,
    Manhunt = 6,
}

impl HeatLevel {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => HeatLevel::None,
            1 => HeatLevel::Suspicion,
            2 => HeatLevel::LocalPursuit,
            3 => HeatLevel::ActiveSearch,
            4 => HeatLevel::TaskForce,
            5 => HeatLevel::Lockdown,
            _ => HeatLevel::Manhunt,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn label(&self) -> &'static str {
        match self {
            HeatLevel::None => "Clean",
            HeatLevel::Suspicion => "Suspicion",
            HeatLevel::LocalPursuit => "Local Pursuit",
            HeatLevel::ActiveSearch => "Active Search",
            HeatLevel::TaskForce => "Task Force",
            HeatLevel::Lockdown => "Lockdown",
            HeatLevel::Manhunt => "Manhunt",
        }
    }

    /// Hvor mange politi-enheder der reagerer ved dette heat-level.
    pub fn response_units(&self) -> u32 {
        match self {
            HeatLevel::None => 0,
            HeatLevel::Suspicion => 1,
            HeatLevel::LocalPursuit => 2,
            HeatLevel::ActiveSearch => 4,
            HeatLevel::TaskForce => 6,
            HeatLevel::Lockdown => 8,
            HeatLevel::Manhunt => 12,
        }
    }

    /// Er helikopter deployeret?
    pub fn has_helicopter(&self) -> bool {
        matches!(self, HeatLevel::ActiveSearch | HeatLevel::TaskForce | HeatLevel::Lockdown | HeatLevel::Manhunt)
    }

    /// Er roadblocks aktive?
    pub fn has_roadblocks(&self) -> bool {
        matches!(self, HeatLevel::TaskForce | HeatLevel::Lockdown | HeatLevel::Manhunt)
    }

    /// Er checkpoints aktive (zone lockdown)?
    pub fn has_checkpoints(&self) -> bool {
        matches!(self, HeatLevel::Lockdown | HeatLevel::Manhunt)
    }

    /// Hvor hurtigt heat falder (per sekund) når spilleren er "out of sight".
    pub fn decay_rate(&self) -> f32 {
        match self {
            HeatLevel::None => 0.0,
            HeatLevel::Suspicion => 2.0,
            HeatLevel::LocalPursuit => 1.0,
            HeatLevel::ActiveSearch => 0.5,
            HeatLevel::TaskForce => 0.3,
            HeatLevel::Lockdown => 0.15,
            HeatLevel::Manhunt => 0.08,
        }
    }
}

/// Wanted-state for en zone: heat-level + heat points (akkumuleret).
/// Heat points stiger ved kriminalitet, falder ved flugt/skjul.
/// Heat-level afledes fra heat points + evidence.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WantedState {
    /// Akkumulerede heat points (0-100+).
    pub heat_points: f32,
    /// Aktuel heat-level (afledt, men cached).
    pub level: HeatLevel,
    /// Er spilleren "in sight" af politi? (stopper decay).
    pub in_sight: bool,
    /// Tidspunkt spilleren sidst blev set (sim_time).
    pub last_seen_time: f32,
    /// Sidst kendte position (x, y) — bruger [f32; 2] fordi heat_core::Vec2 ikke har serde-støtte.
    pub last_seen_pos: [f32; 2],
}

impl Default for WantedState {
    fn default() -> Self {
        Self {
            heat_points: 0.0,
            level: HeatLevel::None,
            in_sight: false,
            last_seen_time: -100.0,
            last_seen_pos: [0.0, 0.0],
        }
    }
}

impl WantedState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tilføj heat points (ved kriminalitet).
    pub fn add_heat(&mut self, points: f32, sim_time: f32, player_pos: heat_core::Vec2) {
        self.heat_points = (self.heat_points + points).max(0.0);
        self.last_seen_time = sim_time;
        self.last_seen_pos = [player_pos.x, player_pos.y];
        self.recompute_level();
    }

    /// Opdatér heat: decay hvis ikke in_sight, recompute level.
    pub fn update(&mut self, dt: f32, sim_time: f32, player_pos: heat_core::Vec2) {
        if !self.in_sight && self.heat_points > 0.0 {
            self.heat_points = (self.heat_points - self.level.decay_rate() * dt).max(0.0);
            self.recompute_level();
        }
        if self.in_sight {
            self.last_seen_time = sim_time;
            self.last_seen_pos = [player_pos.x, player_pos.y];
        }
    }

    /// Beregn heat-level fra heat points.
    fn recompute_level(&mut self) {
        let new_level = if self.heat_points < 1.0 {
            HeatLevel::None
        } else if self.heat_points < 15.0 {
            HeatLevel::Suspicion
        } else if self.heat_points < 30.0 {
            HeatLevel::LocalPursuit
        } else if self.heat_points < 50.0 {
            HeatLevel::ActiveSearch
        } else if self.heat_points < 70.0 {
            HeatLevel::TaskForce
        } else if self.heat_points < 90.0 {
            HeatLevel::Lockdown
        } else {
            HeatLevel::Manhunt
        };
        if new_level != self.level {
            tracing::info!("Heat level: {} → {}", self.level.label(), new_level.label());
            self.level = new_level;
        }
    }

    /// Sæt in_sight flag (fra politi-AI perception).
    pub fn set_sight(&mut self, in_sight: bool, sim_time: f32, player_pos: heat_core::Vec2) {
        self.in_sight = in_sight;
        if in_sight {
            self.last_seen_time = sim_time;
            self.last_seen_pos = [player_pos.x, player_pos.y];
        }
    }

    /// Reducér heat (f.eks. korrupt betjent, skift tøj, respray).
    pub fn reduce_heat(&mut self, points: f32) {
        self.heat_points = (self.heat_points - points).max(0.0);
        self.recompute_level();
    }

    /// Slet alt heat (korrupt betjent full wipe).
    pub fn clear(&mut self) {
        self.heat_points = 0.0;
        self.level = HeatLevel::None;
        self.in_sight = false;
    }

    /// Hvor længe siden spilleren sidst blev set?
    pub fn time_since_seen(&self, sim_time: f32) -> f32 {
        (sim_time - self.last_seen_time).max(0.0)
    }
}

/// Kriminalitetshandlinger og deres heat-point værdier.
#[derive(Debug, Clone, Copy)]
pub enum CrimeType {
    Speeding,           // 2 points
    RecklessDriving,    // 5
    HitAndRun,          // 15
    Assault,            // 10
    WeaponDischarge,    // 12
    Murder,             // 30
    CarTheft,           // 5
    Robbery,            // 15
    CopKilled,          // 50
    MassCasualty,       // 80
}

impl CrimeType {
    pub fn heat_points(&self) -> f32 {
        match self {
            CrimeType::Speeding => 2.0,
            CrimeType::RecklessDriving => 5.0,
            CrimeType::HitAndRun => 15.0,
            CrimeType::Assault => 10.0,
            CrimeType::WeaponDischarge => 12.0,
            CrimeType::Murder => 30.0,
            CrimeType::CarTheft => 5.0,
            CrimeType::Robbery => 15.0,
            CrimeType::CopKilled => 50.0,
            CrimeType::MassCasualty => 80.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            CrimeType::Speeding => "Speeding",
            CrimeType::RecklessDriving => "Reckless Driving",
            CrimeType::HitAndRun => "Hit and Run",
            CrimeType::Assault => "Assault",
            CrimeType::WeaponDischarge => "Weapon Discharge",
            CrimeType::Murder => "Murder",
            CrimeType::CarTheft => "Car Theft",
            CrimeType::Robbery => "Robbery",
            CrimeType::CopKilled => "Cop Killed",
            CrimeType::MassCasualty => "Mass Casualty",
        }
    }
}