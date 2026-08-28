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

**Empty.** Everything deferred out of the 2.1.x line has landed: 2.1.3 (the Rust test
suites), 2.1.4 (removing `rust-old/`, [ADR 0002](../adr/0002-remove-the-rust-old-archive.md)),
2.1.5 (the deferred hardening items) and 2.1.6 (script registry completeness). Next up
is the `2.2.0+` tier below.

## 2.2.0+ — "World's Leading Authority"

> Gaps identified by comparing varna against PHOIBLE, WALS, Glottolog, Unicode CLDR,
> and the IPA specification. Ordered by impact on credibility and utility; the
> `(P1)`-`(P4)` tags are that priority, the version numbers are the intended
> sequence after the `2.1.x` carry-over work lands.
>
> Renumbered from the pre-port `1.1.0`-`1.5.0` plan (2026-08-27) — the old numbers
> predated the 2.0.0 Cyrius port and no longer tracked the shipping version.

### 2.2.x — Phonological Depth (P1) — COMPLETE

All five slices have landed: 2.2.0 (airstream), 2.2.1 (consonant secondary
features), 2.2.2 (vowel features), 2.2.3 (distinctive features) and 2.2.4
(structured tone). See the CHANGELOG.

The original single bullet proposed adding `Click`/`Implosive`/`Ejective` to the
`Manner` enum. That was rejected during 2.2.0: they are airstream mechanisms, not
manners, and folding them in would have destroyed the manner of 19 of the 38
affected phonemes. `Airstream` is a separate axis, as in PHOIBLE.

Three data items surfaced along the way and remain open:

#### Data: ATR vowel systems

Deferred out of 2.2.2. `atr` was named there, but no corpus segment is transcribed
with an ATR/RTR diacritic, and two of the three languages that need the contrast
have incomplete vowel systems — so it is a data item, not a feature bit.

- [ ] **Yoruba** — /i e ɛ a ɔ o u/ is already the classic 7-vowel ATR set;
      /e o/ are +ATR, /ɛ ɔ/ are −ATR. Markable as it stands.
- [ ] **Wolof** — lists only /i e a o/ (+ long). Real Wolof has /i e ɛ a ɔ o u/
      with ATR harmony; the missing vowels have to be added first.
- [ ] **Somali** — lists /i e a o u/ (+ long). Real Somali has a 5-pair ATR system,
      i.e. 10 qualities; likewise incomplete.
- [ ] Then add the `AdvancedTongueRoot` bit, once something sets it.

#### Data: Thai and Vietnamese tone transcription

Surfaced by 2.2.4's structured records. Both languages list a `˨˩˦` tone, which is
the shape of *Mandarin's* third tone; the same wrong value in two unrelated
languages points at a copy-paste from the Mandarin entry.

- [ ] **Thai** — has `˧ ˨˩ ˨˩˦ ˦˥ ˩˧˥`. The falling tone `˥˩` is missing and `˨˩˦`
      stands where it should be.
- [ ] **Vietnamese** — huyền is listed as `˨˩˦`; it is a low falling tone, `˨˩`.
- [ ] Re-check the other seven toned inventories against a reference while there.

#### Data: Sanskrit voiced-aspirate transcription

- [ ] Sanskrit spells its voiced aspirates /ɡʰ d͡ʑʰ ɖʰ d̪ʰ bʰ/ with U+02B0 while
      Hindi, Bengali and Urdu spell the same historical series with U+02B1, so
      2.2.1 derives `Aspirated` for one and `Breathy` for the other. Phonetically
      they are breathy in all four. Decide one transcription and apply it; the
      assertion in `tests/features.tcyr` that pins the current state documents it.

### 2.3.x — Typological Depth (P2)

**2.3.0 (grammar expansion) and 2.3.1 (classification) have landed.** The verified
data for the remaining slice is in [typology-data.md](typology-data.md) — gathered and adversarially
verified, so 2.3.1 and 2.3.2 are encoding work rather than research work. Read that
file's caveats first.

The "Missing script entries" sub-item was listed here in error: all nine scripts
shipped at **2.1.6**.

#### 2.3.2 — Endangerment and geography

- [ ] `EndangermentLevel` enum (Safe/Vulnerable/Threatened/Shifting/Moribund/
      NearlyExtinct/Extinct), macro-area, and approximate lat/long per language.
- [ ] **Source Korean separately.** The gathering pass returned 50 of 51 rows and
      silently dropped `ko`; the verifier did not catch it, because a reviewer sees
      only the claims it is given, never the ones that are missing.
- [ ] Decide the semantics for `la`, `grc`, `sa` and `lzh` before encoding — whether
      a liturgical or scholarly language with no native speakers is `Extinct`.
- [ ] Treat the coordinates as approximate centroids, and say so in the API docs.
      They were generated, not looked up.

#### Data: Glottocode support

Deferred out of 2.3.0 and **not** a coding task. Glottocodes are opaque identifiers
(`stan1293`, `nucl1301`) that can only be looked up. Generating them from memory
would produce plausible-looking fabrications indistinguishable from real ones, which
is the worst failure mode for a reference corpus.

- [ ] Obtain the 51 Glottocodes from Glottolog itself, then add the nullable field.

### 2.4.0 — Gematria & Numeric Letter Values (P2)

Extend `script::numerals` into a full character→number mapping system across scripts. Foundation for classical cipher work and sankhya gematria computation.

- [ ] **Hebrew gematria values**: Standard (א=1..ת=400), ordinal (א=1..ת=22), reduced (digital root)
- [ ] **Arabic abjad numerals**: Standard abjad order (أ=1..غ=1000)
- [ ] **Latin/English ordinal values**: a=1..z=26 (simple gematria, used by classical ciphers)
- [ ] **Cyrillic numeric values**: Church Slavonic letter-number system
- [ ] **`NumericSystem` enum**: Standard, Ordinal, Reduced, Additive — per-script system classification
- [ ] **`char_value(script, system, ch)`**: Unified lookup API across all scripts (returns the value, or a sentinel when unmapped)
- [ ] **`script_alphabet_values(script, system)`**: Full (char, value) mapping table per script
- [ ] **Cipher foundation**: Character↔number round-trip enables Caesar, Vigenère, substitution cipher implementations downstream (a crypto Cyrius project or sankhya)

### 2.5.0 — Coverage Scale (P3)

- [ ] **Data-driven inventories**: Load from PHOIBLE CSV/JSON for 2000+ languages (feature-gated)
- [ ] **Expanded allophone rules**: Mandarin, Spanish, Japanese, Russian, Arabic (currently English only)
- [ ] **Expanded phonotactic profiles**: All core languages (currently 3)
- [ ] **Transliteration tables**: Cyrillic-Latin, Arabic-Latin, Hebrew-Latin, Pinyin (currently 2)
- [ ] **Source provenance**: Track bibliography/reference for each inventory
- [ ] **Multiple inventories per language**: Competing analyses like PHOIBLE

### 2.6.0+ — Differentiators (P4)

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
