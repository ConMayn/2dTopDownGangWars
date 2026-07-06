//! heat_core — custom 2D game engine for Heat City.
//!
//! Fase 0: minimal proof-of-concept. Window + wgpu clear.
//! Se docs/02-technical-design/tdd.md for fuld arkitektur.

pub mod app;
pub mod render;

pub use app::{App, AppConfig, AppError};
pub use render::Renderer;