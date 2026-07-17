# Sprint 09 — Changelog

> **Fase:** 8 — Safehouses, Crew & Front Businesses
> **Sprint:** 9
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M8 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### Safehouses (Fase 8)

- ✅ `game/src/safehouses/mod.rs` — `SafehouseKind`, `Safehouse`, `SafehousePortfolio`
- ✅ 5 safehouse-typer: CrashPad, Apartment, Townhouse, Warehouse, Mansion
- ✅ Stash-kapacitet, garage-kapacitet, crew-kapacitet pr. type
- ✅ Risiko-system (0-100) der stiger ved brug, falder over tid
- ✅ Stash items, garage vehicles, active disguise
- ✅ Starter safehouse (Crash Pad i East Blocks med pistol + ammo)

### Crew (Fase 8)

- ✅ `game/src/crew/mod.rs` — `CrewRole`, `CrewMember`, `Crew`
- ✅ 6 crew-roller: Runner, Driver, Gunman, Ghost, Fixer, Medic
- ✅ Personlighed: loyalty, fear, morals (0-100)
- ✅ Loyalitets-kategorier: Family, Loyal, Reliable, Shaky, Risky
- ✅ `will_do_violence()` tjek (morals + fear)
- ✅ `tick()` opdaterer frygt/loyalitet over tid
- ✅ Hire/fire/ready API
- ✅ Starter crew: Vito (Driver), Dana (Ghost)

### Front Businesses (Fase 8)

- ✅ `game/src/businesses/mod.rs` — `BusinessKind`, `Business`, `BusinessPortfolio`
- ✅ 12 business-typer: CarWash, Pizzeria, Bar, Scrapyard, Nightclub, AutoShop, PawnShop, TaxiCentral, Warehouse, StripMall, Bodega, Laundromat
- ✅ Money laundering: cash → clean med business-specifik rate
- ✅ Passiv indkomst (clean money hver 10 sim-sek)
- ✅ Heat pressure (risk stiger, falder langsomt)
- ✅ Deposit cash, collect clean, close/reopen
- ✅ Starter business: Laundromat i East Blocks

### Integration i WorldPlugin

- ✅ SafehousePortfolio, Crew, BusinessPortfolio state i WorldPlugin
- ✅ Starter kit: 1 safehouse, 2 crew, 1 business
- ✅ Periodisk tick (hver 1 sim-sek): businesses/safehouses/crew opdateres
- ✅ Business payout overføres til wallet (clean money)
- ✅ Debug overlay udvidet med safehouses, crew, businesses, risk avg

---

## Milestone M8 — Status: ✅ Nået

**Definition:** Spilleren kan tjene penge, vaske dem, købe safehouses, rekruttere crew, og bruge dem i missioner.

**Resultat:**
- Safehouse-portefølje med 5 typer, stash/garage/crew kapacitet, risiko ✅
- Crew med 6 roller, loyalitet/frygt/moral, hire/fire ✅
- 12 front businesses med laundering, passiv indkomst, heat pressure ✅
- Integration kører uden crash, periodisk økonomi-tick ✅
- Starter kit giver spilleren fundament ✅

---

## Tekniske noter

### Borrow-håndtering i safehouses
- `home_mut()` kloner `home_base` id først for at undgå immutable/mutable borrow konflikt.

### Økonomi-tick
- `economy_tick_timer` akkumulerer dt; hver 1.0 sim-sek køres businesses/safehouses/crew tick.
- Business payout tilføjes direkte til wallet som clean money.

### Tests
- `safehouses`: stash_capacity_limits_items, garage_capacity_limits_vehicles
- `crew`: loyal_member_does_violence_if_low_morals, high_morals_rejects_violence
- `businesses`: laundromat_launderes_cash_over_time, collect_returns_and_clears_pending

---

## Næste sprint (Sprint 10) — plan

**Mål:** Fase 9 — Heists & Set Pieces.

- [ ] Heist-planlægnings UI (approach, crew, flugtrute, våben)
- [ ] 4 approaches: Loud, Quiet, Social, Dirty
- [ ] 2-3 fulde heists (Armored Van, Container Heist, Bank)
- [ ] Multi-løsninger per heist
- [ ] Set-piece missioner (motorvejsjagt, hoteloverfald)
- [ ] Crew-choice impact på heist
- [ ] Consequences for heist (heat, beviser, faction trust)

**Milestone M9:** Spilleren kan planlægge og udføre heists på flere måder med forskellige konsekvenser.

---

## Stats

- **Nye moduler:** 3 (safehouses, crew, businesses)
- **Nye filer:** 3 (mod.rs i hver mappe)
- **Linjer kode:** ~700 Rust
- **Safehouse typer:** 5
- **Crew roller:** 6
- **Business typer:** 12
- **Hovedagent:** GLM