# Sprint 03 — Changelog

> **Fase:** 2 — World & Movement
> **Sprint:** 3
> **Dato:** 2026-07-14
> **Status:** ✅ Milestone M2 nået
> **Agenter:** GLM (hovedingeniør)

---

## Opnået denne sprint

### World & Movement (Fase 2)

- ✅ `game/src/world/mod.rs` — WorldPlugin (hoved-plugin for Fase 2: byen + spiller + NPC)
- ✅ `game/src/world/tiles.rs` — TileDef, TileType, TileRegistry (data-drevet tile-typer)
- ✅ `game/src/world/tilemap.rs` — Tilemap med grid rendering (kun synlige tiles, culling)
- ✅ `game/src/world/collision.rs` — AABB vs solid tiles, move-and-collide med slide-along-walls
- ✅ `game/src/world/zone.rs` — ZoneDef + ZoneBounds + NpcSpawn (RON-serialiserbare)
- ✅ `game/src/world/npc.rs` — Npc komponent, Patrol komponent, update_npc_patrol system
- ✅ `game/src/main.rs` — opdateret til at bruge WorldPlugin

### Test-zone (East Blocks lille version)

- 25×19 tiles à 32px = 800×608px zone
- Perimeter-mure (solid)
- Asfalt-gader (horisontal + vertikal kryds)
- Fortov langs gader
- 4 bygning-blokke i hjørner (solid)
- Græs-pletter
- 3 NPC'ere med 4-waypoint patrol-ruter

### Gameplay

- Spiller (32×32 blå sprite) bevæger sig med WASD + sprint (Shift, 320 px/s)
- Collision mod bygninger og mure — spilleren slider langs væggene
- NPC'ere (24×24 grønne sprites) patroljerer waypoints autonomt
- NPC'ere har også collision (de undgår bygninger)
- Kamera følger spiller, clamps til zone-bounds (800×608)
- Tilemap renders kun synlige tiles (viewport culling for performance)

---

## Milestone M2 — Status: ✅ Nået

**Definition:** Du kan gå rundt i én lille by-zone med NPC'er der går på stier. Kamera følger dig og er clamped til zone.

**Resultat:**
- Tilemap rendering med 5 tile-typer (asphalt, sidewalk, grass, building, wall) ✅
- Collision system (AABB vs solid tiles, slide-along-walls) ✅
- Player movement med WASD + sprint ✅
- NPC patrol system (waypoint cycling med collision) ✅
- Kamera follow + clamp til zone-bounds ✅
- Viewport culling (kun synlige tiles renders) ✅
- Zone-definition format (RON) klar til fremtidig data-driven zones ✅

---

## Tekniske noter

### hecs 0.10
- `Component` trait er auto-implementeret for `T: Send + Sync + 'static` — ingen manuel `impl Component` nødvendig (fjernede 3 fejlende impls).
- `query_mut::<(&mut A, &B)>()` returnerer en iterator direkte (kan bruges i for-loop).
- `query::<(&A,)>()` kræver `&mut` borrow til IntoIterator — brug `&mut inner.query::<...>()`.
- `World::spawn((component_a, component_b))` for multi-component spawn (tuple er DynamicBundle).

### Collision system
- "Move-and-collide" pattern: flyt X først, resolve, så Y. Giver slide-along-walls.
- Out-of-bounds tiles = solid (verden er indelukket, ingen flugt fra zonen).
- NPC hitbox 24×24, player hitbox 32×32.

### Tilemap rendering
- Viewport culling: beregner synlig tile-range fra kamera-view, renderer kun disse.
- Fallback rendering: tiles uden texture renders som farvede quads (via null texture handle).
- Layer-baseret sortering (ground < entities < effects < UI).

### Zone-data
- ZoneDef er RON-serialiserbar, klar til at loade fra `assets/data/zones/` i Fase 4+.
- NpcSpawn understøtter patrol-rute (waypoints) — klar til data-drevet NPC-spawning.

---

## Næste sprint (Sprint 4) — plan

**Mål:** Start Fase 3 (Vehicles).

- [ ] Vehicle-entitet (data-drevet: VehicleData)
- [ ] Arcade bil-fysik (forward, turn, drift)
- [ ] Stjæl bil (hotwire-tid, proximity check)
- [ ] Ind/udstigning
- [ ] Kollision bil↔verden, bil↔bil, bil↔fodgænger
- [ ] 3-5 forskellige bil-typer (data RON)
- [ ] Skade-modellering (visuelt + stat)
- [ ] Nummerplade-system (data)

**Milestone M3:** Spilleren kan stjæle en bil, køre den rundt, og forlade den. Forskellige biler har forskellig føling.

---

## Stats

- **Nye moduler:** 5 (tiles, tilemap, collision, zone, npc) + world/mod
- **Nye filer:** 7 (game/src/world/* ×6, game/src/main.rs rewrite)
- **Linjer kode:** ~520 Rust (game/world)
- **Hovedagent:** GLM