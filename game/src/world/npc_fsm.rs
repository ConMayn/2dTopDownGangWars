#![allow(dead_code)] // Public API/stub til fremtidige vidne-krime events.

//! NPC FSM — state machine for NPC'er.
//!
//! Fase 4: Idle, Walk, Flee, Panic, Talk.
//! NPC'er reagerer på spilleren (vapen, bilkørsel, nærhed) og på fare (skud, biljagt).

use heat_core::Vec2;

/// NPC-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcState {
    Idle,    // Står stille, måske snakker
    Walk,    // Går mod waypoint / mål
    Flee,    // Flygter fra fare
    Panic,   // Panikker (skriger, løber vilkårligt)
    Talk,    // Snakker med spilleren
}

impl NpcState {
    pub fn label(&self) -> &'static str {
        match self {
            NpcState::Idle => "Idle",
            NpcState::Walk => "Walk",
            NpcState::Flee => "Flee",
            NpcState::Panic => "Panic",
            NpcState::Talk => "Talk",
        }
    }
}

/// NPC-hukommelse: hvem har de set, hvad husker de?
/// Simpelt: en "fear" level der falder over tid, og en "saw_player_with_weapon" flag.
#[derive(Debug, Clone, Copy)]
pub struct NpcMemory {
    /// Frygt-level 0.0-1.0. Stiger ved fare, falder over tid.
    pub fear: f32,
    /// Sidste gang NPC så spilleren med våben (sim_time).
    pub last_saw_weapon: f32,
    /// Har NPC set spilleren lave kriminalitet?
    pub witnessed_crime: bool,
}

impl Default for NpcMemory {
    fn default() -> Self {
        Self {
            fear: 0.0,
            last_saw_weapon: -100.0,
            witnessed_crime: false,
        }
    }
}

/// Opdatér NPC FSM: bestem ny state baseret på omgivelser, udfør state-adfærd.
/// `nearby_danger`: er der fare i nærheden? (f.eks. spilleren har våben, eller
/// en kollision/bil nærmede sig). `dist_to_player`: afstand til spilleren.
/// `player_armed`: har spilleren et våben fremme?
pub fn update_npc_state(
    state: NpcState,
    memory: &mut NpcMemory,
    dist_to_player: f32,
    player_armed: bool,
    sim_time: f32,
    dt: f32,
) -> NpcState {
    // Frygt falder over tid.
    memory.fear = (memory.fear - dt * 0.1).max(0.0);

    // Hvis spilleren er tæt og har våben → frygt stiger.
    if player_armed && dist_to_player < 100.0 {
        memory.fear = (memory.fear + dt * 0.5).min(1.0);
        memory.last_saw_weapon = sim_time;
    }

    // State transitions baseret på frygt.
    if memory.fear > 0.7 {
        NpcState::Panic
    } else if memory.fear > 0.3 {
        NpcState::Flee
    } else if dist_to_player < 40.0 && !player_armed {
        // Tæt på spilleren, ikke farlig → snak.
        NpcState::Talk
    } else {
        // Default: gå på patrol eller idle.
        match state {
            NpcState::Flee | NpcState::Panic => {
                // Hvis frygt faldet nok → gå tilbage til Walk.
                if memory.fear < 0.15 {
                    NpcState::Walk
                } else {
                    state
                }
            }
            _ => NpcState::Walk,
        }
    }
}

/// Beregn bevægelses-delta for NPC baseret på state.
/// `player_pos`: spillerens position (for Flee/Panic/Talk).
/// `patrol_waypoints`: rute for Walk-state.
/// Returnerer (delta, new_waypoint_idx).
pub fn npc_movement(
    npc: &super::npc::Npc,
    state: &NpcState,
    player_pos: Vec2,
    waypoints: &[Vec2],
    dt: f32,
) -> (Vec2, usize) {
    match state {
        NpcState::Idle | NpcState::Talk => (Vec2::ZERO, npc.current_waypoint),
        NpcState::Walk => {
            if waypoints.is_empty() {
                return (Vec2::ZERO, npc.current_waypoint);
            }
            let target = waypoints[npc.current_waypoint % waypoints.len()];
            let to_target = target - npc.pos;
            let dist = to_target.length();
            if dist < 4.0 {
                // Næste waypoint.
                let next = (npc.current_waypoint + 1) % waypoints.len();
                (Vec2::ZERO, next)
            } else {
                let dir = to_target / dist;
                (dir * npc.speed * dt, npc.current_waypoint)
            }
        }
        NpcState::Flee => {
            // Løb væk fra spilleren.
            let away = npc.pos - player_pos;
            let dist = away.length();
            if dist > 0.0 {
                let dir = away / dist;
                (dir * npc.speed * 1.5 * dt, npc.current_waypoint)
            } else {
                (Vec2::ZERO, npc.current_waypoint)
            }
        }
        NpcState::Panic => {
            // Løb i vilkårlig retning (skift retning periodisk).
            let angle = (npc.pos.x + npc.pos.y + dt * 10.0).sin() * std::f32::consts::TAU;
            let dir = Vec2::new(angle.cos(), angle.sin());
            (dir * npc.speed * 2.0 * dt, npc.current_waypoint)
        }
    }
}