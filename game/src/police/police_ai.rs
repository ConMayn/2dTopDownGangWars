#![allow(dead_code)] // Politi-fields/helpers er public API til fremtidige zoner/roadblocks.

//! Police AI — search, pursue, roadblocks.
//!
//! Fase 6: Politi-entiteter der patruljerer, søger efter spilleren ved last_seen_pos,
//! forfølger ved sight, og spawner flere enheder ved højere heat-level.
//! Simpel FSM: Patrol → Search → Pursue → ReturnToPatrol.

use heat_core::Vec2;

use super::heat::{HeatLevel, WantedState};

/// Politistat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoliceState {
    Patrol,      // patruljerer fast rute
    Search,      // søger ved last_seen_pos
    Pursue,      // ser spilleren, forfølger
    ReturnToPatrol, // vender tilbage til patrulje-rute
    Roadblock,   // står ved roadblock
}

impl PoliceState {
    pub fn label(&self) -> &'static str {
        match self {
            PoliceState::Patrol => "Patrol",
            PoliceState::Search => "Search",
            PoliceState::Pursue => "Pursue",
            PoliceState::ReturnToPatrol => "Return",
            PoliceState::Roadblock => "Roadblock",
        }
    }
}

/// Politi-komponent: en police entity i verden.
#[derive(Debug, Clone, Copy)]
pub struct Police {
    pub pos: Vec2,
    pub state: PoliceState,
    pub speed: f32,
    /// Patrulje-waypoint index.
    pub patrol_idx: usize,
    /// Søge-position (last_seen_pos når i Search).
    pub search_target: Vec2,
    /// Sigt-afstand (pixel radius hvor de kan se spilleren).
    pub sight_range: f32,
    /// Sigt-vinkel (radians, synsfelt).
    pub sight_angle: f32,
    /// Heading (retning de kigger).
    pub heading: f32,
    /// Hvilken zone de hører til.
    pub zone: [u8; 16],
    pub zone_len: u8,
}

impl Police {
    pub fn new(pos: Vec2, zone: &str) -> Self {
        let mut zone_bytes = [0u8; 16];
        let n = zone.as_bytes().len().min(16);
        zone_bytes[..n].copy_from_slice(&zone.as_bytes()[..n]);
        Self {
            pos,
            state: PoliceState::Patrol,
            speed: 120.0,
            patrol_idx: 0,
            search_target: pos,
            sight_range: 200.0,
            sight_angle: std::f32::consts::FRAC_PI_3, // 60 grader synsfelt
            heading: 0.0,
            zone: zone_bytes,
            zone_len: n as u8,
        }
    }

    pub fn zone_str(&self) -> &str {
        std::str::from_utf8(&self.zone[..self.zone_len as usize]).unwrap_or("unknown")
    }
}

/// Patrulje-rute for en police entity (waypoints).
#[derive(Debug, Clone)]
pub struct PolicePatrol {
    pub waypoints: Vec<Vec2>,
}

