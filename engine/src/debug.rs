//! Debug — runtime debug-overlay (F1: FPS, entity count).
//!
//! Fase 1: simpel text-baseret overlay der skrives til log og senere
//! tegnes på skærmen. For nu: struktur til FPS-måling.

use std::time::{Duration, Instant};

/// FPS tæller — glidende gennemsnit over 500ms.
pub struct FpsCounter {
    samples: Vec<Instant>,
    window: Duration,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            samples: Vec::with_capacity(120),
            window: Duration::from_millis(500),
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        self.samples.push(now);
        let cutoff = now - self.window;
        self.samples.retain(|t| *t >= cutoff);
    }

    pub fn fps(&self) -> f32 {
        if self.samples.len() < 2 {
            return 0.0;
        }
        let elapsed = self.samples.last().unwrap().duration_since(*self.samples.first().unwrap());
        if elapsed.as_secs_f32() > 0.0 {
            self.samples.len() as f32 / elapsed.as_secs_f32()
        } else {
            0.0
        }
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug overlay state — samler info der kan vises (F1 toggle).
#[derive(Debug, Clone, Default)]
pub struct DebugOverlay {
    pub enabled: bool,
    pub fps: f32,
    pub entity_count: usize,
    pub sim_time: f32,
    pub frame_delta: f32,
}

impl DebugOverlay {
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    /// Formateret debug-streng (til log eller tekst-rendering).
    pub fn text(&self) -> String {
        if !self.enabled {
            return String::new();
        }
        format!(
            "FPS: {:.0} | Entities: {} | Sim: {:.1}s | Frame: {:.1}ms",
            self.fps,
            self.entity_count,
            self.sim_time,
            self.frame_delta * 1000.0
        )
    }
}