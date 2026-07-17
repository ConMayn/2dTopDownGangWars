//! heat_game — Heat City game binary.
//!
//! Fase 11: alle spil-systemer + save, UI, audio.

mod audio;
mod businesses;
mod crew;
mod dialog;
mod director;
mod economy;
mod events;
mod factions;
mod font;
mod heists;
mod missions;
mod news;
mod police;
mod rivals;
mod safehouses;
mod save;
mod sprites;
mod systems;
mod text;
mod ui;
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

    tracing::info!("Heat City Fase 11 — starter");

    let app = AppBuilder::new()
        .plugin(WorldPlugin::new())
        .build()?;
    app.run()?;

    Ok(())
}