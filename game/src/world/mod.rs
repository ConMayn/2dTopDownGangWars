//! World — binder tilemap, zone, NPC sammen.
//!
//! WorldPlugin er en engine Plugin der ejer den nuværende zone,
//! renderer tilemap, opdaterer NPC'ere og lader spilleren bevæge sig
//! med collision mod tilemap.

pub mod collision;
pub mod dialog;
pub mod npc;
pub mod npc_fsm;
pub mod tilemap;
pub mod tiles;
pub mod vehicle;
pub mod zone;

use heat_core::{
    AppError, Color, EntityId, InitContext, Plugin, Rect,
    RenderContext, Sprite, TextureHandle, UpdateContext, Vec2, World,
};
use heat_core::input::Action;
use hecs::Entity;

use collision::move_and_collide;
use npc::{Npc, NpcType, Patrol, update_npc_dialog, update_npcs};
use crate::factions::{FactionAi, FactionRegistry, InfluenceGraph, ReputationState, RepEvent};
use crate::police::{CrimeType, EvidenceKind, EvidenceLedger, Police, PoliceState, WantedState, despawn_excess_police, spawn_police_units, update_police, update_wanted_sight};
use crate::systems::spatial::SpatialGrid;
use crate::systems::world_time::WorldTime;
use tilemap::{Tilemap, TilemapDef};
use tiles::{TileDef, TileRegistry, TileType};
use vehicle::{Vehicle, VehicleRegistry, collide_vehicle_with_tilemap, update_vehicle_physics};
use zone::ZoneDef;

/// Nuværende zone ID (hvilken zone spilleren er i).
/// Fase 5: simpel — én zone (east_blocks). Fase 6+: flere zoner med overgange.
const CURRENT_ZONE: &str = "east_blocks";

/// Player-komponent.
#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub pos: Vec2,
    pub speed: f32,
    /// Entity ID for bilen spilleren pt kører (None = til fods).
    pub in_vehicle: Option<Entity>,
    /// Interact-timer: forhindrer spam af E.
    pub interact_cooldown: f32,
    /// Har spilleren et våben fremme? (placeholder — Fase 6: rigtigt våbensystem).
    pub armed: bool,
}

/// Hoved-plugin for Fase 2-6: byen + spiller + NPC + vehicles + tid + factions + politi.
pub struct WorldPlugin {
    tile_registry: TileRegistry,
    tilemap: Option<Tilemap>,
    zone: Option<ZoneDef>,
    vehicle_registry: VehicleRegistry,
    player_entity: Option<EntityId>,
    player_texture: Option<TextureHandle>,
    npc_texture: Option<TextureHandle>,
    police_texture: Option<TextureHandle>,
    vehicle_textures: Vec<TextureHandle>,
    world_time: WorldTime,
    spatial: SpatialGrid,
    // Fase 5: factions
    faction_registry: FactionRegistry,
    influence_graph: InfluenceGraph,
    reputation: ReputationState,
    faction_ai: FactionAi,
    /// Timer for at generere "seen armed" / "reckless driving" events.
    rep_event_timer: f32,
    /// Tidligere spiller-position (til at detektere hastighed/reckless driving).
    prev_player_pos: Vec2,
    // Fase 6: politi
    wanted: WantedState,
    evidence: EvidenceLedger,
    /// Antal politi-enheder der pt er spawned.
    police_count_target: u32,
    /// Crime timer: for at detektere reckless driving som crime.
    crime_check_timer: f32,
}

impl WorldPlugin {
    pub fn new() -> Self {
        Self {
            tile_registry: TileRegistry::new(),
            tilemap: None,
            zone: None,
            vehicle_registry: VehicleRegistry::from_defaults(),
            player_entity: None,
            player_texture: None,
            npc_texture: None,
            police_texture: None,
            vehicle_textures: Vec::new(),
            world_time: WorldTime::new(),
            spatial: SpatialGrid::new(64.0),
            faction_registry: FactionRegistry::from_defaults(),
            influence_graph: InfluenceGraph::new(),
            reputation: ReputationState::new(),
            faction_ai: FactionAi::new(),
            rep_event_timer: 0.0,
            prev_player_pos: Vec2::ZERO,
            wanted: WantedState::new(),
            evidence: EvidenceLedger::new(),
            police_count_target: 0,
            crime_check_timer: 0.0,
        }
    }

