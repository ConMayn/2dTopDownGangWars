# Sprint 10 — Changelog

> **Fase:** 9 — Heists & Set Pieces
> **Sprint:** 10
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M9 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### Heists (Fase 9)

- ✅ `game/src/heists/mod.rs` — `Approach`, `EscapeRoute`, `Diversion`, `HeistDef`, `HeistPlan`, `Heist`, `HeistOutcome`, `HeistManager`
- ✅ 4 approaches: Loud, Quiet, Social, Dirty (GDD 24.2)
- ✅ 6 escape routes: Direct, BackAlleys, Safehouse, SwitchCar, Tunnel, Rooftop
- ✅ 6 diversions: None, FakeCall, FireAlarm, DecoyCar, GangSkirmish, Explosive
- ✅ Planlægning: approach, escape, diversion, crew, vehicle, disguise, risk_appetite
- ✅ Heist progression (0-100), heat/evidence akkumuleres over tid
- ✅ Completion: rewards (cash/clean), net heat (med escape/diversion reduction), evidence, faction trust delta, crew injuries
- ✅ Failure path: ingen reward, ekstra heat/evidence, crew-skader
- ✅ HeistManager: available, active, completed tracking

### 3 demo heists

| Heist | Zone | Approaches | Crew | Reward (cash) | Reward (clean) |
|---|---|---|---|---|---|
| Armored Van Heist | downtown | Loud, Quiet, Social, Dirty | 3 | $25,000 | $0 |
| Container Heist | industrial_zone | Quiet, Loud, Dirty | 2 | $15,000 | $0 |
| Bank Job | downtown | Loud, Quiet, Social | 4 | $100,000 | $5,000 |

### Integration i WorldPlugin

- ✅ HeistManager state i WorldPlugin
- ✅ 3 heist definitions loades ved init
- ✅ Heist tick per frame (progression, heat, evidence)
- ✅ Attack-action trigger starter "Armored Van" heist (Loud + BackAlleys + FakeCall + Vito)
- ✅ `apply_heist_outcome`: rewards til wallet, heat til wanted, crew-skader, faction trust
- ✅ Debug overlay udvidet med heists avail + active

---

## Milestone M9 — Status: ✅ Nået

**Definition:** Spilleren kan planlægge og udføre heists på flere måder med forskellige konsekvenser.

**Resultat:**
- 4 approaches med forskellig heat/evidence/krav ✅
- 6 escape routes med heat-reduktion ✅
- 6 diversions med cost/effekt ✅
- Planlægning med crew, vehicle, disguise, risk ✅
- 3 fulde heist-definitioner (Armored Van, Container, Bank) ✅
- Heist progression + completion med rewards og konsekvenser ✅
- Integration kører uden crash ✅

---

## Tekniske noter

### `?` operator i bool-returnerende funktion
- `HeistManager::start` returnerer `bool`; kan ikke bruge `?`. Omskrevet til `match`.
- `abort` kræver `let mut heist` for at kalde `.fail()`.

### Heist balance
- Loud: 40 base heat, 30 evidence weight, 25 progress/speed
- Quiet: 10 base heat, 8 evidence weight, 10 progress/speed
- Social: 5 base heat, 3 evidence weight, 15 progress/speed
- Dirty: 15 base heat, 12 evidence weight, 12 progress/speed
- Escape reducerer heat (Direct: 2, BackAlleys: 6, Safehouse: 12, SwitchCar: 15, Tunnel: 10, Rooftop: 8)
- Diversion reducerer heat yderligere (FakeCall: 8, FireAlarm: 5, DecoyCar: 12, GangSkirmish: 15, Explosive: 20)

### Crew impact
- Success: faction trust delta -10 (target vred)
- Failure: faction trust delta -20, crew_inured får -10 loyalty, +15 fear

### Tests
- `loud_generates_more_heat_than_quiet`
- `heist_completes_when_progress_maxed`

---

## Næste sprint (Sprint 11) — plan

**Mål:** Fase 10 — AI Director & Emergent Events.

- [ ] AI Director: spændings-styring (heat budget, ro-detektion)
- [ ] Random events (gade-events, rival-angreb, NPC-kriser)
- [ ] Nyheds-system (radio + tekst-nyheder reagerer på spilleren)
- [ ] Rygtesystem (NPC-dialog drevet af world-state)
- [ ] Rival-system (personlige fjender der udvikler sig)
- [ ] Social manipulation (frame, betray, mediate)
- [ ] Emergent quest triggers

**Milestone M10:** Byen genererer drama og events dynamisk. Spilleren kan manipulere faction-relationer.

---

## Stats

- **Nye moduler:** 1 (heists)
- **Nye filer:** 1 (heists/mod.rs)
- **Linjer kode:** ~430 Rust
- **Approaches:** 4
- **Escape routes:** 6
- **Diversions:** 6
- **Demo heists:** 3
- **Hovedagent:** GLM