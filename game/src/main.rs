//! heat_game — Heat City game binary.
//!
//! Fase 7: WorldPlugin med NPC FSM, missioner, dialog, economy, factions, politi.

mod dialog;
mod economy;
mod factions;
mod missions;
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

    tracing::info!("Heat City Fase 7 — starter");

    let app = AppBuilder::new()
        .plugin(WorldPlugin::new())
        .build()?;
    app.run()?;

    Ok(())
}