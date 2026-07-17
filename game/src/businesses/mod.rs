#![allow(dead_code)] // Business API er public/stub til fremtidig økonomi-loop.

//! Businesses — front-virksomheder der vasker penge og giver passiv indkomst.
//!
//! Fra GDD afsnit 10.3: bilvask, pizzeria, bar, skrotplads, natklub, autoværksted,
//! pantelåner, taxi-central, lagerhal, strip mall, bodega, laundromat.
//!
//! En front: genererer clean money fra cash, tiltrækker politimæssig interesse,
//! giver missioner, giver adgang til lokalområder.

use serde::{Deserialize, Serialize};

/// Type af front business.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusinessKind {
    CarWash,
    Pizzeria,
    Bar,
    Scrapyard,
    Nightclub,
    AutoShop,
    PawnShop,
    TaxiCentral,
    Warehouse,
    StripMall,
    Bodega,
    Laundromat,
}

impl BusinessKind {
    pub fn label(&self) -> &'static str {
        match self {
            BusinessKind::CarWash => "Car Wash",
            BusinessKind::Pizzeria => "Pizzeria",
            BusinessKind::Bar => "Bar",
            BusinessKind::Scrapyard => "Scrapyard",
            BusinessKind::Nightclub => "Nightclub",
            BusinessKind::AutoShop => "Auto Shop",
            BusinessKind::PawnShop => "Pawn Shop",
            BusinessKind::TaxiCentral => "Taxi Central",
            BusinessKind::Warehouse => "Warehouse",
            BusinessKind::StripMall => "Strip Mall",
            BusinessKind::Bodega => "Bodega",
            BusinessKind::Laundromat => "Laundromat",
        }
    }

    /// Købspris i cash.
    pub fn purchase_price(&self) -> u32 {
        match self {
            BusinessKind::CarWash => 30_000,
            BusinessKind::Pizzeria => 40_000,
            BusinessKind::Bar => 60_000,
            BusinessKind::Scrapyard => 80_000,
            BusinessKind::Nightclub => 200_000,
            BusinessKind::AutoShop => 75_000,
            BusinessKind::PawnShop => 50_000,
            BusinessKind::TaxiCentral => 100_000,
            BusinessKind::Warehouse => 120_000,
            BusinessKind::StripMall => 90_000,
            BusinessKind::Bodega => 25_000,
            BusinessKind::Laundromat => 35_000,
        }
    }

    /// Hvor hurtigt den vasker cash→clean (rate per sim-time).
    pub fn laundering_rate(&self) -> f32 {
        match self {
            BusinessKind::Laundromat => 8.0,
            BusinessKind::CarWash => 6.0,
            BusinessKind::Pizzeria => 10.0,
            BusinessKind::Bar => 15.0,
            BusinessKind::Bodega => 5.0,
            BusinessKind::AutoShop => 12.0,
            BusinessKind::Nightclub => 30.0,
            BusinessKind::Scrapyard => 18.0,
            BusinessKind::PawnShop => 14.0,
            BusinessKind::TaxiCentral => 20.0,
            BusinessKind::Warehouse => 25.0,
            BusinessKind::StripMall => 22.0,
        }
    }

    /// Daglig passiv indkomst i clean money (sim-tid).
    pub fn passive_income(&self) -> u32 {
        match self {
            BusinessKind::CarWash => 50,
            BusinessKind::Pizzeria => 80,
            BusinessKind::Bar => 120,
            BusinessKind::Scrapyard => 150,
            BusinessKind::Nightclub => 400,
            BusinessKind::AutoShop => 140,
            BusinessKind::PawnShop => 100,
            BusinessKind::TaxiCentral => 180,
            BusinessKind::Warehouse => 200,
            BusinessKind::StripMall => 160,
            BusinessKind::Bodega => 40,
            BusinessKind::Laundromat => 60,
        }
    }

    /// Hvor meget politi-opmærksomhed den tiltrækker per sim-tid.
    pub fn heat_pressure(&self) -> f32 {
        match self {
            BusinessKind::Laundromat => 0.3,
            BusinessKind::CarWash => 0.2,
            BusinessKind::Pizzeria => 0.15,
            BusinessKind::Bar => 0.4,
            BusinessKind::Bodega => 0.1,
            BusinessKind::AutoShop => 0.25,
            BusinessKind::Nightclub => 0.6,
            BusinessKind::Scrapyard => 0.5,
            BusinessKind::PawnShop => 0.35,
            BusinessKind::TaxiCentral => 0.45,
            BusinessKind::Warehouse => 0.55,
            BusinessKind::StripMall => 0.4,
        }
    }
}

