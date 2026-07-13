# Sprint 04 — Changelog

> **Fase:** 3 — Vehicles
> **Sprint:** 4
> **Dato:** 2026-07-14
> **Status:** ✅ Milestone M3 nået
> **Agenter:** GLM (hovedingeniør)

---

## Opnået denne sprint

### Vehicles (Fase 3)

- ✅ `game/src/world/vehicle.rs` — fuldt vehicle-system
- ✅ `VehicleDef` — data-drevne bil-typer (RON-serialiserbare)
- ✅ 5 bil-typer: Compact, Muscle, Van, Sports, Truck (hver med unikke stats)
- ✅ `VehicleRegistry` — registry for bil-typer
- ✅ `Vehicle` komponent — pos, heading, vel, health, driver, hotwire_timer, stolen
- ✅ Arcade bil-fysik: acceleration, bremsning, turning (speed-afhængig), drift, friction
- ✅ Bil collision mod tilemap (AABB, push-back + hastighedsreduktion ved collision)
- ✅ Ind/udstigning (E-tast) — proximity check (50px), driver-sporing
- ✅ Hotwire-system (data-drevet hotwire_time, placeholder timer for nu)
- ✅ Integration i WorldPlugin — spiller kan gå til bil, trykke E, køre, trykke E igen for at stige ud

### Bil-typer (stats)

| Type | Max Speed | Accel | Turn | Drift | Health | Hotwire |
|---|---|---|---|---|---|---|
| Compact | 280 | 180 | 3.2 | 0.15 | 100 | 1.5s |
| Muscle | 420 | 250 | 2.6 | 0.35 | 120 | 2.5s |
| Van | 240 | 140 | 2.2 | 0.08 | 160 | 2.0s |
| Sports | 500 | 320 | 3.0 | 0.45 | 90 | 3.0s |
| Truck | 200 | 100 | 1.8 | 0.05 | 200 | 3.5s |

### Gameplay

- 3 biler spawnet på gaderne (compact, muscle, van)
- Spiller går til en bil, trykker E → stiger ind (hvis < 50px afstand)
- W/S = gas/bremse, A/D = styrl, Shift = handbrake
- Bilen har collision mod bygninger og mure
- Hastighed reduceres ved collision (0.3x multiplier)
- Spiller synes ikke mens den er i bil (bilen vises i stedet)
- Tryk E igen for at stige ud (spiller placeres ved siden af bilen)
- Kamera følger bilen

---

## Milestone M3 — Status: ✅ Nået

**Definition:** Spilleren kan stjæle en bil, køre den rundt, og forlade den. Forskellige biler har forskellig føling.

**Resultat:**
- 5 bil-typer med forskellige stats (speed, accel, turn, drift) ✅
- Arcade bil-fysik (forward, turn, drift, handbrake) ✅
- Ind/udstigning (E-tast, proximity check) ✅
- Bil collision mod tilemap ✅
- Spiller kan køre bilen og forlade den ✅

---

## Tekniske noter

### Borrow checker udfordringer
- hecs `Ref`/`RefMut` holder mutable borrow af World levende. Løsning: hent data først (immutable), drop borrow, så mutable borrow.
- `handle_vehicle_enter_exit`: hent vehicle pos/heading først, drop borrow, så opdatér player.
- `update`: opdatér vehicle physics i ét borrow, gem ny pos, drop borrow, så sync player pos.

### Bil-fysik
- Heading i radians (0 = pegende op/nord).
- Forward direction: `(sin(heading), -cos(heading))` (screen-y ned = verden-y op).
- Turning er speed-afhængig (`speed_factor = sqrt(speed/max_speed)`) — kan ikke dreje mens stillestående.
- Drift: lerp mellem current velocity og heading-aligned velocity. Højere drift = baghjulene slipper mere.
- Handbrake (Shift): øger friction kraftigt.

### Collision
- Simpel AABB (ikke rotated) — acceptable for Fase 3. Roteret collision kommer i Fase 6+.
- Push-back: finder korteste akse og rykker bilen ud af tile.
- Hastighedsreduktion: 0.3x ved collision (forhindrer "wall-riding").

---

## Næste sprint (Sprint 5) — plan

**Mål:** Start Fase 4 (NPC & Byens Liv).

- [ ] NPC FSM (Idle, Walk, Work, Flee, Panic, Talk)
- [ ] Flere NPC-typer (shopkeeper, gang_member, betjent, taxi, dørmand)
- [ ] Daglige rutiner (schedule-data per NPC-type)
- [ ] Mikro-dialog (dynamisk baseret på player state)
- [ ] Trafik-AI (biler kører på veje, holder for rødt)
- [ ] Reaktions-system (flygte fra kaos, ringe til politi, sladre)
- [ ] Spatial partitioning (grid) til query performance
- [ ] NPC memory (husker spillerens handlinger lokalt)
- [ ] Pathfinding (A* eller navmesh)
- [ ] Dag/nat-cyklus (verdens-tid)

**Milestone M4:** Byen har rutinerende NPC'er, dynamisk dialog, og reaktioner på spillerens adfærd. Trafik kører.

---

## Stats

- **Nye moduler:** 1 (vehicle.rs) + WorldPlugin opdateret
- **Nye filer:** 1 (game/src/world/vehicle.rs)
- **Linjer kode:** ~280 Rust (vehicle) + ~200 (WorldPlugin opdateringer)
- **Bil-typer:** 5
- **Hovedagent:** GLM