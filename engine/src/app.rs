//! App — hoved-loop og lifecycle for Heat City engine.
//!
//! Fase 1: Plugin-baseret arkitektur. App ejer World, Renderer, Input, Time,
//! Assets. Plugins (registreret af game crate) opdaterer sim og renderer.
//! Main loop kører fixed timestep simulering med interpoleret rendering.

use std::sync::Arc;

use thiserror::Error;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::assets::AssetStore;
use crate::debug::{DebugOverlay, FpsCounter};
use crate::ecs::World;
use crate::input::{Action, InputMap, InputState};
use crate::render::{Camera, Renderer, SpriteBatch};
use crate::time::Time;

/// Konfiguration for App-oprettelse.
pub struct AppConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: "Heat City".to_string(),
            window_width: 1280,
            window_height: 720,
        }
    }
}

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

/// Plugin — implementeres af game crate for at tilføje gameplay-systemer.
pub trait Plugin {
    /// Kaldes én gang efter renderer + assets er klar. For init: spawn entities, load assets.
    fn init(&mut self, ctx: &mut InitContext);

    /// Fixed-update (60 Hz). Simulering: bevægelse, AI, fysik.
    fn update(&mut self, ctx: &mut UpdateContext);

    /// Render (hver frame). Tilføj sprites til batch.
    fn render(&mut self, ctx: &mut RenderContext);
}

/// Kontekst givet til Plugin::init.
pub struct InitContext<'a> {
    pub world: &'a mut World,
    pub assets: &'a mut AssetStore,
    pub camera: &'a mut Camera,
}

/// Kontekst givet til Plugin::update (fixed timestep).
pub struct UpdateContext<'a> {
    pub world: &'a mut World,
    pub input: &'a InputState,
    pub camera: &'a mut Camera,
    pub dt: f32,
    pub sim_time: f32,
}

/// Kontekst givet til Plugin::render.
pub struct RenderContext<'a> {
    pub world: &'a World,
    pub batch: &'a mut SpriteBatch,
    pub camera: &'a Camera,
    pub alpha: f32,
    pub debug: &'a DebugOverlay,
}

/// Builder til at konfigurere App før run.
pub struct AppBuilder {
    config: AppConfig,
    plugins: Vec<Box<dyn Plugin>>,
    input_map: InputMap,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
            plugins: Vec::new(),
            input_map: InputMap::default(),
        }
    }

    pub fn config(mut self, config: AppConfig) -> Self {
        self.config = config;
        self
    }

    pub fn plugin<P: Plugin + 'static>(mut self, plugin: P) -> Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    pub fn input_map(mut self, map: InputMap) -> Self {
        self.input_map = map;
        self
    }

    pub fn build(self) -> Result<App, AppError> {
        Ok(App {
            config: self.config,
            plugins: self.plugins,
            input_map: self.input_map,
        })
    }
}

/// Hoved-App.
pub struct App {
    config: AppConfig,
    plugins: Vec<Box<dyn Plugin>>,
    input_map: InputMap,
}

impl App {
    pub fn run(self) -> Result<(), AppError> {
        tracing::info!("Heat City: main loop startet (Fase 1)");

        let event_loop = EventLoop::new().map_err(|e| AppError::EventLoop(e.to_string()))?;

        let mut state = AppState {
            window: None,
            renderer: None,
            config: self.config,
            plugins: self.plugins,
            input_map: self.input_map,
            world: World::new(),
            assets: AssetStore::new(),
            input: InputState::new(),
            time: Time::new(),
            camera: Camera::new(1280.0, 720.0),
            batch: SpriteBatch::new(),
            fps: FpsCounter::new(),
            debug: DebugOverlay::default(),
            initialized: false,
            running: true,
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
    plugins: Vec<Box<dyn Plugin>>,
    input_map: InputMap,
    world: World,
    assets: AssetStore,
    input: InputState,
    time: Time,
    camera: Camera,
    batch: SpriteBatch,
    fps: FpsCounter,
    debug: DebugOverlay,
    initialized: bool,
    running: bool,
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
                        let (w, h) = renderer.surface_size();
                        self.camera.resize(w as f32, h as f32);
                        self.assets.bind_gpu(
                            renderer.device().clone(),
                            renderer.queue().clone(),
                        );
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
                    self.camera
                        .resize(physical_size.width as f32, physical_size.height as f32);
                }
            }
            WindowEvent::RedrawRequested => {
                self.input.handle_event(&WindowEvent::RedrawRequested); // no-op, bare for formen
                self.render_frame();
                self.input.clear_edges();
            }
            ref other => {
                self.input.handle_event(other);
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

impl AppState {
    fn init_plugins(&mut self) {
        if self.initialized {
            return;
        }
        let Some(renderer) = self.renderer.as_ref() else {
            return;
        };
        let _ = renderer; // allerede bound via assets.bind_gpu

        for plugin in &mut self.plugins {
            plugin.init(&mut InitContext {
                world: &mut self.world,
                assets: &mut self.assets,
                camera: &mut self.camera,
            });
        }
        self.initialized = true;
        tracing::info!("Plugins initialiseret ({} plugins)", self.plugins.len());
    }

    fn render_frame(&mut self) {
        self.init_plugins();

        // 1. Tick time.
        let steps = self.time.tick();
        self.fps.tick();
        self.debug.fps = self.fps.fps();
        self.debug.frame_delta = self.time.frame_delta;

        // 2. Process input → actions.
        self.input.end_frame(&self.input_map);

        // 3. Toggle debug på F1.
        if self.input.action_pressed(Action::ToggleDebug) {
            self.debug.toggle();
        }

        // 4. Fixed updates (simulering).
        for _ in 0..steps {
            let mut ctx = UpdateContext {
                world: &mut self.world,
                input: &self.input,
                camera: &mut self.camera,
                dt: self.time.fixed_dt(),
                sim_time: self.time.sim_time,
            };
            for plugin in &mut self.plugins {
                plugin.update(&mut ctx);
            }
            self.time.step_done();
        }
        self.time.calc_alpha();
        self.debug.sim_time = self.time.sim_time;
        self.debug.entity_count = self.world.entity_count();

        // 5. Render.
        self.batch.clear();
        for plugin in &mut self.plugins {
            plugin.render(&mut RenderContext {
                world: &self.world,
                batch: &mut self.batch,
                camera: &self.camera,
                alpha: self.time.alpha,
                debug: &self.debug,
            });
        }

        if let Some(renderer) = self.renderer.as_mut() {
            if self.batch.len() > 0 {
                if let Err(e) = self.batch.render(renderer, &self.assets, &self.camera) {
                    tracing::error!("Render fejl: {e}");
                }
            } else {
                if let Err(e) = renderer.clear_only(&self.camera) {
                    tracing::error!("Clear fejl: {e}");
                }
            }
        }

        // 6. Debug output (log-baseret for Fase 1; on-screen i Fase 11).
        if self.debug.enabled && self.time.sim_time % 1.0 < 0.016 {
            let txt = self.debug.text();
            if !txt.is_empty() && self.time.sim_time > 0.0 {
                tracing::info!("{}", txt);
            }
        }
    }
}