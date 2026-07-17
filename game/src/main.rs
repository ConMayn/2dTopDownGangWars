//! heat_game — Heat City game binary.
//!
//! Fase 4: WorldPlugin med NPC FSM, dag/nat-cyklus, dialog, spatial grid.

mod factions;
mod police;
mod systems;
mod world;

use heat_core::AppBuilder;
use world::WorldPlugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Heat City Fase 6 — starter");

    let app = AppBuilder::new()
        .plugin(WorldPlugin::new())
        .build()?;
    app.run()?;

    Ok(())
}