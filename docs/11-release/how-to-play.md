# Heat City — How to Play

> **Version:** 0.1.0 (Vertical Slice)

---

## Kom godt i gang

### Byg og kør
```bat
tools\cargo.bat build --release
tools\cargo.bat run
```

### Headless test (uden vindue)
```bat
tools\cargo.bat run -- --headless --frame-limit 60 --max-frames 120
```

---

## Kontrol

| Tast | Funktion |
|---|---|
| **W / Op** | Bevæg op |
| **A / Venstre** | Bevæg venstre |
| **S / Ned** | Bevæg ned |
| **D / Højre** | Bevæg højre |
| **Shift** | Sprint (til fods: 320 px/s) |
| **E** | Interact: stig i/ud af bil, start/avancer dialog |
| **Venstre klik (Attack)** | Start heist (proof: Armored Van) |
| **F1 (ToggleDebug)** | Debug overlay: wallet, missions, crew, events, director |

---

## Kerne-loop

### 1. Bevæg dig
- Brug WASD til at gå. Kameraet følger dig.
- Tryk Shift for at løbe (320 px/s).

### 2. Stjæl en bil
- Gå hen til en parkeret bil.
- Tryk **E** for at stige ind.
- Kør med WASD. Shift = handbrake.
- **Advarsel:** Biltyveri genererer heat (+5 heat points).

### 3. Tag en mission
- Tryk **E** for at starte en dialog.
- Vælg "Jeg gør det" for at acceptere missionen.
- Missionen vises i debug overlay (F1).

### 4. Heat & politi
- Kør hurtigt (>300 px/s) = RecklessDriving (+5 heat).
- Politi patruljerer altid (2 enheder for ambiance).
- Ved højere heat kommer flere enheder (op til 12 ved Heat 6).
- Flugt: kør væk fra politi, lade heat decay'e (falder når ikke "in sight").

### 5. Tjen penge
- Fuldfør missioner for cash.
- Din laundromat vasker automatisk cash → clean money over tid.
- Passiv indkomst fra businesses tilføjes til din clean saldo.

### 6. Heist
- Tryk **venstre klik (Attack)** for at starte Armored Van heist.
- Heist progresser over tid (Loud = 25 progress/s).
- Ved completion: +$25,000 cash.
- Heat og evidence genereres; reduceres af escape route (BackAlleys) og diversion (FakeCall).

### 7. AI Director
- Byen skaber drama automatisk.
- Efter ~90 sekunder uden events, director triggger gade-events eller nyheder.
- Rivaler kan angribe dine businesses eller tippe politiet.

### 8. Debug overlay (F1)
Tryk **F1** for at se:
- Wallet (cash/clean)
- Aktive missioner
- Crew status
- Safehouses
- Businesses (risk avg)
- Heists (available/active)
- Director tension + calm time
- Events
- News
- Rivals (hostile count, bounty)
- Menu state
- Radio station
- Audio muted

---

## Tips

- **Hold lav profil** for at undgå heat. Kør ikke for hurtigt nær politi.
- **Brug din laundromat** til at vaske penge. Cash er dirty; clean er brugbar til officielle køb.
- **Din crew** (Vito, Dana) har loyalitet/frygt. Hvis de er med på heists og fejler, mister de loyalitet.
- **Rivaler** udvikler sig. Hvis du ydmyger dem, sætter de måske dusør på dig. Hvis du hjælper dem, kan de blive allierede.
- **AI Director** holder øje med dig. Hvis du har ro for længe, skaber den drama. Hvis du har for meget heat, trækker den sig.

---

## Kendte begrænsninger

- Ingen rigtig UI rendering endnu (HUD er data-only; brug F1 for debug).
- Ingen faktisk lydafspilning (audio API findes men er stub).
- Én zone (East Blocks). Flere zoner planlagt.
- Pistol er et item, ikke et fungerende våben endnu.
- Mission GoToZone fuldfører øjeblikkeligt (stub).

---

*Heat City v0.1.0 — have fun on the streets.*