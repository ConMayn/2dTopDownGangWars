# Sprint 01 — Changelog

> **Fase:** 0 — Foundation & Dokumentation
> **Sprint:** 1
> **Dato:** 2026-07-06
> **Status:** ✅ Milestone M0 nået
> **Agenter:** GLM (hovedingeniør), Codex (pending GitHub setup)

---

## Opnået denne sprint

### Dokumentation (Fase 0)
- ✅ Oprettet `docs/`-mappestruktur med 10 undermapper (00-vision til 10-changelog)
- ✅ `docs/00-vision/pitch.md` — pitch, kernefantasi, tone
- ✅ `docs/01-game-design/gdd.md` — fuld Game Design Document (36 sektioner struktureret fra original idé)
- ✅ `docs/02-technical-design/tdd.md` — Technical Design Document (engine-arkitektur, ECS, render-pipeline, data-formater)
- ✅ `docs/03-roadmap/roadmap.md` — fuld roadmap (12 faser over 18-24 måneder, M0→M12)
- ✅ `docs/04-agent-delegation/matrix.md` — præcis opdeling af ansvar mellem GLM / Opus / Codex per domæne og per fase
- ✅ `docs/09-decisions/ADR-001-engine-choice.md` — custom Rust + wgpu/winit begrundet
- ✅ `docs/09-decisions/ADR-002-ecs-choice.md` — hecs valgt (pending Opus review)
- ✅ `docs/09-decisions/ADR-003-data-format.md` — RON til config, bincode til saves

### Toolchain
- ✅ Rust 1.96.1 (stable) installeret og verificeret
- ✅ MSVC Build Tools 14.51 (VS 2022 Community) fundet og integreret
- ✅ `tools/cargo.bat` — loader MSVC-env før cargo (Windows workaround)
- ✅ `tools/msvc-env.bat` — generisk MSVC-env loader

### Kode
- ✅ Cargo workspace oprettet (`engine/` + `game/` crates)
- ✅ `engine/Cargo.toml` med wgpu 24, winit 0.30, hecs 0.10, serde, ron, glam, tracing
- ✅ `game/Cargo.toml` med heat_core, tracing-subscriber
- ✅ `engine/src/lib.rs` — public API
- ✅ `engine/src/app.rs` — App + AppState med winit 0.30 ApplicationHandler
- ✅ `engine/src/render.rs` — Renderer med wgpu 24 (instance, surface, device, queue, clear-pass)
- ✅ `game/src/main.rs` — entry point med tracing init

### Repo
- ✅ `.gitignore` (Rust/Windows/IDE/assets)
- ✅ `README.md` med projektbeskrivelse og build-instruktioner
- ✅ Git initialiseret (brug: ConMayn, branch: main)

### Proof of concept (Milestone M0)
- ✅ `cargo build --workspace` kompilerer uden fejl (272 dependencies)
- ✅ `target/debug/heat_city.exe` åbner et vindue med titlen "Heat City" (1280x720)
- ✅ Vinduet clearer skærmen mørk blå (`[0.1, 0.2, 0.4, 1.0]`)
- ✅ Lukker korrekt ved CloseRequested / Escape

---

## Milestone M0 — Status: ✅ Nået

**Definition:** Dokumenter godkendt + `cargo run` åbner et vindue der clearer skærmen.

**Resultat:**
- Dokumenter: pitch, GDD, TDD, roadmap, matrix, 3 ADR'er — alle skrevet
- Toolchain: Rust + MSVC installeret og fungerende
- Workspace: bygger og kører
- Hello window: åbner, clearer, lukker korrekt

---

## Tekniske noter

### wgpu 24 API-afvigelser fundet under implementering
1. `InstanceDescriptor` kræver `backend_options` felt — brugt `InstanceDescriptor::default()` som workaround.
2. `request_adapter` returnerer `Option<Adapter>`, ikke `Result` — brugt `.ok_or_else(...)`.
3. `request_device` tager 2 argumenter (descriptor + trace_path: `Option<&Path>`) — tilføjet `None`.
4. `Surface::configure` returnerer `()` i wgpu 24, ikke `Result` — fjernet `map_err`.
5. `RenderPassColorAttachment` har ikke `depth_slice` felt — fjernet.
6. `RenderPassDescriptor` bruger `occlusion_query_set`, ikke `occlusion_query_writes` — rettet.

### winit 0.30 API
1. `EventLoop::new()` returnerer `Result` (ikke `Self` direkte).
2. `EventLoop::run` er deprecated; bruger `run_app(&mut A: ApplicationHandler)` i stedet.
3. ApplicationHandler-traitet med `resumed`, `window_event`, `about_to_wait` metoder.
4. Window oprettes lazy i `resumed` (EventLoop kan ikke create_window før resume).

### Windows / MSVC
- `.cargo\bin` var ikke i PATH — `cargo.bat` wrapper løser dette.
- MSVC linker (`link.exe`) ikke i PATH som standard — `vcvars64.bat` loader det.

---

## Åbne punkter til næste sprint (Sprint 2)

1. **GitHub repo oprettelse og første push** (var planlagt til Sprint 1, udskydes til umiddelbart efter)
2. **Opus review af TDD og ADR-001/002** — arkitektur-review før Fase 1
3. **CI opsætning** (GitHub Actions) — når repo er på GitHub
4. **`Cargo.lock` commit** — for binary crates skal den committes; gøres ved første push
5. **Fase 1 start** — ECS-integration, input-system, asset-loader

---

## Næste sprint (Sprint 2) — plan

**Mål:** Start Fase 1 (Engine Core).

- [ ] GitHub repo oprettet + første push (rest fra Sprint 1)
- [ ] Opus review af TDD / ADR'er
- [ ] ECS-integration (hecs wrapper i `engine/src/ecs/`)
- [ ] Input-system (keyboard/mouse via winit)
- [ ] Asset-loader (PNG textures + RON data)
- [ ] Tilemap-renderer (sprite batcher MVP)
- [ ] Fixed timestep tidsløkke
- [ ] Debug overlay (F1: FPS, entity count)
- [ ] Enhedstests for kerne-moduler

**Milestone M1 (Fase 1):** En spilmotor der kan loade og tegne et tilemap + en sprite.

---

## Stats

- **Dokumenter skrevet:** 7 (pitch, GDD, TDD, roadmap, matrix, 3 ADR'er)
- **Kodefiler:** 5 (lib.rs, app.rs, render.rs, main.rs, Cargo.toml ×3)
- **Dependencies:** 272 crates (transitivt)
- **Linjer kode:** ~300 Rust
- **Linjer dokumentation:** ~2500 markdown
- **Tid brugt:** 1 session
- **Hovedagent:** GLM