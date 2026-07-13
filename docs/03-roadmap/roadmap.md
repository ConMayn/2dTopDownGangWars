# Heat City — Roadmap

> **Status:** Godkendt (Sprint 1).
> **Sidst opdateret:** Sprint 1, uge 1
> **Ejer:** GLM (vedligehold), Opus (review)
> **Tidsramme:** ~18-24 måneder (konservativ estimering, 1 deltidsudvikler + AI agenter).

---

## Milestone-oversigt

| Milestone | Fase | Mål | Estimeret |
|---|---|---|---|
| M0 | 0 — Foundation | Dokumenter + hello window | 4-6 uger |
| M1 | 1 — Engine Core | Motor kan loade + tegne | 8-12 uger |
| M2 | 2 — World & Movement | Gå rundt i én zone | 6-8 uger |
| M3 | 3 — Vehicles | Stjæle og køre biler | 4-6 uger |
| M4 | 4 — NPC & Byens Liv | Byen føles levende | 8-10 uger |
| M5 | 5 — Factions & Rep | Factions reagerer | 6-8 uger |
| M6 | 6 — Wanted / Polici | Intelligent flugt | 6-8 uger |
| M7 | 7 — Missioner & Dialog | Missioner med konsekvenser | 8-10 uger |
| M8 | 8 — Økonomi & Crew | Progression-økonomi | 6-8 uger |
| M9 | 9 — Heists | Planlægnings-heists | 6-8 uger |
| M10 | 10 — AI Director | Emergent drama | 6-8 uger |
| M11 | 11 — Polish, Audio, UI | Færdig oplevelse | 6-8 uger |
| M12 | 12 — Vertical Slice | Spilbar release | 4-6 uger |

**Total:** ~78-118 uger (~18-27 måneder).

---

## Fase 0 — Foundation & Dokumentation (4-6 uger)

**Mål:** Komplet GDD, TDD, roadmap og agent-matrix. Intet gameplay-kode endnu.

### Opgaver

- [x] Mappestruktur + docs-framework
- [x] `docs/00-vision/pitch.md`
- [x] `docs/01-game-design/gdd.md` (v1)
- [x] `docs/02-technical-design/tdd.md` (v1)
- [x] `docs/03-roadmap/roadmap.md`
- [x] `docs/04-agent-delegation/matrix.md`
- [x] `docs/09-decisions/ADR-001-engine-choice.md`
- [x] `docs/09-decisions/ADR-002-ecs-choice.md`
- [x] `docs/09-decisions/ADR-003-data-format.md`
- [x] Installer Rust toolchain, verificer cargo
- [x] Opret cargo workspace (`engine/` + `game/` crates)
- [x] Skriv "hello window" — winit + wgpu der clearer skærmen blå
- [x] `docs/10-changelog/sprint-01.md`
- [x] Git init + GitHub repo oprettelse + push

### Milestone M0

Dokumenter godkendt + `cargo run` åbner et vindue der clearer skærmen.

### Leverancer

- 7 design-dokumenter skrevet
- 3 ADR'er dokumenteret
- Rust toolchain installeret
- Cargo workspace bygget
- Hello window kører
- Git repo initialiseret + pushet til GitHub

---

## Fase 1 — Engine Core (8-12 uger)

**Mål:** En brugbar minimal engine at bygge spil på.

### Opgaver

- [x] ECS-integration (hecs)
- [x] Input-system (winit, keyboard/mouse mapping)
- [x] Renderer: sprite-batching, kamera, layers
- [x] Asset-loader (textures, data RON)
- [~] Tilemap-renderer (udskudt til Fase 2 — sprite-renderer kan tegne tiles)
- [x] Tids-loop (fixed timestep sim, interpoleret render)
- [x] Basis debugging-overlay (FPS, entity count)
- [x] Logging (tracing crate)
- [ ] Enhedstests for kerne-moduler

### Milestone M1

En spilmotor der kan loade og tegne et tilemap + en sprite. Kan flytte en sprite med piletasterne.

