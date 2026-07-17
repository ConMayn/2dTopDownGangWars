# Sprint 07 — Changelog

> **Fase:** 6 — Wanted & Police System
> **Sprint:** 7
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M6 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### Wanted / Politi-system (Fase 6)

- ✅ `game/src/police/heat.rs` — HeatLevel 0-6, WantedState, CrimeType
- ✅ `game/src/police/evidence.rs` — EvidenceKind, EvidenceLedger, identification score
- ✅ `game/src/police/police_ai.rs` — Police komponent, PoliceState FSM, search/pursue/patrol
- ✅ `game/src/police/mod.rs` — samlet modul-API
- ✅ Integration i WorldPlugin:
  - Politi texture genereres og loades
  - 2 patrulje-enheder spawns ved init (ambiance)
  - WantedState opdateres per frame (decay, sight)
  - Police AI kører per frame ( Patrol → Search → Pursue → ReturnToPatrol )
  - Evidence tilføjes når politi ser spilleren (position, biltype)
  - Dynamisk spawn/despawn baseret på heat-level

### Heat-levels 1-6

| Level | Label | Response Units | Decay/sec | Notes |
|---|---|---|---|---|
| 0 | Clean | 0 | 0 | Ingen efterforskning |
| 1 | Suspicion | 1 | 2.0 | Patruljer holder øje |
| 2 | Local Pursuit | 2 | 1.0 | Nærliggende patruljer søger |
| 3 | Active Search | 4 | 0.5 | Sirener, helikopter mulig |
| 4 | Task Force | 6 | 0.3 | Særlige enheder |
| 5 | Lockdown | 8 | 0.15 | Zone-checkpoints |
| 6 | Manhunt | 12 | 0.08 | Specialstyrker, dusørjægere |

### Evidence-system

- 8 bevis-typer: Witness, Camera, LicensePlate, WeaponType, VehicleType, Fingerprints, KnownAssociate, LastPosition
- Vægtet identification score (diminishing returns)
- Efterforskningsstatus: Unknown → PersonOfInterest → Identified → WarrantActive → Manhunt
- API klar til fremtidige flugt-mekanikker (skift bil, fjern beviser, korrupt betjent)

### Police AI FSM

- **Patrol**: følger waypoint-rute
- **Search**: bevæger sig mod `last_seen_pos` indenfor 15 sekunder
- **Pursue**: når spilleren er i synsfelt (afstand + vinkel)
- **ReturnToPatrol**: giver op efter 15+ sekunder
- **Roadblock**: placeholder til fremtidig implementering

### Kamera-fix

- Fixed crash når tilemap er mindre end viewport: `Camera::follow` tjekker nu zone-vs-viewport størrelse FØR clamp, og centrerer zonen hvis den er for lille.

### Render-fix

- Fixed crash pga. manglende bind group når første sprite havde invalid texture. `SpriteBatch::render` finder nu første gyldige texture og fallback til clear_only hvis ingen findes.

---

## Milestone M6 — Status: ✅ Nået

**Definition:** Spilleren kan opbygge heat, politiet reagerer, beviser akkumuleres.

**Resultat:**
- Heat-levels 1-6 med data-drevet respons ✅
- Politi-enheder spawns, patruljerer, søger og forfølger ✅
- WantedState opdateres baseret på sight og decay ✅
- EvidenceLedger optegner beviser når spilleren er i politi-synet ✅
- Integration i WorldPlugin kører uden crash ✅

---

## Tekniske noter

### Module-exports
- `police/heat.rs` bruger `[f32; 2]` til `last_seen_pos` fordi `heat_core::Vec2` ikke implementerer serde (Rust `Serialize`/`Deserialize`).
- `police/evidence.rs` refererer `InvestigationStatus` fra `crate::factions` (ikke `super::reputation` — modul-strukturen er flat).

### hecs query iterator
- `police_ai.rs` query iterationer skal være `&mut inner.query::<&Police>()` for at `QueryBorrow` implementerer `IntoIterator`.

### Camera clamp rækkefølge
- Forrige implementation clampede først, så tjekkede om zonen var mindre end viewport. Det gav `min > max` panic når viewport (1280x720, half=640x360) var større end tilemap (800x608). Nu tjekkes størrelse først.

### Bind group fallback
- Render-pipelinen kræver en BindGroup, men hvis første sprite i batchet brugte en null/invalid texture, blev gruppen aldrig sat. Nu findes første gyldige texture før draw-kaldet.

---

## Næste sprint (Sprint 8) — plan

**Mål:** Fase 7 — Missioner & økonomi.

- [ ] Mission-system (trigger, objectives, rewards)
- [ ] Faction jobs med reputation-krav
- [ ] Pengesystem + butikker (våben, biler, tøj)
- [ ] Simple våben (pistol, shotgun, melee)
- [ ] Loot-system
- [ ] Safehouses / garages
- [ ] Dialog-valg med konsekvenser

**Milestone M7:** Spilleren kan tage jobs, tjene penge, købe gear, påvirke factions gennem missioner.

---

## Stats

- **Nye moduler:** 3 (heat, evidence, police_ai) + police/mod
- **Nye filer:** 4
- **Linjer kode:** ~300 Rust (police) + ~100 (integration/fixes)
- **Heat levels:** 7 (0-6)
- **Evidence typer:** 8
- **Politi states:** 5
- **Hovedagent:** GLM
