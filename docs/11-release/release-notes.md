# Heat City — Release Notes (Vertical Slice)

> **Version:** 0.1.0 (Vertical Slice)
> **Dato:** 2026-07-17
> **Fase:** 12 — Vertical Slice Release
> **Milestone:** M12 ✅

---

## Hvad er dette?

Heat City er et top-down 2D singleplayer crime sandbox. Denne vertical slice demonstrerer kerne-loopet: bevæg dig i byen, stjæl biler, tag missioner, opbyg heat, flygt fra politi, tjene penge, køb businesses, rekrutter crew, udfør heists, og oplev emergent drama fra AI Directoren.

Alle 12 faser (M0-M12) er implementeret som system-arkitektur. Spillet kører uden crash med zero compiler warnings.

---

## Kerne-loop

1. **Start** — Du spawner i East Blocks med $200 cash, en lockpick, en pistol, et crash pad safehouse, 2 crew-medlemmer (Vito Driver, Dana Ghost), og et laundromat.
2. **Bevæg dig** — WASD/pile-taster, Shift for sprint. Kamera følger dig, clamped til zonen.
3. **Stjæl en bil** — Gå hen til en bil, tryk E for at stige ind. Kør med WASD. Dette genererer heat (CarTheft = 5 heat points).
4. **Tag missioner** — Tryk E for at starte dialog med Lil' P. Vælg "Jeg gør det" for at starte "Wrong Car, Wrong Block" missionen. Stjæl en lowrider og returnér til zonen.
5. **Heat & politi** — Kør hurtigt (>300 px/s) = RecklessDriving (+5 heat). Politi patruljerer, og ved højere heat kommer flere enheder. Flugt ved at køre væk og lade heat decay'e.
6. **Tjen penge** — Fuldfør missioner for cash rewards. Brug businesses (laundromat) til at vaske penge (cash→clean).
7. **Køb gear** — Brug penge på våben, biler, safehouse-upgrades (fremtidig UI).
8. **Heists** — Tryk Attack (venstre klik) for at starte Armored Van heist (Loud approach). Heist progresser over tid, genererer heat/evidence, og belønner med $25,000 cash ved success.
9. **AI Director** — Byen skaber drama. Efter ~90 sekunder uden events, director triggger gade-events (gang skirmish, biker convoy), nyheder, eller rival-angreb.
10. **Crew & rivals** — 3 starter rivals (Marcus 'Mad Dog' Reyes, Sledge, Det. Sarah Voss) kan angribe dine businesses, kidnappe crew, eller tippe politiet.

---

## Kontrol

| Tast | Funktion |
|---|---|
| WASD / pile-taster | Bevægelse |
| Shift | Sprint (til fods: 320 px/s, i bil: acceleration) |
| E | Interact (stig i/ud af bil, start/avancer dialog) |
| Venstre klik / Attack | Heist trigger (proof: starter Armored Van) |
| F1 / ToggleDebug | Debug overlay (wallet, missions, crew, events, director, audio) |
| Escape | (Planlagt: pause menu) |

---

## Systemer i vertical slice

### Implementeret og fungerende
- ✅ Tilemap rendering (800x608 px, 5 tile-typer)
- ✅ Spiller-bevægelse med AABB collision
- ✅ 5 bil-typer med arcade fysik + tilemap collision
- ✅ NPC FSM (Idle/Walk/Flee/Panic/Talk) med 3 NPC typer
- ✅ 7 factions med 4-lags reputation + influence-graf
- ✅ Heat 1-6 med politi-AI (Patrol/Search/Pursue/ReturnToPatrol)
- ✅ 8 evidence-typer med investigation status
- ✅ 2 demo missioner med objectives/rewards
- ✅ Dialog-træ med valg/effekter
- ✅ Wallet (cash/clean) + inventory + 14 item-typer
- ✅ 5 safehouse-typer med stash/garage/crew
- ✅ 6 crew-roller med loyalitet/frygt/moral
- ✅ 12 business-typer med money laundering + passiv indkomst
- ✅ 3 heists (Armored Van, Container, Bank) med 4 approaches
- ✅ AI Director med 4 tension-niveauer + 6 event-typer
- ✅ 15 gade-event-typer
- ✅ Nyhedssystem med 7 typer + relevance-aldring
- ✅ 8 rival-typer med grudge/respect progression
- ✅ Save-system (bincode, 5 slots, versioning)
- ✅ HUD-state (11 elementer) + 9 menu-skærme
- ✅ Audio API (6 lyd-typer, 3 radio-stationer, volume-kanaler)

### Planlagt til fremtidig iteration
- ⬜ Rigtig UI rendering (panels, knapper, telefon)
- ⬜ kira audio integration (faktisk lydafspilning)
- ⬜ Flere zoner med overgange
- ⬜ Fuldt våbensystem (skydning, ammo, reload)
- ⬜ Combat (NPC død, skade, health)
- ⬜ Performance-optimering (sprite atlas batching, culling)
- ⬜ Save/load integration med gameplay (gem i safehouse)
- ⬜ Options menu (graphics, audio, controls)
- ⬜ Accessibility (colorblind, text size)
- ⬜ Tutorial / onboarding

---

## Tekniske specifikationer

- **Sprog:** Rust 1.96 stable
- **Engine:** custom (wgpu 24 + winit 0.30 + hecs 0.10)
- **Platform:** Windows (MSVC), Linux planlagt
- **Resolution:** 1280x720 (renderer)
- **Target FPS:** 60
- **Warnings:** 0 compiler warnings
- **Tests:** economy, safehouses, crew, businesses, heists, save, audio

---

## Byg

```bat
tools\cargo.bat build --release
tools\cargo.bat run
```

Headless test:
```bat
tools\cargo.bat run -- --headless --frame-limit 60 --max-frames 120
```

---

## Kendte begrænsninger

1. **Ingen rigtig UI rendering** — HUD er data-only; teksten vises via debug overlay (F1).
2. **Ingen faktisk lyd** — Audio API findes men afspiller ikke lyde endnu.
3. **Én zone** — East Blocks (800x608). Flere zoner planlagt.
4. **Ingen skydevåben** — Pistol er et inventory-item, ikke et fungerende våben endnu.
5. **Mission stubs** — GoToZone fuldfører øjeblikkeligt; StealVehicle kræver rigtig def_id match.
6. **Heist trigger** — Attack-knap starter Armored Van; ingen planlægnings-UI endnu.
7. **Director events** — Spawner events i data; ingen visuel repræsentation i verden endnu.

---

## Næste skridt

Efter vertical slice:
1. Rigtig UI rendering (text, panels, telefon)
2. kira audio integration
3. Combat-system (skydning, health, død)
4. Flere zoner + overgange
5. Performance-optimering
6. Ekstern playtest
7. Bug-fixing fra playtest
8. Balancering

---

## Credits

- **GLM (hovedingeniør):** Implementering af alle 12 faser
- **Opus (arkitektur-review):** Debug, borrow-fixes, crash-resolve
- **CSL Games Studio:** Projektledelse, design

---

*Heat City Vertical Slice v0.1.0 — bygget med Rust, wgpu, winit, hecs.*