### Leverancer

- `engine` crate funktionel
- Renderer kan tegne sprites i layers
- Asset-loader virker
- Fixed timestep sim kører
- Debug overlay (F1: FPS, entity count)

---

## Fase 2 — World & Movement (6-8 uger)

**Mål:** Du kan gå rundt i én lille by-zone.

### Opgaver

- [x] Spiller-entitet (position, velocity, collision)
- [x] Bevægelse (WASD/gamepad)
- [x] Collision (AABB, solid tiles)
- [x] Kamera (follow + clamp til zone bounds)
- [x] Én test-zone (East Blocks lille version — 800x608 px)
- [x] Tilemap med gader, bygninger, fortorve
- [x] Basis NPC'er der går på stier
- [~] Dør/indgang interaktioner (udskudt til Fase 4)
- [x] Zone-definition RON format implementeret
- [~] Zone-overgange (prototype) (udskudt til Fase 5+ hvor flere zoner findes)

### Milestone M2

Du kan gå rundt i én lille by-zone med NPC'er der går på stier. Kamera følger dig og er clamped til zone.

---

## Fase 3 — Vehicles (4-6 uger)

**Mål:** Du kan stjæle og køre biler.

### Opgaver

- [x] Vehicle-entitet (data-drevet: VehicleDef)
- [x] Arcade bil-fysik (forward, turn, drift)
- [x] Stjæl bil (hotwire-tid, proximity check)
- [x] Ind/udstigning
- [x] Kollision bil↔verden (AABB push-back)
- [~] Kollision bil↔bil, bil↔fodgænger (udskudt til Fase 4)
- [x] 3-5 forskellige bil-typer (5: compact, muscle, van, sports, truck)
- [x] Skade-modellering (health field, reduceres ved collision)
- [~] Nummerplade-system (data) (udskudt til Fase 6+)
- [~] Bilfysik justeres for overflade (asfalt, græs, regn) (udskudt til Fase 6+)

### Milestone M3

Spilleren kan stjæle en bil, køre den rundt, og forlade den. Forskellige biler har forskellig føling.

---

## Fase 4 — NPC & Byens Liv (8-10 uger)

**Mål:** Byen føles levende — NPC'er har rutiner og reagerer.

### Opgaver

- [x] NPC FSM (Idle, Walk, Flee, Panic, Talk)
- [x] Roller: pedestrian, shopkeeper, gang_member (3 af mange — flere i Fase 5+)
- [~] Daglige rutiner (schedule-data per NPC-type) (basis patrol-ruter, fuld schedule i Fase 5+)
- [x] Mikro-dialog (dynamisk baseret på player state)
- [~] Trafik-AI (biler kører på veje) (udskudt til Fase 5+)
- [x] Reaktions-system (flygte fra fare, frygt-system)
- [x] Spatial partitioning (grid) til query performance
- [x] NPC memory (husker spillerens handlinger lokalt — fear + witnessed_crime)
- [~] Pathfinding (A* eller navmesh) (udskudt til Fase 5+)
- [x] Dag/nat-cyklus (verdens-tid)

### Milestone M4

Byen har rutinerende NPC'er, dynamisk dialog, og reaktioner på spillerens adfærd. Trafik kører.

---

## Fase 5 — Factions & Reputation (6-8 uger)

**Mål:** Factions reagerer på dig, zoner skifter kontrol.

### Opgaver

- [x] Faction-data model (trust per faction per spiller)
- [x] Faction-definitioner (7 factions: 2 street gangs, mafia, biker, cartel, police, civilians)
- [x] Zone-influence graf (per faction per zone)
- [x] Reputation-hændelser (11 events: JobCompleted, StoleVehicle, Betrayed etc.)
- [x] Street Rep / Faction Trust / Civilian Fear / Police Profile (4 lag)
- [x] Faction-AI (influence drift, konflikter, territory-shifting)
- [~] Graffiti visuel ændring baseret på zone-ejer (udskudt til Fase 11)
- [~] Faction-dialog ("Wrong colors for this block") (NPC attitude API klar, fuld integration i Fase 7)
- [x] Faction-reaktioner på spillerens bil/våben (SeenArmed, RecklessDriving, StoleVehicle events)
- [x] Zone-influence rebalance over tid (drift + konflikter)

