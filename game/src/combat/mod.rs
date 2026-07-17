#![allow(dead_code)] // Combat API er public; integration er WIP.

//! Combat — våben, projectiles, health, damage.
//!
//! Fase 12: simpel top-down combat.
//! - Våben-typer med damage, range, fire rate, spread.
//! - Projectiles der bevæger sig og collide med entities/tiles.
//! - Health på entities (player, NPC, politi).
//! - Visuelle effekter: muzzle flash, blod-partikler.

use serde::{Deserialize, Serialize};
use heat_core::Vec2;

/// Våben-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    Pistol,
    Revolver,
    Shotgun,
    Smg,
    Rifle,
    Sniper,
    Melee,
}

impl WeaponKind {
    pub fn label(&self) -> &'static str {
        match self {
            WeaponKind::Pistol => "Pistol",
            WeaponKind::Revolver => "Revolver",
            WeaponKind::Shotgun => "Shotgun",
            WeaponKind::Smg => "SMG",
            WeaponKind::Rifle => "Rifle",
            WeaponKind::Sniper => "Sniper",
            WeaponKind::Melee => "Melee",
        }
    }

    /// Skade per hit.
    pub fn damage(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 25.0,
            WeaponKind::Revolver => 35.0,
            WeaponKind::Shotgun => 15.0, // per pellet
            WeaponKind::Smg => 18.0,
            WeaponKind::Rifle => 40.0,
            WeaponKind::Sniper => 80.0,
            WeaponKind::Melee => 30.0,
        }
    }

    /// Ildhastighed (skud per sekund).
    pub fn fire_rate(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 4.0,
            WeaponKind::Revolver => 2.5,
            WeaponKind::Shotgun => 1.5,
            WeaponKind::Smg => 10.0,
            WeaponKind::Rifle => 6.0,
            WeaponKind::Sniper => 1.0,
            WeaponKind::Melee => 3.0,
        }
    }

    /// Projectile hastighed (px/s).
    pub fn projectile_speed(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 600.0,
            WeaponKind::Revolver => 700.0,
            WeaponKind::Shotgun => 500.0,
            WeaponKind::Smg => 650.0,
            WeaponKind::Rifle => 900.0,
            WeaponKind::Sniper => 1200.0,
            WeaponKind::Melee => 0.0, // melee = ingen projectile
        }
    }

    /// Antal pellets per skud (shotgun = flere).
    pub fn pellets(&self) -> u32 {
        match self {
            WeaponKind::Shotgun => 6,
            _ => 1,
        }
    }

    /// Spread i radians (præcision; 0 = perfekt).
    pub fn spread(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 0.05,
            WeaponKind::Revolver => 0.03,
            WeaponKind::Shotgun => 0.15,
            WeaponKind::Smg => 0.08,
            WeaponKind::Rifle => 0.02,
            WeaponKind::Sniper => 0.005,
            WeaponKind::Melee => 0.0,
        }
    }

    /// Maksimal range (px).
    pub fn range(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 400.0,
            WeaponKind::Revolver => 500.0,
            WeaponKind::Shotgun => 250.0,
            WeaponKind::Smg => 450.0,
            WeaponKind::Rifle => 700.0,
            WeaponKind::Sniper => 1000.0,
            WeaponKind::Melee => 40.0,
        }
    }

    /// Er det melee (nærkamp)?
    pub fn is_melee(&self) -> bool {
        matches!(self, WeaponKind::Melee)
    }

    /// Heat genereret per skud.
    pub fn heat_per_shot(&self) -> f32 {
        match self {
            WeaponKind::Pistol => 3.0,
            WeaponKind::Revolver => 4.0,
            WeaponKind::Shotgun => 6.0,
            WeaponKind::Smg => 2.0,
            WeaponKind::Rifle => 5.0,
            WeaponKind::Sniper => 8.0,
            WeaponKind::Melee => 1.0,
        }
    }
}

/// Et projectile i verden.
#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub pos: Vec2,
    pub vel: Vec2,
    pub damage: f32,
    /// Tilbageværende range (px).
    pub range_left: f32,
    /// Hvem der affyrede (entity index hash; 0 = player).
    pub owner_id: u32,
    /// Er det fra spilleren?
    pub from_player: bool,
}

impl Projectile {
    pub fn new(pos: Vec2, direction: Vec2, damage: f32, speed: f32, range: f32, from_player: bool) -> Self {
        Self {
            pos,
            vel: direction * speed,
            damage,
            range_left: range,
            owner_id: 0,
            from_player,
        }
    }

    /// Opdatér projectile; returnerer true hvis den er udløbet (range brugt).
    pub fn update(&mut self, dt: f32) -> bool {
        let move_dist = self.vel.length() * dt;
        self.pos += self.vel * dt;
        self.range_left -= move_dist;
        self.range_left <= 0.0
    }
}

