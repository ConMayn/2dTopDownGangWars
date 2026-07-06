# ADR-002: ECS-valg — hecs

**Status:** Godkendt (pending Opus review)
**Dato:** Sprint 1, uge 1
**Beslutningstager:** GLM
**Review:** Opus (kritisk — kræver arkitektur-review før Fase 1)

## Kontekst

Heat City er et systemisk spil med mange entiteter (op til 2000 i full release: NPC'er, biler, genstande, projektiler, partikler). Vi har brug for en ECS-arkitektur (Entity-Component-System) der:

- Giver god cache-locality for batch queries.
- Understøtter dynamisk component add/remove (NPC'er skifter rolle, entiteter får midlertidige tags).
- Er Rust-idiomatisk og sikker.
- Er letvægt (ingen macro-magi, nem at debugge).
- Understøtter parallel queries (til Fase 5+).

Overvejet blev:

1. **hecs** — arketyper-baseret, letvægt, simpel API.
2. **bevy_ecs** (som standalone crate) — feature-rig, men koblet til Bevy's livscyklus.
3. **specs** — den klassiske Rust ECS, sparse-set baseret, men ældre og langsommere udvikling.
4. **Custom ECS** — vi skriver vores egen, maksimal kontrol.

## Beslutning

Vi bruger **hecs** som ECS.

## Begrundelse

### Hvorfor hecs?

1. **Arketyper-baseret.** Komponenter med samme layout gemmes sammen i hukommelsen → fremragende cache-locality for queries som "alle med Position + Velocity". Perfekt til simulation med mange entiteter.
2. **Letvægt.** ~2000 linjer Rust, ingen dependencies udover `hecs` selv. Ingen macro-magi. Let at forstå og debugge.
3. **Simpel API.**
   ```rust
   let mut world = World::new();
   let id = world.spawn((Position(Vec2::ZERO), Velocity(Vec2::ZERO), Player));
   for (pos, vel) in world.query_mut::<(&mut Position, &mut Velocity)>() { /* ... */ }
   ```
4. **Rust-idiomatisk.** Bruger Rust's type-system, ikke runtime reflection.
5. **Parallel queries.** `World::query` (immutable) kan køres fra flere tråde med rayon.
6. **Aktivt vedligeholdt.** Ralith (maintainer) er responsiv.
7. **God dokumentation** og eksempler.

### Hvorfor ikke bevy_ecs?

- Bevy's ECS er fremragende, men er tæt koblet til Bevy's `App`, `Plugin`, `System`-param-derive-makroer. At bruge det standalone kræver man enten bygger på Bevy's lifecycle eller fjerner det. Det er ikke "bare en ECS".
- Bevy er stadig pre-1.0; API-ændringer kan ske.
- Hvis vi vil bruge Bevy's ECS, vil vi lige så godt bruge hele Bevy — og det afviste vi i ADR-001.

### Hvorfor ikke specs?

- specs er den "klassiske" Rust ECS, men udviklingen er langsommere end hecs og bevy_ecs.
- Sparse-set baseret (god til dynamisk component add/remove), men arketyper er generelt hurtigere for vores use-case (mange entiteter med samme komponent-sæt).

### Hvorfor ikke custom ECS?

- At skrive vores egen ECS er muligt, men vi risikerer:
  - Genopfinde hjulet (mange timers debugging af arketyper vs sparse-sets).
  - Mindre tid til selve spillet.
- hecs er tilstrækkeligt simpel, at vi kan forstå al dens kildekode hvis nødvendigt.
- Hvis vi senere har brug for specialiserede queries, kan vi bygge dem ovenpå hecs.

## Konsekvenser

### Positive

- God performance for batch queries (simulation).
- Simpelt API, nemt at lære.
- Letvækt, ingen ekstra dependencies.
- Arketyper passer til vores use-case: NPC'ere, biler, projektiler har hver deres faste komponent-sæt.

### Negative

- **Arketyper har overhead ved hyppigt component add/remove.** Hvis en NPC ofte skifter rolle (f.eks. fra `Idle` til `Fleeing`), flyttes dens data mellem arketyper-lagre. For vores NPC-brug er dette acceptablet — tilstande håndteres via data (f.eks. `enum NpcState`) frem for komponenter.
- **Ingen indbygget system-scheduling.** hecs giver ikke et "systemer køres i denne rækkefølge"-system. Vi bygger selv en simpel `Plugins`-trait der gør dette (se TDD afsnit 3.2).
- **Ingen indbygget event-system.** Vi bygger selv en simpel event-queue (eller bruger `crossbeam-channel`).

### Mitigations

- For NPC-tilstande: brug `enum NpcState` som komponent-data frem for at add/remove komponenter.
- For scheduling: `game/src/main.rs` definerer system-ordning eksplicit.
- For events: en simpel `Vec<Event>` i World, drænet per frame.

## Implementeringsnoter

- hecs version: seneste stable (vil være ~0.10+ ved Fase 1).
- `engine/src/ecs/` wrapper om hecs med hjælpemetoder (spawn, query, despawn).
- `game/src/systems/` indeholder alle gameplay-systemer, der kører over hecs-queries.

## Review-krav

Opus skal reviewe denne ADR før Fase 1. Særligt:

1. **Er arketyper tilstrækkelige for vores NPC-model?** Hvis NPC'er dynamisk skifter komponent-sæt ofte (f.eks. "blir passenger i bil" → add `Passenger` komponent), kan overhead blive et problem. Alternativ: brug `Option<T>` komponenter eller flags.
2. **Skal vi bygge en custom scheduler?** hecs har ikke en. Simpel ordning i main.rs er nok for MVP, men for Fase 5+ med parallelle systemer kan vi have brug for et rigtigere scheduler.
3. **Event-system arkitektur.** En `Vec<Event>` er enkel, men kan blive et bottleneck ved mange events. Alternativ: `crossbeam-channel` eller `flume`.

## Alternativer overvejet og afvist

### bevy_ecs standalone
- **For:** Feature-rig, parallel queries native, event-system.
- **Imod:** Koblet til Bevy-livscyklus, pre-1.0, macro-tung.

### specs
- **For:** Klassisk, veldokumenteret, sparse-set.
- **Imod:** Langsommere udvikling, arketyper generelt hurtigere for vores use-case.

### Custom ECS
- **For:** Maksimal kontrol.
- **Imod:** Tid til at genopfinde, risiko for fejl.

## Referencer

- hecs: https://github.com/Ralith/hecs
- bevy_ecs: https://docs.rs/bevy_ecs
- specs: https://github.com/amethyst/specs
- "ECS Back and Forth" (arketyper vs sparse-sets): https://medium.com/@ajmmertens

## Dokumenthistorik

| Dato | Begivenhed | Forfatter |
|---|---|---|
| Sprint 1, uge 1 | Oprettet | GLM |