# Development Roadmap

> **Current**: 2.4.1 · **Open work only** — shipped releases are in
> [CHANGELOG.md](../../CHANGELOG.md) and the git tags.
>
> Tiers were numbered when the `2.2.0+` plan was drawn up against PHOIBLE, WALS,
> Glottolog, Unicode CLDR and the IPA spec. They are the intended sequence, not a
> commitment to ship in that order; `(P1)`–`(P4)` are priority. The pre-port
> `1.1.0`–`1.5.0` numbering was retired at 2.1.3.

## Data corrections

Accuracy items in the corpus data itself. None is a coding task — each needs a
decision about a language, and none should be changed silently mid-release.

How 2.4.1 handled its four is the standing method here: every value was researched
independently, then adversarially refuted, and **most proposed corrections did not
survive that**. A reference corpus is damaged as much by churn as by error, so a value
defensible under any mainstream description stays, and the variety it commits to is
named in the source comment instead of being left implicit.

### Data: ATR vowel systems

Yoruba and Wolof are both markable as of 2.4.1 (Wolof's four-quality vowel block was
completed to Ka/Unseth's 8 short + 7 long). **Somali is the one thing still gating the
`AdvancedTongueRoot` bit.**

- [ ] **Somali's vowel analysis** — a choice, not an omission. The corpus has Orwin's
      (1994) five qualities `/i e a o u/` + long, whose own wording is that "each of
      these **five** vowels has a fronted (ATR) variant". A ten-quality listing needs a
      symbol decision the sources do not agree on (ʉ̞ vs ʉ vs ʊ̈, ɵ̞ vs ɞ, a vs ɑ), and
      PHOIBLE marks `advancedTongueRoot` "−" on every Somali vowel in both its
      inventories. Pick an analysis and name it, or keep the five and say so.
- [ ] **Then the `AdvancedTongueRoot` bit.** Marking two of three languages would leave
      ATR silently absent from one that has it — worse than uniformly absent. The
      rationale is pinned in `tests/vowel_features.tcyr`; update it when this lands.

### Data: tone rows wanting a single source

Not wrong — under-sourced. 2.4.1 corrected the values that were wrong under every
description (Thai's missing falling tone, Vietnamese huyền, Burmese's checked tone) and
deliberately left the rest, but two rows now mix provenance.

- [ ] **Burmese** — sourced in one cell and unsourced in three. Creaky `˧ˀ` now sits two
      levels from checked, where the literature puts them nearly level. Re-source the
      whole row to one description: Wheatley (1987), `˥˥˦ / ˥˧ˀ / ˧˧˦ / ˥˧ʔ`, or
      Watkins (2001), `˦ / ˥˩ˀ / ˨ / ˥˩ʔ`.
- [ ] **Lao** — the corpus's weakest tone values. No source prints its `˩˧˥` verbatim
      and `˥˩` [51] is inside but not on the attested envelope (41/42/52/53). Both
      proposed replacements were checked and refuted — swapping `˩˧˥` deletes the ຂາ
      tone — so pass the row against Enfield (2007) or Osatananda rather than patching
      cells.

### Data: Glottocode support

Deferred out of 2.3.0 and **not** a coding task. Glottocodes are opaque identifiers
(`stan1293`, `nucl1301`) that can only be looked up. Generating them from memory would
produce plausible-looking fabrications indistinguishable from real ones, which is the
worst failure mode for a reference corpus.

- [ ] Obtain the 51 Glottocodes from Glottolog itself, then add the nullable field.

## Feature tiers

### Shipped tiers — 2.2.x, 2.3.x, 2.4.0, 2.4.1

Complete. Details in the [CHANGELOG](../../CHANGELOG.md); three decisions are recorded
there rather than here because they were rejections, and a reader who only sees the
feature list will propose them again:

- **2.2.0** rejected folding `Click`/`Implosive`/`Ejective` into `Manner`. They are
  airstream mechanisms, and doing so would have destroyed the manner of 19 of the 38
  affected phonemes. `Airstream` is a separate axis, as in PHOIBLE.
- **2.4.0** rejected an "Additive" fourth `NumericSystem` value. It describes how a
  system combines values across a string, not a per-character method, and that axis
  already exists as `NumeralSystemKind`.
- **2.4.1** rejected retranscribing Sanskrit's voiced aspirates to match its siblings,
  which is what this file used to call for. The split is legitimate — Sanskrit's
  inventory is a Pāṇinian varga listing where all ten mahāprāṇa are one class — and the
  inconsistency was fixed by asking (`phoneme_is_breathy`) rather than respelling.

The verified typological working data behind 2.3.x is in
[typology-data.md](typology-data.md); read its caveats before extending that work.

- [ ] **Cipher foundation** — carried out of 2.4.0 to a downstream consumer (a crypto
      Cyrius project, or sankhya); varna now supplies everything it needs. Note for
      whoever builds it: this table is ONE-based while mod-26 cipher arithmetic is
      zero-based, and the 25-letter i/j merge belongs to Polybius-square ciphers, where
      a letter's number is a row/column pair rather than a value in this table at all.

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
      polluting the canonical tables. These must NOT go inline — see
      [architecture note 001](../architecture/001-numeral-mappings-are-the-ordinal-source.md)
      for why. Prerequisite for any early-period Cyrillic material.
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
