#![allow(dead_code)] // Mange evidence-metoder er public API til fremtidige flugt-mekanikker.

//! Evidence — bevis-system for politiefterforskning.
//!
//! Fra GDD afsnit 26: politiet tracker abstrakt beviser:
//! vidner, kameraer, nummerplade, våbentype, biltype, position.
//! Spilleren kan reducere beviser (skift bil, knus bil, betal fixer, frame rival).
//!
//! Efterforskningsstatus: Unknown → PersonOfInterest → Identified → Warrant → Manhunt.

use crate::factions::InvestigationStatus;
use serde::{Deserialize, Serialize};

/// Bevis-type — hver type bidrager til efterforskning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceKind {
    Witness,        // NPC så dig
    Camera,         // kameraoptagelse
    LicensePlate,   // nummerplade læst
    WeaponType,     // våbentype identificeret
    VehicleType,    // biltype identificeret
    Fingerprints,   // fingeraftryk (gameplay-token)
    KnownAssociate, // kendt allieret set med dig
    LastPosition,   // sidste kendte position
}

impl EvidenceKind {
    pub fn weight(&self) -> f32 {
        match self {
            EvidenceKind::Witness => 10.0,
            EvidenceKind::Camera => 15.0,
            EvidenceKind::LicensePlate => 20.0,
            EvidenceKind::WeaponType => 10.0,
            EvidenceKind::VehicleType => 10.0,
            EvidenceKind::Fingerprints => 25.0,
            EvidenceKind::KnownAssociate => 15.0,
            EvidenceKind::LastPosition => 5.0,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            EvidenceKind::Witness => "Witness",
            EvidenceKind::Camera => "Camera",
            EvidenceKind::LicensePlate => "License Plate",
            EvidenceKind::WeaponType => "Weapon Type",
            EvidenceKind::VehicleType => "Vehicle Type",
            EvidenceKind::Fingerprints => "Fingerprints",
            EvidenceKind::KnownAssociate => "Known Associate",
            EvidenceKind::LastPosition => "Last Position",
        }
    }
}

/// Et bevis-instans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub kind: EvidenceKind,
    /// Sim-tidspunkt beviset blev samlet.
    pub time: f32,
    /// Zone hvor beviset blev samlet.
    pub zone: String,
    /// Beskrivelse (f.eks. "red muscle car", "9mm pistol").
    pub detail: String,
}

/// Evidence-ledger — alle beviser politiet har på spilleren.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EvidenceLedger {
    pub evidence: Vec<Evidence>,
    /// Samlet "identification score" 0-100 (afledt af beviser).
    pub identification: f32,
}

impl EvidenceLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tilføj et bevis og opdatér identification score.
    pub fn add(&mut self, kind: EvidenceKind, time: f32, zone: &str, detail: &str) {
        // Undgå duplikater af samme type + detail (f.eks. ikke 10 "LicensePlate: ABC123").
        if self.evidence.iter().any(|e| e.kind == kind && e.detail == detail) {
            return;
        }
        self.evidence.push(Evidence {
            kind,
            time,
            zone: zone.to_string(),
            detail: detail.to_string(),
        });
        self.recompute_identification();
    }

    /// Fjern alle beviser af en bestemt type (f.eks. skift bil → fjern LicensePlate + VehicleType).
    pub fn remove_kind(&mut self, kind: EvidenceKind) {
        self.evidence.retain(|e| e.kind != kind);
        self.recompute_identification();
    }

    /// Fjern alle beviser (korrupt betjent "miste papirerne").
    pub fn clear(&mut self) {
        self.evidence.clear();
        self.identification = 0.0;
    }

    /// Reducér identification (betalt fixer, intimideret vidne).
    pub fn reduce_identification(&mut self, amount: f32) {
        self.identification = (self.identification - amount).max(0.0);
    }

    /// Beregn identification score fra beviser.
    fn recompute_identification(&mut self) {
        let total: f32 = self.evidence.iter().map(|e| e.kind.weight()).sum();
        // Diminishing returns: hver ekstra bevis giver mindre.
        self.identification = (1.0 - (-total / 50.0).exp()) * 100.0;
    }

    /// Efterforskningsstatus afledt af identification score.
    pub fn investigation_status(&self) -> InvestigationStatus {
        if self.identification < 10.0 {
            InvestigationStatus::Unknown
        } else if self.identification < 30.0 {
            InvestigationStatus::PersonOfInterest
        } else if self.identification < 60.0 {
            InvestigationStatus::Identified
        } else if self.identification < 85.0 {
            InvestigationStatus::WarrantActive
        } else {
            InvestigationStatus::Manhunt
        }
    }

    pub fn len(&self) -> usize {
        self.evidence.len()
    }

    pub fn is_empty(&self) -> bool {
        self.evidence.is_empty()
    }
}