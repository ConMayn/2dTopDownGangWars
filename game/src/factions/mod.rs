//! Factions — modul der binder faction definitioner, influence, reputation og AI.
//!
//! Dette er spillets "signatur-feature": byen husker dig. Hver bande, hvert
//! kvarter, politiet og civile husker dine valg.

pub mod faction_ai;
pub mod faction_def;
pub mod influence;
pub mod reputation;

pub use faction_ai::FactionAi;
pub use faction_def::{FactionDef, FactionKind, FactionRegistry};
pub use influence::{InfluenceGraph, ZoneInfluence};
pub use reputation::{
    CivilianRep, FactionStatus, FactionTrust, InvestigationStatus, PoliceProfile,
    RepEvent, ReputationState, apply_event, apply_events,
};