    /// Opret test-assets (PNG filer) for Fase 2-6.
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

        // Police texture: 24x32 mørkeblå
        let police_path = std::env::temp_dir().join("heat_city_police.png");
        let mut pimg = image::ImageBuffer::new(24, 32);
        for p in pimg.pixels_mut() {
            *p = image::Rgba([30, 50, 120, 255]);
        }
        let _ = image::save_buffer(&police_path, &pimg, 24, 32, image::ExtendedColorType::Rgba8);

        // Vehicle textures: en per bil-type, farvet efter VehicleDef.color.
        let mut i = 0;
        for def in self.vehicle_registry.defs() {
            let path = std::env::temp_dir().join(format!("heat_city_vehicle_{i}.png"));
            let mut vimg = image::ImageBuffer::new(def.width as u32, def.height as u32);
            for p in vimg.pixels_mut() {
                *p = image::Rgba([
                    (def.color[0] * 255.0) as u8,
                    (def.color[1] * 255.0) as u8,
                    (def.color[2] * 255.0) as u8,
                    255,
                ]);
            }
            let _ = image::save_buffer(&path, &vimg, def.width as u32, def.height as u32, image::ExtendedColorType::Rgba8);
            let _ = assets.load_texture(&path)?;
            i += 1;
        }

        // Load player, npc og police textures.
        let _ = assets.load_texture(&player_path)?;
        let _ = assets.load_texture(&npc_path)?;
        let _ = assets.load_texture(&police_path)?;
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

    /// Spawn NPC'ere med patrol-ruter (flere typer).
    fn spawn_npcs(&self, world: &mut World, tilemap: &Tilemap) {
        // Pedestrians med patrol-ruter.
        let pedestrian_spawns = [
            (Vec2::new(100.0, 100.0), vec![Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0), Vec2::new(200.0, 150.0), Vec2::new(100.0, 150.0)]),
            (Vec2::new(600.0, 400.0), vec![Vec2::new(600.0, 400.0), Vec2::new(650.0, 400.0), Vec2::new(650.0, 450.0), Vec2::new(600.0, 450.0)]),
            (Vec2::new(400.0, 200.0), vec![Vec2::new(400.0, 200.0), Vec2::new(450.0, 200.0), Vec2::new(450.0, 250.0), Vec2::new(400.0, 250.0)]),
        ];

        for (pos, waypoints) in pedestrian_spawns {
            let npc = Npc::new(pos, NpcType::Pedestrian);
            let patrol = Patrol { waypoints };
            let _entity = world.spawn((npc, patrol));
        }

        // Shopkeeper (stationær nær en bygning).
        let shopkeeper = Npc::new(Vec2::new(150.0, 300.0), NpcType::Shopkeeper);
        let shop_patrol = Patrol { waypoints: vec![Vec2::new(150.0, 300.0), Vec2::new(180.0, 300.0)] };
        let _entity = world.spawn((shopkeeper, shop_patrol));

        // Gang members (patruljerer i grupper).
        let gang_spawns = [
            (Vec2::new(500.0, 100.0), vec![Vec2::new(500.0, 100.0), Vec2::new(550.0, 100.0), Vec2::new(550.0, 150.0), Vec2::new(500.0, 150.0)]),
            (Vec2::new(250.0, 450.0), vec![Vec2::new(250.0, 450.0), Vec2::new(300.0, 450.0), Vec2::new(300.0, 500.0), Vec2::new(250.0, 500.0)]),
        ];
        for (pos, waypoints) in gang_spawns {
            let npc = Npc::new(pos, NpcType::GangMember);
            let patrol = Patrol { waypoints };
            let _entity = world.spawn((npc, patrol));
        }

