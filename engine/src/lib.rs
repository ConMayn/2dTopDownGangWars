//! heat_core — custom 2D game engine for Heat City.
//!
//! Fase 1: Engine Core — ECS, input, time, assets, sprite rendering, camera, debug.
//! Se docs/02-technical-design/tdd.md for fuld arkitektur.

pub mod app;
pub mod assets;
pub mod debug;
pub mod ecs;
pub mod input;
pub mod math;
pub mod render;
pub mod time;

pub use app::{App, AppBuilder, AppConfig, AppError, InitContext, Plugin, RenderContext, UpdateContext};
pub use assets::{AssetStore, GpuTexture, Handle, TextureHandle};
pub use ecs::{Component, EntityId, World};
pub use input::{Action, InputMap, InputState};
pub use math::{Aabb, Color, Rect, Transform, Vec2};
pub use render::{Camera, Renderer, Sprite, SpriteBatch};
pub use time::Time;