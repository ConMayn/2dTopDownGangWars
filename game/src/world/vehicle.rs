#![allow(dead_code)] // Public API til fremtidigt damage/hotwire system.

//! Vehicle — biler, fysik, stjæl/ind/udstigning.
//!
//! Fase 3: arcade bil-fysik (forward, turn, drift), hotwire, ind/udstigning,
//! collision mod tilemap. 3-5 bil-typer defineret data-drevet.

use serde::{Deserialize, Serialize};

use heat_core::Vec2;

/// Bil-type definition (data-drevet, klar til RON).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleDef {
    pub id: String,
    pub name: String,
    /// Pixel dimensioner.
    pub width: f32,
    pub height: f32,
    /// Topfart (px/s).
    pub max_speed: f32,
    /// Acceleration (px/s²).
    pub accel: f32,
    /// Bremsning (px/s²).
    pub brake: f32,
    /// Turning rate (radians/s ved fuld speed).
    pub turn_rate: f32,
    /// Friction når ikke der gasses (px/s²).
    pub friction: f32,
    /// Drift-factor: 0 = ingen drift, 1 = fuld drift (baghjulene slipper).
    pub drift: f32,
    /// Farve tint.
    pub color: [f32; 4],
    /// Hvor lang tid det tager at hotwire (sekunder).
    pub hotwire_time: f32,
    /// Hvor meget health bilen har.
    pub max_health: f32,
}

impl VehicleDef {
    /// Compact car — langsom, let at stjæle.
    pub fn compact() -> Self {
        Self {
            id: "compact".into(),
            name: "Compact".into(),
            width: 48.0,
            height: 28.0,
            max_speed: 280.0,
            accel: 180.0,
            brake: 300.0,
            turn_rate: 3.2,
            friction: 80.0,
            drift: 0.15,
            color: [0.5, 0.5, 0.55, 1.0],
            hotwire_time: 1.5,
            max_health: 100.0,
        }
    }

    /// Muscle car — hurtig, sværere at kontrollere.
    pub fn muscle() -> Self {
        Self {
            id: "muscle".into(),
            name: "Muscle".into(),
            width: 52.0,
            height: 30.0,
            max_speed: 420.0,
            accel: 250.0,
            brake: 350.0,
            turn_rate: 2.6,
            friction: 60.0,
            drift: 0.35,
            color: [0.7, 0.2, 0.2, 1.0],
            hotwire_time: 2.5,
            max_health: 120.0,
        }
    }

    /// Van — langsom, holdbar.
    pub fn van() -> Self {
        Self {
            id: "van".into(),
            name: "Van".into(),
            width: 64.0,
            height: 36.0,
            max_speed: 240.0,
            accel: 140.0,
            brake: 250.0,
            turn_rate: 2.2,
            friction: 90.0,
            drift: 0.08,
            color: [0.3, 0.35, 0.4, 1.0],
            hotwire_time: 2.0,
            max_health: 160.0,
        }
    }

    /// Sports car — meget hurtig, meget drift.
    pub fn sports() -> Self {
        Self {
            id: "sports".into(),
            name: "Sports".into(),
            width: 50.0,
            height: 28.0,
            max_speed: 500.0,
            accel: 320.0,
            brake: 400.0,
            turn_rate: 3.0,
            friction: 50.0,
            drift: 0.45,
            color: [0.9, 0.8, 0.2, 1.0],
            hotwire_time: 3.0,
            max_health: 90.0,
        }
    }

    /// Truck — langsom, meget holdbar.
    pub fn truck() -> Self {
        Self {
            id: "truck".into(),
            name: "Truck".into(),
            width: 72.0,
            height: 40.0,
            max_speed: 200.0,
            accel: 100.0,
            brake: 200.0,
            turn_rate: 1.8,
            friction: 100.0,
            drift: 0.05,
            color: [0.4, 0.3, 0.2, 1.0],
            hotwire_time: 3.5,
            max_health: 200.0,
        }
    }

    /// Hent alle default bil-typer.
    pub fn all_defaults() -> Vec<VehicleDef> {
        vec![
            Self::compact(),
            Self::muscle(),
            Self::van(),
            Self::sports(),
            Self::truck(),
        ]
    }
}

/// Vehicle-komponent: position, heading, velocity, data.
/// `driver` = Some(player_entity) når spilleren kører bilen.
#[derive(Debug, Clone)]
pub struct Vehicle {
    pub pos: Vec2,
    /// Heading i radians (0 = pegende op/nord).
    pub heading: f32,
    /// Hastigheds-vektor (px/s) i world-space.
    pub vel: Vec2,
    /// Hvilken bil-type (index ind i VehicleRegistry).
    pub def_id: String,
    /// Health.
    pub health: f32,
    /// Er en spiller i bilen? (entity ID hvis ja).
    pub driver: Option<hecs::Entity>,
    /// Hotwire-timer: > 0 = under hotwiring.
    pub hotwire_timer: f32,
    /// Er bilen stjålet? (vs ejet/legitimt).
    pub stolen: bool,
}

/// Registry af vehicle-typer.
#[derive(Debug, Clone, Default)]
pub struct VehicleRegistry {
    defs: std::collections::HashMap<String, VehicleDef>,
}

