//! Dialog — simple dialog-træer med valg, betingelser og effekter.
//!
//! Fase 7: text-based dialog med RON-definitioner, betingelser baseret på
//! reputation/quest state, og effekter der ændrer state.

use serde::{Deserialize, Serialize};

/// En dialog-node med tekst og valg.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogNode {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub choices: Vec<DialogChoice>,
}

/// Et valg i en dialog-node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogChoice {
    pub text: String,
    pub next: Option<String>,
    pub condition: Option<DialogCondition>,
    pub effects: Vec<DialogEffect>,
}

/// Betingelse for at et valg vises.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DialogCondition {
    HasItem { item_id: String },
    HasCash { amount: u32 },
    FactionTrustMin { faction: String, min: f32 },
    MissionActive { mission_id: String },
    MissionCompleted { mission_id: String },
}

/// Effekt af at vælge et dialogvalg.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DialogEffect {
    StartMission { mission_id: String },
    AdvanceMission { mission_id: String, objective_idx: usize },
    GiveItem { item_id: String, count: u32 },
    TakeItem { item_id: String, count: u32 },
    GiveCash { amount: u32, clean: bool },
    TakeCash { amount: u32, clean: bool },
    ReputationEvent { event_kind: String },
}

/// Aktiv dialog-state: current node id + træ.
#[derive(Debug, Clone)]
pub struct ActiveDialog {
    pub tree: DialogTree,
    pub current: String,
}

impl ActiveDialog {
    pub fn new(tree: DialogTree, start: &str) -> Option<Self> {
        tree.nodes.get(start)?;
        Some(Self {
            tree,
            current: start.to_string(),
        })
    }

    pub fn current_node(&self) -> &DialogNode {
        // Safe: constructed from valid start.
        self.tree.nodes.get(&self.current).unwrap()
    }

    pub fn choose(&mut self, choice_idx: usize) -> Option<DialogChoice> {
        let node = self.current_node();
        let choice = node.choices.get(choice_idx)?.clone();
        if let Some(ref next) = choice.next {
            self.current = next.clone();
        }
        Some(choice)
    }
}

/// Dialog-træ: map node_id → node.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DialogTree {
    pub start: String,
    pub nodes: std::collections::HashMap<String, DialogNode>,
}

impl DialogTree {
    pub fn new(start: &str) -> Self {
        Self {
            start: start.to_string(),
            nodes: std::collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, node: DialogNode) {
        self.nodes.insert(node.id.clone(), node);
    }
}

/// Utility: byg en RON-streng for et simpelt test-dialog-træ.
pub fn demo_tree() -> DialogTree {
    let mut tree = DialogTree::new("greet");
    tree.add(DialogNode {
        id: "greet".into(),
        speaker: "Lil' P".into(),
        text: "Yo, du ser ud som en der mangler arbejde. Jeg har et job til dig.".into(),
        choices: vec![
            DialogChoice {
                text: "Hvad er jobbet?".into(),
                next: Some("details".into()),
                condition: None,
                effects: vec![],
            },
            DialogChoice {
                text: "Ikke lige nu.".into(),
                next: None,
                condition: None,
                effects: vec![],
            },
        ],
    });
    tree.add(DialogNode {
        id: "details".into(),
        speaker: "Lil' P".into(),
        text: "En rival-bandes bil holder ved den gamle garage. Stjæl den og bring den tilbage.".into(),
        choices: vec![
            DialogChoice {
                text: "Jeg gør det.".into(),
                next: None,
                condition: None,
                effects: vec![DialogEffect::StartMission { mission_id: "steal_rival_car".into() }],
            },
            DialogChoice {
                text: "For risikabelt.".into(),
                next: None,
                condition: None,
                effects: vec![],
            },
        ],
    });
    tree
}
