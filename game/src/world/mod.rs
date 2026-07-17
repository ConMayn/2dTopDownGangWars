#![allow(dead_code)] // `zone` og `Player::speed` er public/stub felter.

//! World — binder alle spil-systemer sammen (Fase 10).
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
use crate::economy::{PlayerEconomy};
use crate::missions::{MissionTracker, default_missions, Objective};
use crate::dialog::{ActiveDialog, demo_tree};
use crate::safehouses::{SafehousePortfolio};
use crate::crew::{Crew};
use crate::businesses::{BusinessPortfolio};
use crate::heists::{HeistManager, HeistPlan, HeistOutcome, Approach, EscapeRoute, Diversion, default_heists};
use crate::director::{DirectorState, DirectorEvent};
use crate::events::{EventManager, EventKind};
use crate::news::{NewsSystem, NewsKind};
use crate::rivals::{RivalSystem, RivalKind};
use crate::save::SaveSlots;
use crate::ui::UiState;
use crate::audio::{AudioSystem, RadioStation};
use crate::combat::{CombatSystem, WeaponKind, Health};
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
    // Fase 7: economy + missions + dialog
    economy: PlayerEconomy,
    mission_tracker: MissionTracker,
    active_dialog: Option<ActiveDialog>,
    /// Cooldown så dialog/mission trigger ikke spammes.
    dialog_cooldown: f32,
    // Fase 8: safehouses + crew + businesses
    safehouses: SafehousePortfolio,
    crew: Crew,
    businesses: BusinessPortfolio,
    /// Timer for periodisk business/safehouse opdatering og indkomst.
    economy_tick_timer: f32,
    // Fase 9: heists
    heist_manager: HeistManager,
    /// Tast for at starte en heist (proof: auto-start første heist med E+Q).
    heist_test_timer: f32,
    // Fase 10: AI director + events + news + rivals
    director: DirectorState,
    event_manager: EventManager,
    news: NewsSystem,
    rivals: RivalSystem,
    // Fase 11: save + UI + audio
    save_slots: SaveSlots,
    ui: UiState,
    audio: AudioSystem,
    // Fase 12: font + text rendering
    font_texture: Option<TextureHandle>,
    /// 1x1 hvid texture til overlays (day/night, farvede quads).
    white_texture: Option<TextureHandle>,
    /// Render-tid (akumulerer dt i render for animation).
    render_time: f32,
    // Combat
    combat: CombatSystem,
    /// Player health.
    player_health: Health,
    /// Retning spilleren kigger (baseret på movement).
    player_facing: Vec2,
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
            economy: PlayerEconomy::with_starter_kit(),
            mission_tracker: MissionTracker::new(),
            active_dialog: None,
            dialog_cooldown: 0.0,
            safehouses: SafehousePortfolio::with_starter(),
            crew: Crew::with_starter(),
            businesses: BusinessPortfolio::with_starter(),
            economy_tick_timer: 0.0,
            heist_manager: HeistManager::new(),
            heist_test_timer: 0.0,
            director: DirectorState::new(),
            event_manager: EventManager::new(),
            news: NewsSystem::new(),
            rivals: RivalSystem::new(),
            save_slots: SaveSlots::new(),
            ui: UiState::new(),
            audio: AudioSystem::new(),
            font_texture: None,
            white_texture: None,
            render_time: 0.0,
            combat: CombatSystem::new(),
            player_health: Health::new(100.0),
            player_facing: Vec2::new(1.0, 0.0),
        }
    }

    /// Opret procedurale sprites (Fase 12 visuel overhaling).
    fn create_test_assets(&self, assets: &mut heat_core::AssetStore) -> Result<(), AppError> {
        use crate::sprites;

        // Player: detaljeret karakter-sprite med våben.
        let player_path = std::env::temp_dir().join("heat_city_player.png");
        let player_img = sprites::generate_player();
        let _ = image::save_buffer(&player_path, &player_img, 32, 32, image::ExtendedColorType::Rgba8);

        // NPC: grøn karakter.
        let npc_path = std::env::temp_dir().join("heat_city_npc.png");
        let npc_img = sprites::generate_npc([100, 180, 100]);
        let _ = image::save_buffer(&npc_path, &npc_img, 24, 24, image::ExtendedColorType::Rgba8);

        // Politi: mørkeblå karakter med badge og våben.
        let police_path = std::env::temp_dir().join("heat_city_police.png");
        let police_img = sprites::generate_police();
        let _ = image::save_buffer(&police_path, &police_img, 32, 32, image::ExtendedColorType::Rgba8);

        // Vehicle textures: detaljerede biler per type.
        let mut i = 0;
        for def in self.vehicle_registry.defs() {
            let path = std::env::temp_dir().join(format!("heat_city_vehicle_{i}.png"));
            let vimg = sprites::generate_vehicle(def.width as u32, def.height as u32, [def.color[0], def.color[1], def.color[2]]);
            let _ = image::save_buffer(&path, &vimg, def.width as u32, def.height as u32, image::ExtendedColorType::Rgba8);
            let _ = assets.load_texture(&path)?;
            i += 1;
        }

        // Tile textures: procedurale mønstre.
        let tile_size = 32u32;
        let asphalt_path = std::env::temp_dir().join("heat_city_tile_asphalt.png");
        let asphalt_img = sprites::generate_asphalt_tile(tile_size);
        let _ = image::save_buffer(&asphalt_path, &asphalt_img, tile_size, tile_size, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&asphalt_path)?;

        let sidewalk_path = std::env::temp_dir().join("heat_city_tile_sidewalk.png");
        let sidewalk_img = sprites::generate_sidewalk_tile(tile_size);
        let _ = image::save_buffer(&sidewalk_path, &sidewalk_img, tile_size, tile_size, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&sidewalk_path)?;

        let building_path = std::env::temp_dir().join("heat_city_tile_building.png");
        let building_img = sprites::generate_building_tile(tile_size);
        let _ = image::save_buffer(&building_path, &building_img, tile_size, tile_size, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&building_path)?;

        let grass_path = std::env::temp_dir().join("heat_city_tile_grass.png");
        let grass_img = sprites::generate_grass_tile(tile_size);
        let _ = image::save_buffer(&grass_path, &grass_img, tile_size, tile_size, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&grass_path)?;

        // Load player, npc og police textures.
        let _ = assets.load_texture(&player_path)?;
        let _ = assets.load_texture(&npc_path)?;
        let _ = assets.load_texture(&police_path)?;

        // Font atlas.
        let font_path = std::env::temp_dir().join("heat_city_font.png");
        let font_img = crate::font::generate_font_atlas();
        let _ = image::save_buffer(&font_path, &font_img, crate::font::ATLAS_W, crate::font::ATLAS_H, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&font_path)?;

        // White pixel texture (til overlays, day/night, farvede quads).
        let white_path = std::env::temp_dir().join("heat_city_white.png");
        let white_img = image::ImageBuffer::from_pixel(1, 1, image::Rgba([255, 255, 255, 255]));
        let _ = image::save_buffer(&white_path, &white_img, 1, 1, image::ExtendedColorType::Rgba8);
        let _ = assets.load_texture(&white_path)?;
        Ok(())
    }

    /// Byg tile registry med basale tile-typer og procedurale textures.
    fn build_tile_registry(&self, assets: &heat_core::AssetStore) -> TileRegistry {
        let mut reg = TileRegistry::new();

        // asfalt (gader) — med gade-streger.
        let asphalt_tex = assets.get_texture_by_path(&std::env::temp_dir().join("heat_city_tile_asphalt.png")).copied();
        reg.insert(
            "asphalt".into(),
            TileType {
                def: TileDef {
                    id: "asphalt".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.12, 0.12, 0.14, 1.0],
                },
                texture: asphalt_tex,
            },
        );
        // fortov — flise-mønster.
        let sidewalk_tex = assets.get_texture_by_path(&std::env::temp_dir().join("heat_city_tile_sidewalk.png")).copied();
        reg.insert(
            "sidewalk".into(),
            TileType {
                def: TileDef {
                    id: "sidewalk".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.25, 0.25, 0.28, 1.0],
                },
                texture: sidewalk_tex,
            },
        );
        // græs.
        let grass_tex = assets.get_texture_by_path(&std::env::temp_dir().join("heat_city_tile_grass.png")).copied();
        reg.insert(
            "grass".into(),
            TileType {
                def: TileDef {
                    id: "grass".into(),
                    solid: false,
                    layer: heat_core::render::LAYER_GROUND,
                    color: [0.15, 0.3, 0.15, 1.0],
                },
                texture: grass_tex,
            },
        );
        // bygning (solid) — mursten + vinduer.
        let building_tex = assets.get_texture_by_path(&std::env::temp_dir().join("heat_city_tile_building.png")).copied();
        reg.insert(
            "building".into(),
            TileType {
                def: TileDef {
                    id: "building".into(),
                    solid: true,
                    layer: heat_core::render::LAYER_ENTITIES,
                    color: [0.35, 0.3, 0.25, 1.0],
                },
                texture: building_tex,
            },
        );
        // mur (solid).
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

    /// Fase 7: Opdatér aktive missioner — auto-advance baseret på stubs.
    fn update_missions(&mut self, world: &heat_core::World, _player_pos: Vec2) {
        let active_indices: Vec<usize> = self.mission_tracker.active();
        for idx in active_indices {
            let mission = &mut self.mission_tracker.missions[idx];
            if let Some(obj) = mission.current_objective().cloned() {
                let advance = match obj {
                    Objective::GoToZone { ref zone } if zone == CURRENT_ZONE => {
                        // Simpel stub: spilleren er i den korrekte zone.
                        true
                    }
                    Objective::StealVehicle { ref def_id } => {
                        // Tjek om spilleren kører et køretøj af den type.
                        if let Some(pe) = self.player_entity {
                            if let Ok(player) = world.inner().get::<&Player>(pe) {
                                if let Some(ve) = player.in_vehicle {
                                    if let Ok(vehicle) = world.inner().get::<&Vehicle>(ve) {
                                        &vehicle.def_id == def_id
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                    Objective::EscapePolice { heat_max } => {
                        self.wanted.level.as_u8() <= heat_max
                    }
                    _ => false,
                };
                if advance {
                    mission.advance();
                    if mission.is_complete() {
                        tracing::info!("Mission completed: {}", mission.def.title);
                        let rewards = mission.def.rewards.clone();
                        self.apply_rewards(&rewards);
                    }
                }
            }
        }
    }

    /// Anvend mission rewards.
    fn apply_rewards(&mut self, rewards: &[crate::missions::Reward]) {
        use crate::missions::Reward;
        use crate::factions::apply_event;
        use crate::factions::RepEvent;
        for reward in rewards {
            match reward {
                Reward::Cash { amount, clean } => {
                    self.economy.wallet.add(*amount, *clean);
                    tracing::info!("Reward: ${} {}", amount, if *clean { "clean" } else { "cash" });
                }
                Reward::Item { item_id, count } => {
                    let mut item = crate::economy::Item::new(
                        item_id,
                        crate::economy::ItemKind::Gift,
                        item_id,
                        0,
                        false,
                    );
                    item.stack = *count;
                    self.economy.inventory.add(item);
                    tracing::info!("Reward item: {} x{}", item_id, count);
                }
                Reward::FactionTrust { faction, delta } => {
                    apply_event(
                        &mut self.reputation,
                        &RepEvent::WonFight { reputation_gain: *delta },
                    );
                    let _ = faction;
                }
                Reward::StreetRep { delta } => {
                    apply_event(
                        &mut self.reputation,
                        &RepEvent::WonFight { reputation_gain: *delta },
                    );
                }
                Reward::Influence { zone, faction, delta } => {
                    if let Some(inf) = self.influence_graph.get_mut(zone) {
                        inf.add_influence(faction, *delta);
                    }
                }
            }
        }
    }

    /// Fase 7: Start dialog når spilleren trykker E (hvis ikke i bil/køretøj).
    fn handle_dialog_interact(&mut self) {
        if self.active_dialog.is_some() {
            return;
        }
        let tree = demo_tree();
        self.active_dialog = ActiveDialog::new(tree, "greet");
        self.dialog_cooldown = 0.3;
        tracing::info!("Dialog started with Lil' P");
    }

    /// Avancér dialogen ét skridt: vælg første valg og anvend effekter.
    fn advance_dialog(&mut self) {
        if self.active_dialog.is_none() {
            return;
        }
        let effects: Vec<crate::dialog::DialogEffect> = {
            let dialog = self.active_dialog.as_mut().unwrap();
            let node = dialog.current_node();
            if node.choices.is_empty() {
                self.active_dialog = None;
                tracing::info!("Dialog ended");
                return;
            }
            // Stub: vælg altid første valg; clone effects så vi kan mutere self efterfølgende.
            if let Some(choice) = dialog.choose(0) {
                let has_next = choice.next.is_some();
                let effects = choice.effects.clone();
                if !has_next {
                    self.active_dialog = None;
                    tracing::info!("Dialog ended");
                }
                effects
            } else {
                return;
            }
        };
        for effect in &effects {
            self.apply_dialog_effect(effect);
        }
    }

    /// Fase 9: Anvend heist outcome — rewards, heat, evidence, crew-skader.
    fn apply_heist_outcome(&mut self, outcome: HeistOutcome) {
        if outcome.success {
            self.economy.wallet.add(outcome.reward_cash, false);
            self.economy.wallet.add(outcome.reward_clean, true);
            tracing::info!(
                "Heist success: +${} cash, +${} clean, +{:.0} heat, +{:.0} evidence",
                outcome.reward_cash,
                outcome.reward_clean,
                outcome.heat,
                outcome.evidence,
            );
        } else {
            tracing::info!(
                "Heist failed: +{:.0} heat, +{:.0} evidence, {} crew injured",
                outcome.heat,
                outcome.evidence,
                outcome.crew_injured.len(),
            );
        }
        // Heat.
        if self.player_entity.is_some() {
            let pos = self.prev_player_pos;
            self.wanted.add_heat(outcome.heat, 0.0, pos);
        }
        // Crew-skader: justér loyalitet/fear.
        for cid in &outcome.crew_injured {
            if let Some(m) = self.crew.get_mut(cid) {
                m.adjust_loyalty(-10.0);
                m.adjust_fear(15.0);
            }
        }
        // Faction trust.
        use crate::factions::{apply_event, RepEvent};
        apply_event(
            &mut self.reputation,
            &RepEvent::WonFight { reputation_gain: outcome.faction_trust_delta },
        );
    }

    /// Fase 10: Håndtér director-triggeret event.
    fn handle_director_event(&mut self, event: DirectorEvent, player_pos: Vec2, sim_time: f32) {
        tracing::info!("Director event: {}", event.label());
        match event {
            DirectorEvent::RandomStreetEvent => {
                // Spawn en gade-event nær spilleren.
                let kind = EventKind::GangSkirmish;
                let pos = [player_pos.x + 100.0, player_pos.y + 50.0];
                self.event_manager.spawn(kind, CURRENT_ZONE, pos);
                self.news.publish(
                    NewsKind::LocalEvent,
                    "Gang skirmish reported",
                    "Two crews are squaring off in East Blocks. Police monitoring.",
                    sim_time,
                );
            }
            DirectorEvent::AmbientFlavor => {
                self.event_manager.spawn(EventKind::BikerConvoy, CURRENT_ZONE, [400.0, 300.0]);
                self.news.publish(
                    NewsKind::LocalEvent,
                    "Biker convoy spotted",
                    "A large biker convoy is rolling through the city.",
                    sim_time,
                );
            }
            DirectorEvent::PolicePressure => {
                self.event_manager.spawn(EventKind::TrafficStop, CURRENT_ZONE, [300.0, 200.0]);
                self.news.publish(
                    NewsKind::PoliceBlotter,
                    "Increased police patrols",
                    "Police are stepping up presence in response to recent activity.",
                    sim_time,
                );
            }
            DirectorEvent::RivalAttack => {
                if let Some(rival) = self.rivals.rivals.first_mut() {
                    let action = rival.on_action_taken();
                    tracing::info!("Rival {} took action: {}", rival.name, action.label());
                    self.news.publish(
                        NewsKind::GangNews,
                        "Rival makes a move",
                        &format!("{} is making moves against you.", rival.name),
                        sim_time,
                    );
                }
            }
            DirectorEvent::ContactCall => {
                self.news.publish_rumor(
                    "A contact wants to reach you. Word is they have a job.",
                    sim_time,
                );
            }
            DirectorEvent::NewsReport => {
                self.news.publish_player_action(
                    "Witnesses describe a suspect fleeing the scene.",
                    sim_time,
                );
            }
        }
    }

    /// Fase 11: Opdatér HUD-state fra alle spil-systemer.
    fn update_hud(&mut self) {
        self.ui.hud.cash = self.economy.wallet.cash;
        self.ui.hud.clean = self.economy.wallet.clean;
        self.ui.hud.set_heat(self.wanted.level.label(), self.wanted.heat_points);
        // Aktive missioner.
        self.ui.hud.active_missions = self
            .mission_tracker
            .missions
            .iter()
            .filter(|m| m.status == crate::missions::MissionStatus::Active)
            .map(|m| m.def.title.clone())
            .collect();
        // Current objective.
        if let Some(idx) = self.mission_tracker.active().first() {
            if let Some(m) = self.mission_tracker.missions.get(*idx) {
                if let Some(obj) = m.current_objective() {
                    self.ui.hud.set_objective(&format!("{:?}", obj));
                } else {
                    self.ui.hud.clear_objective();
                }
            }
        } else {
            self.ui.hud.clear_objective();
        }
        // Verdens-tid.
        self.ui.hud.time_formatted = self.world_time.formatted();
        self.ui.hud.time_of_day_label = self.world_time.time_of_day().label().to_string();
        // Crew status.
        self.ui.hud.crew_status = self
            .crew
            .members
            .iter()
            .map(|m| (m.name.clone(), m.loyalty_status().to_string()))
            .collect();
        // Events i nærheden.
        self.ui.hud.nearby_events = self
            .event_manager
            .active
            .iter()
            .map(|e| e.kind.label().to_string())
            .collect();
        // News ticker.
        if let Some(n) = self.news.latest(None).first() {
            self.ui.hud.set_news_ticker(&n.headline);
        }
        // Dialog.
        if let Some(ref dialog) = self.active_dialog {
            let node = dialog.current_node();
            let choices: Vec<String> = node.choices.iter().map(|c| c.text.clone()).collect();
            self.ui.hud.set_dialog(&node.text, &choices);
        } else {
            self.ui.hud.clear_dialog();
        }
    }

    fn apply_dialog_effect(&mut self,
        effect: &crate::dialog::DialogEffect,
    ) {
        use crate::dialog::DialogEffect;
        use crate::economy::{Item, ItemKind};
        match effect {
            DialogEffect::StartMission { mission_id } => {
                if let Some(def) = default_missions().into_iter().find(|d| d.id == *mission_id) {
                    self.mission_tracker.start(def);
                    tracing::info!("Started mission via dialog: {}", mission_id);
                }
            }
            DialogEffect::AdvanceMission { mission_id, objective_idx } => {
                if let Some(m) = self.mission_tracker.get_active_mut(mission_id) {
                    m.current_objective = *objective_idx;
                }
            }
            DialogEffect::GiveItem { item_id, count } => {
                let item = Item::new(item_id, ItemKind::Gift, item_id, 0, false);
                let mut item = item;
                item.stack = *count;
                self.economy.inventory.add(item);
            }
            DialogEffect::TakeItem { item_id, count } => {
                self.economy.inventory.remove(item_id, *count);
            }
            DialogEffect::GiveCash { amount, clean } => {
                self.economy.wallet.add(*amount, *clean);
            }
            DialogEffect::TakeCash { amount, clean } => {
                self.economy.wallet.spend(*amount, *clean);
            }
            DialogEffect::ReputationEvent { event_kind } => {
                let _ = event_kind;
            }
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
        self.font_texture = ctx
            .assets
            .get_texture_by_path(&std::env::temp_dir().join("heat_city_font.png"))
            .copied();
        self.white_texture = ctx
            .assets
            .get_texture_by_path(&std::env::temp_dir().join("heat_city_white.png"))
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
        self.tile_registry = self.build_tile_registry(ctx.assets);

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

        // Fase 7: load default mission definitions.
        for def in default_missions() {
            self.mission_tracker.start(def);
        }

        // Fase 9: load default heist definitions.
        for def in default_heists() {
            self.heist_manager.add_available(def);
        }

        // Fase 10: seed rivals.
        self.rivals.add(RivalKind::GangLeader, "Marcus 'Mad Dog' Reyes");
        self.rivals.add(RivalKind::BountyHunter, "Sledge");
        self.rivals.add(RivalKind::Cop, "Det. Sarah Voss");

        // Fase 10: seed nyheder.
        self.news.publish(NewsKind::LocalEvent, "Heat City wakes up", "Another day in the city. Stay sharp out there.", 0.0);

        // Fase 11: seed radio-stationer.
        self.audio.add_station(RadioStation::new("hot_97", "Hot 97 FM", "Hip-Hop"));
        self.audio.add_station(RadioStation::new("klassik", "Klassik FM", "Classical"));
        self.audio.add_station(RadioStation::new("scanner", "Police Scanner", "Talk"));

        tracing::info!(
            "WorldPlugin init: tilemap {}x{}, {} factions, {} missions, economy starter $$$, {} safehouses, {} crew, {} businesses, {} heists, {} rivals, {} news, influence graph init, player + NPCs + vehicles",
            px_w as i32,
            px_h as i32,
            self.faction_registry.len(),
            self.mission_tracker.missions.len(),
            self.safehouses.owned.len(),
            self.crew.members.len(),
            self.businesses.owned.len(),
            self.heist_manager.available.len(),
            self.rivals.rivals.len(),
            self.news.count(),
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

        // Fase 7: Mission-update (auto-advance GoToZone / StealVehicle stubs).
        self.update_missions(ctx.world, player_pos);

        // Fase 8: businesses, safehouses, crew tick.
        self.economy_tick_timer += ctx.dt;
        if self.economy_tick_timer >= 1.0 {
            self.economy_tick_timer = 0.0;
            let biz_payout = self.businesses.tick_all(ctx.dt);
            if biz_payout > 0 {
                self.economy.wallet.add(biz_payout, true);
            }
            self.safehouses.tick_all(ctx.dt);
            self.crew.tick_all(ctx.dt);
        }

        // Fase 9: Heist tick + trigger (Attack edge-press = start heist proof).
        if let Some(outcome) = self.heist_manager.tick_active(ctx.dt) {
            self.apply_heist_outcome(outcome);
        }
        self.heist_test_timer += ctx.dt;
        if ctx.input.action_pressed(Action::Attack)
            && self.heist_manager.active.is_none()
            && self.heist_test_timer > 2.0
            && self.player_health.alive
        {
            let mut plan = HeistPlan::new("armored_van", Approach::Loud);
            plan.escape = EscapeRoute::BackAlleys;
            plan.diversion = Diversion::FakeCall;
            // Tag første klar crew-medlem med.
            if let Some(m) = self.crew.ready().first() {
                plan.crew_ids.push(m.id.clone());
            }
            if self.heist_manager.start("armored_van", plan) {
                tracing::info!("Heist startet: Armored Van (Loud)");
            }
            self.heist_test_timer = 0.0;
        }

        // Combat: skydning (Attack held = auto-fire pistol).
        if ctx.input.action_down(Action::Attack) && self.player_health.alive {
            // Opdatér facing baseret på movement.
            let (mx, my) = ctx.input.movement();
            if mx != 0.0 || my != 0.0 {
                self.player_facing = Vec2::new(mx, my).normalize();
            }
            let weapon = WeaponKind::Pistol;
            if self.combat.can_player_fire(weapon) {
                self.combat.player_fire(weapon, player_pos, self.player_facing);
                // Heat per skud.
                self.wanted.add_heat(weapon.heat_per_shot(), ctx.sim_time, player_pos);
                // SFX.
                self.audio.play_sfx("pistol_shot");
            }
        }

        // Combat update: projectiles, muzzle flashes, blood.
        self.combat.update(ctx.dt);

        // Projectile vs NPC collision.
        let mut npc_hits: Vec<(hecs::Entity, f32, Vec2)> = Vec::new();
        {
            let inner = ctx.world.inner();
            for (entity, npc) in &mut inner.query::<&Npc>() {
                for proj in &self.combat.projectiles {
                    if !proj.from_player {
                        continue;
                    }
                    let dist = (proj.pos - npc.pos).length();
                    if dist < 16.0 {
                        npc_hits.push((entity, proj.damage, proj.vel.normalize()));
                        break;
                    }
                }
            }
        }
        // Apply NPC damage.
        for (entity, damage, hit_dir) in npc_hits {
            // Få NPC til at flygte.
            if let Ok(mut npc) = ctx.world.inner_mut().get::<&mut Npc>(entity) {
                npc.state = npc_fsm::NpcState::Flee;
                npc.memory.fear = (npc.memory.fear + 0.3).min(1.0);
                let _ = damage;
                let _ = entity;
            }
            // Spawn blood.
            if let Ok(npc) = ctx.world.inner().get::<&Npc>(entity) {
                self.combat.spawn_blood(npc.pos, hit_dir, 5);
            }
        }

        // Projectile vs politi collision.
        let mut police_hits: Vec<(hecs::Entity, f32, Vec2)> = Vec::new();
        {
            let inner = ctx.world.inner();
            for (entity, police) in &mut inner.query::<&Police>() {
                for proj in &self.combat.projectiles {
                    if !proj.from_player {
                        continue;
                    }
                    let dist = (proj.pos - police.pos).length();
                    if dist < 20.0 {
                        police_hits.push((entity, proj.damage, proj.vel.normalize()));
                        break;
                    }
                }
            }
        }
        for (entity, damage, hit_dir) in police_hits {
            // Politi bliver aggressiv (Pursue) og heat stiger.
            self.wanted.add_heat(20.0, ctx.sim_time, player_pos);
            if let Ok(mut police) = ctx.world.inner_mut().get::<&mut Police>(entity) {
                police.state = PoliceState::Pursue;
                let _ = damage;
            }
            if let Ok(police) = ctx.world.inner().get::<&Police>(entity) {
                self.combat.spawn_blood(police.pos, hit_dir, 5);
            }
            tracing::info!("Cop shot! Heat +20");
        }

        // Fase 10: AI Director, events, news, rivals.
        let heat = self.wanted.heat_points;
        let cash = self.economy.wallet.cash;
        self.director.update(heat, cash, ctx.dt);
        if let Some(director_event) = self.director.should_trigger() {
            self.handle_director_event(director_event, player_pos, ctx.sim_time);
            self.director.on_event_triggered();
        }
        // Tick events: fjern udløbne.
        let _expired = self.event_manager.tick(ctx.dt);
        // Tick news: aldring.
        self.news.tick(ctx.dt);
        // Tick rivals.
        self.rivals.tick(ctx.dt);

        // Fase 11: UI + audio tick; opdatér HUD-state.
        self.audio.tick();
        self.update_hud();

        // Fase 7: Dialog input (E toggler/interagerer med dialog).
        self.dialog_cooldown = (self.dialog_cooldown - ctx.dt).max(0.0);
        if self.dialog_cooldown == 0.0 && ctx.input.action_pressed(Action::Interact) {
            if self.active_dialog.is_some() {
                self.advance_dialog();
                self.dialog_cooldown = 0.3;
            } else {
                self.handle_dialog_interact();
                self.dialog_cooldown = 0.3;
            }
        }
        // Debug: vis økonomi og aktive missioner + Fase 8 stats.
        if ctx.input.action_pressed(Action::ToggleDebug) {
            tracing::info!(
                "Wallet: ${}/${} | Missions: {} active | Dialog: {} | Safehouses: {} | Crew: {} | Businesses: {} (risk avg {:.0}) | Heists avail: {} active: {} | Director: {} (calm {:.0}s) | Events: {} | News: {} | Rivals: {} (hostile {}, bounty ${}) | Menu: {} | Radio: {} | Audio muted: {}",
                self.economy.wallet.cash,
                self.economy.wallet.clean,
                self.mission_tracker.active().len(),
                self.active_dialog.is_some(),
                self.safehouses.owned.len(),
                self.crew.members.len(),
                self.businesses.owned.len(),
                if self.businesses.owned.is_empty() { 0.0 } else {
                    self.businesses.owned.iter().map(|b| b.risk).sum::<f32>() / self.businesses.owned.len() as f32
                },
                self.heist_manager.available.len(),
                self.heist_manager.active.is_some(),
                self.director.tension.label(),
                self.director.calm_time,
                self.event_manager.active_count(),
                self.news.count(),
                self.rivals.rivals.len(),
                self.rivals.hostile_count(),
                self.rivals.total_bounty(),
                self.ui.current_menu.map(|m| m.label()).unwrap_or("None"),
                self.audio.active_radio.as_deref().unwrap_or("None"),
                self.audio.muted,
            );
        }

        // Opdatér prev_player_pos.
        self.prev_player_pos = player_pos;
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        self.render_time += 0.016; // ~60fps; præcis værdi er ikke kritisk for animation.
        let anim_time = self.render_time;
        let Some(tilemap) = &self.tilemap else { return };

        // Render tilemap (med procedurale textures).
        tilemap.render(ctx, &self.tile_registry);

        // Render vehicles — match texture per def_id via index.
        let inner = ctx.world.inner();
        for (_, vehicle) in &mut inner.query::<&Vehicle>() {
            // Find vehicle def index for texture matching.
            let tex_idx = self.vehicle_registry.defs().enumerate().find(|(_, def)| def.id == vehicle.def_id).map(|(i, _)| i);
            let tex = tex_idx.and_then(|i| self.vehicle_textures.get(i).copied());

            if let Some(tex) = tex {
                if let Some(def) = self.vehicle_registry.get(&vehicle.def_id) {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: vehicle.pos,
                        size: Vec2::new(def.width, def.height),
                        rotation: vehicle.heading,
                        color: Color::WHITE,
                        layer: heat_core::render::LAYER_ENTITIES,
                        uv_rect: None,
                    });
                }
            } else if let Some(tex) = self.vehicle_textures.first().copied() {
                // Fallback: brug første texture.
                if let Some(def) = self.vehicle_registry.get(&vehicle.def_id) {
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: vehicle.pos,
                        size: Vec2::new(def.width, def.height),
                        rotation: vehicle.heading,
                        color: Color::WHITE,
                        layer: heat_core::render::LAYER_ENTITIES,
                        uv_rect: None,
                    });
                }
            }
        }

        // Render NPC's — detaljeret karakter-sprite, farve baseret på state + walk bob.
        let inner = ctx.world.inner();
        for (_, (npc,)) in &mut inner.query::<(&Npc,)>() {
            if let Some(tex) = self.npc_texture {
                let color = match npc.state {
                    npc_fsm::NpcState::Panic => Color::rgba(1.0, 0.4, 0.4, 1.0),
                    npc_fsm::NpcState::Flee => Color::rgba(1.0, 0.6, 0.3, 1.0),
                    npc_fsm::NpcState::Talk => Color::rgba(0.4, 0.9, 1.0, 1.0),
                    _ => Color::rgba(npc.color[0], npc.color[1], npc.color[2], npc.color[3]),
                };
                // Walk bob: NPC'er der bevæger sig "hopper" let op/ned.
                let moving = npc.state == npc_fsm::NpcState::Walk || npc.state == npc_fsm::NpcState::Flee;
                let bob = if moving {
                    (anim_time * 8.0).sin() * 1.5
                } else {
                    0.0
                };
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: Vec2::new(npc.pos.x, npc.pos.y + bob),
                    size: Vec2::new(24.0, 24.0),
                    rotation: 0.0,
                    color,
                    layer: heat_core::render::LAYER_ENTITIES,
                    uv_rect: None,
                });
            }
        }

        // Render politi — detaljeret karakter med badge, farve baseret på state + walk bob.
        let inner = ctx.world.inner();
        for (_, (police,)) in &mut inner.query::<(&Police,)>() {
            if let Some(tex) = self.police_texture {
                let color = match police.state {
                    PoliceState::Pursue => Color::rgba(1.0, 0.3, 0.3, 1.0),
                    PoliceState::Search => Color::rgba(1.0, 0.7, 0.2, 1.0),
                    _ => Color::WHITE,
                };
                // Walk bob: politi der er i Pursue/Search hopper mere.
                let moving = police.state == PoliceState::Pursue || police.state == PoliceState::Search || police.state == PoliceState::Patrol;
                let speed_mult = if police.state == PoliceState::Pursue { 12.0 } else { 8.0 };
                let bob = if moving {
                    (anim_time * speed_mult).sin() * 2.0
                } else {
                    0.0
                };
                ctx.batch.add(Sprite {
                    texture: tex,
                    position: Vec2::new(police.pos.x, police.pos.y + bob),
                    size: Vec2::new(32.0, 32.0),
                    rotation: police.heading,
                    color,
                    layer: heat_core::render::LAYER_ENTITIES + 2,
                    uv_rect: None,
                });
            }
        }

        // Render player — detaljeret karakter med våben + walk bob.
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
                    // Heat glow: rød farve baseret på heat level.
                    let heat_glow = if self.wanted.heat_points > 0.0 {
                        let intensity = (self.wanted.heat_points / 100.0).min(1.0);
                        Color::rgba(1.0, 1.0 - intensity * 0.5, 1.0 - intensity * 0.8, 1.0)
                    } else {
                        Color::WHITE
                    };
                    // Walk bob: hvis spilleren bevæger sig, hop karakteren let.
                    let moving = (player_ref.pos - self.prev_player_pos).length() > 0.1;
                    let bob = if moving {
                        (anim_time * 10.0).sin() * 2.0
                    } else {
                        0.0
                    };
                    ctx.batch.add(Sprite {
                        texture: tex,
                        position: Vec2::new(player_ref.pos.x, player_ref.pos.y + bob),
                        size: Vec2::new(32.0, 32.0),
                        rotation: 0.0,
                        color: heat_glow,
                        layer: heat_core::render::LAYER_ENTITIES + 1,
                        uv_rect: None,
                    });
                }
            }
        }

        // Combat rendering: projectiles, muzzle flashes, blood particles.
        if let Some(white_tex) = self.white_texture {
            // Projectiles: små gule prikker.
            for proj in &self.combat.projectiles {
                ctx.batch.add(Sprite {
                    texture: white_tex,
                    position: proj.pos,
                    size: Vec2::new(4.0, 4.0),
                    rotation: 0.0,
                    color: Color::rgba(1.0, 0.9, 0.2, 1.0),
                    layer: heat_core::render::LAYER_EFFECTS,
                    uv_rect: None,
                });
            }
            // Muzzle flashes: lyse orange cirkler (kortvarige).
            for flash in &self.combat.muzzle_flashes {
                let intensity = flash.time_left / 0.05; // fade.
                ctx.batch.add(Sprite {
                    texture: white_tex,
                    position: flash.pos,
                    size: Vec2::new(12.0 * intensity, 12.0 * intensity),
                    rotation: flash.angle,
                    color: Color::rgba(1.0, 0.8 * intensity, 0.2 * intensity, intensity),
                    layer: heat_core::render::LAYER_EFFECTS + 1,
                    uv_rect: None,
                });
            }
            // Blood particles: mørkerøde prikker.
            for blood in &self.combat.blood_particles {
                let alpha = (blood.time_left / 0.5).max(0.0);
                ctx.batch.add(Sprite {
                    texture: white_tex,
                    position: blood.pos,
                    size: Vec2::new(blood.size, blood.size),
                    rotation: 0.0,
                    color: Color::rgba(0.6, 0.1, 0.05, alpha),
                    layer: heat_core::render::LAYER_DECALS,
                    uv_rect: None,
                });
            }
        }

        // Day/night overlay: mørk quad over hele kamera-view (alpha baseret på darkness).
        let darkness = self.world_time.time_of_day().darkness();
        if darkness > 0.01 {
            if let Some(white_tex) = self.white_texture {
                let cam = ctx.camera;
                let overlay_w = cam.viewport_w / cam.zoom;
                let overlay_h = cam.viewport_h / cam.zoom;
                // Nat: mørkeblå tint. Dawn/Evening: varm orange tint.
                let overlay_color = match self.world_time.time_of_day() {
                    crate::systems::world_time::TimeOfDay::Dawn => Color::rgba(0.3, 0.2, 0.4, darkness * 0.4),
                    crate::systems::world_time::TimeOfDay::Evening => Color::rgba(0.4, 0.2, 0.1, darkness * 0.3),
                    crate::systems::world_time::TimeOfDay::Night => Color::rgba(0.02, 0.03, 0.12, darkness * 0.7),
                    _ => Color::rgba(0.0, 0.0, 0.05, darkness * 0.3),
                };
                ctx.batch.add(Sprite {
                    texture: white_tex,
                    position: Vec2::new(cam.position.x, cam.position.y),
                    size: Vec2::new(overlay_w, overlay_h),
                    rotation: 0.0,
                    color: overlay_color,
                    layer: heat_core::render::LAYER_NIGHT_OVERLAY,
                    uv_rect: None,
                });
            }
        }

        // Fase 12: HUD tekst-rendering (overlay i screen-space).
        if let Some(font_tex) = self.font_texture {
            let text_renderer = crate::text::TextRenderer::new();
            // Midlertidig: brug font_texture via text renderer.
            let mut tr = text_renderer;
            tr.set_font(font_tex);
            // HUD position: top-left af skærm (kamera top-left + margin).
            let cam = ctx.camera;
            let hud_x = cam.position.x - cam.viewport_w * 0.5 / cam.zoom + 8.0;
            let hud_y = cam.position.y - cam.viewport_h * 0.5 / cam.zoom + 8.0;
            let scale = 2.0; // 16x16 px per tegn.
            let line_h = 12.0;

            // Linje 1: Wallet.
            let wallet_text = format!("${}  ${}", self.economy.wallet.cash, self.economy.wallet.clean);
            let mut sprites = Vec::new();
            tr.add_text(&mut sprites, Vec2::new(hud_x, hud_y), &wallet_text, scale, Color::rgba(1.0, 0.9, 0.3, 1.0), heat_core::render::LAYER_UI);
            // Linje 2: Heat.
            let heat_text = format!("HEAT: {} ({:.0})", self.wanted.level.label(), self.wanted.heat_points);
            let heat_color = if self.wanted.heat_points > 50.0 {
                Color::rgba(1.0, 0.2, 0.2, 1.0)
            } else if self.wanted.heat_points > 20.0 {
                Color::rgba(1.0, 0.6, 0.2, 1.0)
            } else {
                Color::rgba(0.5, 1.0, 0.5, 1.0)
            };
            tr.add_text(&mut sprites, Vec2::new(hud_x, hud_y + line_h), &heat_text, scale, heat_color, heat_core::render::LAYER_UI);
            // Linje 3: Health.
            let health_text = format!("HP: {:.0}/{:.0}", self.player_health.current, self.player_health.max);
            let health_color = if self.player_health.health_pct() > 0.6 {
                Color::rgba(0.3, 1.0, 0.3, 1.0)
            } else if self.player_health.health_pct() > 0.3 {
                Color::rgba(1.0, 0.8, 0.2, 1.0)
            } else {
                Color::rgba(1.0, 0.2, 0.2, 1.0)
            };
            tr.add_text(&mut sprites, Vec2::new(hud_x, hud_y + line_h * 2.0), &health_text, scale, health_color, heat_core::render::LAYER_UI);
            // Linje 4: Mission objective.
            if let Some(ref obj) = self.ui.hud.current_objective {
                let obj_text = format!("OBJ: {}", &obj[..obj.len().min(40)]);
                tr.add_text(&mut sprites, Vec2::new(hud_x, hud_y + line_h * 3.0), &obj_text, scale, Color::rgba(0.8, 0.8, 1.0, 1.0), heat_core::render::LAYER_UI);
            }
            // Linje 5: News ticker.
            if let Some(ref news) = self.ui.hud.news_ticker {
                let news_text = format!("NEWS: {}", &news[..news.len().min(50)]);
                tr.add_text(&mut sprites, Vec2::new(hud_x, hud_y + line_h * 4.0), &news_text, scale, Color::rgba(0.7, 0.7, 0.7, 1.0), heat_core::render::LAYER_UI);
            }
            // Tilføj tekst-sprites til batch.
            for sprite in sprites {
                ctx.batch.add(sprite);
            }

            // Dialog tekst (centreret på skærm).
            if let Some(ref dialog_text) = self.ui.hud.dialog_text {
                let dialog_x = cam.position.x - crate::text::TextRenderer::text_width(dialog_text, scale) * 0.5;
                let dialog_y = cam.position.y + cam.viewport_h * 0.3 / cam.zoom;
                let mut dialog_sprites = Vec::new();
                tr.add_text(&mut dialog_sprites, Vec2::new(dialog_x, dialog_y), dialog_text, scale, Color::rgba(1.0, 1.0, 1.0, 1.0), heat_core::render::LAYER_UI + 1);
                // Valg.
                for (i, choice) in self.ui.hud.dialog_choices.iter().enumerate() {
                    let choice_text = format!("{}. {}", i + 1, choice);
                    tr.add_text(&mut dialog_sprites, Vec2::new(dialog_x, dialog_y + line_h * (i as f32 + 1.5)), &choice_text, scale, Color::rgba(0.8, 0.9, 1.0, 1.0), heat_core::render::LAYER_UI + 1);
                }
                for sprite in dialog_sprites {
                    ctx.batch.add(sprite);
                }
            }
        }
    }
}