# ADR-001: Engine-valg — Custom Rust + wgpu/winit

**Status:** Godkendt
**Dato:** Sprint 1, uge 1
**Beslutningstager:** Bruger + GLM
**Review:** Opus (pending)

## Kontekst

Heat City er et top-down 2D crime sandbox med dyb systemisk simulation (reputation grafer, NPC FSM'er, AI Director, wanted-system, faction-territorier). Spillet kræver:

- Hukommelsessikkerhed (mange entiteter, ingen leaks).
- Høj performance (60 FPS med 500-2000 aktive entiteter).
- Data-drevet design (alt gameplay defineres i eksterne data-filer).
- Cross-platform potentiale (Windows først, Linux senere).
- Fuldstændig kontrol over render-pipeline (custom shaders, post-effects).
- Lang projekt-horisont (18-24 måneder) — teknologien skal være stabil.

Overvejet blev:

1. **Godot 4** (GDScript/C#) — gratis, open-source, fremragende 2D, lynhurtig prototyping.
2. **Unity + C#** — kraftfuld, modne 2D-værktøjer, stort økosystem.
3. **GameMaker / GMS2** — specialiseret i 2D, hurtig prototyping.
4. **Bevy (Rust ECS)** — moderne, data-drevet, ECS-native, men under udvikling.
5. **Custom engine (Rust + wgpu/winit)** — fuld kontrol, maksimal læring, ingen engine-lock-in.
6. **Custom engine (C++ + wgpu)** — fuld kontrol, men hukommelses-sikkerhed manuelt.

## Beslutning

Vi bygger en **custom engine i Rust** med:

- **wgpu** som GPU-abstraktion (Vulkan/DX12/Metal via et sikker API).
- **winit** som windowing/input.
- **hecs** som ECS (se ADR-002).
- **serde + RON** som data-format (se ADR-003).

## Begrundelse

### Hvorfor custom engine frem for Godot/Unity?

1. **Fuld kontrol over render-pipeline.** For top-down 2D med mange sprites, layers, post-effects (regn, vignette), partikel-systemer og lighting, vil en custom renderer give os præcis den performance og det visuelle vi vil have, uden at kæmpe med en engines antagelser.
2. **ECS-native simulation.** Vores spil er tungt systemisk (mange entiteter, systemer der kører over queries). En ECS-arkitektur fra bunden passer bedre end at tilpasse en engines objekt-model.
3. **Data-drevet design fra dag 1.** Al gameplay i eksterne RON-filer. Custom engine gør det trivielt at hot-reload data uden engine overhead.
4. **Ingen engine lock-in.** Vi ejer al kode. Kan porteres, moddes, open-sources senere.
5. **Læringsmål.** Brugeren har udtrykt ønske om at bygge fra bunden. Dette er et læreprojekt så vel som et spil.

### Hvorfor Rust frem for C++?

1. **Hukommelsessikkerhed uden garbage collector.** Kritisk for et spil med mange entiteter, ingen leaks, ingen use-after-free.
2. **Moderne pakkesystem (cargo).** Dependency-håndtering er trivielt vs C++'s cmake/meson/vcpkg helvede.
3. **wgpu er Rust-native.** wgpu er skrevet i Rust, understøtter Rust som first-class.
4. **Type-systemet fanger fejl ved compile-tid.** Mindre tid i debugger, mere tid i iteration.
5. **Null-safety.** Ingen null pointer dereferences.
6. **Fearless concurrency.** Når vi skal parallelisere (Fase 5+), er Rayon trivial at bruge sikkert.

### Hvorfor wgpu frem for raw Vulkan/DX12?

1. **Ét API, alle platforme.** wgpu abstraherer Vulkan (Linux), DX12 (Windows), Metal (macOS), WebGPU (web).
2. **Sikker.** wgpu validerer GPU-kald, fanger fejl uden at crashe GPU'en.
3. **Moderne.** Baseret på WebGPU-specifikationen, fremtidssikret.
4. **Aktivt udviklet.** Stort community, wgpu-teamet er samarbejdsvillige.
5. **Performance.** Næsten raw-metal performance via descriptor-batching.

### Hvorfor winit?

1. De-facto standard for Rust-spil.
2. Cross-platform, understøtter alle input-devices.
3. Integration med wgpu via `wgpu::Surface` er triviel.

## Konsekvenser

### Positive

- Fuldstændig kontrol over alt teknologi.
- Maksimal læring for brugeren.
- Ingen licens-afhængighed af en engine.
- Performance kan optimeres præcist hvor det batter.
- Data-drevet design er naturligt.

### Negative

- **Længere udviklingstid.** Vi bygger motoren selv, ikke bare spillet. Fase 1 tager 8-12 uger før vi har en brugbar motor.
- **Større teknisk risiko.** Hvis wgpu har en bug, må vi debugge det selv (eller rapportere upstream).
- **Ingen indbygget editor.** Vi skal bygge debugging tooling selv (F1-overlay, asset inspector, etc.).
- **Mindre community end Unity/Godot.** Færre tutorials, færre færdige assets.
- **wgpu-learningkurve.** wgpu har en stejl indlæringskurve hvis man ikke kender moderne GPU-API'er.

### Neutral

- Vi bliver bedre Rust-udviklere.
- Spillet er potentelt moddable fra bunden.

## Implementeringsnoter

- Rust 1.96.1 (stable) installeret via rustup. Toolchain: `stable-x86_64-pc-windows-msvc`.
- MSVC Build Tools 14.51 installeret (VS 2022 Community). `tools/cargo.bat` loader MSVC-env før cargo.
- Workspace planlagt: `engine/` crate (heat_core) + `game/` crate (heat_game binary).
- "Hello window" proof-of-concept (Fase 0): winit + wgpu clearer skærmen blå. Beviser toolchain + rendering virker.

## Alternativer overvejet og afvist

### Godot 4
- **For:** Lynhurtig prototyping, gratis, open-source, fremragende 2D.
- **Imod:** Vi ejer ikke koden til motoren. ECS er ikke native. Data-drevet design kræver tilpasning. Brugeren ønskede custom engine.

### Unity
- **For:** Modne 2D-værktøjer, stort økosystem.
- **Imod:** Proprietær, licenskrav, C# ikke Rust, objekt-model ikke ECS-native.

### Bevy
- **For:** Rust ECS-native, moderne, data-drevet.
- **Imod:** Under aktiv udvikling (ikke 1.0), API-ændringer hyppige, risikabelt for 18-måneders projekt. Bevy's indbyggede antagelser kan begrænse.

### C++ + wgpu
- **For:** Fuldstændig kontrol, maksimal performance.
- **Imod:** Hukommelses-sikkerhed manuelt, længere debug-tid, pakkehåndtering sværere.

## Review-krav

Opus skal reviewe denne ADR før Fase 1 start. Særligt:
- Er wgpu tilstrækkeligt modent til 18-måneders projekt?
- Er hecs det rette ECS-valg (se ADR-002)?
- Er der platform-risici vi overser?

## Referencer

- wgpu: https://wgpu.rs
- winit: https://github.com/rust-windowing/winit
- hecs: https://github.com/Ralith/hecs
- rustup: https://rustup.rs

## Dokumenthistorik

| Dato | Begivenhed | Forfatter |
|---|---|---|
| Sprint 1, uge 1 | Oprettet | GLM |