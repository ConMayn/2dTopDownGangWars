#![allow(dead_code)] // Economy API er public/stub til fremtidig shop/loot.

//! Economy — penge og inventory for spilleren.
//!
//! Fase 7: to valutaer (cash = sorte penge, clean = hvide penge),
//! inventory af items (våben, værktøj, osv.).

use serde::{Deserialize, Serialize};

/// To slags valuta: cash kan bruges til gadehandel/bestikkelse,
/// clean money kan bruges til ejendomme og officielle køb.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Wallet {
    pub cash: u32,
    pub clean: u32,
}

impl Wallet {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tilføj penge. `clean=true` lægger til clean-saldoen.
    pub fn add(&mut self, amount: u32, clean: bool) {
        if clean {
            self.clean += amount;
        } else {
            self.cash += amount;
        }
    }

    /// Forsøg at trække et beløb fra den valgte valuta.
    /// Returnerer false hvis der ikke er dækning.
    pub fn spend(&mut self, amount: u32, clean: bool) -> bool {
        let balance = if clean { &mut self.clean } else { &mut self.cash };
        if *balance >= amount {
            *balance -= amount;
            true
        } else {
            false
        }
    }

    pub fn total(&self) -> u64 {
        self.cash as u64 + self.clean as u64
    }
}

/// Item-type — våben, værktøj, udstyr, osv.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemKind {
    Weapon,
    Ammo,
    Armor,
    Lockpick,
    Scanner,
    Medkit,
    ZipTies,
    Crowbar,
    SprayCan,
    Disguise,
    VehicleMod,
    Gift,
    EvidenceBag,
    Key,
}

impl ItemKind {
    pub fn label(&self) -> &'static str {
        match self {
            ItemKind::Weapon => "Weapon",
            ItemKind::Ammo => "Ammo",
            ItemKind::Armor => "Armor",
            ItemKind::Lockpick => "Lockpick",
            ItemKind::Scanner => "Scanner",
            ItemKind::Medkit => "Medkit",
            ItemKind::ZipTies => "Zip Ties",
            ItemKind::Crowbar => "Crowbar",
            ItemKind::SprayCan => "Spray Can",
            ItemKind::Disguise => "Disguise",
            ItemKind::VehicleMod => "Vehicle Mod",
            ItemKind::Gift => "Gift",
            ItemKind::EvidenceBag => "Evidence Bag",
            ItemKind::Key => "Key",
        }
    }
}

/// Et stack-bart item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub kind: ItemKind,
    pub name: String,
    pub description: String,
    pub stack: u32,
    pub max_stack: u32,
    pub value: u32,
    pub illegal: bool,
}

impl Item {
    pub fn new(id: &str, kind: ItemKind, name: &str, value: u32, illegal: bool) -> Self {
        Self {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            description: String::new(),
            stack: 1,
            max_stack: 99,
            value,
            illegal,
        }
    }
}

/// Spillerens inventory.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// Tilføj et item; forsøg at stacke med eksisterende.
    pub fn add(&mut self, item: Item) {
        if let Some(existing) = self.items.iter_mut().find(|i| i.id == item.id && i.stack < i.max_stack) {
            existing.stack = (existing.stack + item.stack).min(existing.max_stack);
        } else {
            self.items.push(item);
        }
    }

    /// Fjern en mængde af et item. Returnerer true hvis det lykkedes.
    pub fn remove(&mut self, id: &str, amount: u32) -> bool {
        if let Some(idx) = self.items.iter().position(|i| i.id == id) {
            if self.items[idx].stack > amount {
                self.items[idx].stack -= amount;
                return true;
            } else if self.items[idx].stack == amount {
                self.items.remove(idx);
                return true;
            }
        }
        false
    }

    pub fn has(&self, id: &str) -> bool {
        self.items.iter().any(|i| i.id == id)
    }

    pub fn count(&self, id: &str) -> u32 {
        self.items.iter().find(|i| i.id == id).map(|i| i.stack).unwrap_or(0)
    }
}

/// Spillerens economy-state (wallet + inventory).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerEconomy {
    pub wallet: Wallet,
    pub inventory: Inventory,
}

impl PlayerEconomy {
    pub fn new() -> Self {
        Self::default()
    }

    /// Startkapital (fase 7 proof: lidt cash, en lockpick).
    pub fn with_starter_kit() -> Self {
        let mut inv = Inventory::new();
        inv.add(Item::new("lockpick", ItemKind::Lockpick, "Lockpick", 25, false));
        inv.add(Item::new("pistol", ItemKind::Weapon, "Pistol", 500, true));
        Self {
            wallet: Wallet { cash: 200, clean: 0 },
            inventory: inv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wallet_spend_fails_when_insufficient() {
        let mut w = Wallet::new();
        assert!(!w.spend(100, false));
        w.add(100, false);
        assert!(w.spend(50, false));
        assert_eq!(w.cash, 50);
    }

    #[test]
    fn inventory_stacks() {
        let mut inv = Inventory::new();
        inv.add(Item::new("lockpick", ItemKind::Lockpick, "Lockpick", 25, false));
        inv.add(Item::new("lockpick", ItemKind::Lockpick, "Lockpick", 25, false));
        assert_eq!(inv.count("lockpick"), 2);
    }
}
