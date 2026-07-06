//! App — hoved-loop og lifecycle for Heat City engine.
//!
//! Fase 0 proof: åbner et vindue, clearer skærmen, kører indtil Escape/Close.
//! Fase 1+: udvides med ECS, input, assets, fixed timestep, plugins.
//!
//! Bruger winit 0.30's ApplicationHandler-model: EventLoop::run driver main loop,
//! og events dispatches til AppState.

use std::sync::Arc;
use std::time::Instant;

use thiserror::Error;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::render::Renderer;

/// Konfiguration for App-oprettelse.
pub struct AppConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub clear_color: [f64; 4],
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "Heat City".to_string(),
            window_width: 1280,
            window_height: 720,
            clear_color: [0.1, 0.2, 0.4, 1.0], // mørk blå
        }
    }
}

/// App-fejl.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("window error: {0}")]
    Window(String),
    #[error("gpu error: {0}")]
    Gpu(String),
    #[error("render error: {0}")]
    Render(String),
    #[error("event loop error: {0}")]
    EventLoop(String),
}

/// Hoved-App. Entry point — opret med `App::new`, kør med `app.run()`.
pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Result<Self, AppError> {
        Ok(Self { config })
    }

    /// Kør main loop. Blokerer indtil vinduet lukker.
    pub fn run(self) -> Result<(), AppError> {
        tracing::info!("Heat City: main loop startet");

        let event_loop = EventLoop::new().map_err(|e| AppError::EventLoop(e.to_string()))?;

        let mut state = AppState {
            window: None,
            renderer: None,
            config: self.config,
            running: true,
            last_frame: Instant::now(),
        };

        let result = event_loop
            .run_app(&mut state)
            .map_err(|e| AppError::EventLoop(e.to_string()));

        tracing::info!("Heat City: main loop afsluttet");
        result
    }
}

/// Internt state der implementerer winit's ApplicationHandler.
struct AppState {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    config: AppConfig,
    running: bool,
    last_frame: Instant,
}

impl ApplicationHandler for AppState {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title(&self.config.window_title)
            .with_inner_size(winit::dpi::PhysicalSize::new(
                self.config.window_width,
                self.config.window_height,
            ));
        match event_loop.create_window(attrs) {
            Ok(window) => {
                let window = Arc::new(window);
                match pollster::block_on(Renderer::new(window.clone())) {
                    Ok(renderer) => {
                        self.window = Some(window);
                        self.renderer = Some(renderer);
                    }
                    Err(e) => {
                        tracing::error!("Renderer init fejlet: {e}");
                        event_loop.exit();
                    }
                }
            }
            Err(e) => {
                tracing::error!("Window creation fejlet: {e}");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                self.running = false;
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(physical_size.width, physical_size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                let _dt = self.last_frame.elapsed();
                self.last_frame = Instant::now();

                if let Some(renderer) = self.renderer.as_mut() {
                    if let Err(e) = renderer.render(self.config.clear_color) {
                        tracing::error!("Render fejl: {e}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}