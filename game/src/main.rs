//! heat_game — Heat City game binary.
//!
//! Fase 1 proof: TestPlugin med en sprite der bevæger sig med WASD.
//! Beviser: ECS, input, fixed timestep, sprite rendering, camera.

use heat_core::{
    AppBuilder, Color, InitContext, Plugin,
    RenderContext, Sprite, TextureHandle, UpdateContext, Vec2,
    EntityId, Rect,
};
use heat_core::input::Action;

/// Player-komponent: position + speed.
#[derive(Debug, Clone, Copy)]
struct Player {
    pos: Vec2,
    speed: f32,
}

/// Test-plugin: spawner en player, bevæger med WASD, renderer sprite.
struct TestPlugin {
    texture: Option<TextureHandle>,
    player_entity: Option<heat_core::EntityId>,
}

impl TestPlugin {
    fn new() -> Self {
        Self {
            texture: None,
            player_entity: None,
        }
    }
}

impl Plugin for TestPlugin {
    fn init(&mut self, ctx: &mut InitContext) {
        // Generer en simpel test-texture (64x64 rød kvadrat) og load den.
        // For Fase 1 bruger vi en placeholder: vi opretter en texture fra code.
        // Da AssetStore::load_texture læser fra sti, skriver vi en PNG til temp først.
        let tex_path = std::env::temp_dir().join("heat_city_test_player.png");
        let mut imgbuf = image::ImageBuffer::new(64, 64);
        for pixel in imgbuf.pixels_mut() {
            *pixel = image::Rgba([220, 60, 60, 255]);
        }
        let _ = image::save_buffer(&tex_path, &imgbuf, 64, 64, image::ExtendedColorType::Rgba8);

        self.texture = ctx.assets.load_texture(&tex_path).ok();

        // Spawn player i centrum af verden.
        self.player_entity = Some(ctx.world.spawn_one(Player {
            pos: Vec2::new(400.0, 300.0),
            speed: 200.0,
        }));

        // Sæt camera bounds (lille test-verden 800x600).
        ctx.camera.set_bounds(heat_core::Rect::new(0, 0, 800, 600));
        ctx.camera.position = Vec2::new(400.0, 300.0);

        tracing::info!("TestPlugin init: player spawned, texture loaded");
    }

    fn update(&mut self, ctx: &mut UpdateContext) {
        // Bevæg player med WASD.
        let (mx, my) = ctx.input.movement();
        let speed = if ctx.input.action_down(Action::Sprint) { 400.0 } else { 200.0 };
        let dx = mx * speed * ctx.dt;
        let dy = my * speed * ctx.dt;

        if let Some(entity) = self.player_entity {
            if let Ok(mut player_ref) = ctx.world.inner_mut().get::<&mut Player>(entity) {
                let player = &mut *player_ref;
                player.pos.x += dx;
                player.pos.y += dy;
                // Clamp til verden.
                player.pos.x = player.pos.x.clamp(32.0, 768.0);
                player.pos.y = player.pos.y.clamp(32.0, 568.0);
                // Camera follow.
                ctx.camera.follow(player.pos);
            }
        }
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        if let (Some(tex), Some(entity)) = (self.texture, self.player_entity) {
            if let Ok(player_ref) = ctx.world.inner().get::<&Player>(entity) {
                let pos = player_ref.pos;
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: pos,
                    size: Vec2::new(64.0, 64.0),
                    rotation: 0.0,
                    color: Color::WHITE,
                    layer: heat_core::render::LAYER_ENTITIES,
                });
            }
        }

        // Tegn et "grid" af grå fliser som baggrund (viser at rendering virker).
        let grid_tex = self.texture;
        if let Some(tex) = grid_tex {
            for x in (0..800).step_by(64) {
                for y in (0..600).step_by(64) {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: Vec2::new(x as f32 + 32.0, y as f32 + 32.0),
                        size: Vec2::new(62.0, 62.0),
                        rotation: 0.0,
                        color: Color::rgba(0.15, 0.15, 0.18, 1.0),
                        layer: heat_core::render::LAYER_GROUND,
                    });
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Heat City Fase 1 — starter");

    let app = AppBuilder::new()
        .plugin(TestPlugin::new())
        .build()?;
    app.run()?;

    Ok(())
}