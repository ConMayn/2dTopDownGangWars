# Heat City — Technical Design Document (TDD)

> **Status:** Levende dokument. Opdateres løbende.
> **Sidst opdateret:** Sprint 1, uge 1
> **Ejer:** Opus (arkitektur-review), GLM (forfatter + implementer)
> **Afhængigheder:** ADR-001 (engine-valg), ADR-002 (ECS), ADR-003 (data-format)

---

## Indholdsfortegnelse

1. [Teknisk stack](#1-teknisk-stack)
2. [Workspace-struktur](#2-workspace-struktur)
3. [Engine-arkitektur](#3-engine-arkitektur)
4. [ECS-design](#4-ecs-design)
5. [Render-pipeline](#5-render-pipeline)
6. [Tids-loop & simulering](#6-tids-loop--simulering)
7. [Input-system](#7-input-system)
8. [Asset-pipeline](#8-asset-pipeline)
9. [Data-formater](#9-data-formater)
10. [Threading-model](#10-threading-model)
11. [Simulations-arkitektur](#11-simulations-arkitektur)
12. [Save-system](#12-save-system)
13. [Modding & data-drevet design](#13-modding--data-drevet-design)
14. [Performance-mål](#14-performance-mål)
15. [Fejlhåndtering & logging](#15-fejlhåndtering--logging)
16. [Platform-understøttelse](#16-platform-understøttelse)
17. [Build & CI](#17-build--ci)
18. [Sikkerhed & licenses](#18-sikkerhed--licenses)

---

## 1. Teknisk stack

| Komponent | Valg | Begrundelse |
|---|---|---|
| Sprog | Rust (stable) | Hukommelsessikkerhed, null-safety, moderne pakkesystem, zero-cost abstractions. Perfekt til systemiske spil med mange entiteter. |
| GPU/API | wgpu | Cross-platform (Vulkan/DX12/Metal/WebGPU), moderne, aktivt udviklet, sikker wrapper om native GPU APIs. |
| Windowing | winit | De-facto standard, cross-platform, understøtter alle input-devices. |
| ECS | hecs | Letvægt, arketyper-baseret, god performance, enkel API. (ADR-002) |
| Data (config) | RON | Læsbart for mennesker, typed, bedre end JSON/YAML til spilkonfig. |
| Data (saves) | serde + bincode | Kompakt binært format, versions-støttet. |
| Audio | kira | Letvægt, looping, ambient, musik + SFX. Kommer i Fase 11. |
| Version styring | Git | Standard, decentraliseret. |
| Build | cargo workspace | Multi-crate, fælles target-dir. |
| CI | GitHub Actions | Gratis for offentlig repo, Linux/Windows/macOS runners. |
| Editor | VS Code | Installeret, Rust-analyzer, god debugging. |
| Profilering | perf, Tracy (senere) | Performance optimering i Fase 11. |

**Platform-krav:**
- **Windows:** Primær udviklingsplatform (MSVC toolchain, wgpu → DX12).
- **Linux:** Sekundær (senere). wgpu → Vulkan.
- **macOS:** Ikke prioriteret, men wgpu → Metal gør det teknisk muligt.

---

## 2. Workspace-struktur

```
2dTopDownGangWars/                  # repo root
├── Cargo.toml                      # workspace root
├── .gitignore
├── README.md
├── docs/                           # al dokumentation
├── engine/                         # "heat_core" crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── app.rs                  # main loop
│       ├── ecs/                   # ECS wrapper over hecs
│       ├── render/                # wgpu renderer
│       ├── input/                 # winit input
│       ├── assets/                # asset loader
│       ├── time/                  # timestep system
│       ├── math/                  # Vec2, AABB, transforms
│       └── debug/                 # FPS overlay, profiler
├── game/                           # "heat_game" crate (binary)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                # entry point
│       ├── world/                 # world state, zones
│       ├── entities/              # player, vehicles, npc
│       ├── systems/               # gameplay systems
│       ├── factions/              # faction logic
│       ├── police/                # wanted, heat, AI
│       ├── missions/              # mission system
│       ├── dialog/                # dialog trees
│       ├── economy/               # cash/clean/laundry
│       ├── crew/                  # crew system
│       ├── director/              # AI director
│       ├── ui/                    # HUD, menus, phone
│       └── data/                  # static data (zones, items)
├── assets/                         # runtime assets
│   ├── textures/
│   ├── audio/
│   ├── data/                      # RON files
│   └── maps/
└── tools/                         # build scripts, converters
    ├── cargo.bat                  # MSVC env loader (Windows)
    └── msvc-env.bat
```

### Crate-afhængigheder

```
game → engine
game → hecs
game → serde, ron
engine → wgpu, winit, hecs
```

**Princip:** Engine er "dum" — den ved intet om spil. Game crate indeholder al gameplay-logik. Dette adskiller rendering/simulering fra gameplay og gør engine genbruglig.

---

## 3. Engine-arkitektur

### 3.1 Lag

```
┌──────────────────────────────────────────┐
│  game crate (gameplay, simulation)       │
├──────────────────────────────────────────┤
│  engine crate (heat_core)                │
│  ┌────────────┐ ┌────────────┐          │
│  │  render    │ │   input    │          │
│  └────────────┘ └────────────┘          │
│  ┌────────────┐ ┌────────────┐          │
│  │  assets    │ │   time    │          │
│  └────────────┘ └────────────┘          │
│  ┌────────────┐ ┌────────────┐          │
│  │   ecs      │ │   math    │          │
│  └────────────┘ └────────────┘          │
├──────────────────────────────────────────┤
│  wgpu (GPU)   winit (window/input)       │
└──────────────────────────────────────────┘
```

### 3.2 App-loop

```rust
// engine/src/app.rs (koncept)
pub struct App {
    world: World,              // hecs ECS world
    renderer: Renderer,
    input: InputState,
    assets: AssetStore,
    time: Time,
    plugins: Vec<Box<dyn Plugin>>,
}

pub trait Plugin {
    fn update(&mut self, ctx: &mut UpdateContext);
    fn render(&mut self, ctx: &mut RenderContext);
}
```

Game crate registrerer gameplay-systemer som plugins. Engine kalder dem i fast rækkefølge: input → simulation → rendering.

---

## 4. ECS-design

### 4.1 Valg: hecs (ADR-002)

- Arketyper-baseret (god cache-locality for batch queries).
- Letvægt, ingen macro-magi.
- Rust-idiomatisk.

### 4.2 Kerne-komponenter (Fase 1-3)

```rust
// Position i verden
struct Position(Vec2);
struct Rotation(f32);
struct Velocity(Vec2);

// Rendering
struct Sprite { texture: Handle<Texture>, layer: u32 }
struct Animated { anim: Handle<Animation>, time: f32 }

// Fysik
struct Collider { bounds: AABB }
struct Solid;                  // blokerer bevægelse

// Identitet
struct Player;
struct Vehicle { data: VehicleData }
struct Npc { data: NpcData }
struct Faction { id: FactionId }

// Tagging
struct Name(String);
struct Tag(&'static str);
```

### 4.3 Systemer

Systemer er funktioner der kører over queries:

```rust
fn movement_system(world: &mut World, dt: f32, input: &InputState) {
    for (pos, vel, _player) in world.query_mut::<(&mut Position, &mut Velocity, &Player)>() {
        // ...
    }
}
```

Systemer opdeles i faser: PreSim, Sim, PostSim, Render. Kørselsrækkefølge defineres i `game/src/main.rs`.

---

## 5. Render-pipeline

### 5.1 2D batched sprite renderer

- Én vertex/index buffer per texture-atlas.
- Sortering: layer (background/midground/foreground/UI).
- Alpha blending slået til for sprites.
- Kamera: 2D transform + zoom, clamp til zone bounds.

### 5.2 Layers

| Layer | Indhold |
|---|---|
| 0 (ground) | tilemap, asfalt, græs |
| 1 (decals) | graffiti, blod, dæk-mærker |
| 2 (entities) | biler, NPC'er, spiller |
| 3 (overhead) | trækroner, tag-udhæng |
| 4 (effects) | partikler, lys |
| 5 (ui) | HUD, menuer, telefon |

### 5.3 Shader-pipeline

Fase 1: simpel vertex+fragment shader (textured quad).
Senere faser: outlines, lighting (normal-maps), post-effects (vignette, rain).

---

## 6. Tids-loop & simulering

### 6.1 Fixed timestep til simulering

```rust
const FIXED_DT: f32 = 1.0 / 60.0;  // 60 Hz simulation
let mut accumulator = 0.0;

loop {
    let frame_time = timer.tick();
    accumulator += frame_time;
    while accumulator >= FIXED_DT {
        input.poll();
        simulation.update(FIXED_DT);
        accumulator -= FIXED_DT;
    }
    let alpha = accumulator / FIXED_DT;
    renderer.render(alpha);  // interpolation
}
```

### 6.2 Begrundelse

- Deterministisk simulering (vigtigt for NPC-rutiner, wanted-afkøling).
- Stabil fysik uanset framerate.
- Rendering kan køre uncapped (144 Hz+ monitore).

---

## 7. Input-system

### 7.1 Abstraktion

```rust
pub struct InputState {
    keys: HashSet<KeyCode>,
    mouse: MouseState,
    gamepad: Option<GamepadState>,
    actions: HashMap<Action, bool>,  // mapped actions
}

pub enum Action { MoveUp, MoveDown, MoveLeft, MoveRight,
                  EnterVehicle, ExitVehicle, Fire, /* ... */ }
```

### 7.2 Rebinding

Input-mapping er data-drevet (RON-fil). Spilleren kan remappe. Default: WASD + mus, gamepad-understøttelse i Fase 4+.

---

## 8. Asset-pipeline

### 8.1 Formater

| Type | Format | Begrundelse |
|---|---|---|
| Textures | PNG (kilde) → RGBA8 (runtime) | Tabsløst, universelt. |
| Audio | OGG Vorbis | Kompakt, God kvalitet, patent-fri. |
| Config/data | RON | Læsbart, typed. |
| Saves | bincode (binært) | Kompakt, versions-støttet. |
| Maps | RON / TMX (Tiled) | TMX importer i Fase 2. |

### 8.2 Loader

Asset-loader kører asynkront (thread pool). Textures uploades til GPU når klar. Handles er typeløse refs der resolves ved brug.

```rust
pub struct AssetStore {
    textures: HashMap<String, Handle<Texture>>,
    audio: HashMap<String, Handle<AudioSource>>,
    data: HashMap<String, serde_value::Value>,
}
```

---

## 9. Data-formater

### 9.1 Zone-definition (RON)

```ron
Zone(
    id: "east_blocks",
    name: "East Blocks",
    bounds: Rect(min: (0, 0), max: (2000, 1500)),
    owner: Faction("southline_kings"),
    police_intensity: 0.3,
    economic_level: 0.4,
    civilian_fear: 0.2,
    gang_presence: 0.7,
    weapon_level: 0.5,
    drug_market: 0.6,
    traffic_density: 0.5,
)
```

### 9.2 Faction-definition

```ron
Faction(
    id: "southline_kings",
    name: "Southline Kings",
    type: StreetGang,
    home_zones: ["east_blocks"],
    allies: [],
    enemies: ["los_cuervos"],
    income: 500,
    aggression: 0.7,
    discipline: 0.3,
)
```

### 9.3 Mission-definition

```ron
Mission(
    id: "wrong_car_wrong_block",
    title: "Wrong Car, Wrong Block",
    giver: Contact("spider"),
    prerequisites: PreReq(street_rep_min: 10),
    steps: [
        Step(type: StealCar, target: "spider_car", zone: "east_blocks"),
        Step(type: Choice(
            options: [
                ("return_apologize", ...),
                ("sell_quick", ...),
                ("gift_to_rival", ...),
                ("keep_war", ...),
            ]
        )),
    ],
    rewards: Cash(500),
    consequences: [RepChange("southline_kings", -20)],
)
```

### 9.4 Versions-støttet save

```rust
#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub version: u32,
    pub player: PlayerState,
    pub factions: HashMap<FactionId, FactionState>,
    pub zones: HashMap<ZoneId, ZoneState>,
    pub world_time: f64,
    pub missions: MissionProgress,
    // ...
}
```

---

## 10. Threading-model

### 10.1 Fase 1-4: Single-threaded simulering

Forenklet, deterministisk, lettere at debugge. wgpu-rendering kører async (intern i wgpu).

### 10.2 Fase 5+: Parallelle systemer

Nogle systemer kan paralleliseres (f.eks. NPC AI, pathfinding):
- rayon til parallel queries.
- Simulering forbliver deterministisk via fast timestep.

### 10.3 Asset-loading

Async via thread pool, uafhængig af simulering.

---

## 11. Simulations-arkitektur

### 11.1 Verdens-state

```rust
pub struct World {
    pub entities: hecs::World,
    pub zones: HashMap<ZoneId, ZoneState>,
    pub factions: HashMap<FactionId, FactionState>,
    pub player: PlayerState,
    pub time: WorldTime,
    pub weather: Weather,
    pub news_queue: Vec<NewsEvent>,
    pub rumors: Vec<Rumor>,
}
```

### 11.2 Systemer per frame

1. **Input** — poll, map actions.
2. **AI Director** — vurder spænding, evt. trigger events.
3. **NPC rutiner** — opdater pathfinding, schedules.
4. **Faction AI** — beslutninger, patruljer, konflikter.
5. **Politi** — wanted update, patrol, search, chase.
6. **Spiller-systemer** — movement, vehicle, combat, inventory.
7. **Verden** — zone influence, weather, time, news.
8. **Mission-system** — check triggers, advance states.
9. **UI** — HUD, dialog, phone.
10. **Rendering** — interpolated, capped.

### 11.3 Spatial partitioning (Fase 4+)

- Grid-baseret (cell size ~64 px).
- Bruges til: collision, AI perception, query "hvem er i nærheden".
- Undgår O(n²) ved mange NPC'er.

---

## 12. Save-system

### 12.1 Slots

- Auto-save per in-game dag (eller per mission).
- Manuel save: kun i safehouses (design-regel fra GDD).
- 5 save slots.

### 12.2 Format

Bincode (binært), versions-støttet. Hvis save-version < current, kør migration.

### 12.3 Hvad gemmes

- Spillerens state (position, health, inventory, skills).
- Faction trust per faction.
- Zone influence per zone.
- Mission progress (completed, active, failed).
- Crew state.
- Safehouse state.
- World time, weather, news history.
- Police profile.
- Beviser (evidence ledger).

### 12.4 Hvad IKKE gemmes

- Position af hver NPC (respawner baseret på zone-tilstand).
- Midlertidige effekter (partikler, blod).

---

## 13. Modding & data-drevet design

### 13.1 Princip

Al gameplay-data er ekstern (RON i `assets/data/`). Kode læser data, ikke hardcoder. Dette gør:
- Balancering nemt (ingen recompile).
- Modding mulig.
- A/B test af design nemt.

### 13.2 Hot-reload (Fase 5+)

Data-filer kan genindlæses ved runtime (F5). Hurtig iteration.

### 13.3 Modding-support

- Modder kan placere RON-filer i `mods/`-mappe.
- Engine merger modded data med base-data.
- Ikke et prioritet før MVP, men designes til det.

---

## 14. Performance-mål

### 14.1 MVP (Fase 12)

- 60 FPS stable på mid-range hardware.
- Op til 500 aktive entiteter (NPC + vehicles).
- Simulering: < 8 ms per frame (fixed timestep).
- Rendering: < 8 ms per frame.
- Memory: < 1 GB.

### 14.2 Full release

- 60-144 FPS.
- Op til 2000 aktive entiteter.
- Load time: < 5 sek.

### 14.3 Optimeringer

- Sprite atlas batching.
- Frustum/zone culling.
- Spatial partitioning (grid).
- Instanced rendering (senere).
- Asynkron asset loading.

---

## 15. Fejlhåndtering & logging

### 15.1 Strategi

- Engine-fejl: `Result<T, EngineError>`, recoverable.
- Uundgåelige fejl (GPU tabt, asset mangler): `panic!` med klar besked + crash log.
- Gameplay-fejl: log warning, fortsæt.

### 15.2 Logging

- `tracing` crate (structured logging).
- Log niveauer: ERROR, WARN, INFO, DEBUG, TRACE.
- Fil-rotation i `logs/heat_city.log`.
- In-game debug overlay (F1): FPS, entity count, memory.

---

## 16. Platform-understøttelse

| Platform | Status | Note |
|---|---|---|
| Windows 10/11 (x64) | Primær | MSVC toolchain, wgpu → DX12. |
| Linux | Sekundær (senere) | wgpu → Vulkan. |
| macOS | Ikke prioriteret | Teknisk muligt via wgpu → Metal. |
| Web | Ikke planlagt | wgpu understøtter WebGPU, men for tungt for dette spil. |

---

## 17. Build & CI

### 17.1 Lokalt build

```bat
tools\cargo.bat build --release
tools\cargo.bat run
```

(`cargo.bat` loader MSVC env først, se `tools/`.)

### 17.2 CI (GitHub Actions, når repo oprettes)

```yaml
# .github/workflows/build.yml (skitse)
on: [push, pull_request]
jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --workspace --release
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo fmt -- --check
```

### 17.3 Pre-commit hooks (valgfri, senere)

- `cargo fmt --check`
- `cargo clippy`
- Konventional commits (se changelog-format).

---

## 18. Sikkerhed & licenses

### 18.1 Lisenser

- Vores kode: Proprietær (CSL Games Studio) indtil videre.
- Afhængigheder: kun MIT/Apache-2.0/BSD/ISC licenserede crates.
- Tredjeparts assets: kun med licens dokumenteret i `assets/LICENSES.md`.

### 18.2 Sikkerhed

- Ingen netværkskode i MVP (singleplayer).
- Save-filer valideres ved load (undgå korruption).
- Ingen `unsafe` uden review og SAFETY-kommentar.
- Crates auditeres før tilføjelse (cargo-audit, senere).

---

## Åbne spørgsmål (til Opus review)

1. **hecs vs custom ECS:** hecs er arketyper-baseret. For meget dynamic component add/remove (f.eks. NPC der skifter rolle hyppigt) kan arketyper give overhead. Alternativ: spektakulær `bevy_ecs` som standalone, eller en custom sparse-set ECS. Opus review kræves.
2. **Threading-strategi:** Start single-threaded, eller rayon fra Fase 5?
3. **Map-format:** RON eller Tiled TMX? TMX kræver parser, men giver visuel editor.
4. **Audio-strategi:** kira vs rodio vs egen mixer? Udskydes til Fase 11.
5. **Networking-kode:** Selvom singleplayer, skal telefontjenester (f.eks. fetched news) simuleres? Ja, via world-state, ikke netværk.

---

## Dokumentations-status

| Afsnit | Status | Ejer |
|---|---|---|
| 1-9 | Skitse færdig | GLM |
| 10-12 | Skitse færdig | GLM |
| 13-18 | Skitse færdig | GLM |
| Review | Pending | Opus |