#![allow(dead_code)] // UI API er public/stub til fremtidig rendering.

//! UI — HUD, menuer, telefon, inventory display.
//!
//! Fase 11: HUD-state der viser wallet, heat, mission objective, crew status,
//! safehouse info, news ticker. Renderes som overlay-tekst (proof).
//! Fremtid: rigtig UI rendering med panels, knapper, telefon-interface.

use serde::{Deserialize, Serialize};

/// HUD-elementer der vises konstant.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HudState {
    /// Cash (sorte penge).
    pub cash: u32,
    /// Clean money.
    pub clean: u32,
    /// Heat-level label.
    pub heat_label: String,
    /// Heat points.
    pub heat_points: f32,
    /// Aktive mission titler.
    pub active_missions: Vec<String>,
    /// Nuværende mission objective beskrivelse.
    pub current_objective: Option<String>,
    /// Verdens-tid formateret.
    pub time_formatted: String,
    /// Time-of-day label.
    pub time_of_day_label: String,
    /// Aktive crew-medlemmer (navn + status).
    pub crew_status: Vec<(String, String)>,
    /// Aktive events i nærheden.
    pub nearby_events: Vec<String>,
    /// Nyheds-ticker (seneste headline).
    pub news_ticker: Option<String>,
    /// Dialog-tekst hvis aktiv.
    pub dialog_text: Option<String>,
    /// Dialog-valg.
    pub dialog_choices: Vec<String>,
}

impl HudState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_heat(&mut self, label: &str, points: f32) {
        self.heat_label = label.to_string();
        self.heat_points = points;
    }

    pub fn set_objective(&mut self, text: &str) {
        self.current_objective = Some(text.to_string());
    }

    pub fn clear_objective(&mut self) {
        self.current_objective = None;
    }

    pub fn set_dialog(&mut self, text: &str, choices: &[String]) {
        self.dialog_text = Some(text.to_string());
        self.dialog_choices = choices.to_vec();
    }

    pub fn clear_dialog(&mut self) {
        self.dialog_text = None;
        self.dialog_choices.clear();
    }

    pub fn set_news_ticker(&mut self, headline: &str) {
        self.news_ticker = Some(headline.to_string());
    }

    pub fn render_debug(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Cash: ${}  Clean: ${}", self.cash, self.clean));
        lines.push(format!("Heat: {} ({:.0})", self.heat_label, self.heat_points));
        if let Some(ref obj) = self.current_objective {
            lines.push(format!("Objective: {}", obj));
        }
        if !self.active_missions.is_empty() {
            lines.push(format!("Missions: {}", self.active_missions.join(", ")));
        }
        lines.push(format!("Time: {} ({})", self.time_formatted, self.time_of_day_label));
        if !self.crew_status.is_empty() {
            let crew: Vec<String> = self.crew_status.iter().map(|(n, s)| format!("{} [{}]", n, s)).collect();
            lines.push(format!("Crew: {}", crew.join(", ")));
        }
        if !self.nearby_events.is_empty() {
            lines.push(format!("Events: {}", self.nearby_events.join(", ")));
        }
        if let Some(ref news) = self.news_ticker {
            lines.push(format!("News: {}", news));
        }
        if let Some(ref dialog) = self.dialog_text {
            lines.push(format!("--- Dialog ---"));
            lines.push(dialog.clone());
            for (i, choice) in self.dialog_choices.iter().enumerate() {
                lines.push(format!("  [{}] {}", i + 1, choice));
            }
        }
        lines.join("\n")
    }
}

/// Menu-state (fremtidig: pause menu, options).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuScreen {
    None,
    Pause,
    Phone,
    Inventory,
    SafehouseMenu,
    MissionLog,
    FactionLog,
    Options,
    SaveLoad,
}

impl MenuScreen {
    pub fn label(&self) -> &'static str {
        match self {
            MenuScreen::None => "",
            MenuScreen::Pause => "Pause",
            MenuScreen::Phone => "Phone",
            MenuScreen::Inventory => "Inventory",
            MenuScreen::SafehouseMenu => "Safehouse",
            MenuScreen::MissionLog => "Missions",
            MenuScreen::FactionLog => "Factions",
            MenuScreen::Options => "Options",
            MenuScreen::SaveLoad => "Save/Load",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiState {
    pub current_menu: Option<MenuScreen>,
    pub hud: HudState,
}

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open_menu(&mut self, screen: MenuScreen) {
        self.current_menu = Some(screen);
    }

    pub fn close_menu(&mut self) {
        self.current_menu = None;
    }

    pub fn toggle_menu(&mut self, screen: MenuScreen) {
        if self.current_menu == Some(screen) {
            self.close_menu();
        } else {
            self.open_menu(screen);
        }
    }
}