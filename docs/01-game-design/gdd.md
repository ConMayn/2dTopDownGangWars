# Heat City — Game Design Document (GDD)

> **Status:** Levende dokument. Opdateres løbende.
> **Sidst opdateret:** Sprint 1, uge 1
> **Ejer:** GLM (forfatter), Opus (review), Codex (vedligehold)
> **Kilde:** Original idé fra projektstarter (36 sektioner)

---

## Indholdsfortegnelse

1. [Spillets identitet](#1-spillets-identitet)
2. [Byen](#2-byen)
3. [Bydynamik & territorier](#3-bydynamik--territorier)
4. [Factions](#4-factions)
5. [Omdømme-system](#5-omdømmesystem)
6. [Wanted-system / politi](#6-wantedsystem--politi)
7. [NPC-systemer](#7-npc-systemer)
8. [Dialogsystem](#8-dialogsystem)
9. [Missionstyper](#9-missionstyper)
10. [Kriminalitetsøkonomi](#10-kriminalitetsøkonomi)
11. [Våben og udstyr](#11-våben-og-udstyr)
12. [Køretøjer](#12-køretøjer)
13. [Territoriekrig](#13-territoriekrig)
14. [Crew-system](#14-crewsystem)
15. [AI Director](#15-ai-director)
16. [Nyheder, radio og rygter](#16-nyheder-radio-og-rygter)
17. [Tøj og identitet](#17-tøj-og-identitet)
18. [Safehouses](#18-safehouses)
19. [Mission-struktur](#19-missionstruktur)
20. [Moralske valg](#20-moralske-valg)
21. [Sjove småsystemer](#21-sjove-småsystemer)
22. [Random events](#22-random-events)
23. [Lawful jobs](#23-lawful-jobs)
24. [Heists](#24-heists)
25. [Rival-system](#25-rivalsystem)
26. [Politiefterforskning](#26-politiefterforskning)
27. [Social manipulation](#27-social-manipulation)
28. [Telefon / kontakter](#28-telefon--kontakter)
29. [Humor og tone](#29-humor-og-tone)
30. [Progression](#30-progression)
31. [Gameplay loops](#31-gameplay-loops)
32. [Mission-eksempler](#32-missioneksempler)
33. [Endgame](#33-endgame)
34. [Det særlige ved spillet](#34-det-særlige-ved-spillet)
35. [Drømme-feature-liste](#35-drømme-feature-liste)
36. [MVP-scope](#36-mvp-scope)

---

## 1. Spillets identitet

**Genre:** Top-down 2D singleplayer crime sandbox.
**Kamera:** Top-down, fast zoom, clamp til zone-grænser.
**Tone:** Gritty mørk satire. "Hvad koster det?" ikke "god/ond".
**Platform:** PC (Windows først, Linux senere). Desktop-fokus.
**Multiplayer:** Nej. Singleplayer med dyb simulation.

**Kernefantasi:** Du starter som nobody. Byen husker dig. Bander ændrer adfærd. Politiet lærer dine vaner. NPC'er har små liv. Alt kan eskalere. Du bliver en del af byens kriminelle økosystem.

**Spillets pitch (én sætning):**
> Et moderne top-down crime sandbox, hvor du ikke bare laver missioner i en by — du bliver en del af byens kriminelle økosystem. Hver bande, hvert kvarter, politiet og civile husker dine valg.

---

## 2. Byen

Byen er en karakter i sig selv. San Andreas-agtig struktur, men top-down.

### 2.1 Zoner

| Zone | Beskrivelse | Risiko | Loot |
|---|---|---|---|
| **Downtown** | Banker, kontorer, dyre biler, politi tæt på, overvågning | Højt | Højt |
| **East Blocks** | Tæt bolig, gadebander, graffiti, små butikker, baggårde, trap houses, basketballbaner | Medium | Medium |
| **Industrial Zone** | Lagre, havn, skrotpladser, illegale races, smugler-missioner, våbenhandel, sorte markeder | Medium | Højt |
| **Suburbs** | Rolige villakvarter, lav kriminalitet, høj politirespons. Perfekt til indbrud, flugt, social camouflage | Lavt | Medium |
| **Old Town** | Smalle gader, mafia-restauranter, gamle kirker, baglokaler, klubber, gambling, "respect"-baseret | Medium | Højt |
| **Desert Outskirts** | Lange veje, trailere, meth-miljøer, våbenmilitser, biker-gangs, skjulte depoter | Lavt-Medium | Medium |
| **Beach / Tourist District** | Natklubber, hurtige penge, scams, stoffer i natteliv, korrupte dørmænd, festmissioner | Medium | Højt |
| **Government District** | Domhus, politihovedkvarter, rådhus, overvågning, protester, korruption | Meget højt | Meget højt |

### 2.2 Zone-data (hver zone har)

- ejer/fraktion
- politiintensitet
- økonomisk niveau
- civilbefolkningens frygt
- bandetilstedeværelse
- våbenniveau
- stofmarked
- biltyper
- lokale rygter
- ønsket/ikke ønsket adfærd
- butikker der kan åbne/lukke
- NPC'er der husker ting

### 2.3 Zone-eksempel

> Hvis du laver meget ballade i East Blocks, bliver politiet mere synligt der. Men hvis politiet presser området hårdt, bliver banderne mere desperate. Så de kan begynde at røve butikker, slås med rivaler eller forsøge at hyre dig til gengældelse.

**Design-princip:** Zoner er ikke statiske mission-markers. De føles som en by, der bliver varmere.

---

## 3. Bydynamik & territorier

Se afsnit 13 (Territoriekrig) for den fulde influence-graf-model. Her: overordnet princip.

- Byen er opdelt i territorier.
- Hvert område har influence-procenter per faction.
- Spillerens handlinger ændrer zonen.
- Zonen skaber nye muligheder og problemer.
- Reaktioner er systemiske, ikke scriptede.

---

## 4. Factions

Ikke "rød bande vs blå bande". Hver gruppe har personlighed, økonomi og spillestil.

### 4.1 Street gangs

Holder til i bestemte kvarterer. Går op i territorium, respekt, hævn, graffiti, lokale butikker, biler, småkrig.

**Mekanikker:**
- du kan få/miste respekt
- tilstande: "på god fod", "neutral", "mistænkelig", "jagtet", "familie"
- de kan beskytte dig i deres zone
- de kan angribe dig, hvis du kører i rivalers biler
- de reagerer på dit tøj, bil, våben, tidligere valg

### 4.2 Mafia

Mere rolig, farligere, rigere.

**Missioner:** afpresning, beskyttelsespenge, snigmord, casinogæld, korruption, transport af folk, "forsvind denne bil", diplomati mellem grupper.

**Design-regel:** Mindre kaotisk end gadebanderne, men mere brutal når du fejler.

### 4.3 Biker gang

Holder til i udkanten, barer, værksteder, motorveje.

**Features:** motorcykler, våbenhandel, eskorte-missioner, konvojer, rivalisering med smuglere, klubhus som safehouse.

### 4.4 Cartel / smuglernetværk

Mere internationalt. Havn, lufthavn, ørken, lagerbygninger.

**Missioner:** hente pakker, undgå checkpoints, aflede politi, flytte varer, forhandle med lokale bander.

### 4.5 Korrupte cops

Ikke officiel faction i starten, men noget spilleren kan opdage.

**Kan:** sælge information, slette wanted-level midlertidigt, advare om raids, hyre dig til beskidte opgaver, presse dig hvis du ved for meget.

### 4.6 Lokale civile netværk

Ikke kriminelle, men vigtige: butiksejere, taxachauffører, mekanikere, dørmænd, hjemløse informanter, journalister, advokater, præster, bartendere, ambulancefolk, eks-bandemedlemmer.

**Giver:** rygter, skjulesteder, missioner, rabatter, konsekvenser.

---

## 5. Omdømme-system

Spillets stærkeste feature. Ikke ét tal — flere lag.

### 5.1 Street Rep

Hvor farlig/respekteret du virker på gaden.

**Stiger ved:** vinde fights, gennemføre jobs, holde aftaler, hjælpe lokale, tage hævn, køre vildt, overleve politi-jagter.

**Falder ved:** fejle offentligt, stikke af, dræbe allierede, miste varer, arbejde for rivaler.

### 5.2 Faction Trust

Hver bande har sin egen holdning til dig. Eksempel:
- Los Cuervos: 72% trust
- Southline Kings: -30% trust
- Old Harbor Mafia: neutral
- Police Vice Unit: investigating

### 5.3 Civilian Fear / Love

Civile i et område kan: frygte dig, sladre om dig, hjælpe dig, ignorere dig, ringe til politiet, skjule dig, give dig rygter.

**Eksempel:** Hvis du ofte hjælper et kvarter, advarer folk dig: "Hey, don't go down 5th Street. Cops been circling." Hvis du terroriserer, løber de, lukker butikker, eller peger dig ud.

### 5.4 Police Profile

Politiet bygger en profil på dig:
- foretrukne biler
- våbentype
- kendte zoner
- kendte allierede
- aggressionsniveau
- flugtmønstre

Jo mere kendt du bliver, jo mere målrettet bliver jagten.

---

## 6. Wanted-system / politi

GTA-agtig wanted-level, men mere detaljeret.

### 6.1 Heat-levels

| Heat | Navn | Adfærd |
|---|---|---|
| 1 | Mistanke | Patruljer holder øje. Kan stoppe dig, hvis du kører råddent. |
| 2 | Lokal jagt | Nærliggende patruljer leder. Sirener. Roadblocks kan begynde. |
| 3 | Aktiv eftersøgning | Flere biler, helikopter, politi kender bilens farve/type. |
| 4 | Task force | Særlige enheder, spike strips, koordinering, hårdere AI. |
| 5 | Lockdown | Hele zonen lukkes delvist. Checkpoints. Civil trafik ændrer sig. |
| 6 | Manhunt | Byen føles anderledes. Nyhedshelikoptere, specialstyrker, informanter, dusørjægere. |

### 6.2 Politiets evner

- søge sidst kendte position
- huske nummerplade
- genkende biltype
- reagere på vidner
- sætte vejspærringer
- blokere broer
- bruge helikopter-søgelys
- tjekke safehouses, hvis de bliver kompromitteret
- presse dine kontakter
- raide bandernes områder
- fejlagtigt anholde NPC'er
- skabe konflikt med bander

### 6.3 Escape-system

Du slipper ikke bare ved at køre væk. Du kan:
- skifte bil
- male bil om
- gemme dig i garage
- slukke lys om natten
- køre ind i parkeringshus
- flygte gennem gyder
- skifte tøj
- betale en korrupt betjent
- få bande til at skabe afledning
- gemme dig i en folkemængde
- smide våben
- brænde bilen af
- køre ud af jurisdiktion

**Design-regel:** Flugtspillet skal være sjovere end "bare køre væk".

---

## 7. NPC-systemer

**Vigtigt:** NPC'er må ikke bare være pynt.

### 7.1 Roller

arbejder, butiksejer, skoleelev, hjemløs, turist, dealer, betjent, taxachauffør, mekaniker, dørmand, bandemedlem, journalist, ambulancefører, præst, skraldemand, tow-truck driver, pizzabud.

### 7.2 Små rutiner

Ikke Sims-niveau, men nok til at byen føles levende:
- morgen-trafik
- frokostrush
- natklub-køer
- politi-skift
- bander hænger ud om aftenen
- butikker lukker
- havnen er aktiv om natten
- villakvarterer bliver stille efter kl. 22
- industrizonen bliver farlig efter midnat

### 7.3 NPC-dialog

Små dynamiske linjer baseret på: din bil, dit tøj, dit våben, dit ry, din faction, vejret, tidspunkt, seneste nyheder, missioner du har lavet, om politiet leder efter dig.

**Eksempler:**
- "Bro, that's Spider's car. You trying to get killed?"
- "Don't park that thing here. Cops been towing everything today."
- "People say you burned down Rico's garage. That true?"
- "Wrong colors for this block, friend."
- "Man, I saw you on the news."
- "Keep walking. I don't know you."

### 7.4 Emergent quests

NPC'er kan give emergent quests uden marker:
- Butiksejer: "Those guys been coming every Friday. I need someone to scare them off."
- Random NPC: "My brother got picked up last night. Cops took his car. You know anyone who can get it back?"

---

## 8. Dialogsystem

Korte, skarpe samtaler. Ikke lange cutscenes.

### 8.1 Attitude-valg

Spilleren kan svare med: rolig, truende, sjov, professionel, løgn, loyal, grådig, respektfuld, provokerende.

### 8.2 Eksempel

En bandeleder spørger, om du arbejder for rivalerne. Svarmuligheder:
- "No. I work for money."
- "You asking because you're scared?"
- "I did one job. Didn't know it was your problem."
- "Tell me what it costs to make this go away."

**Design-regel:** Konsekvenserne skal ikke altid være tydelige. Nogle gange husker folk bare tonen.

---

## 9. Missionstyper

For at undgå "kør til X, dræb Y".

### 9.1 Klassiske crime missions

drive-by, biltyveri, flugtchauffør, afhentning, levering, våbentransport, afpresning, sabotage, beskyt en person, indbrud, røveri, kidnapning, ødelæg rivalens biler, eskortér konvoj, plant falske beviser, skaf information, bryd ind i politigarage, få en person ud af byen.

### 9.2 Street-level missions

Små lokale jobs: hent en scooter fra en gård, find en stjålet bil, skræm en lokal idiot væk, beskyt en butik, saml penge ind, hjælp en mekaniker med reservedele, deltag i ulovligt street race, find en person i en bestemt zone, smid graffiti over rivalens tags, følg efter en mistænkelig bil, lok politiet væk fra en handel.

### 9.3 Store set-piece missions

Mere filmiske: bankrøveri, politistation break-in, havnecontainer-heist, casino job, fængselsbus-angreb, tunnel-flugt, bydel-lockdown, motorvejsjagt, hoteloverfald, stor bandealliance, final war mellem tre factions.

### 9.4 FiveM-inspirerede civ/job-systemer

Selvom det er singleplayer, kan byen have rollespilsagtige jobs: taxi, tow truck, mekaniker, bud, skraldemand, bartender, sikkerhedsvagt, street medic, privatdetektiv, repo man, bilforhandler, våbenkurér, klub-promoter.

**Design-regel:** De kan bruges som lovlig facade, penge, adgang til områder eller dække for kriminalitet.

**Eksempel:** Som tow-truck driver kan du lovligt slæbe biler væk. Senere bruger du det til at stjæle en bil fra politiets næse.

---

## 10. Kriminalitetsøkonomi

Penge er ikke bare "køb større våben". Der skal være valg.

### 10.1 Udgifter

våben, ammo, biler, bilmodifikationer, safehouses, bestikkelse, falske papirer, lægehjælp, advokat, information, tøj, crew-medlemmer, garage, skjulesteder, radio-scanner, burner phones, black market upgrades, faction gifts, virksomheder som front.

### 10.2 Sorte penge vs rene penge

To slags økonomi:

**Cash:** Hurtigt. Bruges til gadehandel, våben, bestikkelse.

**Clean money:** Kan bruges til ejendomme, virksomheder, officielle køb.

**Design-spørgsmål:** Vil du bare være kaotisk gaderøver, eller vil du bygge struktur?

### 10.3 Front businesses

Kan købes/kontrolleres: bilvask, pizzeria, bar, skrotplads, natklub, autoværksted, pantelåner, taxi-central, lagerhal, strip mall, bodega, laundromat.

**Giver:** passiv indkomst, missioner, safehouse, faction-konflikter, politiinteresse, lokale fordele.

---

## 11. Våben og udstyr

Spilagtigt, ikke våben-simulator.

### 11.1 Våbentyper

næver, bat, kniv-agtige melee, pistol, revolver, shotgun, SMG, rifle, sniper/top-down marksman, molotov-agtigt area weapon, granat-agtigt kaosvåben, taser, paintball/ikke-dødelige våben, hjemmelavede "junk weapons".

### 11.2 Udstyr

body armor, lockpick tool, scanner, fake ID, burner phone, jammer, medkit, zip ties, tracking device, crowbar, disguise clothes, duffel bag, spray can, car tracker remover.

### 11.3 Våbenkvalitet

Ikke kun skade. Våben kan have: larm, synlighed, reload, heat-risk, holdbarhed, ammo-pris, intimidation, faction-style.

**Design-regel:** Et stort våben gør dig farligere, men også mere synlig.

---

## 12. Køretøjer

Biler er halvdelen af spillet.

### 12.1 Biltyper

compact, lowriders, muscle cars, vans, trucks, motorcycles, scooters, police cars, ambulancer, brandbiler, taxi, tow trucks, armored vans, garbage trucks, forklifts, golf carts. (Både og helikoptere muligvis som NPC/politi, ikke spiller).

### 12.2 Bilstatistik

speed, acceleration, handling, durability, heat visibility, storage, off-road, armor, police attention, faction identity.

### 12.3 Bil-systemer

nummerplader, bilfarve, skade, motorlyd, bagagerum, skjulte rum, hotwire-tid, alarm, GPS tracker, police database, garage storage, chop shop, insurance/fake papers.

### 12.4 Bilmodifikationer

maling, plader, motor, dæk, armor, nitro, police scanner, hidden compartment, reinforced bumper, fake taxi sign, tinted windows, lowrider hydraulics (for stil).

---

## 13. Territoriekrig

Bander kan vinde og tabe områder.

### 13.1 System

Hver zone har influence-procenter. Eksempel:
- Southline Kings: 60%
- Police: 20%
- Mafia: 10%
- Neutral: 10%

Når du hjælper en faction, ændrer zonen sig.

### 13.2 Konsekvenser

**Hvis en bande overtager:**
- deres graffiti dukker op
- deres biler patruljerer
- deres musik spilles fra biler
- butikker betaler beskyttelse
- rivaler bliver jaget væk
- politiet kan trække sig eller slå hårdere ned

**Hvis politiet overtager:**
- flere checkpoints
- færre åbne dealers
- civile føler sig trygge
- bander flytter under jorden
- missioner bliver mere hemmelige

**Hvis mafiaen overtager:**
- området ser roligere ud
- mere usynlig kontrol
- flere sorte biler
- dyre restauranter
- baglokaler åbner

---

## 14. Crew-system

Spilleren kan samle et lille crew. Singleplayer-companions.

### 14.1 Crew-typer

driver, shooter, hacker/radio scanner, mechanic, medic, negotiator, lookout, burglar, fence/sælger, corrupt ex-cop, street kid/informant, biker muscle, mafia cleaner.

### 14.2 Personlighed

Hvert medlem har: loyalitet, frygt, grådighed, faction-forbindelser, moralgrænser, skills, relation til spilleren, chance for at panikke, chance for at forråde dig.

### 14.3 Eksempler

- **Rico "Clutch":** Sindssyg driver. Kan slippe fra politi. Men hader mafiaen.
- **Maya Vex:** Scanner og information. Kan finde patruljemønstre. Har gæld til en bande.
- **Old Frank:** Tidligere politimand. Kan skaffe uniformer og info. Men politiet holder øje med ham.
- **Tiny D:** Street-level kontakt. Billig, loyal, men skaber altid problemer.

### 14.4 Crew i missioner

Du vælger crew før større jobs. Valg betyder noget:
- billig crew = større risiko
- loyalt crew = mindre forræderi
- kendt crew = mere politi-heat
- rivaliserende crew = intern konflikt

---

## 15. AI Director

Et skjult system der skaber drama. Ikke tilfældig kaos hele tiden. Mere styret spænding.

### 15.1 Overvåger

- hvor længe spilleren har haft ro
- hvor meget heat der er
- hvilke factions der er vrede
- om spilleren mangler penge
- om spilleren bruger samme rute ofte
- om spilleren gentager samme strategi
- om byen føles for tom

### 15.2 Kan trigge

- politi spotter dig
- rivaler laver drive-by
- en kontakt ringer
- en butik bliver røvet
- et random street race starter
- en NPC genkender dig
- en informant tilbyder info
- en bande begynder at angribe rivalzone
- din bil bliver stjålet
- safehouse bliver overvåget
- nyhedsrapport om dine handlinger

**Pointen:** Byen føles levende uden at blive irriterende.

---

## 16. Nyheder, radio og rygter

Mega vigtigt for stemning.

### 16.1 Radio

Når du kører bil, kan du høre: musikstationer, politiscanner, talk radio, nyheder, reklamer, bande-dedikationer, falske konspirationer, lokale rygter.

### 16.2 Nyheder reagerer på dig

Efter missioner: "Police are investigating a violent incident near Old Harbor. Witnesses describe a red muscle car leaving the scene." Hvis du brugte rød muscle car, ved du: pis.

### 16.3 Rygtesystem

NPC'er og kontakter siger ting baseret på verden:
- "Cops are buying new cruisers."
- "Bikers moved guns through the north road."
- "Mafia's looking for a driver."
- "Don't go downtown in a stolen van today."
- "Southline put money on your head."

---

## 17. Tøj og identitet

Tøj er ikke bare kosmetik.

### 17.1 Tøj påvirker

- hvordan bander ser dig
- om politi stopper dig
- om du kan komme ind steder
- om civile frygter dig
- om du kan ligne en arbejder
- om du kan bruge forklædning

### 17.2 Typer

streetwear, suit, mechanic outfit, taxi uniform, delivery outfit, security guard, construction worker, biker vest, gang colors, tourist clothes, police-style disguise (farligt hvis opdaget).

### 17.3 Gang colors

Hvis du går i bestemte farver i forkert område, kan du blive stoppet. Men du kan også bruge det taktisk: provokere rivaler, infiltrere, skabe falsk konflikt, beskytte dig i venligt område.

---

## 18. Safehouses

Safehouses skal have gameplayværdi.

### 18.1 Funktioner

gemme spil, skifte tøj, stash våben, vaske heat ned, parkere biler, møde crew, planlægge jobs, høre politi-scanner, opgradere udstyr, skjule dig efter jagt.

### 18.2 Typer

billig lejlighed, motelværelse, garage, klubhus, mafia safe apartment, trailer i ørkenen, baglokale i butik, penthouse, skjult bunker under skrotplads.

### 18.3 Safehouse-risk

Hvis du bruger samme safehouse for meget under heat, kan politiet eller rivaler finde det. Spilleren skal sprede risiko.

---

## 19. Mission-struktur

Flere lag af missioner.

- **Main story:** Større historie om at stige i underverdenen.
- **Faction arcs:** Hver faction har sin egen kampagne.
- **Dynamic missions:** Genereres ud fra byens tilstand.
- **Personal missions:** Crew-medlemmer og kontakter har egne historier.

---

## 20. Moralske valg

Ikke moraliserende, men valg har konsekvenser.

**Eksempler:**
- Du kan røve en butik, men butiksejeren kan senere lukke, og området mister en nyttig kontakt.
- Du kan hjælpe en bande med at overtage et kvarter, men civile bliver mere bange.
- Du kan stikke en allieret, men får adgang til mafiaen.
- Du kan betale en gæld for et crew-medlem, og de bliver loyale.
- Du kan bruge ekstrem vold, men politiet prioriterer dig højere.
- Du kan holde lav profil og tjene mindre, men bygge stærkere netværk.

**Design-regel:** Ikke "god/ond". Mere "hvad er prisen?"

---

## 21. Sjove småsystemer

- bilradio afbrydes af politinyheder om dig
- NPC'er filmer dig med telefon
- videoer af dig kan øge heat
- graffiti ændrer sig efter territorie
- aviser/nyhedssider nævner dine actions
- en tow truck kan fjerne din favoritbil
- rivaler kan stjæle fra din garage
- politiet kan beslaglægge våben fra et kompromitteret stash
- butikker lukker skodder, hvis du kommer ind med våben
- dørmænd genkender dig
- taxi-chauffører kan sladre
- mekanikere kan snyde dig, hvis de ikke respekterer dig
- folk løber hurtigere fra dig, hvis du har ry for kaos
- børn/civile må ikke være mål; de kan bruges som stemnings-NPC'er, men ikke som kaosobjekter
- regn gør biler sværere at styre
- nat gør kriminalitet lettere, men også farligere
- helligdage/events ændrer byen

---

## 22. Random events

Top-down verdenen har små ting der sker af sig selv.

**Gade-events:** to bander skændes ved en tankstation, politi stopper en bil, ambulance kører til ulykke, butik bliver røvet, bilulykke spærrer vej, ulovligt race starter ved lyskryds, NPC løber fra politiet, rivaler leder efter dig, informant bliver jagtet, demonstration i government district, brand i lagerbygning, mafia-begravelse, biker-konvoj, blackout i et område, stormvejr skaber kaos.

**Design-regel:** Spilleren kan blande sig eller ignorere det. Ikke alt skal være en mission marker.

---

## 23. Lawful jobs som kontrast

For at crime føles fedt, skal lovlighed også findes.

**Mulige lovlige aktiviteter:** taxi, delivery, mechanic, towing, security, street racing (lovligt/ulovligt), repo jobs, bus driver (måske for humor), food truck, courier, private investigator, bounty-like recovery work.

**Giver:** lav heat, kontakter, adgang, cover identity, information, rene penge.

**Design-regel:** Du kan spille som low-profile fixer, ikke bare kaosmaskine.

---

## 24. Heists

Store jobs føles som egne mini-systemer.

### 24.1 Planlægningsfase

Du vælger: approach, crew, flugtrute, køretøj, våben, disguise, afledning, stash point, insider, hvor meget risiko du tager.

### 24.2 Approaches

- **Loud:** Hurtigt, voldsomt, meget heat.
- **Quiet:** Mere planlægning, mindre heat, men sværere.
- **Social:** Brug relationer, forklædning, adgang, bestikkelse.
- **Dirty:** Frame en anden faction.

### 24.3 Eksempel: Armored Van Heist

Muligheder: ram bilen med truck, hack rute-info via kontakt, bestik chauffør, angrib ved tunnel, brug falsk politi-checkpoint, lad rivalbande gøre arbejdet og stjæl fra dem bagefter.

**Design-regel:** Det samme job kan løses på flere måder.

---

## 25. Rival-system

Personlige fjender. Ikke kun factions.

### 25.1 Rivaler kan være

bandeleder, betjent, dusørjæger, tidligere crew-medlem, journalist, mafia-enforcer, gaderacer, korrupt politichef.

### 25.2 De udvikler sig

Hvis du ydmyger en rival, kan de: sætte dusør på dig, angribe dine forretninger, tippe politiet, kidnappe et crew-medlem, udfordre dig, sprede rygter, sabotere din bil.

### 25.3 Rivaler kan også blive allierede

Hvis du hjælper dem mod en større fjende. Det giver drama.

---

## 26. Politiefterforskning

Ved større forbrydelser starter ikke bare wanted level, men en efterforskning.

### 26.1 Beviser

Spillet tracker abstrakt: vidner, kameraer, nummerplade, fingeraftryk (gameplay-token), våbentype, biltype, kendte kontakter, sidste position.

### 26.2 Spilleren kan reducere beviser

- skift bil
- fjern nummerplade
- få bilen knust på skrotplads
- betal fixer
- intimider vidne (uden detaljeret realisme)
- hack/slet kameraoptagelser via mission
- få korrupt betjent til at "miste papirerne"
- frame rival

### 26.3 Efterforskningsstatus

Unknown suspect → Person of interest → Identified → Warrant active → Manhunt.

**Design-regel:** Gør kriminalitet mere strategisk.

---

## 27. Social manipulation

En af de mest spændende nye features.

### 27.1 Muligheder

- få én bande til at tro, at en anden stjal fra dem
- sælge information til begge sider
- mægle fred for profit
- starte krig for at skabe åbning
- bruge politiet mod en rival
- hjælpe civile for at svække en bande
- støtte en lokal kandidat/politiker for at ændre politi-pres
- lække information til journalist

### 27.2 Konsekvens

Bykortet ændrer sig. En zone kan blive: hot zone, contested, police crackdown, under truce, black market hub, abandoned, rich redevelopment zone, gang war zone.

---

## 28. Telefon / kontakter

Telefonen er mission hub, men også socialt system.

### 28.1 Kontakter

gang leaders, mechanics, lawyers, crooked cops, dealers, drivers, fixers, fences, journalists, bartenders, medics, real estate people, crew.

### 28.2 De kan skrive

- "Need you. Now."
- "Don't come here. Cops."
- "You owe me."
- "Someone asked about you."
- "I got a car you'll like."
- "Your boy got picked up."
- "Southline wants peace. Or blood. Hard to tell."

### 28.3 Services du kan ringe efter

car delivery, weapon stash, lawyer, clean car, taxi, backup, distraction, fake emergency call, tow truck, medic, bribe broker.

---

## 29. Humor og tone

Top-down formatet giver plads til satire.

### 29.1 Tone

En blanding af: 90'er crime sandbox, San Andreas-bandeenergi, FiveM-rollespil, mørk satire, absurde radioreklamer, systemisk kaos, karakterdrevet underverden.

### 29.2 Humoridéer

- politiradio der misforstår alt
- overdramatiske lokale nyheder
- bander med ekstremt specifikke regler
- mafia-boss der går mere op i restaurantanmeldelser end mord
- tow-truck NPC der altid dukker op på det værste tidspunkt
- en advokat der kun arbejder, hvis du har loyalty-card
- talk radio med vanvittige borgere
- graffiti der kommenterer dine handlinger
- NPC'er der bliver trætte af, at du altid stjæler deres biltype

---

## 30. Progression

Ikke kun levels.

### 30.1 Spilleren udvikler sig gennem

penge, kontakter, ry, adgang til zoner, bedre safehouses, crew, våben, biler, information, faction status, fear/love, story choices.

### 30.2 Skill-system (diskret)

Skills stiger gennem brug: driving, shooting, stealth, negotiation, intimidation, mechanics, street knowledge, police evasion, leadership.

**Design-regel:** Ikke for RPG-tungt. Skal stadig være hurtigt og arcade.

---

## 31. Gameplay loops

### Basic loop
Lav småjobs → tjen penge → køb bedre gear → få kontakter → tag større jobs → mere heat → håndter konsekvenser.

### Faction loop
Hjælp faction → få adgang → få fjender → påvirk territorium → lås op for større missioner.

### Police loop
Begå crime → skab heat → flygt/skjul → slet spor → vend tilbage med ny strategi.

### Business loop
Køb front → beskyt den → brug den til missioner → vask penge → tiltræk politi/rivaler.

### Crew loop
Find folk → brug dem → hjælp dem → gør dem loyale → risikér forræderi/død/arrestation.

### City loop
Dine handlinger ændrer byen → byen skaber nye muligheder og problemer → du reagerer.

---

## 32. Mission-eksempler

### "Wrong Car, Wrong Block"
Du stjæler en fed bil. Problemet er, at bilen tilhører en lokal bandeleder. Nu har du fire valg: lever den tilbage og undskyld, sælg den hurtigt, brug den som gave til rivalerne, behold den og accepter krigen.

### "Clean Plate"
En korrupt betjent tilbyder at slette en aktiv sag, hvis du henter en taske fra politiets beslaglæggelse. Problemet: tasken tilhører mafiaen.

### "Funeral Traffic"
En mafia-begravelse lammer Old Town. En rival vil have dig til at plante et signal på en bil. Mafiaen vil have dig til at beskytte konvojen. Politiet vil have billeder af deltagerne. Tre muligheder. Alle gør nogen sure.

### "Tow Job"
Du arbejder som tow-truck driver. Et job virker normalt. Da du slæber bilen væk, opdager du, at der ligger våben i bagagerummet. Nu ringer tre forskellige mennesker og siger, bilen er deres.

### "Heat Sink"
Du skal skabe politikaos i én zone, så en anden faction kan lave et job et andet sted. Du kan gøre det diskret med falske tips eller brutalt med kaos.

### "The Block Vote"
En lokal community-leder prøver at få politiet til at rydde en bande ud. Banden hyrer dig til at skræmme ham. Mafiaen vil hellere købe ham. En journalist vil afsløre alle.

### "Dead Drop"
Du skal hente en pakke fra en skraldespand. Men byen har garbage trucks med rigtige ruter. Hvis du kommer for sent, kører pakken væk gennem byen.

### "No Sirens"
Du skal transportere en skadet NPC gennem byen uden at tiltrække politi. Du kan køre til læge, illegal clinic, hospital eller lade en faction hjælpe.

### "Ghost Plate"
En bil med falske plader bruges til flere crimes. Politiet tror, det er dig. Du skal finde bilen, før dit heat eksploderer.

### "Three Calls"
Du får tre opkald på samme tid: dit crew-medlem er fanget, din forretning bliver angrebet, en stor mission starter nu. Du kan ikke nå alt. Verden fortsætter.

---

## 33. Endgame

Endgame er ikke kun "du ejer alt".

### Mulige slutretninger

- **Criminal empire:** Du kontrollerer virksomheder, territorier, kontakter. Banderne arbejder gennem dig.
- **Ghost fixer:** Ingen kan bevise noget. Lav offentlig profil, men styrer byen gennem favors.
- **Gang king:** Du vælger én faction og hjælper dem med at dominere byen.
- **Chaos legend:** Alle frygter dig. Politiet hader dig. Bander vil bruge dig eller dræbe dig.
- **Informant route:** Du arbejder hemmeligt med politiet og vælter underverdenen indefra.
- **Burn it all:** Du spiller alle ud mod hinanden, skaber krig og forlader byen rig.

---

## 34. Det særlige ved spillet

1. **Byen husker dig** — ry, vidner, rygter, områder, biler, tøj, kontakter.
2. **Factions er systemer, ikke mission-givere** — reagerer, kæmper, mister territorium, laver fejl, bliver desperate.
3. **Politiet efterforsker, ikke bare jagter** — wanted-level + bevis-system + profiler.
4. **NPC'er har mikro-dialog og lokale reaktioner** — byen levende uden kæmpe 3D-budget.
5. **Crime + civil jobs blandes** — taxi, tow truck, mekaniker, delivery er værktøjer, ikke minigames.
6. **Social manipulation** — snyd, mægle, forråde, frame andre.
7. **Top-down gør kaos overskueligt** — store biljagter, roadblocks, gang wars læses klart fra oven.

---

## 35. Drømme-feature-liste

Top-down storby med distrikter · gå/køre frit · levende trafik · stjæl biler · politi med heat-level · efterforskning og beviser · factions med territorier · dynamisk omdømme · NPC-dialog baseret på dit ry · butikker, barer, klubber, garages · safehouses · våbenbutikker og black market · bilmodifikation · radio og politiscanner · crew-system · front businesses · heists med planlægning · random events · rivaler der udvikler sig · nyheder der omtaler dig · tøj/farver påvirker reaktioner · lovlige jobs som dække · korrupte cops · informanter · faction wars · økonomi med dirty/clean money · sociale konsekvenser · flere slutninger · dynamisk bykort · vejspærringer og lockdowns · skjulte depoter · chop shops · garages · street races · smuglerkonvojer · gang graffiti · NPC'er der husker dig · missioner uden marker, via rygter · karakterer med loyalitet/forræderi · pressen og rygtesystem · mulighed for at spille brutal, smart, loyal, grådig eller manipulerende.

---

## 36. MVP-scope

Se `docs/03-roadmap/roadmap.md` for den fulde faseopdeling. MVP (Milestone M12) er en vertical slice med:

- 1-2 by-zoner fungerende
- 1 faction arc
- Kerne-loop fungerende (småjobs → penge → gear → større jobs → heat → flugt)
- Wanted-system basis
- Reputation-system basis
- Bevægelse + køretøjer
- Simple NPC'er med mikro-dialog
- Én heist-type

**MVP ekskluderer (til senere faser):**
- Alle 8 zoner
- Alle 6 faction-typer
- AI Director (kommer i Fase 10)
- Lyd/musik (Fase 11)
- Polishing

---

## Dokumentations-status

| Dokument | Status | Ejer |
|---|---|---|
| pitch.md | Færdig | GLM |
| gdd.md | Færdig (v1) | GLM |
| systems/ | Pending | GLM + Opus |
| tdd.md | Pending | Opus + GLM |
| roadmap.md | Pending | GLM |