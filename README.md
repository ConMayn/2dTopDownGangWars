# Heat City

> **Working title (placeholder).** Et moderne top-down 2D singleplayer crime sandbox.

Et top-down 2D crime sandbox inspireret af GTA II's frihed, San Andreas' bandeverden og FiveM's rollespilsfølelse — men med moderne systemisk singleplayer-sandbox. Du starter som en nobody i en opdelt by. Byen husker dig. Bander ændrer adfærd. Politiet lærer dine vaner. NPC'er har små liv. Alt kan eskalere.

## Status

**Fase 0 — Foundation & Dokumentation (Sprint 1).**

Se `docs/03-roadmap/roadmap.md` for den fulde roadmap (12 faser over ~18-24 måneder).

## Tech stack

- **Sprog:** Rust (stable)
- **GPU:** wgpu (Vulkan/DX12/Metal)
- **Windowing:** winit
- **ECS:** hecs
- **Data:** RON (config) + bincode (saves)
- **Audio:** kira (planned, Fase 11)

Se `docs/02-technical-design/tdd.md` for fuld teknisk arkitektur.

## Dokumentation

Al dokumentation ligger i `docs/`:

- `00-vision/` — pitch, kernefantasi, tone
- `01-game-design/` — Game Design Document (GDD)
- `02-technical-design/` — Technical Design Document (TDD)
- `03-roadmap/` — roadmap, milestones, sprint-plan
- `04-agent-delegation/` — AI agent rolle-matrix
- `05-asset-pipeline/` — asset spec
- `06-ai-director/` — emergent behavior design
- `07-lore-world/` — by-zoner, factions, NPC'er
- `08-testing/` — test-strategi
- `09-decisions/` — Architecture Decision Records (ADR'er)
- `10-changelog/` — versionshistorik per sprint

## Byg

Krav:
- Rust stable (installer via https://rustup.rs)
- MSVC Build Tools (Visual Studio 2022 Community med "Desktop development with C++")

```bat
tools\cargo.bat build --release
tools\cargo.bat run
```

(`tools/cargo.bat` loader MSVC-env før cargo, så link.exe findes.)

## Licens

Proprietær (CSL Games Studio). Alle rettigheder forbeholdes.