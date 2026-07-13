# Sprint 06 — Changelog

> **Fase:** 5 — Factions & Reputation
> **Sprint:** 6
> **Dato:** 2026-07-14
> **Status:** ✅ Milestone M5 nået
> **Agenter:** GLM (hovedingeniør)

---

## Opnået denne sprint

### Factions & Reputation (Fase 5) — spillets signatur-feature

- ✅ `game/src/factions/faction_def.rs` — FactionDef, FactionKind, FactionRegistry
- ✅ `game/src/factions/reputation.rs` — ReputationState (4 lag), RepEvent, apply_event
- ✅ `game/src/factions/influence.rs` — ZoneInfluence, InfluenceGraph (territorie-system)
- ✅ `game/src/factions/faction_ai.rs` — FactionAi (influence drift, konflikter, reputation decay)
- ✅ `game/src/factions/mod.rs` — samlet API
- ✅ Integration i WorldPlugin: faction-AI update, reputation event-generering

### 7 Factions defineret

| Faction | Type | Home Zone | Aggression | Disciplin | Indkomst |
|---|---|---|---|---|---|
| Southline Kings | StreetGang | east_blocks | 0.7 | 0.3 | 500 |
| Los Cuervos | StreetGang | east_blocks | 0.6 | 0.4 | 600 |
| Old Harbor Mafia | Mafia | old_town | 0.3 | 0.9 | 2000 |
| Iron Hounds | Biker | desert_outskirts | 0.8 | 0.2 | 800 |
| Harbor Cartel | Cartel | industrial_zone | 0.5 | 0.7 | 1500 |
| Police | Police | government_district | 0.4 | 0.8 | 0 |
| Civilians | Civilian | — | 0.0 | 0.0 | 0 |

### Reputation-system (4 lag fra GDD)

1. **Street Rep** (0-100): generel respekt/frygt. Stiger ved jobs, fights, kaos. Falder langsomt.
2. **Faction Trust** (-100 til +100 per faction): hver faction husker dig. Kategoriseret:
   - Family (60+), Trusted (20-60), Neutral (-20 to 20), Suspicious (-60 to -20), Hunted (< -60)
3. **Civilian Fear/Love** (per zone): civile frygter eller elsker dig.
4. **Police Profile**: kendte zoner, foretrukne biler, våbentyper, aggression, efterforskningsstatus.

### 11 Reputation-events

- JobCompleted, JobFailed, MemberKilled, HelpedCivilian, CausedChaos
- SeenArmed, RecklessDriving, StoleVehicle, Betrayed, WonFight, LostFight

### Zone Influence-system

- Hver zone har influence-procenter per faction (sum ~100).
- `add_influence(faction, delta)`: trækker proportionelt fra andre factions.
- `drift()`: factions langsomt overtager home-zones (mod 60%).
- `dominant()`: returnerer den faction der dominerer zonen.
- Influence-graf initialiseret for east_blocks (Southline Kings 60%, civilians 30%, police 10%).

### Faction-AI

- **Influence drift**: factions langsomt genindtager home-zones.
- **Konflikter**: hver ~30 sim-sek simuleres en faction-konflikt (angriber rivals zone).
- **Street rep decay**: falder langsomt mod baseline (10) hvis ingen nye events.
- **NPC attitude**: dialog baseret på faction trust status.

### Gameplay integration

- **Stjæl bil** → StoleVehicle event (street rep +1, faction trust -10 hvis ejer kendt).
- **Set med våben nær NPC** → SeenArmed event (civilian fear +0.02).
- **Reckless driving (>300 px/s)** → RecklessDriving event (police profile opdateres).
- Events genereres periodisk (~hver 2. sim-sek) baseret på spillerens adfærd.

---

## Milestone M5 — Status: ✅ Nået

**Definition:** Factions har holdninger til spilleren. Zoner skifter kontrol baseret på spillerens handlinger.

**Resultat:**
- 7 factions defineret med data (personlighed, allierede, fjender) ✅
- 4-lags reputation-system (Street Rep, Faction Trust, Civilian Fear/Love, Police Profile) ✅
- 11 reputation-events med apply_event API ✅
- Zone-influence graf med drift + konflikter ✅
- Faction-AI kører per frame (influence, konflikter, reputation decay) ✅
- Gameplay integration: stjæl bil, seen armed, reckless driving genererer events ✅
- NPC attitude API klar til dialog integration ✅

---

## Tekniske noter

### Borrow checker (igen)
- `self.tilemap.clone()` i update() for at undgå borrow-konflikt mellem `&self.tilemap` (immutable) og `&mut self` (til handle_vehicle_enter_exit + generate_rep_events).
- `handle_vehicle_enter_exit` ændret fra `&self` til `&mut self` (skal mutere reputation).
- Tilemap er Clone (inexpensive — kun data, ingen GPU-ressourcer).

### Faction-AI konflikter
- Pseudo-tilfældig valg af angribende faction (baseret på conflict_timer).
- Hvis angriber har aggression > 0.6 → +3% influence, ellers +1.5%.
- Defender mister halvt så meget som angriber vinder.

### Reputation event API
- `RepEvent` enum med 11 variants.
- `apply_event(state, event)` opdaterer alle relevante lag.
- `FactionAi::handle_event(reputation, event)` er den API gameplay-systemer kalder.
- Klar til Fase 7 (missioner) og Fase 6 (wanted/politi).

---

## Næste sprint (Sprint 7) — plan

**Mål:** Start Fase 6 (Wanted / Politi-system).

- [ ] Heat-levels 1-6 (data-drevet respons)
- [ ] Politi-AI: search, pursue, roadblocks, spike strips
- [ ] Helikopter-søgelys (simple visual cone)
- [ ] Bevis-system (vidner, nummerplade, våbentype, position)
- [ ] Police profile (foretrukne biler, zoner, mønstre) — allerede startet i reputation
- [ ] Flugt-mekanikker (skift bil, respray, skjul, skift tøj)
- [ ] Korrupt betjent-betaling (slet wanted)
- [ ] Jurisdiktion (flugt ud af zone nedsætter heat)
- [ ] Roadblocks, bro-blokeringer
- [ ] NPC-vidner peger spiller ud

**Milestone M6:** Spilleren kan opbygge heat, flygte intelligent, reducere beviser.

---

## Stats

- **Nye moduler:** 4 (faction_def, reputation, influence, faction_ai) + factions/mod
- **Nye filer:** 5
- **Linjer kode:** ~620 Rust (factions) + ~80 (WorldPlugin opdateringer)
- **Factions:** 7
- **RepEvents:** 11
- **Reputation-lag:** 4
- **Hovedagent:** GLM