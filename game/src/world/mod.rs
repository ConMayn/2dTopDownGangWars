//! World — binder tilemap, zone, NPC sammen.
//!
//! WorldPlugin er en engine Plugin der ejer den nuværende zone,
//! renderer tilemap, opdaterer NPC'ere og lader spilleren bevæge sig
//! med collision mod tilemap.

pub mod collision;
pub mod npc;
pub mod tilemap;
pub mod tiles;
pub mod zone;

use heat_core::{
    AppError, Color, EntityId, InitContext, Plugin, Rect,
    RenderContext, Sprite, TextureHandle, UpdateContext, Vec2, World,
};
use heat_core::input::Action;

use collision::move_and_collide;
use npc::{Npc, NpcType, Patrol, update_npc_patrol};
use tilemap::{Tilemap, TilemapDef};
use tiles::{TileDef, TileRegistry, TileType};
use zone::ZoneDef;

/// Player-komponent.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
}

/// Hoved-plugin for Fase 2: byen + spiller + NPC.
pub struct WorldPlugin {
    tile_registry: TileRegistry,
    tilemap: Option<Tilemap>,
    zone: Option<ZoneDef>,
    player_entity: Option<EntityId>,
    player_texture: Option<TextureHandle>,
    npc_texture: Option<TextureHandle>,
}

impl WorldPlugin {
    pub fn new() -> Self {
        Self {
            tile_registry: TileRegistry::new(),
            tilemap: None,
            zone: None,
            player_entity: None,
            player_texture: None,
            npc_texture: None,
        }
    }

    /// Opret test-assets (PNG filer) for Fase 2 — i Fase 5+ hentes rigtige assets.
    fn create_test_assets(&self, assets: &mut heat_core::AssetStore) -> Result<(), AppError> {
        // Player texture: 32x32 blå
        let player_path = std::env::temp_dir().join("heat_city_player.png");
        let mut img = image::ImageBuffer::new(32, 32);
        for p in img.pixels_mut() {
            *p = image::Rgba([80, 140, 220, 255]);
        }
        let _ = image::save_buffer(&player_path, &img, 32, 32, image::ExtendedColorType::Rgba8);

        // NPC texture: 24x24 grøn
        let npc_path = std::env::temp_dir().join("heat_city_npc.png");
        let mut img2 = image::ImageBuffer::new(24, 24);
        for p in img2.pixels_mut() {
            *p = image::Rgba([100, 180, 100, 255]);
        }
        let _ = image::save_buffer(&npc_path, &img2, 24, 24, image::ExtendedColorType::Rgba8);

        // Load dem (gemmer handles internt)
        let _ = assets.load_texture(&player_path)?;
        let _ = assets.load_texture(&npc_path)?;
        Ok(())
    }

    /// Byg tile registry med basale tile-typer.
    fn build_tile_registry(&self) -> TileRegistry {
        let mut reg = TileRegistry::new();
        // asfalt (gader)
        reg.insert(
            "asphalt".into(),
            TileType {
                def: TileDef {
                    id: "asphalt".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.12, 0.12, 0.14, 1.0],
                },
                texture: None,
            },
        );
        // fortov
        reg.insert(
            "sidewalk".into(),
            TileType {
                def: TileDef {
                    id: "sidewalk".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.25, 0.25, 0.28, 1.0],
                },
                texture: None,
            },
        );
        // græs
        reg.insert(
            "grass".into(),
            TileType {
                def: TileDef {
                    id: "grass".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.15, 0.3, 0.15, 1.0],
                },
                texture: None,
            },
        );
        // bygning (solid)
        reg.insert(
            "building".into(),
            TileType {
                def: TileDef {
                    id: "building".into(),
                    solid: true,
                    layer: heat_core::render::LAYER_ENTITIES,
                    color: [0.35, 0.3, 0.25, 1.0],
                },
                texture: None,
            },
        );
        // mur (solid)
        reg.insert(
            "wall".into(),
            TileType {
                def: TileDef {
                    id: "wall".into(),
                    solid: true,
                    layer: heat_core::render::LAYER_ENTITIES,
                    color: [0.4, 0.35, 0.35, 1.0],
                },
                texture: None,
            },
        );
        reg
    }

    /// Byg en test-zone (East Blocks lille version).
    /// 25x19 tiles à 32px = 800x608 px.
    fn build_test_tilemap(&self) -> Tilemap {
        let w = 25usize;
        let h = 19usize;
        let mut tiles = vec!["asphalt".to_string(); w * h];

        // Kant-mure (hele perimeter = solid).
        for x in 0..w {
            tiles[x] = "wall".into(); // top
            tiles[(h - 1) * w + x] = "wall".into(); // bottom
        }
        for y in 0..h {
            tiles[y * w] = "wall".into(); // left
            tiles[y * w + (w - 1)] = "wall".into(); // right
        }

        // Fortov langs gader (horisontal + vertikal sti).
        let mid_y = h / 2;
        let mid_x = w / 2;
        for x in 1..w - 1 {
            tiles[mid_y * w + x] = "sidewalk".into();
            tiles[(mid_y + 1) * w + x] = "asphalt".into(); // gade
            tiles[(mid_y - 1) * w + x] = "asphalt".into();
        }
        for y in 1..h - 1 {
            tiles[y * w + mid_x] = "sidewalk".into();
            tiles[y * w + (mid_x + 1)] = "asphalt".into();
            tiles[y * w + (mid_x - 1)] = "asphalt".into();
        }

        // Bygninger i hjørner (solid blocks).
        let buildings = [
            (2, 2, 4, 3),
            (18, 2, 4, 3),
            (2, 14, 4, 3),
            (18, 14, 4, 3),
        ];
        for (bx, by, bw, bh) in buildings {
            for y in by..by + bh {
                for x in bx..bx + bw {
                    if y < h - 1 && x < w - 1 {
                        tiles[y * w + x] = "building".into();
                    }
                }
            }
        }

        // Græs i et par pletter.
        tiles[3 * w + 3] = "grass".into();
        tiles[3 * w + 4] = "grass".into();
        tiles[4 * w + 3] = "grass".into();
        tiles[15 * w + 21] = "grass".into();
        tiles[15 * w + 22] = "grass".into();

        let def = TilemapDef {
            width: w,
            height: h,
            tiles,
        };
        Tilemap::new(def, 32.0)
    }

    /// Spawn NPC'ere med patrol-ruter.
    fn spawn_npcs(&self, world: &mut World, tilemap: &Tilemap) {
        let npc_spawns = [
            (Vec2::new(100.0, 100.0), vec![Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0), Vec2::new(200.0, 150.0), Vec2::new(100.0, 150.0)]),
            (Vec2::new(600.0, 400.0), vec![Vec2::new(600.0, 400.0), Vec2::new(650.0, 400.0), Vec2::new(650.0, 450.0), Vec2::new(600.0, 450.0)]),
            (Vec2::new(400.0, 200.0), vec![Vec2::new(400.0, 200.0), Vec2::new(450.0, 200.0), Vec2::new(450.0, 250.0), Vec2::new(400.0, 250.0)]),
        ];

        for (pos, waypoints) in npc_spawns {
            let npc = Npc {
                pos,
                npc_type: NpcType::Pedestrian,
                speed: 60.0,
                current_waypoint: 0,
                color: [0.4, 0.7, 0.4, 1.0],
            };
            let patrol = Patrol { waypoints };
            let entity = world.spawn((npc, patrol));
            let _ = tilemap; // brugt til validering senere
        }
    }
}

