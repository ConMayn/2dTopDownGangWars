//! Dialog — dynamisk mikro-dialog for NPC'er.
//!
//! Fase 4: simple linjer baseret på NPC-state, tidspunkt, spillerens våben,
//! og NPC-type. Ikke træ-struktur endnu — det kommer i Fase 7.
//!
//! Målet: byen føles levende, NPC'er siger ting der giver mening.

use super::npc::NpcType;
use super::npc_fsm::NpcState;
use crate::systems::world_time::TimeOfDay;

/// Dialog-kontekst: alt NPC'en "ved" for at sige noget relevant.
pub struct DialogContext {
    pub time_of_day: TimeOfDay,
    pub player_armed: bool,
    pub npc_state: NpcState,
    pub npc_type: NpcType,
    pub distance: f32,
}

/// Generér en dialog-linje for en NPC givent kontekst.
/// Returnerer None hvis NPC'en ikke siger noget.
pub fn generate_line(ctx: &DialogContext) -> Option<String> {
    // Prioriter state-baserede linjer først.
    match ctx.npc_state {
        NpcState::Panic => {
            let lines = [
                "Hey! What are you doing?!",
                "Someone call the cops!",
                "Get away from me!",
                "I don't want any trouble!",
            ];
            return Some(lines[(ctx.distance as usize) % lines.len()].to_string());
        }
        NpcState::Flee => {
            let lines = [
                "Not sticking around for this.",
                "I'm out of here.",
                "You're crazy, you know that?",
                "This is not my problem.",
            ];
            return Some(lines[(ctx.distance as usize) % lines.len()].to_string());
        }
        NpcState::Talk if ctx.distance < 30.0 => {
            // Snak-linjer baseret på type og tid.
            return Some(talk_line(ctx));
        }
        _ => {}
    }

    // Proximity-based comments når spilleren er nær.
    if ctx.distance < 60.0 && ctx.player_armed {
        let lines = [
            "Whoa, put that away.",
            "Wrong block for that, friend.",
            "You're gonna get us both killed.",
            "Man, that's not a toy.",
        ];
        return Some(lines[(ctx.distance as usize) % lines.len()].to_string());
    }

    // Tid-baserede generelle kommentarer.
    if ctx.distance < 40.0 && (ctx.npc_state == NpcState::Walk || ctx.npc_state == NpcState::Idle) {
        let line = match ctx.time_of_day {
            TimeOfDay::Dawn => "Early bird, huh?",
            TimeOfDay::Morning => "Morning. Coffee hasn't kicked in yet.",
            TimeOfDay::Afternoon => "Nice day. If you ignore the cops.",
            TimeOfDay::Evening => "Evening. Streets get weird after dark.",
            TimeOfDay::Night => "You shouldn't be out here. Neither should I.",
        };
        // Kun en NPC-type (Pedestrian) siger tidslinjer standardmæssigt.
        if ctx.npc_type == NpcType::Pedestrian && ctx.distance < 35.0 {
            return Some(line.to_string());
        }
    }

    None
}

fn talk_line(ctx: &DialogContext) -> String {
    match ctx.npc_type {
        NpcType::Pedestrian => {
            if ctx.player_armed {
                "Hey. You look like trouble.".to_string()
            } else {
                "Need something?".to_string()
            }
        }
        NpcType::Shopkeeper => {
            if ctx.time_of_day == TimeOfDay::Night {
                "We're closing soon. Buy something or move on.".to_string()
            } else {
                "Welcome. Don't touch anything.".to_string()
            }
        }
        NpcType::GangMember => {
            if ctx.player_armed {
                "Wrong colors, wrong block, wrong idea.".to_string()
            } else {
                "You lost, or just stupid?".to_string()
            }
        }
    }
}