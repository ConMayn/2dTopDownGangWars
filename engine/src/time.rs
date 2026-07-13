//! Time — fixed timestep tidsløkke.
//!
//! Simulering kører med faste 60 Hz updates (FIXED_DT).
//! Rendering interpolerer mellem sim-tilstande for smooth visuals.
//! Se TDD afsnit 6.

use std::time::Instant;

/// Fast simulation timestep. 60 Hz.
pub const FIXED_DT: f32 = 1.0 / 60.0;

/// Maksimal akkumulator. Forebygger "spiral of death" hvis simuleringen
/// ikke kan nå 60 Hz — vi dropper sim-trin frem for at falde bagud.
const MAX_ACCUMULATOR: f32 = 0.25;

/// Tids-håndtering for main loop.
pub struct Time {
    last_frame: Instant,
    accumulator: f32,
    /// Real frame delta (for UI/animations der ikke er simuleret).
    pub frame_delta: f32,
    /// Antal fixed-update trin der skal køres denne frame.
    pub sim_steps: u32,
    /// Interpolationsfaktor 0.0-1.0 mellem sim-trin (til render).
    pub alpha: f32,
    /// Total sim-tid i sekunder (kun fixed updates tæller).
    pub sim_time: f32,
    /// Total real-tid i sekunder.
    pub real_time: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            accumulator: 0.0,
            frame_delta: 0.0,
            sim_steps: 0,
            alpha: 0.0,
            sim_time: 0.0,
            real_time: 0.0,
        }
    }

    /// Kaldes i starten af hver frame. Måler real delta, opdaterer akkumulator.
    /// Returnerer antal fixed-update trin der skal køres.
    pub fn tick(&mut self) -> u32 {
        let now = Instant::now();
        let real_delta = now.duration_since(self.last_frame).as_secs_f32();
        self.last_frame = now;
        self.frame_delta = real_delta;
        self.real_time += real_delta;

        self.accumulator += real_delta.min(MAX_ACCUMULATOR);
        self.sim_steps = (self.accumulator / FIXED_DT) as u32;
        self.sim_steps
    }

    /// Kaldes efter hvert fixed-update trin. Trækker FIXED_DT fra akkumulator.
    pub fn step_done(&mut self) {
        self.accumulator -= FIXED_DT;
        self.sim_time += FIXED_DT;
    }

    /// Beregn interpolationsfaktor efter alle sim-trin er kørt.
    pub fn calc_alpha(&mut self) {
        self.alpha = self.accumulator / FIXED_DT;
    }

    pub fn fixed_dt(&self) -> f32 {
        FIXED_DT
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}