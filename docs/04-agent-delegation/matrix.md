# Heat City — Agent Delegation Matrix

> **Status:** Godkendt (Sprint 1).
> **Ejer:** GLM (vedligehold)
> **Formål:** Præcis opdeling af ansvar mellem AI agenter så arbejdet er veldefineret og overlap minimeres.

---

## Agenter

| Agent | Rolle | Modellens styrker |
|---|---|---|
| **GLM (mig)** | Hovedingeniør. Skriver mest kode. | Pris-effektiv, intelligent nok til de fleste opgaver, god til systematisk arbejde. **Ikke visuelle opgaver.** |
| **Claude Opus** | Arkitekt & tunge problemer. | Stærk til kompleks arkitektur, algoritmer, narrative, visuelle scripts. |
| **Codex** | Allrounder / support. | Stærk til refaktorering, tests, docs, tooling, bug-fixes. |

---

## Ansvarsfordeling per domæne

### Engine (engine crate)

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| ECS wrapper / integration | GLM | Opus | |
| Render-pipeline (wgpu shaders) | GLM | Opus | Enkle shaders. Avancerede (partikler, post-effects) → Opus |
| Input-system | GLM | Codex | |
| Asset-loader | GLM | Codex | |
| Tids-loop / fixed timestep | GLM | Opus | Determinisme kritisk |
| Threading-model | Opus | GLM | Fase 5+ |
| Profiling / optimering | Opus | GLM | Fase 11 |
| Logging / debug overlay | GLM | Codex | |

### Gameplay (game crate)

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| Player movement / vehicle physics | GLM | Codex | |
| Collision system | GLM | Codex | |
| NPC FSM / rutiner | GLM | Opus | Komplekse FSM → Opus review |
| Faction AI / territory logic | GLM | Opus | Graf-algoritmer → Opus |
| Reputation-system | GLM | Opus | Multi-faction trust-matematik → Opus review |
| Wanted / politi-AI | GLM | Opus | Søge-algoritmer → Opus review |
| AI Director | Opus | GLM | Spændings-styring er arkitektonisk svært |
| Mission-system | GLM | Codex | Data-drevet, GLM skriver format |
| Dialog-træer | GLM | Codex | |
| Økonomi (cash/clean/laundry) | GLM | Codex | |
| Crew-system | GLM | Codex | |
| Heist-planlægning | GLM | Opus | Multi-løsnings-design → Opus review |
| Save-system | GLM | Codex | |
| UI / HUD | GLM | Codex | Fase 11 |
| Audio integration | GLM | Codex | Fase 11 |

### Simulation & world

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| Zone-system / influence graph | GLM | Opus | |
| World time / weather | GLM | Codex | |
| News / rumor propagation | GLM | Opus | Emergent → Opus review |
| Spatial partitioning | Opus | GLM | Arkitektonisk valg |
| Pathfinding (A* / navmesh) | GLM | Opus | |

### Visuelt / shaders / art-systemer

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| Partikel-systemer | Opus | GLM | Visuelt → Opus per user note |
| Post-effects (vignette, rain) | Opus | GLM | Visuelt → Opus |
| Lighting / normal-map sprites | Opus | GLM | Visuelt → Opus |
| Sprite-atlas tooling | Codex | GLM | |
| Asset pipeline scripts | Codex | GLM | |

### Story / narrative / dialog indhold

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| Hovedhistorie / story arc | Opus | GLM | Tung narrative → Opus |
| Faction arcs | Opus | GLM | |
| Mission scripting (content) | Opus | GLM | |
| NPC dialog linjer | Opus | GLM | Kreativt → Opus |
| Radio / news / reklamer indhold | Opus | GLM | |
| Mission definition data (RON) | GLM | Codex | Data entry |

### Tooling / tests / docs

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| Enhedstests | Codex | GLM | |
| Integrationstests | Codex | GLM | |
| CI / build scripts | Codex | GLM | |
| Refaktorering | Codex | GLM | |
| Bug-fixing | Codex | GLM | |
| Dokumentations-generering | Codex | GLM | |
| API docs (rustdoc) | Codex | GLM | |
| Changelog vedligeholdelse | Codex | GLM | |
| Asset validators | Codex | GLM | |

### Arkitektur / beslutninger

| Opgavetype | Primær | Review | Note |
|---|---|---|---|
| ADR'er | Opus | GLM | Arkitektoniske valg → Opus |
| GDD opdateringer | GLM | Opus | |
| TDD opdateringer | Opus | GLM | |
| Scope / feature ændringer | Opus | GLM | |