### Milestone M5

Factions har holdninger til spilleren. Zoner skifter kontrol baseret på spillerens handlinger. Bander patruljerer deres territorium.

---

## Fase 6 — Wanted / Polici-system (6-8 uger)

**Mål:** Fuldt wanted-system med intelligent flugt.

### Opgaver

- [ ] Heat-levels 1-6 (data-drevet respons)
- [ ] Polti-AI: search, pursue, roadblocks, spike strips
- [ ] Helikopter-søgelys (simple visual cone)
- [ ] Bevis-system (vidner, nummerplade, våbentype, position)
- [ ] Police profile (foretrukne biler, zoner, mønstre)
- [ ] Flugt-mekanikker (skift bil, respray, skjul, skift tøj)
- [ ] Korrupt betjent-betaling (slet wanted)
- [ ] Jurisdiktion (flugt ud af zone nedsætter heat)
- [ ] Roadblocks, bro-blokeringer
- [ ] NPC-vidner peger spiller ud

### Milestone M6

Spilleren kan opbygge heat, flygte intelligent, reducere beviser. Politi lærer spillerens vaner.

---

## Fase 7 — Missioner & Dialog (8-10 uger)

**Mål:** Spilleren kan tage og fuldføre missioner med konsekvenser.

### Opgaver

- [ ] Mission-definition format (RON)
- [ ] Mission-typer: street, klassisk, dynamic, personal
- [ ] Dialog-træ-system (choices, attitudes, consequences)
- [ ] Mission-udløsere (kontakter, rygter, telefon)
- [ ] Main story skeleton (5-10 missioner)
- [ ] Faction arc-missioner (3-5 per faction)
- [ ] Dynamic missions (genereres fra by-state)
- [ ] Mission consequences (rep changes, faction trust, zone influence)
- [ ] Telefon-system (kontakter kan ringe)
- [ ] Mission UI (phone, objective display)

### Milestone M7

Spilleren kan tage missioner fra forskellige kilder, fuldføre dem, og opleve konsekvenser i verden.

---

## Fase 8 — Økonomi, Safehouses, Crew (6-8 uger)

**Mål:** Full progression-økonomi virker.

### Opgaver

