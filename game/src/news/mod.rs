#![allow(dead_code)] // News API er public/stub til fremtidig radio/UI.

//! News — nyheder, radio og rygter.
//!
//! Fra GDD afsnit 16:
//! Radio: musikstationer, politiscanner, talk radio, nyheder, reklamer,
//! bande-dedikationer, konspirationer, rygter.
//! Nyheder reagerer på spillerens handlinger ("Police are investigating
//! a violent incident near Old Harbor. Witnesses describe a red muscle car.").
//! Rygtesystem: NPC'er siger ting baseret på verden.

use serde::{Deserialize, Serialize};

/// Nyheds-type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NewsKind {
    /// Politi-efterforskning.
    PoliceBlotter,
    /// Bande-nyhed.
    GangNews,
    /// Lokal begivenhed.
    LocalEvent,
    /// Økonomi/forretning.
    Business,
    /// Vejr/trafik.
    Weather,
    /// Spiller-relateret nyhed (reaktion på handling).
    PlayerAction,
    /// Rygte (uspecificeret kilde).
    Rumor,
}

impl NewsKind {
    pub fn label(&self) -> &'static str {
        match self {
            NewsKind::PoliceBlotter => "Police Blotter",
            NewsKind::GangNews => "Gang News",
            NewsKind::LocalEvent => "Local Event",
            NewsKind::Business => "Business",
            NewsKind::Weather => "Weather",
            NewsKind::PlayerAction => "Player Action",
            NewsKind::Rumor => "Rumor",
        }
    }
}

/// En nyhedsartikel / radio-segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub id: String,
    pub kind: NewsKind,
    pub headline: String,
    pub body: String,
    /// Sim-tidspunkt nyheden blev udgivet.
    pub time: f32,
    /// Zone nyheden handler om (eller None for generel).
    pub zone: Option<String>,
    /// Hvor "varm" nyheden er (falder over tid, fjernes når 0).
    pub relevance: f32,
}

impl NewsItem {
    pub fn new(id: &str, kind: NewsKind, headline: &str, body: &str, time: f32) -> Self {
        Self {
            id: id.to_string(),
            kind,
            headline: headline.to_string(),
            body: body.to_string(),
            time,
            zone: None,
            relevance: 100.0,
        }
    }

    /// Opdatér relevance (falder over tid).
    pub fn tick(&mut self, dt: f32) -> bool {
        self.relevance = (self.relevance - dt * 0.5).max(0.0);
        self.relevance <= 0.0
    }
}

/// Nyheds-system: samler og alder nyheder.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewsSystem {
    pub items: Vec<NewsItem>,
    pub id_counter: u32,
}

impl NewsSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&mut self, kind: NewsKind, headline: &str, body: &str, time: f32) -> &NewsItem {
        self.id_counter += 1;
        let id = format!("news_{}", self.id_counter);
        self.items.push(NewsItem::new(&id, kind, headline, body, time));
        tracing::info!("News [{}]: {}", kind.label(), headline);
        self.items.last().unwrap()
    }

    /// Publicer en spiller-relateret nyhed baseret på handling.
    pub fn publish_player_action(&mut self, action_desc: &str, time: f32) {
        self.publish(
            NewsKind::PlayerAction,
            "Police investigate incident",
            &format!("Police are investigating an incident. {}", action_desc),
            time,
        );
    }

    /// Publicer et rygte.
    pub fn publish_rumor(&mut self, text: &str, time: f32) {
        self.publish(
            NewsKind::Rumor,
            "Street rumor",
            text,
            time,
        );
    }

    /// Opdatér alle nyheder; fjern irrelevante.
    pub fn tick(&mut self, dt: f32) -> usize {
        let mut keep = Vec::new();
        let mut removed = 0;
        for mut item in self.items.drain(..) {
            let expired = item.tick(dt);
            if expired {
                removed += 1;
            } else {
                keep.push(item);
            }
        }
        self.items = keep;
        removed
    }

    pub fn latest(&self, kind: Option<NewsKind>) -> Vec<&NewsItem> {
        let mut items: Vec<&NewsItem> = self
            .items
            .iter()
            .filter(|i| kind.map(|k| i.kind == k).unwrap_or(true))
            .collect();
        items.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
        items
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }
}