/// En købt/ejet business-instans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Business {
    pub id: String,
    pub kind: BusinessKind,
    pub zone: String,
    pub address: String,
    /// Cash der er "i pipeline" til at blive vasket.
    pub cash_pending: u32,
    /// Clean money akkumuleret siden sidste hævning.
    pub clean_pending: u32,
    /// Risiko for politi-razzia (0-100).
    pub risk: f32,
    /// Aktiv? False = lukket midlertidigt (efter razzia eller beslutning).
    pub active: bool,
    /// Tid siden sidste indkomst-tick.
    pub income_timer: f32,
}

impl Business {
    pub fn new(id: &str, kind: BusinessKind, zone: &str, address: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            zone: zone.to_string(),
            address: address.to_string(),
            cash_pending: 0,
            clean_pending: 0,
            risk: 0.0,
            active: true,
            income_timer: 0.0,
        }
    }

    /// Indsæt cash der skal vaskes.
    pub fn deposit_cash(&mut self, amount: u32) {
        self.cash_pending = self.cash_pending.saturating_add(amount);
    }

    /// Opdatér business per sim-frame.
    /// `dt` er sim-delta. Returnerer clean money klar til hævning.
    pub fn tick(&mut self, dt: f32) -> u32 {
        if !self.active {
            return 0;
        }
        // Vask: cash → clean med businessens rate.
        let rate = self.kind.laundering_rate();
        let to_launder = (rate * dt) as u32;
        if to_launder > 0 && self.cash_pending >= to_launder {
            self.cash_pending -= to_launder;
            self.clean_pending += to_launder;
        }
        // Passiv indkomst (clean money).
        self.income_timer += dt;
        let interval = 10.0; // hæv hver 10 sim-sek for proof.
        let mut payout = 0u32;
        if self.income_timer >= interval {
            self.income_timer = 0.0;
            payout = self.kind.passive_income();
            self.clean_pending += payout;
        }
        // Risiko stiger, falder langsomt.
        self.risk = (self.risk + self.kind.heat_pressure() * dt).min(100.0);
        self.risk = (self.risk - dt * 0.05).max(0.0);
        payout
    }

    /// Hæv akkumuleret clean money (overfør til wallet).
    pub fn collect(&mut self) -> u32 {
        let amount = self.clean_pending;
        self.clean_pending = 0;
        amount
    }

    /// Luk midlertidigt.
    pub fn close(&mut self) {
        self.active = false;
    }

    /// Genåbn.
    pub fn reopen(&mut self) {
        self.active = true;
    }
}

/// Spillerens business-portefølje.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessPortfolio {
    pub owned: Vec<Business>,
}

impl BusinessPortfolio {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn buy(&mut self, biz: Business) {
        self.owned.push(biz);
    }

    pub fn get(&self, id: &str) -> Option<&Business> {
        self.owned.iter().find(|b| b.id == id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Business> {
        self.owned.iter_mut().find(|b| b.id == id)
    }

    /// Opdatér alle businesses; returnerer total payout denne tick.
    pub fn tick_all(&mut self, dt: f32) -> u32 {
        let mut total = 0u32;
        for b in &mut self.owned {
            total = total.saturating_add(b.tick(dt));
        }
        total
    }

    /// Hæv clean money fra alle businesses.
    pub fn collect_all(&mut self) -> u32 {
        let mut total = 0u32;
        for b in &mut self.owned {
            total = total.saturating_add(b.collect());
        }
        total
    }

    /// Start med et lille laundromat (proof).
    pub fn with_starter() -> Self {
        let mut port = Self::new();
        port.buy(Business::new("biz_starter", BusinessKind::Laundromat, "east_blocks", "12 East Blocks"));
        port
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laundromat_launderes_cash_over_time() {
        let mut b = Business::new("t", BusinessKind::Laundromat, "z", "a");
        b.deposit_cash(1000);
        b.tick(60.0);
        assert!(b.clean_pending > 0);
        assert!(b.cash_pending < 1000);
    }

    #[test]
    fn collect_returns_and_clears_pending() {
        let mut b = Business::new("t", BusinessKind::Laundromat, "z", "a");
        b.clean_pending = 500;
        assert_eq!(b.collect(), 500);
        assert_eq!(b.clean_pending, 0);
    }
}