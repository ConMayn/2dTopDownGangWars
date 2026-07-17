#![allow(dead_code)] // Audio API er public/stub; kira integration i Fase 11+.

//! Audio — lyd-system (SFX, ambient, music, radio).
//!
//! Fase 11 stub: API klar, men ingen faktisk afspilning endnu.
//! Fremtid: kira integration til looping ambient, musik + SFX.
//! Radio-system: musikstationer, talk radio, scanner, nyheder (GDD 16.1).

use serde::{Deserialize, Serialize};

/// Lyd-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoundKind {
    Sfx,        // kort effekt (skud, bilstart, kollision)
    Ambient,    // baggrunds-stemning
    Music,      // musiktrack
    Radio,      // bilradio
    Voice,      // dialog/voice-over
    Ui,         // UI-klik
}

impl SoundKind {
    pub fn label(&self) -> &'static str {
        match self {
            SoundKind::Sfx => "SFX",
            SoundKind::Ambient => "Ambient",
            SoundKind::Music => "Music",
            SoundKind::Radio => "Radio",
            SoundKind::Voice => "Voice",
            SoundKind::Ui => "UI",
        }
    }
}

/// En lyd der kan afspilles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub id: String,
    pub kind: SoundKind,
    pub path: String,
    /// Volume 0.0-1.0.
    pub volume: f32,
    /// Loop?
    pub looping: bool,
    /// 3D position (None = UI/2D lyd).
    pub pos: Option<[f32; 2]>,
}

impl Sound {
    pub fn new(id: &str, kind: SoundKind, path: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            path: path.to_string(),
            volume: 1.0,
            looping: matches!(kind, SoundKind::Ambient | SoundKind::Music | SoundKind::Radio),
            pos: None,
        }
    }
}

/// Radio-station.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioStation {
    pub id: String,
    pub name: String,
    pub genre: String,
    /// Track-liste (paths).
    pub tracks: Vec<String>,
    pub current_track: usize,
}

impl RadioStation {
    pub fn new(id: &str, name: &str, genre: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            genre: genre.to_string(),
            tracks: Vec::new(),
            current_track: 0,
        }
    }

    pub fn next_track(&mut self) {
        if !self.tracks.is_empty() {
            self.current_track = (self.current_track + 1) % self.tracks.len();
        }
    }
}

/// Audio-system state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioSystem {
    /// Master volume 0.0-1.0.
    pub master_volume: f32,
    /// SFX volume.
    pub sfx_volume: f32,
    /// Music volume.
    pub music_volume: f32,
    /// Radio volume.
    pub radio_volume: f32,
    /// Voice volume.
    pub voice_volume: f32,
    /// Aktive radio-station (hvis i bil).
    pub active_radio: Option<String>,
    /// Tilgængelige stationer.
    pub stations: Vec<RadioStation>,
    /// Muted?
    pub muted: bool,
    /// Afspilnings-kø (id, kind).
    pub play_queue: Vec<(String, SoundKind)>,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            master_volume: 0.8,
            sfx_volume: 0.8,
            music_volume: 0.6,
            radio_volume: 0.5,
            voice_volume: 0.9,
            active_radio: None,
            stations: Vec::new(),
            muted: false,
            play_queue: Vec::new(),
        }
    }

    pub fn play_sfx(&mut self, id: &str) {
        self.play_queue.push((id.to_string(), SoundKind::Sfx));
    }

    pub fn play_ui(&mut self, id: &str) {
        self.play_queue.push((id.to_string(), SoundKind::Ui));
    }

    pub fn play_voice(&mut self, id: &str) {
        self.play_queue.push((id.to_string(), SoundKind::Voice));
    }

    pub fn set_radio(&mut self, station_id: &str) {
        self.active_radio = Some(station_id.to_string());
    }

    pub fn stop_radio(&mut self) {
        self.active_radio = None;
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn add_station(&mut self, station: RadioStation) {
        self.stations.push(station);
    }

    /// Opdatér audio-system; tøm play-kø (simulerer afspilning).
    pub fn tick(&mut self) {
        self.play_queue.clear();
    }

    pub fn effective_volume(&self, kind: SoundKind) -> f32 {
        if self.muted {
            return 0.0;
        }
        let channel = match kind {
            SoundKind::Sfx | SoundKind::Ui => self.sfx_volume,
            SoundKind::Ambient | SoundKind::Music => self.music_volume,
            SoundKind::Radio => self.radio_volume,
            SoundKind::Voice => self.voice_volume,
        };
        self.master_volume * channel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mute_zeroes_volume() {
        let mut a = AudioSystem::new();
        a.toggle_mute();
        assert_eq!(a.effective_volume(SoundKind::Sfx), 0.0);
    }

    #[test]
    fn effective_volume_combines_master_and_channel() {
        let mut a = AudioSystem::new();
        a.master_volume = 0.5;
        a.sfx_volume = 0.4;
        assert!((a.effective_volume(SoundKind::Sfx) - 0.2).abs() < 0.01);
    }
}