- [ ] Cash / clean money (to separate balancer)
- [ ] Money laundering (front businesses)
- [ ] Safehouses (gem, stash, garage, crew-møde, tøj-skift)
- [ ] 3-5 forskellige safehouse-typer
- [ ] Safehouse-risk (kan kompromitteres ved overuse)
- [ ] Crew-rekruttering (3-5 starter NPC's)
- [ ] Crew-medlemmer har personlighed (loyalitet, frygt, moralgrænser)
- [ ] Crew i missioner (vælg før job)
- [ ] Front businesses (passiv indkomst, missioner, police interest)
- [ ] Våben-/tøjsystem (inventory, stat-effects, faction-style)

### Milestone M8

Spilleren kan tjene penge, vaske dem, købe safehouses, rekruttere crew, og bruge dem i missioner.

---

## Fase 9 — Heists & Set Pieces (6-8 uger)

**Mål:** Heists med planlægningsfase virker.

### Opgaver

- [ ] Heist-planlægnings UI (approach, crew, flugtrute, våben)
- [ ] 4 approaches: Loud, Quiet, Social, Dirty
- [ ] 2-3 fulde heists (f.eks. Armored Van, Container Heist, Bank)
- [ ] Multi-løsninger per heist
- [ ] Set-piece missioner (motorvejsjagt, hoteloverfald)
- [ ] Crew-choice impact på heist
- [ ] Consequences for heist (heat, beviser, faction trust)

### Milestone M9

Spilleren kan planlægge og udføre heists på flere måder med forskellige konsekvenser.

---

## Fase 10 — AI Director & Emergent Events (6-8 uger)

**Mål:** Byen skaber drama uden spillerens input.

### Opgaver

- [ ] AI Director: spændings-styring (heat budget, ro-detektion)
- [ ] Random events (gade-events, rival-angreb, NPC-kriser)
- [ ] Nyheds-system (radio + tekst-nyheder reagerer på spilleren)
- [ ] Rygtesystem (NPC-dialog drevet af world-state)
- [ ] Rival-system (personlige fjender der udvikler sig)
- [ ] Social manipulation (frame, betray, mediate)
- [ ] Politiefterforskning (evidence ledger, investigation status)
- [ ] Emergent quest triggers

### Milestone M10

Byen genererer drama og events dynamisk. Spilleren kan manipulere faction-relationer.

---

## Fase 11 — Polish, Audio, UI (6-8 uger)

**Mål:** Spillet føles færdigt oplevelsesmæssigt.

### Opgaver

- [ ] Audio-engine (kira integration)
- [ ] Lyd-design pipeline (SFX, ambient, music)
- [ ] Radio-system (musikstationer, talk radio, scanner, nyheder)
- [ ] UI: HUD, menuer, kort, telefon, inventory
- [ ] Performance-optimering (spatial partitioning, batching, culling)
- [ ] Fejlfinding, balancering
- [ ] Save-system fuld implementering
- [ ] Options menu (graphics, audio, controls)
- [ ] Accessibility (colorblind, text size)

### Milestone M11

Spillet har lyd, komplet UI, og kører stabilt.

---

## Fase 12 — Vertical Slice Release (4-6 uger)

**Mål:** Spilbar vertical slice distribuerbar.

### Opgaver

- [ ] 1-2 fuldt fungerende zoner
- [ ] 1 faction arc komplet
- [ ] Kerne-loop fungerende end-to-end
- [ ] 1 heist-type fungerende
- [ ] Ekstern playtest (3-5 spillere)
- [ ] Installer / distribuerbar build
- [ ] Bug-fixing fra playtest
- [ ] Release-notes og "how to play" guide

### Milestone M12

Vertical slice spilbar og distribuerbar. Spillere kan opleve kerne-loopet fra start til en natural afslutning.

---

## Sprint-planlægning

Hver fase opdeles i 2-ugers sprints. Sprint backlog defineres i sprint-start, review i sprint-slut.

### Sprint 1 (nuværende)

**Mål:** Fase 0 færdig.

- [x] Mappestruktur
- [x] pitch.md
- [x] gdd.md
- [x] tdd.md
- [x] roadmap.md
- [ ] matrix.md
- [ ] ADR-001, ADR-002, ADR-003
- [x] Rust toolchain installeret
- [ ] Git init + GitHub
- [ ] Cargo workspace
- [ ] Hello window
- [ ] changelog sprint-01

### Fremtidige sprints

Planlægges i sprint-start-møder (eller sprint-start-dokumenter). Backlog justeres baseret på review.

---

## Risk-register

| Risiko | Sandsynlighed | Impact | Mitigation |
|---|---|---|---|
| Scope creep (for mange features) | Høj | Høj | MVP-first, fase-opdeling, ADR for scope-udvidelser |
| Performance (mange NPC'er) | Medium | Høj | Spatial partitioning tidligt, profiling |
| wgpu-learningkurve | Medium | Medium | Start med simpel sprite-renderer, udvid gradvist |
| Data-drevet overhead (for meget RON parsing) | Lav | Lav | Cache parsed data, brug bincode til saves |
| AI agent inkonsistens | Medium | Medium | Klare code-style guides, review per PR |
| Motivationstab (langt projekt) | Medium | Høj | Fejrbare milestones, synlig progression |

---

## Dokumentations-status

| Dokument | Status |
|---|---|
| roadmap.md | Færdig (v1) |
| sprint-01 backlog | In progress |
| sprint-02 backlog | Planlægges ved sprint-start |