        let _ = tilemap;
        tracing::info!("Spawned NPCs: 3 pedestrians, 1 shopkeeper, 2 gang members");
    }

    /// Spawn biler på gaderne.
    fn spawn_vehicles(&self, world: &mut World) {
        let spawns = [
            ("compact", Vec2::new(400.0, 320.0), 0.0),
            ("muscle", Vec2::new(200.0, 320.0), 0.0),
            ("van", Vec2::new(600.0, 320.0), 3.14),
        ];

        for (def_id, pos, heading) in spawns {
            let def = self.vehicle_registry.get(def_id);
            if let Some(def) = def {
                let vehicle = Vehicle {
                    pos,
                    heading,
                    vel: Vec2::ZERO,
                    def_id: def_id.to_string(),
                    health: def.max_health,
                    driver: None,
                    hotwire_timer: 0.0,
                    stolen: false,
                };
                world.spawn_one(vehicle);
            }
        }
    }

    /// Håndter ind/udstigning af biler. Kaldes ved E-tast.
    fn handle_vehicle_enter_exit(
        &mut self,
        world: &mut World,
        player_entity: hecs::Entity,
        sim_time: f32,
    ) {
        // Tjek om spiller allerede er i en bil.
        let in_vehicle = world
            .inner()
            .get::<&Player>(player_entity)
            .ok()
            .map(|p| p.in_vehicle)
            .flatten();

        if let Some(vehicle_entity) = in_vehicle {
            // Stig ud — hent vehicle position først, så opdatér player.
            let vehicle_pos = world
                .inner()
                .get::<&Vehicle>(vehicle_entity)
                .ok()
                .map(|v| v.pos);
            let vehicle_heading = world
                .inner()
                .get::<&Vehicle>(vehicle_entity)
                .ok()
                .map(|v| v.heading);

            if let Ok(mut player_ref) = world.inner_mut().get::<&mut Player>(player_entity) {
                player_ref.in_vehicle = None;
                player_ref.interact_cooldown = 0.5;
                if let (Some(vp), Some(vh)) = (vehicle_pos, vehicle_heading) {
                    let offset = Vec2::new(0.0, vh.cos() * 40.0);
                    player_ref.pos = vp + offset;
                }
            }
            if let Ok(mut vehicle_ref) = world.inner_mut().get::<&mut Vehicle>(vehicle_entity) {
                vehicle_ref.driver = None;
                vehicle_ref.vel = Vec2::ZERO;
            }
            tracing::info!("Steg ud af bil");
            return;
        }

        // Ikke i bil — find nærmeste bil og stig ind hvis tæt nok.
        let player_pos = world
            .inner()
            .get::<&Player>(player_entity)
            .ok()
            .map(|p| p.pos)
            .unwrap_or(Vec2::ZERO);

        let mut closest: Option<(hecs::Entity, f32)> = None;
        for (entity, vehicle) in &mut world.inner().query::<&Vehicle>() {
            if vehicle.driver.is_some() {
                continue; // allerede optaget
            }
            let dist = (vehicle.pos - player_pos).length();
            if dist < 50.0 {
                // inden for rækkevidde
                if closest.is_none() || dist < closest.unwrap().1 {
                    closest = Some((entity, dist));
                }
            }
        }

        if let Some((vehicle_entity, _)) = closest {
            // Stig ind.
            if let Ok(mut player_ref) = world.inner_mut().get::<&mut Player>(player_entity) {
                player_ref.in_vehicle = Some(vehicle_entity);
                player_ref.interact_cooldown = 0.5;
            }
            if let Ok(mut vehicle_ref) = world.inner_mut().get::<&mut Vehicle>(vehicle_entity) {
                vehicle_ref.driver = Some(player_entity);
                vehicle_ref.stolen = true;
            }
            tracing::info!("Steg ind i bil");

            // Generér StoleVehicle reputation event.
            let event = RepEvent::StoleVehicle {
                faction: String::new(), // ukendt ejer for nu
            };
            self.faction_ai.handle_event(&mut self.reputation, &event);

            // Fase 6: Car theft → heat.
            let theft_pos = world
                .inner()
                .get::<&Player>(player_entity)
                .ok()
                .map(|p| p.pos)
                .unwrap_or(Vec2::ZERO);
            self.wanted.add_heat(CrimeType::CarTheft.heat_points(), sim_time, theft_pos);
            tracing::info!("Crime: {} (+{:.0} heat)", CrimeType::CarTheft.label(), CrimeType::CarTheft.heat_points());
        }
    }

    /// Generér reputation-events baseret på spillerens adfærd.
    /// Kaldes periodisk (~hver 2. sim-sek).
    fn generate_rep_events(&mut self, player_pos: Vec2, player_armed: bool, sim_time: f32) {
        let mut events: Vec<RepEvent> = Vec::new();

        // 1. Spiller med våben i nærheden af NPC'er → SeenArmed.
        if player_armed {
            let nearby = self.spatial.query_radius(player_pos, 80.0);
            if !nearby.is_empty() {
                events.push(RepEvent::SeenArmed {
                    zone: CURRENT_ZONE.to_string(),
                });
            }
        }

        // 2. Reckless driving: hvis spilleren bevæger sig hurtigt.
        let player_speed = (player_pos - self.prev_player_pos).length() / 0.016;
        if player_speed > 300.0 {
            events.push(RepEvent::RecklessDriving {
                zone: CURRENT_ZONE.to_string(),
            });
        }

        // Fase 6: Crimes → heat.
        if player_speed > 300.0 {
            self.wanted.add_heat(CrimeType::RecklessDriving.heat_points(), sim_time, player_pos);
        } else if player_speed > 200.0 {
            self.wanted.add_heat(CrimeType::Speeding.heat_points(), sim_time, player_pos);
        }

        // Apply events.
        for event in &events {
            self.faction_ai.handle_event(&mut self.reputation, event);
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
        self.police_texture = ctx
            .assets
            .get_texture_by_path(&std::env::temp_dir().join("heat_city_police.png"))
            .copied();

        // Load vehicle textures.
        self.vehicle_textures.clear();
        let mut i = 0;
        for _def in self.vehicle_registry.defs() {
            let path = std::env::temp_dir().join(format!("heat_city_vehicle_{i}.png"));
            if let Some(h) = ctx.assets.get_texture_by_path(&path).copied() {
                self.vehicle_textures.push(h);
            }
            i += 1;
        }

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
            in_vehicle: None,
            interact_cooldown: 0.0,
            armed: false,
        }));

        // Spawn NPC's (inkl. flere typer).
        if let Some(ref tm) = self.tilemap {
            self.spawn_npcs(ctx.world, tm);
        }

        // Spawn vehicles.
        self.spawn_vehicles(ctx.world);

        // Initialisér influence-graf for east_blocks zone.
        // Southline Kings dominerer (60%), civilians 30%, police 10%.
        self.influence_graph.init_zone(CURRENT_ZONE, "southline_kings", 10.0);

        // Fase 6: Spawn initiale politi-patruljer (2 enheder for ambiance).
        spawn_police_units(ctx.world, 2, CURRENT_ZONE, Vec2::new(px_w * 0.5, px_h * 0.5));
        self.police_count_target = 2;

        tracing::info!(
            "WorldPlugin init: tilemap {}x{}, {} factions, influence graph init, player + NPCs + vehicles",
            px_w as i32,
            px_h as i32,
            self.faction_registry.len(),
        );
    }

    fn update(&mut self, ctx: &mut UpdateContext) {
        // Klon tilemap ud af self for at undgå borrow-konflikt med &mut self kald.
        let Some(tilemap) = self.tilemap.clone() else { return };

        // Advance world time.
        self.world_time.advance(ctx.dt);

        // Decay interact cooldown.
        if let Some(entity) = self.player_entity {
            if let Ok(mut player_ref) = ctx.world.inner_mut().get::<&mut Player>(entity) {
                player_ref.interact_cooldown = (player_ref.interact_cooldown - ctx.dt).max(0.0);
            }
        }

        // Håndter ind/udstigning (E-tast).
        let interact_pressed = ctx.input.action_pressed(Action::Interact);
        if interact_pressed {
            if let Some(player_entity) = self.player_entity {
                self.handle_vehicle_enter_exit(ctx.world, player_entity, ctx.sim_time);
            }
        }

        // Opdatér player eller vehicle.
        if let Some(player_entity) = self.player_entity {
            let in_vehicle = ctx
                .world
                .inner()
                .get::<&Player>(player_entity)
                .ok()
                .map(|p| p.in_vehicle)
                .flatten();

            if let Some(vehicle_entity) = in_vehicle {
                // Spiller kører bil — opdatér bil-fysik.
                let (mx, my) = ctx.input.movement();
                let handbrake = ctx.input.action_down(Action::Sprint);

                let new_vehicle_pos = {
                    if let Ok(mut vehicle_ref) = ctx.world.inner_mut().get::<&mut Vehicle>(vehicle_entity) {
                        let def_id = vehicle_ref.def_id.clone();
                        if let Some(def) = self.vehicle_registry.get(&def_id) {
                            update_vehicle_physics(&mut vehicle_ref, def, -my, mx, handbrake, ctx.dt);
                            collide_vehicle_with_tilemap(&mut vehicle_ref, def, &tilemap, &self.tile_registry);
                            Some(vehicle_ref.pos)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(vp) = new_vehicle_pos {
                    if let Ok(mut player_ref) = ctx.world.inner_mut().get::<&mut Player>(player_entity) {
                        player_ref.pos = vp;
                        ctx.camera.follow(vp);
                    }
                }
            } else {
                // Spiller er til fods — normal movement.
                let (mx, my) = ctx.input.movement();
                let speed = if ctx.input.action_down(Action::Sprint) { 320.0 } else { 180.0 };
                let delta = Vec2::new(mx * speed * ctx.dt, my * speed * ctx.dt);

                if let Ok(mut player_ref) = ctx.world.inner_mut().get::<&mut Player>(player_entity) {
                    let player = &mut *player_ref;
                    let half = Vec2::new(16.0, 16.0);
                    let result = move_and_collide(player.pos, half, delta, &tilemap, &self.tile_registry);
                    player.pos = result.new_pos;
                    ctx.camera.follow(player.pos);
                }
            }
        }

        // NPC update med FSM, movement, collision.
        let (player_pos, player_armed) = if let Some(player_entity) = self.player_entity {
            ctx.world
                .inner()
                .get::<&Player>(player_entity)
                .ok()
                .map(|p| (p.pos, p.armed))
                .unwrap_or((Vec2::ZERO, false))
        } else {
            (Vec2::ZERO, false)
        };

        update_npcs(ctx.world, &tilemap, &self.tile_registry, player_pos, player_armed, ctx.sim_time, ctx.dt);

        // Opdatér spatial grid (rebuild per frame).
        self.spatial.clear();
        {
            let inner = ctx.world.inner();
            for (entity, npc) in &mut inner.query::<&Npc>() {
                self.spatial.insert(entity, npc.pos);
            }
        }

        // NPC dialog for dem tæt på spilleren.
        update_npc_dialog(ctx.world, player_pos, player_armed, self.world_time.time_of_day());

        // Fase 5: Faction-AI update (influence drift, konflikter, reputation decay).
        self.faction_ai.update(
            &self.faction_registry,
            &mut self.influence_graph,
            &mut self.reputation,
            ctx.dt,
        );

        // Generér reputation events baseret på spillerens adfærd.
        self.rep_event_timer += ctx.dt;
        if self.rep_event_timer > 2.0 {
            self.rep_event_timer = 0.0;
            self.generate_rep_events(player_pos, player_armed, ctx.sim_time);
        }

        // Fase 6: Wanted / Politi system.
        self.wanted.update(ctx.dt, ctx.sim_time, player_pos);
        update_wanted_sight(ctx.world, &mut self.wanted, player_pos, ctx.sim_time);
        update_police(ctx.world, &self.wanted, player_pos, ctx.dt);

        // Spawn/despawn politi baseret på heat-level (min 2 for ambiance).
        let desired_units = self.wanted.level.response_units().max(2);
        if desired_units != self.police_count_target {
            if desired_units > self.police_count_target {
                spawn_police_units(ctx.world, desired_units - self.police_count_target, CURRENT_ZONE, player_pos);
            } else {
                despawn_excess_police(ctx.world, desired_units);
            }
            tracing::info!("Politi: {} enheder (heat: {})", desired_units, self.wanted.level.label());
            self.police_count_target = desired_units;
        }

        // Evidence: når politi ser spilleren, saml beviser periodisk.
        if self.wanted.in_sight {
            self.crime_check_timer += ctx.dt;
            if self.crime_check_timer > 3.0 {
                self.crime_check_timer = 0.0;
                self.evidence.add(
                    EvidenceKind::LastPosition,
                    ctx.sim_time,
                    CURRENT_ZONE,
                    &format!("({:.0},{:.0})", player_pos.x, player_pos.y),
                );
                // VehicleType bevis hvis i bil.
                if let Some(pe) = self.player_entity {
                    let in_veh = ctx.world.inner().get::<&Player>(pe).ok().map(|p| p.in_vehicle).flatten();
                    if let Some(ve) = in_veh {
                        if let Ok(v) = ctx.world.inner().get::<&Vehicle>(ve) {
                            self.evidence.add(EvidenceKind::VehicleType, ctx.sim_time, CURRENT_ZONE, &v.def_id);
                        }
                    }
                }
            }
        }

        // Opdatér prev_player_pos.
        self.prev_player_pos = player_pos;
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        let Some(tilemap) = &self.tilemap else { return };

        // Render tilemap (kun synlige tiles).
        tilemap.render(ctx, &self.tile_registry);

        // Render vehicles.
        let inner = ctx.world.inner();
        let vehicle_idx = 0usize;
        for (_, vehicle) in &mut inner.query::<&Vehicle>() {
            let tex = self.vehicle_textures.iter().enumerate().find(|(_, _)| {
                // Match vehicle def index. Vi bygger textures i samme rækkefølge som defs().
                // Da defs() returnerer en iterator, og vi ikke kan matche direkte,
                // bruger vi en simpel hash-map lookup i en rigtig implementation.
                // For nu: bare brug første texture (Fase 3 proof).
                true
            }).map(|(_, t)| *t);

            if let Some(tex) = tex {
                // Find bil-dimensioner fra registry.
                if let Some(def) = self.vehicle_registry.get(&vehicle.def_id) {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: vehicle.pos,
                        size: Vec2::new(def.width, def.height),
                        rotation: vehicle.heading,
                        color: Color::WHITE,
                        layer: heat_core::render::LAYER_ENTITIES,
                    });
                }
            }
        }

        // Render NPC's (farve baseret på state).
        let inner = ctx.world.inner();
        for (_, (npc,)) in &mut inner.query::<(&Npc,)>() {
            if let Some(tex) = self.npc_texture {
                // Farve baseret på state.
                let color = match npc.state {
                    npc_fsm::NpcState::Panic => Color::rgba(1.0, 0.3, 0.3, 1.0),
                    npc_fsm::NpcState::Flee => Color::rgba(0.9, 0.5, 0.3, 1.0),
                    npc_fsm::NpcState::Talk => Color::rgba(0.3, 0.8, 1.0, 1.0),
                    _ => Color::rgba(npc.color[0], npc.color[1], npc.color[2], npc.color[3]),
                };
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: npc.pos,
                    size: Vec2::new(24.0, 24.0),
                    rotation: 0.0,
                    color,
                    layer: heat_core::render::LAYER_ENTITIES,
                });
            }
        }

        // Render politi (Fase 6).
        let inner = ctx.world.inner();
        for (_, (police,)) in &mut inner.query::<(&Police,)>() {
            if let Some(tex) = self.police_texture {
                let color = match police.state {
                    PoliceState::Pursue => Color::rgba(1.0, 0.4, 0.4, 1.0),
                    PoliceState::Search => Color::rgba(1.0, 0.8, 0.3, 1.0),
                    _ => Color::WHITE,
                };
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: police.pos,
                    size: Vec2::new(24.0, 32.0),
                    rotation: police.heading,
                    color,
                    layer: heat_core::render::LAYER_ENTITIES + 2,
                });
            }
        }

        // Render player (kun hvis ikke i bil — bilen vises i stedet).
        if let (Some(tex), Some(entity)) = (self.player_texture, self.player_entity) {
            let in_vehicle = ctx
                .world
                .inner()
                .get::<&Player>(entity)
                .ok()
                .map(|p| p.in_vehicle)
                .flatten();

            if in_vehicle.is_none() {
                if let Ok(player_ref) = ctx.world.inner().get::<&Player>(entity) {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: player_ref.pos,
                        size: Vec2::new(32.0, 32.0),
                        rotation: 0.0,
                        color: Color::WHITE,
                        layer: heat_core::render::LAYER_ENTITIES + 1,
                    });
                }
            }
        }
    }
}