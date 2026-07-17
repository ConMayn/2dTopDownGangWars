# Heat City

> **Working title (placeholder).** Et moderne top-down 2D singleplayer crime sandbox.

Et top-down 2D crime sandbox inspireret af GTA II's frihed, San Andreas' bandeverden og FiveM's rollespilsfølelse — men med moderne systemisk singleplayer-sandbox. Du starter som en nobody i en opdelt by. Byen husker dig. Bander ændrer adfærd. Politiet lærer dine vaner. NPC'er har små liv. Alt kan eskalere.

## Status

**Fase 12 — Vertical Slice (Sprint 13).** Alle 12 faser implementeret (M0-M12).

| Fase | Milestone | Status |
|---|---|---|
| 0 — Foundation | M0 | ✅ Dokumenter + hello window |
| 1 — Engine Core | M1 | ✅ wgpu/winit/hecs, sprite-batching, kamera, assets, time |
| 2 — World & Movement | M2 | ✅ Tilemap, collision, spiller, NPC patrulje, kamera follow |
| 3 — Vehicles | M3 | ✅ 5 bil-typer, arcade fysik, stjæl/ind/ud, tilemap collision |
| 4 — NPC & Byens Liv | M4 | ✅ FSM, dag/nat, spatial grid, mikro-dialog, 3 NPC typer |
| 5 — Factions & Rep | M5 | ✅ 7 factions, 4-lags reputation, influence, 11 events |
| 6 — Wanted / Polici | M6 | ✅ Heat 1-6, evidence, politi-AI (patrol/search/pursue) |
| 7 — Missioner & Dialog | M7 | ✅ Missioner, objectives, rewards, dialog-træer, economy |
| 8 — Safehouses & Crew | M8 | ✅ 5 safehouse-typer, 6 crew-roller, 12 businesses, laundering |
| 9 — Heists | M9 | ✅ 4 approaches, 6 escape routes, 6 diversions, 3 heists |
| 10 — AI Director | M10 | ✅ Tension-styring, 15 events, news, 8 rival-typer |
| 11 — Polish, Audio, UI | M11 | ✅ Save (bincode), HUD, 9 menu screens, audio API, radio |
| 12 — Vertical Slice | M12 | ✅ Kerne-loop, release-notes, how-to-play |

Se `docs/03-roadmap/roadmap.md` for den fulde roadmap.

## Tech stack

- **Sprog:** Rust (stable)
- **GPU:** wgpu (Vulkan/DX12/Metal)
- **Windowing:** winit
- **ECS:** hecs
- **Data:** RON (config) + bincode (saves)
- **Audio:** audio API (kira integration planlagt)

Se `docs/02-technical-design/tdd.md` for fuld teknisk arkitektur.

## Spil-systemer (12 faser)

- **Engine:** wgpu renderer, sprite-batching, kamera (follow + clamp), fixed timestep, asset loader
- **World:** tilemap (800x608), AABB collision, 5 bil-typer med arcade fysik
- **NPC:** FSM (Idle/Walk/Flee/Panic/Talk), 3 NPC typer (pedestrian/shopkeeper/gang), spatial grid
- **Factions:** 7 factions, 4-lags reputation (street rep, faction trust, civilian fear/love, police profile), influence-graf, 11 rep events
- **Politi:** Heat 1-6, 8 evidence-typer, politi-AI (Patrol/Search/Pursue/ReturnToPatrol), dynamic spawn/despawn
- **Missioner:** data-drevne missioner, 7 objective-typer, 5 reward-typer, dialog-træer med valg/betingelser/effekter
- **Economy:** wallet (cash/clean), inventory, items, 14 item-typer
- **Safehouses:** 5 typer (CrashPad→Mansion), stash/garage/crew kapacitet, risiko
- **Crew:** 6 roller, loyalitet/frygt/moral, hire/fire, tick
- **Businesses:** 12 front-typer, money laundering, passiv indkomst, heat pressure
- **Heists:** 4 approaches (Loud/Quiet/Social/Dirty), 6 escape routes, 6 diversions, 3 demo heists
- **AI Director:** 4 tension-niveauer, 6 event-typer, 15 gade-events, nyhedssystem, 8 rival-typer
- **Save:** bincode, 5 slots, versioning, migration
- **UI:** HUD (11 elementer), 9 menu-skærme, dialog display
- **Audio:** 6 lyd-typer, radio-stationer, volume-kanaler, mute

## Dokumentation

Al dokumentation ligger i `docs/`:

- `00-vision/` — pitch, kernefantasi, tone
- `01-game-design/` — Game Design Document (GDD)
- `02-technical-design/` — Technical Design Document (TDD)
- `03-roadmap/` — roadmap, milestones, sprint-plan
- `04-agent-delegation/` — AI agent rolle-matrix
- `09-decisions/` — Architecture Decision Records (ADR'er)
- `10-changelog/` — versionshistorik per sprint (Sprint 01-13)

## Byg

Krav:
- Rust stable (installer via https://rustup.rs)
- MSVC Build Tools (Visual Studio 2022 Community med "Desktop development with C++")

```bat
tools\cargo.bat build --release
tools\cargo.bat run
```

(`tools/cargo.bat` loader MSVC-env før cargo, så link.exe findes.)

Headless test (uden vindue):
```bat
tools\cargo.bat run -- --headless --frame-limit 60 --max-frames 120
```

## Spil

Se `docs/11-release/how-to-play.md` for fuld spilguide.

**Kontrol:**
- **WASD / pile-taster** — bevægelse
- **Shift** — sprint
- **E** — interager (kør bil / start dialog)
- **Venstre klik / Attack** — heist trigger (proof)
- **F1 (ToggleDebug)** — debug overlay (wallet, missions, crew, events, director)

## Licens

Proprietær (CSL Games Studio). Alle rettigheder forbeholdes.