/// Opdatér politi-AI: states, movement, perception.
/// `wanted`: aktuel wanted state (heat, last_seen, in_sight).
/// `player_pos`: spillerens position.
/// `dt`: sim delta.
pub fn update_police(
    world: &mut heat_core::World,
    wanted: &WantedState,
    player_pos: Vec2,
    dt: f32,
) {
    let heat = wanted.level;
    if heat == HeatLevel::None {
        // Ingen heat: politi patruljerer normalt.
        let inner = world.inner_mut();
        for (_, (police, patrol)) in inner.query_mut::<(&mut Police, &PolicePatrol)>() {
            police.state = PoliceState::Patrol;
            if !patrol.waypoints.is_empty() {
                let target = patrol.waypoints[police.patrol_idx % patrol.waypoints.len()];
                let to_target = target - police.pos;
                let dist = to_target.length();
                if dist < 8.0 {
                    police.patrol_idx = (police.patrol_idx + 1) % patrol.waypoints.len();
                } else {
                    let dir = to_target / dist;
                    police.pos += dir * police.speed * dt;
                    police.heading = dir.y.atan2(dir.x);
                }
            }
        }
        return;
    }

    // Heat aktiv: politi reagerer.
    let last_seen = Vec2::new(wanted.last_seen_pos[0], wanted.last_seen_pos[1]);
    let time_since = wanted.time_since_seen(wanted.last_seen_time);

    let inner = world.inner_mut();
    for (_, (police, patrol)) in inner.query_mut::<(&mut Police, &PolicePatrol)>() {
        // Perception: kan politiet se spilleren?
        let to_player = player_pos - police.pos;
        let dist_to_player = to_player.length();
        let can_see = if dist_to_player < police.sight_range {
            // Tjek synsvinkel.
            let angle_to_player = to_player.y.atan2(to_player.x);
            let mut angle_diff = (angle_to_player - police.heading).abs();
            if angle_diff > std::f32::consts::PI {
                angle_diff = std::f32::consts::TAU - angle_diff;
            }
            angle_diff < police.sight_angle
        } else {
            false
        };

        // State machine.
        police.state = if can_see && dist_to_player < police.sight_range {
            PoliceState::Pursue
        } else if time_since < 15.0 {
            // Spiller set for nylig → søg mod last_seen.
            PoliceState::Search
        } else if police.state == PoliceState::Pursue || police.state == PoliceState::Search {
            // Givet op → return to patrol.
            PoliceState::ReturnToPatrol
        } else {
            PoliceState::Patrol
        };

        // Movement baseret på state.
        let speed = if heat >= HeatLevel::TaskForce { police.speed * 1.4 } else { police.speed };
        match police.state {
            PoliceState::Pursue => {
                let dir = to_player / dist_to_player.max(0.001);
                police.pos += dir * speed * dt;
                police.heading = dir.y.atan2(dir.x);
            }
            PoliceState::Search => {
                let to_search = last_seen - police.pos;
                let dist = to_search.length();
                if dist > 8.0 {
                    let dir = to_search / dist;
                    police.pos += dir * speed * dt;
                    police.heading = dir.y.atan2(dir.x);
                }
            }
            PoliceState::ReturnToPatrol => {
                if !patrol.waypoints.is_empty() {
                    let target = patrol.waypoints[police.patrol_idx % patrol.waypoints.len()];
                    let to_target = target - police.pos;
                    let dist = to_target.length();
                    if dist < 8.0 {
                        police.state = PoliceState::Patrol;
                    } else {
                        let dir = to_target / dist;
                        police.pos += dir * police.speed * 0.7 * dt;
                        police.heading = dir.y.atan2(dir.x);
                    }
                }
            }
            PoliceState::Patrol => {
                if !patrol.waypoints.is_empty() {
                    let target = patrol.waypoints[police.patrol_idx % patrol.waypoints.len()];
                    let to_target = target - police.pos;
                    let dist = to_target.length();
                    if dist < 8.0 {
                        police.patrol_idx = (police.patrol_idx + 1) % patrol.waypoints.len();
                    } else {
                        let dir = to_target / dist;
                        police.pos += dir * police.speed * dt;
                        police.heading = dir.y.atan2(dir.x);
                    }
                }
            }
            PoliceState::Roadblock => {
                // Står stille.
            }
        }
    }
}

/// Opdatér wanted-state baseret på om nogen politi ser spilleren.
pub fn update_wanted_sight(
    world: &heat_core::World,
    wanted: &mut WantedState,
    player_pos: Vec2,
    sim_time: f32,
) {
    let mut any_sees = false;
    let inner = world.inner();
    for (_, police) in &mut inner.query::<&Police>() {
        let to_player = player_pos - police.pos;
        let dist = to_player.length();
        if dist < police.sight_range {
            let angle_to_player = to_player.y.atan2(to_player.x);
            let mut angle_diff = (angle_to_player - police.heading).abs();
            if angle_diff > std::f32::consts::PI {
                angle_diff = std::f32::consts::TAU - angle_diff;
            }
            if angle_diff < police.sight_angle {
                any_sees = true;
                break;
            }
        }
    }
    wanted.set_sight(any_sees, sim_time, player_pos);
}

/// Spawn politi-enheder baseret på heat-level.
/// `count` = wanted.level.response_units() - eksisterende_count.
pub fn spawn_police_units(world: &mut heat_core::World, count: u32, zone: &str, center: Vec2) {
    for i in 0..count {
        let offset = Vec2::new(
            ((i as f32 * 60.0) % 200.0) - 100.0,
            ((i as f32 * 40.0) % 120.0) - 60.0,
        );
        let pos = center + offset;
        let police = Police::new(pos, zone);
        let patrol = PolicePatrol {
            waypoints: vec![
                pos,
                pos + Vec2::new(80.0, 0.0),
                pos + Vec2::new(80.0, 80.0),
                pos + Vec2::new(0.0, 80.0),
            ],
        };
        world.spawn((police, patrol));
    }
}

/// Fjern politi-enheder (når heat falder til None).
pub fn despawn_excess_police(world: &mut heat_core::World, keep: u32) {
    let mut to_remove: Vec<hecs::Entity> = Vec::new();
    let mut count = 0u32;
    {
        let inner = world.inner();
        for (entity, _) in &mut inner.query::<&Police>() {
            count += 1;
            if count > keep {
                to_remove.push(entity);
            }
        }
    }
    for entity in to_remove {
        let _ = world.inner_mut().despawn(entity);
    }
}