impl VehicleRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_defaults() -> Self {
        let mut reg = Self::default();
        for def in VehicleDef::all_defaults() {
            reg.defs.insert(def.id.clone(), def);
        }
        reg
    }

    pub fn get(&self, id: &str) -> Option<&VehicleDef> {
        self.defs.get(id)
    }

    pub fn defs(&self) -> impl Iterator<Item = &VehicleDef> {
        self.defs.values()
    }
}

/// Opdatér bil-fysik: acceleration, turning, drift, collision.
/// Returnerer ny position og heading. Kaldes når spiller kører bilen.
pub fn update_vehicle_physics(
    vehicle: &mut Vehicle,
    def: &VehicleDef,
    input_accel: f32,  // -1..1 (W/S)
    input_steer: f32,  // -1..1 (A/D)
    handbrake: bool,
    dt: f32,
) {
    // 1. Acceleration / bremsing langs heading.
    let forward_dir = Vec2::new(vehicle.heading.sin(), -vehicle.heading.cos());
    let speed = vehicle.vel.length();
    let forward_speed = vehicle.vel.dot(forward_dir);

    if input_accel > 0.0 {
        // Gas: øg hastighed langs heading.
        let new_speed = (forward_speed + def.accel * input_accel * dt).min(def.max_speed);
        vehicle.vel = forward_dir * new_speed;
    } else if input_accel < 0.0 {
        // Brems/bak.
        if forward_speed > 0.0 {
            // Brems.
            let new_speed = (forward_speed - def.brake * dt).max(0.0);
            vehicle.vel = forward_dir * new_speed;
        } else {
            // Bak (halv max speed).
            let new_speed = (forward_speed + def.accel * input_accel * dt).max(-def.max_speed * 0.4);
            vehicle.vel = forward_dir * new_speed;
        }
    } else {
        // Ingen gas: friction.
        let friction = if handbrake { def.brake } else { def.friction };
        let new_speed = if forward_speed > 0.0 {
            (forward_speed - friction * dt).max(0.0)
        } else {
            (forward_speed + friction * dt).min(0.0)
        };
        vehicle.vel = forward_dir * new_speed;
    }

    // 2. Turning — hastigheds-afhængig (kan ikke dreje når stillestående).
    let speed_factor = (speed / def.max_speed).clamp(0.0, 1.0).sqrt();
    let reverse_factor = if forward_speed < 0.0 { -1.0 } else { 1.0 };
    vehicle.heading += input_steer * def.turn_rate * speed_factor * reverse_factor * dt;

    // 3. Drift: bøj velocity mod heading (gradual alignment).
    let target_dir = Vec2::new(vehicle.heading.sin(), -vehicle.heading.cos());
    let drift_lerp = 1.0 - def.drift * (1.0 - speed_factor);
    let new_vel = vehicle.vel.lerp(target_dir * vehicle.vel.length(), 1.0 - drift_lerp.min(1.0));
    vehicle.vel = new_vel;

    // 4. Position update.
    vehicle.pos += vehicle.vel * dt;
}

/// Beregn vehicle collision mod tilemap. Flyt bilen tilbage hvis den rammer solid tile.
pub fn collide_vehicle_with_tilemap(
    vehicle: &mut Vehicle,
    def: &VehicleDef,
    tilemap: &super::tilemap::Tilemap,
    registry: &super::tiles::TileRegistry,
) {
    let ts = tilemap.tile_size;
    let half_w = def.width * 0.5;
    let half_h = def.height * 0.5;

    // Simpel AABB collision (ikke rotated — til Fase 3 er dette acceptable).
    let box_ = heat_core::Aabb::from_center(vehicle.pos, Vec2::new(half_w, half_h));
    let min_tx = (box_.min.x / ts).floor() as i32;
    let max_tx = (box_.max.x / ts).floor() as i32;
    let min_ty = (box_.min.y / ts).floor() as i32;
    let max_ty = (box_.max.y / ts).floor() as i32;

    let mut hit = false;
    let mut push_back = Vec2::ZERO;

    for ty in min_ty..=max_ty {
        for tx in min_tx..=max_tx {
            if tilemap.is_solid(tx, ty, registry) {
                let tile_min = Vec2::new(tx as f32 * ts, ty as f32 * ts);
                let tile_max = Vec2::new((tx + 1) as f32 * ts, (ty + 1) as f32 * ts);
                let tile_box = heat_core::Aabb::new(tile_min, tile_max);
                if box_.intersects(&tile_box) {
                    hit = true;
                    // Find korteste push-back retning.
                    let center = box_.center();
                    let tile_center = tile_box.center();
                    let diff = center - tile_center;
                    let overlap_x = half_w + ts * 0.5 - diff.x.abs();
                    let overlap_y = half_h + ts * 0.5 - diff.y.abs();
                    if overlap_x < overlap_y {
                        push_back.x = overlap_x * diff.x.signum();
                    } else {
                        push_back.y = overlap_y * diff.y.signum();
                    }
                }
            }
        }
    }

    if hit {
        vehicle.pos += push_back;
        // Reducer hastighed ved collision.
        vehicle.vel *= 0.3;
    }
}