---

## Per-fase opdeling

### Fase 0 (Sprint 1, nuværende)

- **GLM:** Alt dokumentation (pitch, GDD, TDD, roadmap, matrix, ADR'er), toolchain installation, cargo workspace, hello window.
- **Opus:** Review TDD-arkitektur, ECS-valg, data-format.
- **Codex:** GitHub repo oprettelse, .gitignore, changelog.

### Fase 1 (Engine Core)

- **GLM:** ECS-integration, input, renderer, asset-loader, tilemap, tidsløkke, debug.
- **Opus:** Review render-pipeline, threading-beslutninger, performance-budget.
- **Codex:** Enhedstests for kerne-moduler, CI setup.

### Fase 2 (World & Movement)

- **GLM:** Player entity, movement, collision, kamera, zone-system, test-zone.
- **Opus:** Spatial partitioning-arkitektur, pathfinding-strategi.
- **Codex:** Tests for collision, kamera-clamp.

### Fase 3 (Vehicles)

- **GLM:** Vehicle entity, fysik, stjæle-bil, kollisioner, bil-typer data.
- **Opus:** Fysik-model review (arcade vs sim balance).
- **Codex:** Bil-data validering, tests.

### Fase 4 (NPC & Byens Liv)

- **GLM:** NPC FSM, roller, rutiner, trafik-AI, reaktioner, pathfinding.
- **Opus:** FSM-arkitektur review, NPC memory-model.
- **Codex:** Tests, NPC-data validering.

### Fase 5 (Factions & Reputation)

- **GLM:** Faction-data, influence graf, reputation events, faction-AI, graffiti.
- **Opus:** Influence-graph matematik, faction-beslutnings-træer, balancering.
- **Codex:** Tests, faction-data validering.

### Fase 6 (Wanted / Polici)

- **GLM:** Heat-levels, politi-AI, beviser, flugt-mekanikker, profile.
- **Opus:** Søge-algoritmer, roadblock-strategi, profile-modellering.
- **Codex:** Tests, bevis-system validering.

### Fase 7 (Missioner & Dialog)

- **GLM:** Mission-format, mission-typer, triggers, consequences, telefon.
- **Opus:** Main story skeleton, faction arcs, mission-scripting content, dialog indhold.
- **Codex:** Mission-data validering, tests.

### Fase 8 (Økonomi & Crew)

- **GLM:** Cash/clean/laundry, safehouses, crew-system, fronts, våben/tøj inventory.
- **Opus:** Økonomi-balancering, crew-moral-model.
- **Codex:** Tests, data validering.

### Fase 9 (Heists)

- **GLM:** Heist-plan UI, 4 approaches, set-piece missioner.
- **Opus:** Heist-design review, multi-løsnings-design, set-piece koreografi.
- **Codex:** Tests.

### Fase 10 (AI Director)

- **Opus:** AI Director algoritmer, spændings-styring, event triggers.
- **GLM:** Random events, news, rumors, rival-system, social manipulation, investigation.
- **Codex:** Tests.

### Fase 11 (Polish)

- **GLM:** Audio integration, UI, save-system, options, accessibility.
- **Opus:** Partikler, post-effects, lighting, lyd-design pipeline.
- **Codex:** Performance optimering, CI for release builds, bug-fixing.

### Fase 12 (Vertical Slice)

- **GLM:** Integration, bug-fixing, bygnings-setup.
- **Opus:** Playtest-analyse, story polish, balancering.
- **Codex:** Installer build, release tests, dokumentation.

---

## Samarbejdsregler

1. **Ingen agent overskriver andres arbejde uden review.** Hvis Opus har skrevet en shader, retter GLM den ikke uden Opus' godkendelse.
2. **Alt kode reviewes af mindst én anden agent** før "merge" (commit til main).
3. **ADRs opdateres** når en beslutning ændres. Ingen stille ændringer til arkitektur.
4. **Changelog opdateres per sprint** af Codex.
5. **Tests skal passere** før commit. CI (når oprettet) blokerer push ved fejl.

---

## Kommunikations-kanal (fysisk)

Da vi er asynkrone agenter, "kommunikerer" vi via:
- Dokumenter i `docs/` (især ADR'er og changelog).
- Commit-beskeder (konventional commits).
- Code comments ved ikke-trivielle beslutninger.
- TODO-markers i kode med agent-navn (f.eks. `// TODO(opus): review thread-safety`).

---

## Dokumentations-status

| Dokument | Status |
|---|---|
| matrix.md | Færdig (v1) |