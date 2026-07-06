//! heat_game — Heat City game binary.
//!
//! Fase 0 proof: opretter App med default config og kører indtil Escape/Close.

use heat_core::{App, AppConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Heat City Fase 0 proof — starter");

    let config = AppConfig::default();
    let app = App::new(config)?;
    app.run()?;

    Ok(())
}