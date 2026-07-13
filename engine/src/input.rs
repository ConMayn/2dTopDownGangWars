//! Input — keyboard/mouse state tracking.
//!
//! InputState opdateres af App via winit events og læses af gameplay-systemer.
//! Actions er høj-niveau mappings (MoveUp, Fire, etc.) der kan rebindes.

use std::collections::HashSet;

use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Høj-niveau spil-actions. Mappes fra KeyCode via en InputMap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Sprint,
    Interact,
    Attack,
    ToggleDebug,
    Escape,
}

/// Input-state for én frame. Opdateres af App, læses af systemer.
#[derive(Debug, Clone, Default)]
pub struct InputState {
    /// Keys der er nede lige nu.
    keys_down: HashSet<KeyCode>,
    /// Keys der blev trykket ned DENNE frame (edge detection).
    keys_pressed: HashSet<KeyCode>,
    /// Keys der blev sluppet DENNE frame.
    keys_released: HashSet<KeyCode>,
    /// Musens position i skærm-koordinater (pixels).
    mouse_pos: (f32, f32),
    /// Musens delta siden sidste frame.
    mouse_delta: (f32, f32),
    /// Museknapper der er nede.
    mouse_buttons_down: HashSet<MouseButton>,
    /// Scroll-delta denne frame.
    scroll_delta: f32,
    /// Actions der er aktive (pressed eller held) — beregnes fra keys_down.
    actions_down: HashSet<Action>,
    /// Actions der blev aktiveret DENNE frame.
    actions_pressed: HashSet<Action>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Opdater fra en winit WindowEvent. Kaldes af App.
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Pressed,
                    physical_key: PhysicalKey::Code(code),
                    ..
                },
                ..
            } => {
                if self.keys_down.insert(*code) {
                    self.keys_pressed.insert(*code);
                }
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state: ElementState::Released,
                    physical_key: PhysicalKey::Code(code),
                    ..
                },
                ..
            } => {
                self.keys_down.remove(code);
                self.keys_released.insert(*code);
            }
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = (position.x as f32, position.y as f32);
                self.mouse_delta = (new_pos.0 - self.mouse_pos.0, new_pos.1 - self.mouse_pos.1);
                self.mouse_pos = new_pos;
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => {
                self.mouse_buttons_down.insert(*button);
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                self.mouse_buttons_down.remove(button);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 40.0,
                };
            }
            _ => {}
        }
    }

    /// Kaldes af App EFTER alle events er processed for denne frame.
    /// Beregner actions fra keys, så systemer kan bruge dem.
    pub fn end_frame(&mut self, map: &InputMap) {
        self.actions_down.clear();
        self.actions_pressed.clear();
        for (code, action) in &map.bindings {
            if self.keys_down.contains(code) {
                self.actions_down.insert(*action);
            }
            if self.keys_pressed.contains(code) {
                self.actions_pressed.insert(*action);
            }
        }
    }

    /// Kaldes af App EFTER render, for at cleare edge-events.
    /// keys_pressed/released tømmes så næste frame starter rent.
    pub fn clear_edges(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.actions_pressed.clear();
        self.mouse_delta = (0.0, 0.0);
        self.scroll_delta = 0.0;
    }

    // --- Query API ---

    pub fn key_down(&self, code: KeyCode) -> bool {
        self.keys_down.contains(&code)
    }

    pub fn key_pressed(&self, code: KeyCode) -> bool {
        self.keys_pressed.contains(&code)
    }

    pub fn action_down(&self, action: Action) -> bool {
        self.actions_down.contains(&action)
    }

    pub fn action_pressed(&self, action: Action) -> bool {
        self.actions_pressed.contains(&action)
    }

    pub fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    /// Movement vector fra WASD/piletaster, normaliseret.
    pub fn movement(&self) -> (f32, f32) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if self.action_down(Action::MoveLeft) {
            x -= 1.0;
        }
        if self.action_down(Action::MoveRight) {
            x += 1.0;
        }
        if self.action_down(Action::MoveUp) {
            y -= 1.0; // skærm-y ned = verden-y op i top-down
        }
        if self.action_down(Action::MoveDown) {
            y += 1.0;
        }
        let len = (x * x + y * y).sqrt();
        if len > 0.0 {
            (x / len, y / len)
        } else {
            (0.0, 0.0)
        }
    }
}

/// Input mapping: KeyCode → Action. Kan rebindes, data-drevet.
#[derive(Debug, Clone)]
pub struct InputMap {
    pub bindings: Vec<(KeyCode, Action)>,
}

impl Default for InputMap {
    fn default() -> Self {
        use KeyCode::*;
        Self {
            bindings: vec![
                (KeyW, Action::MoveUp),
                (ArrowUp, Action::MoveUp),
                (KeyS, Action::MoveDown),
                (ArrowDown, Action::MoveDown),
                (KeyA, Action::MoveLeft),
                (ArrowLeft, Action::MoveLeft),
                (KeyD, Action::MoveRight),
                (ArrowRight, Action::MoveRight),
                (ShiftLeft, Action::Sprint),
                (KeyE, Action::Interact),
                (Space, Action::Attack),
                (F1, Action::ToggleDebug),
                (Escape, Action::Escape),
            ],
        }
    }
}