# Development Roadmap

> **Status**: v2.1 (Cyrius) | **Current**: 2.1.2
>
> Open work only — shipped releases are in [CHANGELOG.md](../../CHANGELOG.md).
>
> Items marked `[S]` also unblock **sankhya** (ancient mathematical systems).
>
> Backlog signatures are illustrative and pre-date the port; they land as Cyrius
> forms (tagged enums, sentinel returns, `str` literals — no `Cow`/`Option`
> types). Rust mechanics named in them (`Cow`, `#[non_exhaustive]`, `criterion`,
> `serde`) map to Cyrius equivalents per [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md).

## Shipped

Release history lives in [CHANGELOG.md](../../CHANGELOG.md) and the git tags —
`2.1.2` (hardening sweep), `2.1.1` / `2.1.0` (toolchain maintenance), `2.0.0`
(the Cyrius port), and the `0.x`–`1.0` Rust-crate line before it. This file
tracks only what is still open.

## 2.1.x — Carry-over

Work identified but deliberately not taken in the release that surfaced it.

### 2.1.3 — Port the Rust integration suites [blocks 2.1.4]

The 2.0.0 port carried every module's public API and its in-module unit tests,
but **not** the four standalone suites under `rust-old/tests/` — 147 tests that
never had a Cyrius counterpart:

| Suite | Tests | Status |
|---|---|---|
| `invariants.rs` | 33 | Structural guarantees; **not ported** |
| `adversarial.rs` | 54 | Edge-case/robustness; **not ported** |
| `integration.rs` | 33 | Cross-module; ~15 are serde-only, rest **not ported** |
| `serde_roundtrip.rs` | 27 | N/A — no serde in Cyrius (see [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md)) |

This is the highest-value item on the list. `adversarial.rs` is precisely the
class of test that would have caught the 2.1.2 heap overflows years earlier — it
already contains `registry_info_very_long_code`, `phoneme_find_very_long_string`,
`error_display_very_long` and `transliteration_null_bytes`.

The invariants were probed against current data during the 2.1.2 review: 26 of
the 27 checkable ones hold. Porting them is guarding behaviour that is already
correct, not repairing it — but nothing currently stops a future edit breaking
them. Suggested landing spots: `tests/invariants.tcyr` and
`tests/adversarial.tcyr`.

For the serde suite, the equivalent Cyrius concern is the hand-built JSON in
`src/mcp.cyr`, which has 22 assertions in `tests/mcp.tcyr` against 27 Rust
roundtrip tests. Worth a pass for output well-formedness (escaping, in
particular — see the note under 2.1.5).

### 2.1.4 — Remove `rust-old/` [gated on 2.1.3]

`rust-old/` is the frozen parity oracle: 35 files, 8,386 lines, 484K. The public
API port is verified complete (all 150 Rust `pub fn` have Cyrius counterparts;
48/48 inventories match by name), so nothing in `src/` depends on it. What still
does is 2.1.3: once the directory is gone, we can no longer read what those 147
tests asserted. Port first, delete second.

When it goes, sweep with it:

- 93 `rust-old/...` provenance references across 49 files (`# Ported from
  rust-old/src/...` headers in `src/*.cyr` and `tests/*.tcyr`). Decide: keep as
  historical provenance, or rewrite to cite [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md)
- `docs/benchmarks-rust-vs-cyrius.md` — a Rust-vs-Cyrius comparison that outlives
  its subject; keep as a historical document or fold into the ADR
- The `rust-old/target/` entry in `.gitignore`
- The "Rust archive" line in `CLAUDE.md` and the `rust-old/` mentions in
  `README.md`, `SECURITY.md`, `docs/guides/getting-started.md`,
  `docs/development/state.md`

The directory is tracked in git, so tags `1.0.0` and `2.0.0` keep it recoverable
after deletion.

### 2.1.5 — Deferred from the 2.1.2 hardening sweep

- [ ] **Cache the pre-built data constructors.** Every `phoneme_*` / `script_*` /
      `grammar_*` / `swadesh_*` / `translit_*` / `numerals_*` constructor rebuilds
      its whole structure on each call — `phoneme_english()` allocates an
      inventory, a vec and 36 phoneme records every time, and `registry_phonemes`
      calls it fresh on every lookup. With a bump allocator that never frees, a
      consumer polling the registry leaks steadily, and this is the single largest
      remaining cost in the inventory benchmarks.

      Held back deliberately: caching turns each constructor into a shared
      singleton, so a consumer mutating a returned inventory through the
      `phoneme_builder_*` functions would corrupt every other caller's copy. That
      is a public-API semantic change wanting its own release and migration note —
      `registry_all_codes` took exactly this change in 2.1.2 and is now documented
      read-only. Options: cache and document read-only; or cache plus an explicit
      `phoneme_clone` for callers needing a mutable copy.
- [ ] **JSON escaping in `src/mcp.cyr`.** Tool payloads interpolate strings into
      hand-built JSON with no escaping. Every value written today is either a
      validated ISO code or internal data, so nothing malformed reaches the output
      — but the invariant is unenforced and one new field could break it.
- [ ] **`varna_translate_ipa` returns a bare string** where the other four MCP
      tools return JSON objects. A consumer parsing every payload as JSON gets
      malformed data from this one. Decide the contract and make it uniform.

### 2.1.6 — Script registry completeness

Nine registered languages name a primary script that `script_by_code` cannot
resolve, so `registry_primary_script` silently returns 0 for them: **Thai**
(Thai), **Bengali** (Beng), **Tamil** (Taml), **Amharic** (Ethi), **Hebrew**
(Hebr), **Georgian** (Geor), **Burmese** (Mymr), **Khmer** (Khmr), **Lao**
(Laoo). `src/script.cyr` defines 10 scripts; the registry names 17.

