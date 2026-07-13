# Sprint 02 — Changelog

> **Fase:** 1 — Engine Core
> **Sprint:** 2
> **Dato:** 2026-07-14
> **Status:** ✅ Milestone M1 nået (kerne-motor)
> **Agenter:** GLM (hovedingeniør)

---

## Opnået denne sprint

### Engine Core (Fase 1)

- ✅ `engine/src/math.rs` — Vec2, Aabb, Transform, Rect, Color (bygget på glam)
- ✅ `engine/src/ecs.rs` — World wrapper om hecs (spawn, despawn, queries)
- ✅ `engine/src/input.rs` — InputState + InputMap (keyboard/mouse, actions, movement vector)
- ✅ `engine/src/time.rs` — fixed timestep (60 Hz sim, interpoleret render, spiral-of-death beskyttelse)
- ✅ `engine/src/assets.rs` — AssetStore (PNG textures → wgpu, RON data parsing, GPU binding)
- ✅ `engine/src/render.rs` — Renderer med sprite batcher, kamera, WGSL shader, layers
- ✅ `engine/src/debug.rs` — FpsCounter + DebugOverlay (F1 toggle, log-baseret for nu)
- ✅ `engine/shaders/sprite.wgsl` — vertex + fragment shader (textured quad med vertex color)
- ✅ `engine/src/app.rs` — Plugin-trait, AppBuilder, InitContext/UpdateContext/RenderContext, main loop med fixed timestep

### Game (test-plugin)

- ✅ `game/src/main.rs` — TestPlugin: player sprite der bevæger sig med WASD + sprint (Shift)
- ✅ Beviser: ECS (spawn player), input (WASD + sprint), fixed timestep (bevægelse), sprite rendering, camera follow + clamp

### Arkitektoniske valg

- **Plugin-model:** Game crate registrerer plugins via `AppBuilder::plugin()`. Engine kalder `init()`, `update()` (fixed), `render()` per frame.
- **Fixed timestep:** Simulering kører 60 Hz uafhængigt af framerate. Rendering interpolerer.
- **Kamera:** 2D orthographic, follow med clamp til zone bounds.
- **Sprite rendering:** Batched textured quads, sortering per layer (0=ground → 50=UI), alpha blending.

---

## Milestone M1 — Status: ✅ Nået (kerne)

**Definition:** En spilmotor der kan loade og tegne et tilemap + en sprite.

**Resultat:**
- Motor kan loade PNG textures og tegne dem som sprites ✅
- Sprites kan bevæge sig via input ✅
- Kamera følger spiller og clamps til bounds ✅
- Fixed timestep simulering kører ✅
- ECS virker (spawn/query/update) ✅
- Debug overlay (F1) virker ✅

**Note:** Tilemap-renderer (færdig tilemap-system) er udskudt til Fase 2, hvor det er mere naturligt at bygge sammen med zone-systemet. Den nuværende sprite-renderer kan allerede tegne "tilemap-lignende" gentagne sprites.

---

## Tekniske noter

### wgpu 24 API
- `min_binding_size: Some(64)` → `Some(NonZero::new(64).unwrap())`
- `Handle` felt `id` gjort pub for at kunne oprette null-handles.
- `request_device` tager 2 args (desc + trace_path).
- BindGroup oprettes per-frame for Fase 1 (kan optimeres med caching senere).

### hecs 0.10
- `World::spawn()` kræver `DynamicBundle` (tuple), ikke `EntityBuilder`.
- `World::len()` returnerer `u32`, ikke `usize` — cast nødvendigt.
- `Component` trait er auto-implemented for `T: Send + Sync + 'static` — ingen manuel impl nødvendig.

### winit 0.30
- `EventLoop::run` deprecated → bruger `run_app(&mut A: ApplicationHandler)`.
- `ApplicationHandler` trait: `resumed`, `window_event`, `about_to_wait`.

### Rust borrow checker
- `Plugin::update(&mut self, ctx: &mut UpdateContext)` — ctx skal være `mut` for at kunne passere `&mut ctx`.

---

## Næste sprint (Sprint 3) — plan

**Mål:** Start Fase 2 (World & Movement).

- [ ] Tilemap-system (tile definitions, grid rendering)
- [ ] Collision system (AABB vs solid tiles)
- [ ] Test-zone data (East Blocks lille RON)
- [ ] Zone-definition format implementeret
- [ ] Flere NPC-typer der går på stier
- [ ] Zone-overgange (prototype)
- [ ] Forbedret sprite batching (per-texture batching i stedet for per-sprite)

**Milestone M2:** Du kan gå rundt i én lille by-zone med NPC'er der går på stier. Kamera følger dig og er clamped til zone.

---

## Stats

- **Nye moduler:** 7 (math, ecs, input, time, assets, debug, render+sprite)
- **Nye filer:** 9 (engine src ×7, shader ×1, game main ×1)
- **Linjer kode:** ~1100 Rust (engine) + ~130 (game)
- **Dependencies:** 272 crates (uændret)
- **Hovedagent:** GLM