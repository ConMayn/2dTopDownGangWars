#![allow(dead_code)] // Safehouse API er public/stub til fremtidig UI/save.

//! Safehouses — spillerens gemmesteder.
//!
//! Et safehouse kan: gemme spil, opbevare stash, garage, holde crew-møder,
//! lade spilleren skifte tøj. Forskellige typer har forskellige kapaciteter.
//! Safehouses kan kompromitteres ved overuse eller politi-opmærksomhed.

use serde::{Deserialize, Serialize};

/// Safehouse-type (fra GDD afsnit 18).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SafehouseKind {
    /// Et værelse — minimalt stash, ingen garage, billig (cash).
    CrashPad,
    /// Lejlighed — stash + lille garage, medium pris (cash).
    Apartment,
    /// Byhus — stash + garage + crew-rum, dyr (cash eller clean).
    Townhouse,
    /// Lager / industri-unit — stor garage + stash, til store operationer.
    Warehouse,
    /// Herrehus — fuld luksus, garage, crew, stash; meget dyr (clean).
    Mansion,
}

impl SafehouseKind {
    pub fn label(&self) -> &'static str {
        match self {
            SafehouseKind::CrashPad => "Crash Pad",
            SafehouseKind::Apartment => "Apartment",
            SafehouseKind::Townhouse => "Townhouse",
            SafehouseKind::Warehouse => "Warehouse",
            SafehouseKind::Mansion => "Mansion",
        }
    }

    /// Pris (cash). Mansions kræver clean money.
    pub fn base_price(&self) -> u32 {
        match self {
            SafehouseKind::CrashPad => 5_000,
            SafehouseKind::Apartment => 25_000,
            SafehouseKind::Townhouse => 80_000,
            SafehouseKind::Warehouse => 150_000,
            SafehouseKind::Mansion => 500_000,
        }
    }

    /// Kræver clean money (officielt køb)?
    pub fn requires_clean(&self) -> bool {
        matches!(self, SafehouseKind::Townhouse | SafehouseKind::Mansion)
    }

    /// Stash-kapacitet (items).
    pub fn stash_capacity(&self) -> u32 {
        match self {
            SafehouseKind::CrashPad => 5,
            SafehouseKind::Apartment => 20,
            SafehouseKind::Townhouse => 50,
            SafehouseKind::Warehouse => 200,
            SafehouseKind::Mansion => 100,
        }
    }

    /// Garage-kapacitet (køretøjer).
    pub fn garage_capacity(&self) -> u32 {
        match self {
            SafehouseKind::CrashPad => 0,
            SafehouseKind::Apartment => 1,
            SafehouseKind::Townhouse => 2,
            SafehouseKind::Warehouse => 6,
            SafehouseKind::Mansion => 4,
        }
    }

    /// Crew-kapacitet (antal medlemmer der kan mødes her).
    pub fn crew_capacity(&self) -> u32 {
        match self {
            SafehouseKind::CrashPad => 0,
            SafehouseKind::Apartment => 1,
            SafehouseKind::Townhouse => 3,
            SafehouseKind::Warehouse => 5,
            SafehouseKind::Mansion => 8,
        }
    }
}

/// Et købt/ejet safehouse instans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Safehouse {
    pub id: String,
    pub kind: SafehouseKind,
    pub zone: String,
    /// "Addresse" — label der vises i UI.
    pub address: String,
    /// Resterende risiko (0-100). Høj = politi kan ransage.
    pub risk: f32,
    /// Stash: opbevarede items (id → count).
    pub stash: Vec<(String, u32)>,
    /// Garage: køretøjer (def_id).
    pub garage: Vec<String>,
    /// Aktiveret tøj-disguise (item-id) eller None.
    pub active_disguise: Option<String>,
}

impl Safehouse {
    pub fn new(id: &str, kind: SafehouseKind, zone: &str, address: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            zone: zone.to_string(),
            address: address.to_string(),
            risk: 0.0,
            stash: Vec::new(),
            garage: Vec::new(),
            active_disguise: None,
        }
    }

    /// Tilføj item til stash. Returnerer false hvis fuldt.
    pub fn stash_add(&mut self, item_id: &str, count: u32) -> bool {
        let total: u32 = self.stash.iter().map(|(_, c)| *c).sum();
        if total + count > self.kind.stash_capacity() {
            return false;
        }
        if let Some((_, c)) = self.stash.iter_mut().find(|(id, _)| id == item_id) {
            *c += count;
        } else {
            self.stash.push((item_id.to_string(), count));
        }
        true
    }

    /// Tilføj køretøj til garage. Returnerer false hvis fuldt.
    pub fn garage_add(&mut self, def_id: &str) -> bool {
        if self.garage.len() as u32 >= self.kind.garage_capacity() {
            return false;
        }
        self.garage.push(def_id.to_string());
        true
    }

    /// Opdatér risiko: stiger ved brug (stash opbevaring, crew-møder),
    /// falder langsomt over tid.
    pub fn tick(&mut self, dt: f32, usage_pressure: f32) {
        self.risk = (self.risk + usage_pressure * dt).min(100.0);
        self.risk = (self.risk - dt * 0.1).max(0.0);
    }

    /// Sæt aktiv disguise.
    pub fn set_disguise(&mut self, item_id: Option<String>) {
        self.active_disguise = item_id;
    }
}

/// Spillerens safehouse-portefølje.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SafehousePortfolio {
    pub owned: Vec<Safehouse>,
    /// ID på det safehouse der er "hjem-base" (hvor save-spawn er).
    pub home_base: Option<String>,
}

impl SafehousePortfolio {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start-med et lille crash pad i East Blocks.
    pub fn with_starter() -> Self {
        let mut sh = Safehouse::new("sh_starter", SafehouseKind::CrashPad, "east_blocks", "23 East Blocks Apt 1B");
        sh.stash_add("pistol", 1);
        sh.stash_add("pistol_ammo", 30);
        let mut port = Self::new();
        port.home_base = Some(sh.id.clone());
        port.owned.push(sh);
        port
    }

    pub fn buy(&mut self, sh: Safehouse) {
        self.owned.push(sh);
    }

    pub fn get(&self, id: &str) -> Option<&Safehouse> {
        self.owned.iter().find(|s| s.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Safehouse> {
        self.owned.iter_mut().find(|s| s.id == id)
    }

    pub fn home(&self) -> Option<&Safehouse> {
        self.home_base.as_ref().and_then(|id| self.get(id))
    }

    pub fn home_mut(&mut self) -> Option<&mut Safehouse> {
        let id = self.home_base.clone()?;
        self.get_mut(&id)
    }

    /// Opdatér alle safehouses.
    pub fn tick_all(&mut self, dt: f32) {
        for sh in &mut self.owned {
            sh.tick(dt, 0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stash_capacity_limits_items() {
        let mut sh = Safehouse::new("t", SafehouseKind::CrashPad, "z", "a");
        assert!(sh.stash_add("x", 5));
        assert!(!sh.stash_add("y", 1));
    }

    #[test]
    fn garage_capacity_limits_vehicles() {
        let mut sh = Safehouse::new("t", SafehouseKind::Apartment, "z", "a");
        assert!(sh.garage_add("sedan"));
        assert!(!sh.garage_add("van"));
    }
}