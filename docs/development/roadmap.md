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

## Data corrections — open

Four data-accuracy items surfaced by the 2.2.x and 2.3.x work. None is a coding task:
each needs a decision about linguistic data, and three were deliberately left alone
rather than changed silently mid-release. Collected here because they were previously
filed under section headings now marked COMPLETE, which read as though they were done.

They are independent of each other and of the feature roadmap below, so any can be
taken in isolation.

### Data: ATR vowel systems

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

### Data: Thai and Vietnamese tone transcription

Surfaced by 2.2.4's structured records. Both languages list a `˨˩˦` tone, which is
the shape of *Mandarin's* third tone; the same wrong value in two unrelated
languages points at a copy-paste from the Mandarin entry.

- [ ] **Thai** — has `˧ ˨˩ ˨˩˦ ˦˥ ˩˧˥`. The falling tone `˥˩` is missing and `˨˩˦`
      stands where it should be.
- [ ] **Vietnamese** — huyền is listed as `˨˩˦`; it is a low falling tone, `˨˩`.
- [ ] Re-check the other seven toned inventories against a reference while there.

### Data: Sanskrit voiced-aspirate transcription

- [ ] Sanskrit spells its voiced aspirates /ɡʰ d͡ʑʰ ɖʰ d̪ʰ bʰ/ with U+02B0 while
      Hindi, Bengali and Urdu spell the same historical series with U+02B1, so
      2.2.1 derives `Aspirated` for one and `Breathy` for the other. Phonetically
      they are breathy in all four. Decide one transcription and apply it; the
      assertion in `tests/features.tcyr` that pins the current state documents it.

### Data: Glottocode support

Deferred out of 2.3.0 and **not** a coding task. Glottocodes are opaque identifiers
(`stan1293`, `nucl1301`) that can only be looked up. Generating them from memory
would produce plausible-looking fabrications indistinguishable from real ones, which
is the worst failure mode for a reference corpus.

- [ ] Obtain the 51 Glottocodes from Glottolog itself, then add the nullable field.

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

### 2.3.x — Typological Depth (P2) — COMPLETE

**The 2.3.x line is complete** — 2.3.0 (grammar expansion), 2.3.1 (classification)
and 2.3.2 (vitality and geography). The verified working data is in [typology-data.md](typology-data.md) — gathered and adversarially
verified, so 2.3.1 and 2.3.2 are encoding work rather than research work. Read that
file's caveats first.

The "Missing script entries" sub-item was listed here in error: all nine scripts
shipped at **2.1.6**.

### 2.4.0 — Gematria & Numeric Letter Values — COMPLETE

Shipped at 2.4.0. `NumericSystem` (standard / ordinal / reduced), `char_value`,
`string_value_in`, `script_alphabet_values`, `gematria_by_script`,
`gematria_all_scripts`, and 136 mapped characters over Hebrew, Arabic (Mashriqi),
Latin, Church Slavonic and Greek — Greek by reuse of `numerals_greek_isopsephy`,
not by copy. Ordinal and reduced are derived, never stored.

"Additive" was deliberately not added as a fourth `NumericSystem` value: it
describes how a system combines values across a string rather than a per-character
method, and that axis already exists as `NumeralSystemKind`.

The release also fixed two defects it uncovered in the pre-existing Greek table —
ordinals shifted from tau to omega by an inline final sigma, and the three missing
numeral-only signs that made `χξϛ` (666) uncomputable — by root-causing both to an
ordinal derived from a raw table index. See the CHANGELOG.

- [ ] **Cipher foundation** — character↔number round-trip for Caesar, Vigenère and
      substitution ciphers. Carried to a downstream consumer (a crypto Cyrius
      project, or sankhya); varna now supplies everything it needs. Note for
      whoever builds it: this table is ONE-based, while mod-26 cipher arithmetic
      is zero-based, and the 25-letter i/j merge belongs to Polybius-square
      ciphers, where a letter's number is a row/column pair rather than a value
      in this table at all.

### 2.4.x — Follow-on from the 2.4.0 verification (P3)

Surfaced by the independent reviews of 2.4.0 — one over the four alphabet tables,
one adversarial over the whole change. Every shipped value was confirmed correct
against canonical sources and is now pinned per letter, so none of these are
corrections.

- [ ] **Cache `script_alphabet_values`** — it is the only table builder in
      `numerals.cyr` without a `_c_` cache, and returns ~856 B of fresh bump
      memory per call for one of only 15 possible (script, method) results. The
      bytes are the function's product rather than a throwaway, so it is not the
      same defect class as the string-walk leak fixed in 2.4.0 — but the
      allocator never frees, so a caller in a loop still grows the heap without
      bound. Caching means callers share the pair records, which is already true
      of every other cached table here. Deferred deliberately: the aliasing change
      wants its own release, not a footnote in the one that introduced the API.

- [ ] **Variant-glyph alias map** — a decode-side layer that normalises a glyph to
      its canonical letter before lookup, so Church Slavonic izhitsa ѵ (400), ot ѿ
      (800), monograph uk ꙋ, and koppa ҁ (90, pre-1300) can be read without
      polluting the canonical tables. These must NOT go inline: the mappings vec is
      the ordinal source, so an inserted variant shifts every position after it —
      precisely the Greek final-sigma bug. Prerequisite for any early-period
      Cyrillic material.
- [ ] **Maghrebi abjad as a second Arabic table** — differs from the shipped
      Mashriqi reckoning on exactly six letters (sin, sad, shin, dad, zah, ghayn)
      and agrees on the other 22. Only worth building if North African or Andalusi
      sources enter the corpus.
- [ ] **Glagolitic numerals (`Glag`)** — a genuinely different system, numbering in
      its own alphabetical order (buky=2, zhivete=7, where Cyrillic gives both no
      value). A separate table, not an edit to the Cyrillic one.
- [ ] **Mispar gadol** — the Hebrew method giving the finals 500-900 instead of
      their base values. A fourth `NumericSystem`, and the first one that would
      need per-character data the standard table does not already carry.

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
