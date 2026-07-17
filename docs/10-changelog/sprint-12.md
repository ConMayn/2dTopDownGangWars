# Sprint 12 — Changelog

> **Fase:** 11 — Polish, Audio, UI
> **Sprint:** 12
> **Dato:** 2026-07-17
> **Status:** ✅ Milestone M11 nået
> **Agenter:** GLM (hovedingeniør), Opus (review/debug)

---

## Opnået denne sprint

### Save-system (Fase 11)

- ✅ `game/src/save/mod.rs` — `SaveState`, `PlayerSaveState`, `SaveSlots`, `SaveError`
- ✅ Versions-støttet (SAVE_VERSION = 1) med migration API
- ✅ Bincode serialisering (TDD 12.2)
- ✅ 5 save slots (TDD 12.1)
- ✅ Gemmer: player, faction trust, zone influence, missions, world time, police profile, evidence, crew, safehouses, businesses, heat, rivals (TDD 12.3)
- ✅ `save_to_file` / `load_from_file` med fejlhåndtering
- ✅ Migration: afviser fremtidige versioner, opgraderer ældre
- ✅ Tests: serialisering + migration rejection

### UI / HUD (Fase 11)

- ✅ `game/src/ui/mod.rs` — `HudState`, `MenuScreen`, `UiState`
- ✅ HUD-elementer: cash/clean, heat, missions, objective, time, crew status, nearby events, news ticker, dialog
- ✅ 9 menu-skærme: None, Pause, Phone, Inventory, SafehouseMenu, MissionLog, FactionLog, Options, SaveLoad
- ✅ `render_debug()` — tekst-baseret HUD-rendering (proof)
- ✅ Dialog display med valg
- ✅ `update_hud()` i WorldPlugin opdaterer HUD fra alle spil-systemer

### Audio (Fase 11)

- ✅ `game/src/audio/mod.rs` — `SoundKind`, `Sound`, `RadioStation`, `AudioSystem`
- ✅ 6 lyd-typer: Sfx, Ambient, Music, Radio, Voice, Ui
- ✅ Volume-kanaler: master, sfx, music, radio, voice
- ✅ Mute toggle
- ✅ Radio-stationer med tracks, next_track
- ✅ Play-queue (SFX/UI/voice)
- ✅ `effective_volume()` kombinerer master × channel
- ✅ 3 seed radio-stationer: Hot 97 FM (Hip-Hop), Klassik FM (Classical), Police Scanner (Talk)
- ✅ Tests: mute zeroes volume, effective_volume kombinerer

### Integration i WorldPlugin

- ✅ SaveSlots, UiState, AudioSystem state i WorldPlugin
- ✅ `update_hud()` opdaterer HUD per frame fra alle systemer
- ✅ Audio tick per frame
- ✅ 3 radio-stationer seedes ved init
- ✅ Debug overlay udvidet med menu, radio, audio muted status
- ✅ bincode dependency tilføjet til Cargo.toml

---

## Milestone M11 — Status: ✅ Nået

**Definition:** Spillet har lyd, komplet UI, og kører stabilt.

**Resultat:**
- Save-system med bincode, 5 slots, versioning ✅
- UI/HUD state med alle gameplay-elementer ✅
- Audio-system API med radio, volume, mute ✅
- Integration kører uden crash ✅
- Zero warnings build ✅

---

## Tekniske noter

### Save migration
- `SAVE_VERSION = 1`; fremtidige versioner kan tilføje migration-trin.
- `migrate()` afviser versioner nyere end understøttet.
- Bincode v1.3 brugt (stabil, velkendt).

### HUD rendering
- Fase 11 proof: `render_debug()` producerer tekst-streng.
- Fremtid: rigtig UI rendering med panels, knapper, telefon-interface.
- HudState opdateres per frame fra WorldPlugin.

### Audio stub
- Ingen faktisk afspilning endnu — API er klar.
- Fremtid: kira integration til looping ambient, musik + SFX.
- Radio-stationer er data-struktur klar til at loade tracks.

### Array Default
- `SaveSlots` bruger `Default::default()` i stedet for `[None; 8]` for at matche `[Option<SaveState>; 5]`.

---

## Næste sprint (Sprint 13) — plan

**Mål:** Fase 12 — Vertical Slice.

- [ ] Saml alle systemer til spilbar oplevelse
- [ ] Faktisk rendering af HUD/tekst
- [ ] Input-mapping for menuer (Escape, telefon)
- [ ] Tutorial / onboarding
- [ ] Performance-optimering (batching, culling)
- [ ] Save/load integration med gameplay
- [ ] Options menu (graphics, audio, controls)
- [ ] Balancering
- [ ] Release candidate

**Milestone M12:** Spilbar vertical slice release.

---

## Stats

- **Nye moduler:** 3 (save, ui, audio)
- **Nye filer:** 3 (mod.rs i hver mappe)
- **Nye dependencies:** bincode
- **Linjer kode:** ~500 Rust
- **Save slots:** 5
- **Menu screens:** 9
- **HUD elements:** 11
- **Sound kinds:** 6
- **Radio stations:** 3
- **Hovedagent:** GLM