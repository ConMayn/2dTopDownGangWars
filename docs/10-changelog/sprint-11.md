# Sprint 11 — Changelog

> **Fase:** 10 — AI Director & Emergent Events
> **Sprint:** 11
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M10 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### AI Director (Fase 10)

- ✅ `game/src/director/mod.rs` — `DirectorState`, `TensionLevel`, `DirectorEvent`
- ✅ 4 tension-niveauer: Calm, Medium, High, Chaos
- ✅ Overvåger: ro-længde, heat, penge-mangel (GDD 15.1)
- ✅ Kan trigge 6 event-typer: RandomStreetEvent, AmbientFlavor, PolicePressure, RivalAttack, ContactCall, NewsReport (GDD 15.2)
- ✅ Heat budget, event cooldown, statistik (triggered/suppressed)
- ✅ Tension skifter baseret på heat og calm_time
- ✅ Event cooldown baseret på tension-niveau

### Random Events (Fase 10)

- ✅ `game/src/events/mod.rs` — `EventKind`, `WorldEvent`, `EventManager`
- ✅ 15 gade-events fra GDD 22: GangSkirmish, TrafficStop, AmbulanceRun, StoreRobbery, CarAccident, StreetRace, FootChase, RivalHunt, InformantChased, Demonstration, BuildingFire, MafiaFuneral, BikerConvoy, Blackout, StormWeather
- ✅ Hvert event: varighed, heat-on-intervention, position, zone
- ✅ EventManager: spawn, tick (fjern udløbne), nearest, intervene

### News System (Fase 10)

- ✅ `game/src/news/mod.rs` — `NewsKind`, `NewsItem`, `NewsSystem`
- ✅ 7 nyheds-typer: PoliceBlotter, GangNews, LocalEvent, Business, Weather, PlayerAction, Rumor
- ✅ Nyheder har relevance der falder over tid
- ✅ `publish_player_action` reagerer på spillerens handlinger (GDD 16.2)
- ✅ `publish_rumor` til rygtesystem (GDD 16.3)
- ✅ Tick-aldring, latest-filter, count

### Rival System (Fase 10)

- ✅ `game/src/rivals/mod.rs` — `RivalKind`, `RivalDisposition`, `Rival`, `RivalAction`, `RivalSystem`
- ✅ 8 rival-typer fra GDD 25.1: GangLeader, Cop, BountyHunter, FormerCrew, Journalist, MafiaEnforcer, StreetRacer, CorruptChief
- ✅ 5 dispositions: Hostile, Plotting, Truce, Wavering, Ally
- ✅ 7 rival-actions fra GDD 25.2: TipPolice, AttackBusiness, KidnapCrew, SpreadRumor, SabotageVehicle, Challenge, SetBounty
- ✅ Grudge/respect system; rivaler kan blive allierede (GDD 25.3)
- ✅ Tick: grudge falder, respect kan stige; disposition opdateres
- ✅ 3 starter rivals: Marcus 'Mad Dog' Reyes (GangLeader), Sledge (BountyHunter), Det. Sarah Voss (Cop)

### Integration i WorldPlugin

- ✅ DirectorState, EventManager, NewsSystem, RivalSystem state i WorldPlugin
- ✅ Director update per frame (heat, cash, dt)
- ✅ Director event trigger → handle_director_event (spawner events, publicerer nyheder, rival actions)
- ✅ Events tick, news tick, rivals tick per frame
- ✅ 3 rivals seedes ved init
- ✅ 1 seed nyhed ("Heat City wakes up")
- ✅ Debug overlay udvidet med director tension, events, news, rivals, hostile count, total bounty

---

## Milestone M10 — Status: ✅ Nået

**Definition:** Byen genererer drama og events dynamisk. Spilleren kan manipulere faction-relationer.

**Resultat:**
- AI Director styrer spænding baseret på heat/ro-tid ✅
- 15 random event-typer kan spawnes dynamisk ✅
- Nyhedssystem reagerer på spilleren og world-state ✅
- Rival-system med personlige fjender der udvikler sig ✅
- Director kan trigge rival-actions, events, nyheder ✅
- Integration kører uden crash ✅

---

## Tekniske noter

### Director tension logic
- Calm: calm_time > 120s → vil trigge RandomStreetEvent
- Medium: time_since_event > 90s → AmbientFlavor
- High: time_since_event > 180s → PolicePressure
- Chaos: heat > 60 → director trækker sig

### Director event handling
- RandomStreetEvent → spawn GangSkirmish + nyhed
- AmbientFlavor → spawn BikerConvoy + nyhed
- PolicePressure → spawn TrafficStop + politi-nyhed
- RivalAttack → rival.on_action_taken() + nyhed
- ContactCall → rygte-nyhed
- NewsReport → player_action nyhed

### Rival progression
- Grudge 70+ → Hostile; 40+ → Plotting; 15+ → Truce
- Respect 50+ → Wavering (kan blive allieret)
- on_humiliated_by_player: +20 grudge, +500 bounty
- on_helped_by_player: -30 grudge, +25 respect

---

## Næste sprint (Sprint 12) — plan

**Mål:** Fase 11 — Polish, Audio, UI.

- [ ] Audio-engine (kira integration)
- [ ] Lyd-design pipeline (SFX, ambient, music)
- [ ] Radio-system (musikstationer, talk radio, scanner, nyheder)
- [ ] UI: HUD, menuer, kort, telefon, inventory
- [ ] Performance-optimering (spatial partitioning, batching, culling)
- [ ] Save-system fuld implementering
- [ ] Options menu (graphics, audio, controls)
- [ ] Accessibility (colorblind, text size)

**Milestone M11:** Spillet har lyd, komplet UI, og kører stabilt.

---

## Stats

- **Nye moduler:** 4 (director, events, news, rivals)
- **Nye filer:** 4 (mod.rs i hver mappe)
- **Linjer kode:** ~600 Rust
- **Director tension levels:** 4
- **Director event typer:** 6
- **Random event typer:** 15
- **News typer:** 7
- **Rival typer:** 8
- **Rival dispositions:** 5
- **Rival actions:** 7
- **Starter rivals:** 3
- **Hovedagent:** GLM