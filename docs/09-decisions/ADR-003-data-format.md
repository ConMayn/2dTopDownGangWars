# ADR-003: Data-format — RON til config, bincode til saves

**Status:** Godkendt
**Dato:** Sprint 1, uge 1
**Beslutningstager:** GLM
**Review:** Opus (minor — ikke kritisk arkitektur)

## Kontekst

Heat City er et data-drevet spil. Al gameplay-konfiguration (zoner, factions, missioner, våben, biler, NPC-roller, dialog-træer) er defineret eksternt, ikke hardcoded. Dette gør:

- Balancering nemt (ingen recompile for at tune en factions aggression).
- Modding muligt.
- Hurtig iteration under udvikling.

Vi har brug for to data-formater:
1. **Config/data** (læsbart for mennesker, ændres ofte under udvikling).
2. **Save-filer** (binært, kompakt, versions-støttet, skrives sjældent).

Overvejet blev:

Til config:
- JSON
- YAML
- TOML
- RON (Rusty Object Notation)
- Binært (MessagePack, bincode)

Til saves:
- JSON
- YAML
- bincode
- MessagePack

## Beslutning

- **Config-data:** RON (Rusty Object Notation) via `ron` crate + `serde`.
- **Save-filer:** bincode via `bincode` crate + `serde`.

## Begrundelse

### RON til config

1. **Designet til spil.** RON blev skabt specifikt til spil-konfiguration (oprindeligt af Amethyst-projektet). Det er praktisk talt talt for vores use-case.
2. **Læsbart for mennesker.** Syntaks er Ruby/Python-agtig med optional typer:
   ```ron
   Zone(
       id: "east_blocks",
       name: "East Blocks",
       bounds: Rect(min: (0, 0), max: (2000, 1500)),
       police_intensity: 0.3,
   )
   ```
3. **Typed.** Integrerer med Rust-typer via serde. En `enum FactionType` parses automatisk.
4. **Understøtter comments.** `// linje` og `/* blok */` — kritisk for at dokumentere config.
5. **Trivial at parse.** `ron::from_str::<ZoneDef>(&str)`.
6. **God error reporting.** Fejl peger på linje/kolonne, nyttigt ved redigering.

### Hvorfor ikke JSON?

- Ingen comments.
- Ingen multiline strings.
- Quotes overalt, sværere at læse ved komplekst struktur.
- Dårligere error messages.

### Hvorfor ikke YAML?

- Indryknings-sensitivt (notorisk fejlbehæftet).
- Implicit typing (f.eks. `no` parses som boolean false).
- Langsom parser.
- Overkompliceret spec (ANSI Yaml-1.1 har 4 dokument-typer).

### Hvorfor ikke TOML?

- Godt til flad config (Cargo.toml), men svært at læse ved dybt nestede strukturer (missioner med steps, choices, consequences).
- Array-of-tables syntaks er tung.

### bincode til saves

1. **Kompakt.** Binært, ingen string-overhead.
2. **Hurtigt.** Minimal parse-tid.
3. **serde-native.** Samme `#[derive(Serialize, Deserialize)]` som RON.
4. **Versions-støttet.** Vi kan tilføje `version: u32` felt og migrere gamle saves.

### Hvorfor ikke JSON/YAML til saves?

- Større filer (text vs binary).
- Langsommere at parse.

### Hvorfor ikke MessagePack?

- Bincode er Rust-native og hurtigere for vores typer.
- MessagePack er cross-sprog, hvilket vi ikke har brug for.

## Konsekvenser

### Positive

- Config er læsbart og editérbart uden at bygge koden.
- Saves er kompakte og hurtige at loade.
- serde-derive betyder at vi kun definerer typer én gang.
- Versions-støttede saves er trivielle.

### Negative

- RON er mindre kendt end JSON/YAML. Ny udvikler (eller AI-agent) skal lære syntaksen.
- bincode-saves er ikke menneske-læsbare (umulig at redigere save-filer manuelt uden et værktøj).
- RON er langsommere at parse end bincode — men vi parser kun config én gang ved opstart (eller hot-reload), så det er ligegyldigt.

### Mitigations

- Eksempel-filer i `assets/data/` for hver data-type.
- Save-editor værktøj (i `tools/`) kan konvertere bincode ↔ RON for debugging.
- `serde` med field-attributes håndterer backwards-compat: `#[serde(default)]` for nye felter.

## Implementeringsnoter

- Alle config-filer i `assets/data/` med extension `.ron`.
- Save-filer i `%APPDATA%/HeatCity/saves/` (Windows) med extension `.sav`.
- Data-typer i `game/src/data/` modul, hver med `#[derive(Debug, Clone, Serialize, Deserialize)]`.
- Asset-loader i `engine/src/assets/` håndterer RON-parsing.
- Save-system i `game/src/save/` håndterer bincode.

### Eksempel: Zone data-type

```rust
// game/src/data/zone.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneDef {
    pub id: String,
    pub name: String,
    pub bounds: RectDef,
    pub owner: Option<String>,       // faction id
    pub police_intensity: f32,       // 0.0 - 1.0
    pub economic_level: f32,
    pub civilian_fear: f32,
    pub gang_presence: f32,
    pub weapon_level: f32,
    pub drug_market: f32,
    pub traffic_density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RectDef {
    pub min: (i32, i32),
    pub max: (i32, i32),
}
```

### Tilsvarende RON-fil (`assets/data/zones/east_blocks.ron`)

```ron
ZoneDef(
    id: "east_blocks",
    name: "East Blocks",
    bounds: Rect(min: (0, 0), max: (2000, 1500)),
    owner: Some("southline_kings"),
    police_intensity: 0.3,
    economic_level: 0.4,
    civilian_fear: 0.2,
    gang_presence: 0.7,
    weapon_level: 0.5,
    drug_market: 0.6,
    traffic_density: 0.5,
)
```

## Versionering af save-format

```rust
#[derive(Serialize, Deserialize)]
pub struct SaveFile {
    pub version: u32,
    pub data: SaveData,
}

impl SaveFile {
    pub fn load(path: &Path) -> Result<Self, SaveError> {
        let bytes = std::fs::read(path)?;
        let save: SaveFile = bincode::deserialize(&bytes)?;
        match save.version {
            1 => Ok(save.migrate_v1_to_current()),
            CURRENT_VERSION => Ok(save),
            _ => Err(SaveError::UnsupportedVersion(save.version)),
        }
    }
}
```

## Review-krav

Opus bør kort reviewe. Ikke kritisk arkitektur, men:
- Er RON + bincode et konsistent valg?
- Skal saves krypteres (anti-cheat)? Nej for singleplayer, men valgfrit.

## Alternativer overvejet og afvist

Se "Begrundelse" ovenfor.

## Referencer

- RON: https://github.com/ron-rs/ron
- bincode: https://github.com/bincode-org/bincode
- serde: https://serde.rs

## Dokumenthistorik

| Dato | Begivenhed | Forfatter |
|---|---|---|
| Sprint 1, uge 1 | Oprettet | GLM |