impl Plugin for WorldPlugin {
    fn init(&mut self, ctx: &mut InitContext) {
        // Opret test assets.
        let _ = self.create_test_assets(ctx.assets);
        self.player_texture = ctx
            .assets
            .get_texture_by_path(&std::env::temp_dir().join("heat_city_player.png"))
            .copied();
        self.npc_texture = ctx
            .assets
            .get_texture_by_path(&std::env::temp_dir().join("heat_city_npc.png"))
            .copied();

        // Tile registry.
        self.tile_registry = self.build_tile_registry();

        // Tilemap.
        let tilemap = self.build_test_tilemap();
        let px_w = tilemap.pixel_width();
        let px_h = tilemap.pixel_height();
        self.tilemap = Some(tilemap);

        // Sæt kamera-bounds til tilemap-størrelse.
        ctx.camera.set_bounds(Rect::new(0, 0, px_w as i32, px_h as i32));
        ctx.camera.position = Vec2::new(px_w * 0.5, px_h * 0.5);

        // Spawn player i centrum.
        self.player_entity = Some(ctx.world.spawn_one(Player {
            pos: Vec2::new(px_w * 0.5, px_h * 0.5),
            speed: 180.0,
        }));

        // Spawn NPC's.
        if let Some(ref tm) = self.tilemap {
            self.spawn_npcs(ctx.world, tm);
        }

        tracing::info!(
            "WorldPlugin init: tilemap {}x{}, player + NPCs spawned",
            px_w as i32,
            px_h as i32
        );
    }

    fn update(&mut self, ctx: &mut UpdateContext) {
        let Some(tilemap) = &self.tilemap else { return };

        // Player movement med collision.
        let (mx, my) = ctx.input.movement();
        let speed = if ctx.input.action_down(Action::Sprint) { 320.0 } else { 180.0 };
        let delta = Vec2::new(mx * speed * ctx.dt, my * speed * ctx.dt);

        if let Some(entity) = self.player_entity {
            if let Ok(mut player_ref) = ctx.world.inner_mut().get::<&mut Player>(entity) {
                let player = &mut *player_ref;
                let half = Vec2::new(16.0, 16.0); // player hitbox 32x32
                let result = move_and_collide(player.pos, half, delta, tilemap, &self.tile_registry);
                player.pos = result.new_pos;
                ctx.camera.follow(player.pos);
            }
        }

        // NPC patrol update.
        update_npc_patrol(ctx.world, tilemap, &self.tile_registry, ctx.dt);
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        let Some(tilemap) = &self.tilemap else { return };

        // Render tilemap (kun synlige tiles).
        tilemap.render(ctx, &self.tile_registry);

        // Render NPC's.
        let inner = ctx.world.inner();
        for (_, (npc,)) in &mut inner.query::<(&Npc,)>() {
            if let Some(tex) = self.npc_texture {
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: npc.pos,
                    size: Vec2::new(24.0, 24.0),
                    rotation: 0.0,
                    color: Color::rgba(npc.color[0], npc.color[1], npc.color[2], npc.color[3]),
                    layer: heat_core::render::LAYER_ENTITIES,
                });
            }
        }

        // Render player.
        if let (Some(tex), Some(entity)) = (self.player_texture, self.player_entity) {
            if let Ok(player_ref) = ctx.world.inner().get::<&Player>(entity) {
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: player_ref.pos,
                    size: Vec2::new(32.0, 32.0),
                    rotation: 0.0,
                    color: Color::WHITE,
                    layer: heat_core::render::LAYER_ENTITIES + 1, // lidt over NPC's
                });
            }
        }
    }
}