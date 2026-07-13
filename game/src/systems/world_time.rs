//! World time — dag/nat-cyklus for Heat City.
//!
//! Simuleret tid: 1 spil-time = ~2 minutter real-tid (24 timer = ~48 min).
//! Verdenen skifter mellem Morning, Day, Evening, Night.
//! NPC-adfærd, trafik-tæthed, politi-tilstedeværelse afhænger af tidspunkt.

use serde::{Deserialize, Serialize};

/// 1 spil-time i sekunder real-tid. 24 timer = TIME_SCALE * 24.
pub const TIME_SCALE: f32 = 120.0; // 2 min per spil-time

/// Tidsperiode af dagen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeOfDay {
    Dawn,      // 5:00 - 8:00
    Morning,   // 8:00 - 12:00
    Afternoon, // 12:00 - 17:00
    Evening,   // 17:00 - 21:00
    Night,     // 21:00 - 5:00
}

impl TimeOfDay {
    pub fn from_hour(hour: f32) -> Self {
        let h = hour.rem_euclid(24.0);
        if h >= 5.0 && h < 8.0 {
            TimeOfDay::Dawn
        } else if h >= 8.0 && h < 12.0 {
            TimeOfDay::Morning
        } else if h >= 12.0 && h < 17.0 {
            TimeOfDay::Afternoon
        } else if h >= 17.0 && h < 21.0 {
            TimeOfDay::Evening
        } else {
            TimeOfDay::Night
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            TimeOfDay::Dawn => "Dawn",
            TimeOfDay::Morning => "Morning",
            TimeOfDay::Afternoon => "Afternoon",
            TimeOfDay::Evening => "Evening",
            TimeOfDay::Night => "Night",
        }
    }

    /// Mørkefaktor 0.0 (dag) - 1.0 (nat). Bruges til rendering tint.
    pub fn darkness(&self) -> f32 {
        match self {
            TimeOfDay::Dawn => 0.3,
            TimeOfDay::Morning => 0.0,
            TimeOfDay::Afternoon => 0.0,
            TimeOfDay::Evening => 0.15,
            TimeOfDay::Night => 0.55,
        }
    }

    /// Er det nat (mere kriminalitet, færre civile)?
    pub fn is_night(&self) -> bool {
        matches!(self, TimeOfDay::Night | TimeOfDay::Dawn)
    }
}

/// Verdens-ur. Sporer spil-tid (24-timers cyklus).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WorldTime {
    /// Aktuel time 0.0-24.0.
    pub hour: f32,
    /// Dag nummer (starter ved 1).
    pub day: u32,
}

impl WorldTime {
    pub fn new() -> Self {
        Self { hour: 8.0, day: 1 } // start kl. 8:00
    }

    /// Advance tiden med dt sekunder real-tid.
    pub fn advance(&mut self, dt: f32) {
        self.hour += dt / TIME_SCALE;
        if self.hour >= 24.0 {
            self.hour -= 24.0;
            self.day += 1;
        }
    }

    pub fn time_of_day(&self) -> TimeOfDay {
        TimeOfDay::from_hour(self.hour)
    }

    /// Formateret tid "HH:MM, Dag N".
    pub fn formatted(&self) -> String {
        let h = self.hour.floor() as u32;
        let m = ((self.hour - h as f32) * 60.0) as u32;
        format!("{:02}:{:02}, Day {}", h, m, self.day)
    }
}

impl Default for WorldTime {
    fn default() -> Self {
        Self::new()
    }
}