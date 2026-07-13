//! heat_game — Heat City game binary.
//!
//! Fase 2: WorldPlugin med tilemap, collision, NPC patrol og player movement.
//! Beviser: by-zone (East Blocks lille), gå med WASD, collision mod bygninger,
//! NPC'ere der patroljerer waypoints, kamera follow + clamp.

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

    tracing::info!("Heat City Fase 2 — starter");

    let app = AppBuilder::new()
        .plugin(WorldPlugin::new())
        .build()?;
    app.run()?;

    Ok(())
}