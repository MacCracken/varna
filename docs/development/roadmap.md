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

## Data corrections

Data-accuracy items surfaced by the 2.2.x and 2.3.x work. **Three of the four were
taken at 2.4.1**; Glottocodes remain open because they need a source, not a decision.
The two tone rows still listed below are under-sourced rather than wrong.

Every value 2.4.1 touched was checked by independent research and then adversarially
refuted, and most proposed changes did not survive that: of the corrections offered,
only four were made. That ratio is the point of the section — a reference corpus is
damaged as much by churn as by error, so a value that is defensible under any
mainstream description stays, and the variety it commits to gets named in the source
comment instead.

### Data: ATR vowel systems

Deferred out of 2.2.2, **partly closed at 2.4.1**. `atr` was named there, but no corpus
segment is transcribed with an ATR/RTR diacritic, and the languages that need the
contrast had incomplete vowel systems — so it is a data item, not a feature bit.

- [x] **Wolof** — closed at 2.4.1. Was /i e a o/ (+ long), a four-quality system that
      matches no analysis and was byte-identical to the Classical Nahuatl block. Now
      the full 8 short + 7 long of Ka (1994) / Unseth (2009): /i u e o ɛ ɔ ə a/, with
      the ATR pairing recorded in the source comment.
- [x] **Yoruba** — /i e ɛ a ɔ o u/ was already the classic 7-vowel ATR set and needs
      no segments added. /e o/ are +ATR, /ɛ ɔ/ −ATR, /a/ −ATR, /i u/ +ATR
      non-contrastively (there are no −ATR high vowels in Standard Yoruba).
- [ ] **Somali** — the one remaining gap, and it is a genuine choice rather than an
      omission. The corpus has Orwin's (1994) five qualities /i e a o u/ + long, which
      is a named analysis, not a truncation; Orwin's own wording is that "each of these
      **five** vowels has a fronted (ATR) variant". A ten-quality listing needs a
      decision on symbols that sources do not agree on (ʉ̞ vs ʉ vs ʊ̈, ɵ̞ vs ɞ, a vs ɑ),
      and PHOIBLE marks `advancedTongueRoot` "−" on every Somali vowel in both its
      inventories. Pick an analysis and name it, or leave the five and say so.
- [ ] Then add the `AdvancedTongueRoot` bit. **Still gated, and the gate still holds**:
      Yoruba and Wolof are now markable but Somali is not, and marking two of three
      would leave ATR silently absent from a language that has it. Somali first.

### Data: Thai and Vietnamese tone transcription

**Closed at 2.4.1** — but not as originally diagnosed, and the original diagnosis was
actively misleading. It read: "Both languages list a `˨˩˦` tone, which is the shape of
*Mandarin's* third tone; the same wrong value in two unrelated languages points at a
copy-paste." That inference was wrong. `˨˩˦` [214] is independently correct for Thai's
rising tone จัตวา *and* for Mandarin's third tone — both languages genuinely have a
concave low rise — so its appearing in both is evidence of nothing. **Acting on the
old text literally would have deleted Thai's one correct contour value.** Kept here
as a record of how a plausible cross-language inference can be exactly backwards.

- [x] **Thai** — the real defect was narrower and worse: there was no falling tone at
      all, `˨˩˦` sat in the falling slot, and slot 5 held `˩˧˥`, a contour no
      description of Thai contains. Now `˧ ˨˩ ˦˩ ˦˥ ˨˩˦`, all Tingsabadh & Abramson
      (1993). Falling is `˦˩` [41], theirs, not the `˥˩` [51] this roadmap used to
      name — that is Gandour's, and mixing traditions inside one row is what produced
      the defect.
- [x] **Vietnamese** — huyền was `˨˩˦`. Here the copy-paste diagnosis *does* hold: it
      is a low falling tone, `˨˩`, and no dialect gives it a dipping contour.
- [x] **The other seven toned inventories re-checked.** One more real error: Burmese's
      checked tone was `˩ʔ`, a low level, where every description gives a high onset —
      it was the corpus's generic checked placeholder. Now `˥˧ʔ`. Everything else was
      confirmed and deliberately left alone: Lao's six (including its `˩˧˥`, which is
      its genuine low-onset rise), Mandarin's neutral `˧`, Literary Chinese's four,
      Vietnamese nặng, Hausa and Somali as two-level systems, Burmese low `˨˩`.

- [ ] **Burmese wants a single-source pass.** 2.4.1 fixed the one cell that was wrong
      under every source, which leaves the row sourced in one cell and unsourced in
      three — creaky `˧ˀ` now sits two levels from checked where the literature puts
      them nearly level. Re-source the whole row to one description: Wheatley (1987),
      `˥˥˦ / ˥˧ˀ / ˧˧˦ / ˥˧ʔ`, or Watkins (2001), `˦ / ˥˩ˀ / ˨ / ˥˩ʔ`.
- [ ] **Lao wants the same.** Its values are the corpus's weakest: no source prints
      [135] verbatim and `˥˩` [51] is inside but not on the attested envelope
      (41/42/52/53). Both proposed replacements were checked and refuted — swapping
      `˩˧˥` deletes the ຂາ tone — so the row is under-sourced, not wrong. Pass it
      against Enfield (2007) or Osatananda.

### Data: Sanskrit voiced-aspirate transcription

**Closed at 2.4.1 by deciding NOT to change the data.** The item read "Decide one
transcription and apply it"; the decision is that the split is legitimate and both
sides stay.

- [x] Sanskrit keeps /ɡʰ d͡ʑʰ ɖʰ d̪ʰ bʰ/ with U+02B0. Its inventory is a Pāṇinian varga
      listing — the comments are *Kavarga, Chavarga, Tavarga, Pavarga* — under which
      all ten mahāprāṇa are one natural class, the class Grassmann's Law and
      Bartholomae's Law operate over. ⟨bʰ⟩ on a voiced base is the IPA chart's own
      example for the diacritic, read as murmured release. Hindi, Bengali and Urdu
      keep U+02B1, which is their literature's convention. Retranscribing either side
      would break `phoneme_find` for consumers to fix a query that has a correct
      answer without it.
- [x] The premise that motivated the item was also wrong: the two spellings do **not**
      produce different distinctive-feature vectors. `src/features.cyr` derives
      `DF_SPREAD_GLOTTIS` from `Aspirated` and `Breathy` alike, so Sanskrit /ɡʰ/ and
      Hindi /ɡʱ/ were already bit-identical there. The real inconsistency was one
      level up, at the raw `PhonemeFeature` bit.
- [x] Fixed by asking instead of spelling: `phoneme_is_breathy(p)` is true for
      `Breathy`, or for any **voiced** segment marked `Aspirated`. "Which segments are
      breathy?" now agrees across all four languages — 5 each — while each keeps its
      own transcription. The voicing test is load-bearing and pinned: /kʰ/ is not
      breathy in Sanskrit, Hindi, or Zulu.

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