/// Health-komponent for entities der kan tage skade.
#[derive(Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub max: f32,
    pub alive: bool,
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max, alive: true }
    }

    pub fn take_damage(&mut self, amount: f32) {
        if !self.alive {
            return;
        }
        self.current = (self.current - amount).max(0.0);
        if self.current <= 0.0 {
            self.alive = false;
        }
    }

    pub fn heal(&mut self, amount: f32) {
        if !self.alive {
            return;
        }
        self.current = (self.current + amount).min(self.max);
    }

    pub fn is_dead(&self) -> bool {
        !self.alive
    }

    pub fn health_pct(&self) -> f32 {
        if self.max <= 0.0 { 0.0 } else { self.current / self.max }
    }
}

/// Visuel effekt: muzzle flash (lysglimt ved skud).
#[derive(Debug, Clone, Copy)]
pub struct MuzzleFlash {
    pub pos: Vec2,
    /// Tilbageværende tid (sek).
    pub time_left: f32,
    /// Retning (radians).
    pub angle: f32,
}

impl MuzzleFlash {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self { pos, time_left: 0.05, angle }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.time_left -= dt;
        self.time_left <= 0.0
    }
}

/// Visuel effekt: blod-partikel (ved hit).
#[derive(Debug, Clone, Copy)]
pub struct BloodParticle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub time_left: f32,
    pub size: f32,
}

impl BloodParticle {
    pub fn new(pos: Vec2, direction: Vec2) -> Self {
        let angle = direction.y.atan2(direction.x) + (rand_angle() * 0.5);
        let speed = 50.0 + rand_angle() * 100.0;
        Self {
            pos,
            vel: Vec2::new(angle.cos() * speed, angle.sin() * speed),
            time_left: 0.3 + rand_angle() * 0.2,
            size: 3.0 + rand_angle() * 2.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.pos += self.vel * dt;
        self.vel *= 0.9; // friction
        self.time_left -= dt;
        self.time_left <= 0.0
    }
}

/// Pseudo-random [-1, 1] uden rand crate (brug sim_time seeding).
fn rand_angle() -> f32 {
    // Simpel pseudo-random baseret på en statisk tæller.
    use std::sync::atomic::{AtomicU32, Ordering};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let v = (n.wrapping_mul(2654435761) % 1000) as f32 / 500.0 - 1.0;
    v
}

/// Combat-system state: projectiles, muzzle flashes, blood particles.
#[derive(Debug, Clone, Default)]
pub struct CombatSystem {
    pub projectiles: Vec<Projectile>,
    pub muzzle_flashes: Vec<MuzzleFlash>,
    pub blood_particles: Vec<BloodParticle>,
    /// Cooldown for spillerens våben (sek til næste skud).
    pub player_fire_cooldown: f32,
}

impl CombatSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Affyr et våben fra en position i en retning.
    pub fn fire(&mut self, weapon: WeaponKind, pos: Vec2, direction: Vec2, from_player: bool) {
        for _ in 0..weapon.pellets() {
            let spread = weapon.spread();
            let angle = direction.y.atan2(direction.x) + rand_angle() * spread;
            let dir = Vec2::new(angle.cos(), angle.sin());
            let proj = Projectile::new(
                pos,
                dir,
                weapon.damage(),
                weapon.projectile_speed(),
                weapon.range(),
                from_player,
            );
            self.projectiles.push(proj);
        }
        // Muzzle flash.
        self.muzzle_flashes.push(MuzzleFlash::new(pos, direction.y.atan2(direction.x)));
    }

    /// Opdatér alle projectiles, muzzle flashes, blood particles.
    /// Returnerer lister over: (projectile_idx, hit_pos) for projectiles der hit noget.
    pub fn update(&mut self, dt: f32) {
        // Projectiles.
        let mut alive = Vec::new();
        for mut proj in self.projectiles.drain(..) {
            let expired = proj.update(dt);
            if !expired {
                alive.push(proj);
            }
        }
        self.projectiles = alive;

        // Muzzle flashes.
        let mut alive_flashes = Vec::new();
        for mut flash in self.muzzle_flashes.drain(..) {
            let expired = flash.update(dt);
            if !expired {
                alive_flashes.push(flash);
            }
        }
        self.muzzle_flashes = alive_flashes;

        // Blood particles.
        let mut alive_blood = Vec::new();
        for mut blood in self.blood_particles.drain(..) {
            let expired = blood.update(dt);
            if !expired {
                alive_blood.push(blood);
            }
        }
        self.blood_particles = alive_blood;

        // Player fire cooldown.
        self.player_fire_cooldown = (self.player_fire_cooldown - dt).max(0.0);
    }

    /// Kan spilleren skyde nu?
    pub fn can_player_fire(&self, weapon: WeaponKind) -> bool {
        self.player_fire_cooldown <= 0.0 && !weapon.is_melee()
    }

    /// Spiller affyrer våben; sæt cooldown.
    pub fn player_fire(&mut self, weapon: WeaponKind, pos: Vec2, direction: Vec2) {
        if self.can_player_fire(weapon) {
            self.fire(weapon, pos, direction, true);
            self.player_fire_cooldown = 1.0 / weapon.fire_rate();
        }
    }

    /// Tilføj blod-partikler ved et hit.
    pub fn spawn_blood(&mut self, pos: Vec2, direction: Vec2, count: usize) {
        for _ in 0..count {
            self.blood_particles.push(BloodParticle::new(pos, direction));
        }
    }
}