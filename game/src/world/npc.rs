//! NPC — entiteter der lever i byen.
//!
//! Fase 2: simple NPC'er der patroljerer waypoints.
//! Fase 4+: FSM med Idle/Walk/Work/Flee/Panic, rutiner, dialog.

use heat_core::Vec2;

/// NPC-komponent: position + patrol + current state.
#[derive(Debug, Clone, Copy)]
pub struct Npc {
    pub pos: Vec2,
    pub npc_type: NpcType,
    pub speed: f32,
    pub current_waypoint: usize,
    pub color: [f32; 4],
}

/// NPC-type (bestemmer adfærd, udseende, dialog).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    Pedestrian,
    Shopkeeper,
    GangMember,
}

impl NpcType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "shopkeeper" => NpcType::Shopkeeper,
            "gang_member" => NpcType::GangMember,
            _ => NpcType::Pedestrian,
        }
    }
}

/// Patrol-rute (waypoints). Gemmes separat da Vec ikke kan være i hecs Component
/// (kræver Send + Sync + 'static, hvilket Vec er, men vi holder det seperat for
/// at Npc forbliver Copy).
#[derive(Debug, Clone)]
pub struct Patrol {
    pub waypoints: Vec<Vec2>,
}

/// Opdatér NPC patrol: bevæg mod næste waypoint, cycle.
pub fn update_npc_patrol(
    world: &mut heat_core::World,
    tilemap: &super::tilemap::Tilemap,
    registry: &super::tiles::TileRegistry,
    dt: f32,
) {
    // For hver NPC med patrol: bevæg mod current_waypoint.
    let inner = world.inner_mut();
    for (_, (npc, patrol)) in inner.query_mut::<(&mut Npc, &Patrol)>() {
        if patrol.waypoints.is_empty() {
            continue;
        }
        let target = patrol.waypoints[npc.current_waypoint % patrol.waypoints.len()];
        let to_target = target - npc.pos;
        let dist = to_target.length();
        if dist < 4.0 {
            // Nået waypoint → næste.
            npc.current_waypoint = (npc.current_waypoint + 1) % patrol.waypoints.len();
        } else {
            let dir = to_target / dist;
            let delta = dir * npc.speed * dt;
            // Simple collision: brug move_and_collide.
            let half = Vec2::new(12.0, 12.0); // NPC hitbox 24x24
            let result = super::collision::move_and_collide(npc.pos, half, delta, tilemap, registry);
            npc.pos = result.new_pos;
        }
    }
}