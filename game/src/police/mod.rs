//! Police — wanted-system, evidence, politi-AI.
//!
//! Fase 6: Heat-levels 1-6, bevis-system, politi-entiteter med search/pursue FSM.

#![allow(unused_imports)] // Re-exports er public API til senere gameplay-systemer.

pub mod evidence;
pub mod heat;
pub mod police_ai;

pub use evidence::{Evidence, EvidenceKind, EvidenceLedger};
pub use heat::{CrimeType, HeatLevel, WantedState};
pub use police_ai::{Police, PolicePatrol, PoliceState, despawn_excess_police, spawn_police_units, update_police, update_wanted_sight};