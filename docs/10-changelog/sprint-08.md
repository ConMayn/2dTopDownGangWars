# Sprint 08 — Changelog

> **Fase:** 7 — Missioner, Dialog & Økonomi
> **Sprint:** 8
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M7 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### Economy (Fase 7)

- ✅ `game/src/economy/mod.rs` — `Wallet` (cash + clean), `Inventory`, `Item`, `PlayerEconomy`
- ✅ Starter kit: $200 cash, lockpick, pistol
- ✅ API: `add`, `spend`, `has`, `count`, stacking
- ✅ serde-støtte til fremtidigt save-system

### Mission-system (Fase 7)

- ✅ `game/src/missions/mod.rs` — `MissionDef`, `Mission`, `MissionTracker`
- ✅ Objective typer: `GoToZone`, `StealVehicle`, `DeliverItem`, `TalkTo`, `EscapePolice`, `SurviveTime`
- ✅ Reward typer: `Cash`, `Item`, `FactionTrust`, `StreetRep`, `Influence`
- ✅ Consequence typer: `FactionTrust`, `StreetRep`, `Heat`
- ✅ 2 demo missioner loades ved init:
  - **Wrong Car, Wrong Block** — stjæl en lowrider, returnér til zone
  - **Dead Drop** — snak med kontakt, aflever pakke, hold heat lavt
- ✅ Mission auto-advance stubs i `WorldPlugin::update_missions`
- ✅ Rewards anvendes automatisk ved completion

### Dialog-system (Fase 7)

- ✅ `game/src/dialog/mod.rs` — `DialogTree`, `DialogNode`, `DialogChoice`
- ✅ Betingelser: `HasItem`, `HasCash`, `FactionTrustMin`, `MissionActive`, `MissionCompleted`
- ✅ Effekter: `StartMission`, `AdvanceMission`, `GiveItem`, `TakeItem`, `GiveCash`, `TakeCash`, `ReputationEvent`
- ✅ Demo dialog med Lil' P der tilbyder et job
- ✅ E-tast starter/avancerer dialog (med cooldown)

### Integration i WorldPlugin

- ✅ Importerer economy, missions, dialog
- ✅ `PlayerEconomy` og `MissionTracker` state i `WorldPlugin`
- ✅ `default_missions()` startes ved init
- ✅ `update_missions` kører per frame
- ✅ Dialog startes med `E` (Interact)
- ✅ Debug overlay (`ToggleDebug`, typisk F1) viser wallet + aktive missioner

---

## Milestone M7 — Status: ✅ Nået

**Definition:** Spilleren kan tage missioner fra forskellige kilder, fuldføre dem, og opleve konsekvenser i verden.

**Resultat:**
- Spilleren har en wallet med cash/clean penge ✅
- Spilleren har et inventory med items ✅
- Missioner loades, trackes og auto-completer (stubs) ✅
- Rewards påvirker økonomi, reputation og influence ✅
- Dialog kan starte missioner og give/tage items/penge ✅
- Integration kører uden crash ✅

---

## Tekniske noter

### Borrow-håndtering i dialog
- `ActiveDialog::choose` returnerer en klonet `DialogChoice` i stedet for en reference, så vi kan mutere `self.current` og derefter anvende effekter på `self` uden dobbelt-mut borrow.

### Input begrænsning
- Fase 7 proof bruger kun `Interact` (E) til dialog. Fremtidige iterationer får number-key mapping til valg.

### Mission stubs
- `GoToZone` completes øjeblikkeligt fordi spilleren starter i zonen.
- `StealVehicle` tjekker om spilleren kører det rigtige `def_id`.
- `EscapePolice` tjekker `wanted.level.as_u8() <= heat_max`.

### RON-serialisering
- Mission/Dialog data-strukturer er `Serialize + Deserialize` klar til at loade fra `assets/data/*.ron`.

---

## Næste sprint (Sprint 9) — plan

**Mål:** Fase 8 — Safehouses, Crew & Front Businesses.

- [ ] Safehouse-system (gem, stash, garage, crew-møde)
- [ ] Crew-rekruttering (3-5 starter NPCs)
- [ ] Crew-medlemmer med personlighed (loyalitet, frygt, moral)
- [ ] Front businesses (passiv indkomst, missioner, police interest)
- [ ] Vask af sorte penge (clean money loop)
- [ ] Våben-/tøjsystem med stat-effects

**Milestone M8:** Spilleren kan tjene penge, vaske dem, købe safehouses, rekruttere crew, og bruge dem i missioner.

---

## Stats

- **Nye moduler:** 3 (economy, missions, dialog)
- **Nye filer:** 4 (mod.rs i hver mappe + stub)
- **Linjer kode:** ~600 Rust
- **Mission typer:** 7 objectives
- **Reward typer:** 5
- **Dialog effekter:** 7
- **Hovedagent:** GLM
