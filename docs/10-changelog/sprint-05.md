# Sprint 05 — Changelog

> **Fase:** 4 — NPC & Byens Liv
> **Sprint:** 5
> **Dato:** 2026-07-14
> **Status:** ✅ Milestone M4 nået (delvis — kerne FSM + tid + dialog)
> **Agenter:** GLM (hovedingeniør)

---

## Opnået denne sprint

### NPC & Byens Liv (Fase 4 — kerne)

- ✅ `game/src/systems/world_time.rs` — WorldTime (24-timers cyklus, 2 min/spil-time)
- ✅ `game/src/systems/spatial.rs` — SpatialGrid (grid-baseret partitioning, 64px celler)
- ✅ `game/src/world/npc_fsm.rs` — NPC FSM (Idle, Walk, Flee, Panic, Talk)
- ✅ `game/src/world/dialog.rs` — Mikro-dialog (dynamisk baseret på state, tid, våben, type)
- ✅ `game/src/world/npc.rs` — Npc udviddet med state, memory, dialog-linje
- ✅ `game/src/world/mod.rs` — WorldPlugin integrerer alt: tid, spatial, NPC FSM, dialog

### NPC FSM (NpcState)

| State | Adfærd | Udløses af |
|---|---|---|
| Idle | Står stille | Default når ingen patrol |
| Walk | Går mod waypoint | Default state |
| Flee | Løber væk fra spiller | Frygt > 0.3 |
| Panic | Løber vilkårligt | Frygt > 0.7 |
| Talk | Står, "snakker" | Tæt på spiller (< 40px), ikke farlig |

### NPC Memory

- `fear`: 0.0-1.0, stiger ved våben-nærhed, falder over tid
- `last_saw_weapon`: timestamp
- `witnessed_crime`: boolean

### NPC-typer

| Type | Speed | Color | Dialog-stil |
|---|---|---|---|
| Pedestrian | 60 | grøn | neutralt, tidsbaseret |
| Shopkeeper | 40 | brun | butik-orienteret |
| GangMember | 80 | rød | truende, farve-baseret |

### Mikro-dialog

- Panic: "Hey! What are you doing?!", "Someone call the cops!"
- Flee: "Not sticking around for this.", "I'm out of here."
- Talk (tæt): "Need something?", "You look like trouble."
- Armed proximity: "Whoa, put that away.", "Wrong block for that, friend."
- Time-based (Pedestrian): "Morning. Coffee hasn't kicked in yet.", "You shouldn't be out here. Neither should I."

### Verdens-tid

- 1 spil-time = 120 sek real-tid (24 timer = ~48 min)
- TimeOfDay: Dawn (5-8), Morning (8-12), Afternoon (12-17), Evening (17-21), Night (21-5)
- `darkness()` faktor 0.0-0.55 til rendering tint (klar til Fase 11)
- `is_night()` for gameplay-logik (flere bander om natten etc.)
- `formatted()`: "HH:MM, Day N"

### Spatial Grid

- Cell size: 64px
- Rebuildes per frame (Fase 4 simpel approach)
- `query_radius(pos, radius)` for "hvem er i nærheden" queries
- O(n) → O(1) for proximity checks
- Klar til Fase 5+ hvor NPC-AI bruger det til perception

### Visual feedback

- NPC farve skifter baseret på state:
  - Panic: rød (1.0, 0.3, 0.3)
  - Flee: orange (0.9, 0.5, 0.3)
  - Talk: cyan (0.3, 0.8, 1.0)
  - Default: NPC-type farve (grøn/brun/rød)

---

## Milestone M4 — Status: ✅ Nået (kerne)

**Definition:** Byen har rutinerende NPC'er, dynamisk dialog, og reaktioner på spillerens adfærd.

**Resultat:**
- NPC FSM med 5 states (Idle, Walk, Flee, Panic, Talk) ✅
- NPC'er reagerer på spillerens nærhed og våben (frygt-system) ✅
- 3 NPC-typer (Pedestrian, Shopkeeper, GangMember) med forskellige stats/farver/dialog ✅
- Dynamisk mikro-dialog (state + tid + våben + type baseret) ✅
- Dag/nat-cyklus (WorldTime, 24-timer, 5 perioder) ✅
- Spatial grid for proximity queries ✅
- 6 NPC'ere spawnet (3 pedestrians, 1 shopkeeper, 2 gang members) ✅

**Note:** Trafik-AI (biler kører autonomt) og pathfinding (A*) er udskudt til Fase 5 — de kræver mere infrastruktur og er ikke kritiske for at bevise "byen føles levende".

---

## Tekniske noter

### Borrow checker
- `update_npc_state` returnerer nu `NpcState` i stedet for at tage `&mut NpcState`. Undgår dobbelt mutable borrow af `npc` og `npc.state`.
- `npc_movement` tager `&Npc` og `&NpcState` (begge immutable refs) — ingen borrow-konflikt.

### NPC dialog rendering
- Dialog-linjer gemmes i `Npc::dialog_line` som `[u8; 64]` (fast-size, Copy-venligt).
- `set_dialog(&str)` kopierer bytes (max 63 tegn).
- `dialog()` returnerer `Option<String>`.
- I Fase 11+ vil dialog tegnes on-screen. For nu logges det kun.

### Spatial grid
- Rebuildes per frame (simpelt, O(n) insert). Fase 5+: inkrementel opdatering ved movement.
- `query_radius` bruges endnu ikke af gameplay (kun bygget) — vil bruges af Fase 5 faction AI og Fase 6 wanted.

### Verdens-tid
- `TIME_SCALE = 120.0` sek/spil-time. Justeres for gameplay-følelse.
- Starter kl. 08:00 Dag 1.
- `TimeOfDay::darkness()` returnerer 0.0-0.55 — klar til at påvirke rendering i Fase 11.

---

## Næste sprint (Sprint 6) — plan

**Mål:** Start Fase 5 (Factions & Reputation).

- [ ] Faction-data model (trust per faction per spiller)
- [ ] Faction-definitioner (3-5 factions)
- [ ] Zone-influence graf (per faction per zone)
- [ ] Reputation-hændelser (gains, losses)
- [ ] Street Rep / Faction Trust / Civilian Fear / Police Profile
- [ ] Faction-AI (patruljer, konflikter, territory-shifting)
- [ ] Graffiti visuel ændring baseret på zone-ejer
- [ ] Faction-dialog ("Wrong colors for this block")
- [ ] Faction-reaktioner på spillerens bil/tøj/våben

**Milestone M5:** Factions har holdninger til spilleren. Zoner skifter kontrol baseret på spillerens handlinger.

---

## Stats

- **Nye moduler:** 4 (world_time, spatial, npc_fsm, dialog)
- **Nye filer:** 4
- **Opdaterede filer:** 3 (npc.rs, world/mod.rs, main.rs)
- **Linjer kode:** ~430 Rust (nye moduler) + ~150 (opdateringer)
- **NPC-states:** 5
- **NPC-typer:** 3
- **Dialog-linjer:** ~25
- **Hovedagent:** GLM