#![allow(dead_code)] // NPC helpers er public API til fremtidig dialog/UI.

//! NPC — entiteter der lever i byen.
//!
//! Fase 2: simple patrol.
//! Fase 4: FSM (Idle/Walk/Flee/Panic/Talk), memory, reaktioner, mikro-dialog.

use heat_core::Vec2;

use super::npc_fsm::{NpcMemory, NpcState, npc_movement, update_npc_state};

/// NPC-komponent: position + type + state + memory.
#[derive(Debug, Clone, Copy)]
pub struct Npc {
    pub pos: Vec2,
    pub npc_type: NpcType,
    pub speed: f32,
    pub current_waypoint: usize,
    pub color: [f32; 4],
    pub state: NpcState,
    pub memory: NpcMemory,
    /// Timer til at skifte "idle" tilbage til "walk".
    pub idle_timer: f32,
    /// Aktuel dialog-linje (tom = ingen).
    pub dialog_line: [u8; 64],
    pub dialog_len: u8,
    /// Health (0-100). Dør ved 0.
    pub health: f32,
    /// Er NPC i live?
    pub alive: bool,
}

impl Npc {
    pub fn new(pos: Vec2, npc_type: NpcType) -> Self {
        let speed = match npc_type {
            NpcType::Pedestrian => 60.0,
            NpcType::Shopkeeper => 40.0,
            NpcType::GangMember => 80.0,
        };
        let color = match npc_type {
            NpcType::Pedestrian => [0.4, 0.7, 0.4, 1.0],
            NpcType::Shopkeeper => [0.6, 0.5, 0.3, 1.0],
            NpcType::GangMember => [0.7, 0.2, 0.2, 1.0],
        };
        Self {
            pos,
            npc_type,
            speed,
            current_waypoint: 0,
            color,
            state: NpcState::Walk,
            memory: NpcMemory::default(),
            idle_timer: 0.0,
            dialog_line: [0; 64],
            dialog_len: 0,
            health: 50.0,
            alive: true,
        }
    }

    /// Sæt dialog-linje (max 63 tegn).
    pub fn set_dialog(&mut self, text: &str) {
        self.dialog_len = 0;
        let bytes = text.as_bytes();
        let n = bytes.len().min(63);
        self.dialog_line[..n].copy_from_slice(&bytes[..n]);
        self.dialog_len = n as u8;
    }

    pub fn dialog(&self) -> Option<String> {
        if self.dialog_len == 0 {
            None
        } else {
            Some(String::from_utf8_lossy(&self.dialog_line[..self.dialog_len as usize]).to_string())
        }
    }
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

/// Patrol-rute (waypoints). Separat komponent (Npc forbliver Copy).
#[derive(Debug, Clone)]
pub struct Patrol {
    pub waypoints: Vec<Vec2>,
}

/// Opdatér alle NPC'er: FSM, movement, collision, dialog.
/// `player_pos`: spillerens position.
/// `player_armed`: har spilleren et våben fremme? (placeholder: altid false i Fase 4).
/// `sim_time`: aktuel sim-tid.
pub fn update_npcs(
    world: &mut heat_core::World,
    tilemap: &super::tilemap::Tilemap,
    registry: &super::tiles::TileRegistry,
    player_pos: Vec2,
    player_armed: bool,
    sim_time: f32,
    dt: f32,
) {
    // Først: opdatér states og movement.
    {
        let inner = world.inner_mut();
        for (_, (npc, patrol)) in inner.query_mut::<(&mut Npc, &Patrol)>() {
            let dist_to_player = (npc.pos - player_pos).length();
            npc.state = update_npc_state(
                npc.state,
                &mut npc.memory,
                dist_to_player,
                player_armed,
                sim_time,
                dt,
            );

            let (delta, new_wp) = npc_movement(
                npc,
                &npc.state,
                player_pos,
                &patrol.waypoints,
                dt,
            );
            npc.current_waypoint = new_wp;

            if delta != Vec2::ZERO {
                let half = Vec2::new(12.0, 12.0);
                let result = super::collision::move_and_collide(npc.pos, half, delta, tilemap, registry);
                npc.pos = result.new_pos;
            }
        }
    }

    // Dialog: tøm alle linjer (de gælder kun én frame — NPC "siger" det en gang).
    {
        let inner = world.inner_mut();
        for (_, npc) in inner.query_mut::<&mut Npc>() {
            npc.dialog_len = 0;
        }
    }
}

/// Opdatér NPC-dialog for NPC'er tæt på spilleren.
/// Bruger DialogContext og generate_line.
pub fn update_npc_dialog(
    world: &mut heat_core::World,
    player_pos: Vec2,
    player_armed: bool,
    time_of_day: crate::systems::world_time::TimeOfDay,
) {
    let inner = world.inner_mut();
    for (_, npc) in inner.query_mut::<&mut Npc>() {
        let dist = (npc.pos - player_pos).length();
        if dist > 80.0 {
            continue; // kun NPC'er i nærheden
        }
        let ctx = super::dialog::DialogContext {
            time_of_day,
            player_armed,
            npc_state: npc.state,
            npc_type: npc.npc_type,
            distance: dist,
        };
        if let Some(line) = super::dialog::generate_line(&ctx) {
            npc.set_dialog(&line);
        }
    }
}