Not a port regression — the Rust crate had the identical gap, and its
`registry_script_codes_resolve` invariant explicitly tolerated it ("Some scripts
may not be registered yet (Thai, Beng, etc.)"). Adding the nine closes the gap
and lets 2.1.3 port that invariant in its strict form.

## Post-1.0 Roadmap — "World's Leading Authority"

> Gaps identified by comparing varna against PHOIBLE, WALS, Glottolog, Unicode CLDR,
> and the IPA specification. Prioritized by impact on credibility and utility.
>
> The `1.x` numbering below is the **pre-port** plan and no longer tracks the
> shipping version (varna is on `2.1.x`). Read the headings as priority tiers
> P1-P4, not as releases; they will be renumbered when one is scheduled.

### 1.1.0 — Phonological Depth (P1)

- [ ] **Distinctive feature system**: Add `DistinctiveFeatures` bundle with 20+ binary features per phoneme (sonorant, continuant, strident, anterior, distributed, ATR/RTR, spread/constricted glottis, syllabic, etc.) — PHOIBLE parity
- [ ] **Manner expansion**: Add `Click`, `Implosive`, `Ejective` to `Manner` enum — reclassify Zulu clicks, Georgian ejectives, Hausa implosives
- [ ] **Consonant secondary features**: `aspirated`, `labialized`, `palatalized`, `prenasalized`, `long` fields on the `PhonemeKind.Consonant` variant
- [ ] **Vowel features**: `long`, `nasalized`, `atr` (Advanced Tongue Root) fields on the `PhonemeKind.Vowel` variant
- [ ] **Tone as structured data**: Replace string tone labels with `Tone` structs (contour, register, features)

### 1.2.0 — Typological Depth (P2)

- [ ] **Grammar expansion** toward WALS parity: alignment type (nom-acc/erg-abs/active-stative), adposition order, tense/aspect system, evidentiality, negation strategy, adjective order, relative clause order, article type
- [ ] **Language classification**: Add `family`, `subfamily`, `genus` to `LanguageInfo` (Indo-European > Germanic > West Germanic)
- [ ] **Missing script entries**: Hebrew, Thai, Tamil, Georgian, Ethiopic, Myanmar, Khmer, Lao, Bengali (9 scripts for already-registered languages)
- [ ] **Glottocode support**: Add a nullable `glottocode` field (str) alongside ISO 639 codes
- [ ] **Endangerment status**: `EndangermentLevel` enum (Safe/Vulnerable/Threatened/Shifting/Moribund/NearlyExtinct/Extinct)
- [ ] **Geographic metadata**: Latitude/longitude per language, macro-area classification

### 1.3.0 — Gematria & Numeric Letter Values (P2)

Extend `script::numerals` into a full character→number mapping system across scripts. Foundation for classical cipher work and sankhya gematria computation.

- [ ] **Hebrew gematria values**: Standard (א=1..ת=400), ordinal (א=1..ת=22), reduced (digital root)
- [ ] **Arabic abjad numerals**: Standard abjad order (أ=1..غ=1000)
- [ ] **Latin/English ordinal values**: a=1..z=26 (simple gematria, used by classical ciphers)
- [ ] **Cyrillic numeric values**: Church Slavonic letter-number system
- [ ] **`NumericSystem` enum**: Standard, Ordinal, Reduced, Additive — per-script system classification
- [ ] **`char_value(script, system, ch)`**: Unified lookup API across all scripts (returns the value, or a sentinel when unmapped)
- [ ] **`script_alphabet_values(script, system)`**: Full (char, value) mapping table per script
- [ ] **Cipher foundation**: Character↔number round-trip enables Caesar, Vigenère, substitution cipher implementations downstream (a crypto Cyrius project or sankhya)

### 1.4.0 — Coverage Scale (P3)

- [ ] **Data-driven inventories**: Load from PHOIBLE CSV/JSON for 2000+ languages (feature-gated)
- [ ] **Expanded allophone rules**: Mandarin, Spanish, Japanese, Russian, Arabic (currently English only)
- [ ] **Expanded phonotactic profiles**: All core languages (currently 3)
- [ ] **Transliteration tables**: Cyrillic-Latin, Arabic-Latin, Hebrew-Latin, Pinyin (currently 2)
- [ ] **Source provenance**: Track bibliography/reference for each inventory
- [ ] **Multiple inventories per language**: Competing analyses like PHOIBLE

### 1.5.0+ — Differentiators (P4)

- [ ] PHOIBLE-compatible export format
- [ ] WALS feature code mapping
- [ ] Typological cross-cutting queries ("all SOV languages with ejectives")
- [ ] ISO 639 validation (static lookup table)
- [ ] Prosody patterns (intonation contours, rhythm class: stress/syllable/mora-timed)
- [ ] Morphological analyzer (stemming, lemmatization per language)
- [ ] Historical phonology (sound change rules, Proto-IE reconstructions)
- [ ] Sign language phonology (handshape, location, movement features)
- [ ] ScriptType.Featural for Hangul reclassification

## Downstream

- [ ] shabda + shabdakosh consuming varna for multilingual G2P (external Cyrius
      project work — the last unmet v1.0 criterion; sankhya, jnana and vidya
      already consume it)

> The old **v1.0 Criteria** checklist retired here: varna passed it before the
> 2.0.0 port and everything on it except the line above is met. One of its
> entries — "full JSON roundtrip tests for all public types" — did **not**
> survive the port intact: the Rust `serde_roundtrip.rs` suite was never carried
> over and `src/mcp.cyr` hand-builds its JSON. See 2.1.3.
