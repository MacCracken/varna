# Development Roadmap

> **Current**: 2.5.1 · **Open work only.** Shipped releases are in
> [CHANGELOG.md](../../CHANGELOG.md) and the git tags.
>
> Releases are listed in their intended order, which is not a commitment to ship
> in that order. `(P0)`–`(P4)` are priority. A large item lands in small bites,
> each verified before the next (CLAUDE.md, *Task Sizing*). Reorganized at 2.5.1:
> the old tiers were renumbered around the serde gap, and `2.5.0 — Coverage Scale`
> became 2.6.0 because 2.5.0 shipped as a toolchain release.

| Release | Theme | Priority |
|---|---|---|
| **2.5.2** | bayan / serde integration: restore what the port dropped | **P0** |
| 2.5.3 | Data accuracy: Somali ATR, Burmese / Lao tone rows, Glottocodes | P2 |
| 2.5.4 | Hygiene and follow-ons: a numerals cache, layout-sensitive benchmarks, test-scoped deps, sakshi logging | P3 |
| 2.6.0 | Coverage scale: allophony, phonotactics, transliteration, numerals, provenance, PHOIBLE | P3 |
| 2.7.0 | Typology and interchange: PHOIBLE export, WALS codes, cross-cutting queries, ISO 639 | P4 |
| 2.8.0+ | Linguistic depth: prosody, morphology, historical phonology, sign language, Hangul | P4 |

## 2.5.2 — bayan / serde integration (P0)

**The port dropped serialization, and the docs said it hadn't.** The v1.x crate derived
`Serialize` / `Deserialize` on 44 public types (21 structs, 23 enums) across 13 modules, called `serde_json`
in 11, and round-tripped every public type in `tests/serde_roundtrip.rs` (27 tests).
The Cyrius port had none of it: no varna type emitted or read JSON, and the only JSON
in the tree was `src/mcp.cyr`'s hand-built output. The false claims were corrected at
2.5.1, and ADRs 0001 and 0002 carry amendments. 2.5.1's CHANGELOG has the full account.

**Settled at 2.5.1 (revisit only with a measurement).** Use bayan's thin JSON profile,
not the stdlib monolith: an optional `[deps.bayan]` with
`modules = ["dist/bayan-json.cyr"]`, on by default, at the tag the toolchain pin
vendors. The monolith's Unicode tables made the live binary 414,664 B against
44,408 B. Call the qualified `bayan_json_v_*` names.

**Decided: [ADR 0003](../adr/0003-json-wire-format-matches-v1x-serde.md)** — the v1.x
serde format, field for field, every enum by name. Each enum takes `#derive(Serialize)`
for its by-name codec (a struct derive cannot write an enum field by name, so record
writers / readers are hand-written around those codecs in `src/json.cyr`). Readers are
as strict as serde's defaults and accept v1.x documents.

**Done so far** (CHANGELOG `[Unreleased]`):
- ADR 0003.
- **phoneme**: `phoneme_*` and `phoneme_inventory_*` to / from JSON, with codecs on the
  eight phoneme enums.
- **phonology**: `allophone_rule_*`, `allophone_rule_set_*`, `syllable_template_*`,
  `phonotactic_constraint_*` and `phonotactics_*` to / from JSON, with codecs on
  `Environment`, `ConstraintKind` and `SyllablePosition`.
- **`Before` / `After(PhonemeClass)`**: ported as `EnvBefore` / `EnvAfter` and
  `PhonemeClass`, with JSON. See ADR 0003's amendment for the hand-written names.
- **script**: `script_*`, `numeral_mapping_*`, `numeral_system_*` and
  `transliteration_table_*` to / from JSON, with codecs on `ScriptType`, `Direction`,
  `ScriptStatus`, `NumeralSystemKind` and `LetterRole`.
- `tests/json_roundtrip.tcyr`: 16 of `serde_roundtrip.rs`'s 27 tests ported, plus exact
  text, v1.x input and malformed input (mutation-checked).
- Benchmarks: inventory, phonotactics, script, numerals, transliteration.

Tone has no bite of its own: v1.x had no Tone type (tones were strings), and
inventories already write them as strings.

**Next session starts here.** The pattern is fixed: `#derive(Serialize)` on each enum,
a hand-written writer / reader pair per record in `src/json.cyr` in v1.x field order,
equality helpers, the ported tests, then exact-text and malformed cases in
`tests/json_roundtrip.tcyr`, a mutation check, and a bench pair. Read the v1.x types
with `git show 2.1.3:rust-old/src/<path>.rs`. Watch for **enum variant names
colliding across enums**: Cyrius enum constants are global, which is what blocked
`PhonemeClass`. When a mutation check reports no summary, the test crashed: that is not
a kill, so run an unmutated control first.

Remaining bites, in order:

- [ ] **grammar, lexicon** (with cognate and swadesh), **dialect, registry**.
- [ ] **Malformed input**: `adversarial.rs`'s cases against every decoder.
- [ ] **`-D MCP` / `-D DAIMON` / `-D HOOSH` onto the serializers**: replace
      `src/mcp.cyr`'s hand-built JSON, and encode daimon's registration payload.
      bote-core stays a commented recipe in `cyrius.cyml`; wiring it is not in scope.
- [ ] **Consumer hazard, before the serializers enter `dist/varna.cyr`.** shabda and
      shabdakosh declare the `bayan` monolith in their own `[deps].stdlib`, and
      vendoring it beside `bayan-json` defines 27 JSON symbols twice. Either keep
      `[deps.bayan]` optional so consumers opt in (the ai-hwaccel pattern), or move
      those two to the profile in step. Re-check `dist/varna.deps` and the
      `# Requires` block after each `distlib`.
- [ ] **Benchmarks** for each module's serializers as it lands (the inventory pair is in).
- [ ] **Docs**: write the real API into the README, the overview and the usage guide,
      and close ADR 0001's amendment.

## 2.5.3 — Data accuracy (P2)

Accuracy items in the corpus data. None is a coding task: each needs a decision about a
language, and none should change silently mid-release. 2.4.1 set the method. Every
value is researched independently, then adversarially refuted, and **most proposed
corrections did not survive that**. A reference corpus is damaged as much by churn as
by error, so a value defensible under any mainstream description stays, and the
variety it commits to is named in the source comment.

- [ ] **Somali's vowel analysis**, a choice rather than an omission. This is the one
      thing still gating the `AdvancedTongueRoot` bit; Yoruba and Wolof are both
      markable since 2.4.1. The corpus has Orwin's (1994) five qualities `/i e a o u/`
      + long, whose own wording is that "each of these **five** vowels has a fronted
      (ATR) variant". A ten-quality listing needs a symbol decision the sources do not
      agree on (ʉ̞ vs ʉ vs ʊ̈, ɵ̞ vs ɞ, a vs ɑ), and PHOIBLE marks `advancedTongueRoot`
      "−" on every Somali vowel in both of its inventories. Pick an analysis and name
      it, or keep the five and say so.
- [ ] **Then the `AdvancedTongueRoot` bit.** Marking two of the three languages would
      leave ATR silently absent from one that has it, which is worse than uniformly
      absent. The rationale is pinned in `tests/vowel_features.tcyr`; update it when
      this lands.
- [ ] **Burmese tone row**: sourced in one cell and unsourced in three. Creaky `˧ˀ` sits
      two levels from checked, where the literature puts them nearly level. Re-source
      the whole row to one description: Wheatley (1987), `˥˥˦ / ˥˧ˀ / ˧˧˦ / ˥˧ʔ`, or
      Watkins (2001), `˦ / ˥˩ˀ / ˨ / ˥˩ʔ`.
- [ ] **Lao tone row**: the corpus's weakest tone values. No source prints its `˩˧˥`
      verbatim, and `˥˩` [51] is inside the attested envelope (41/42/52/53) but not on
      it. Both proposed replacements were checked and refuted, since swapping `˩˧˥`
      deletes the ຂາ tone. Check the whole row against Enfield (2007) or Osatananda
      rather than patching cells.
- [ ] **Glottocodes** (deferred from 2.3.0). They are opaque identifiers (`stan1293`)
      that can only be looked up, and generating them from memory would produce
      plausible fabrications. Obtain the 51 from Glottolog itself, then add the
      nullable field.

## 2.5.4 — Hygiene and follow-ons (P3)

- [ ] **Cache `script_alphabet_values`** (from the 2.4.0 review). It is the only table
      builder in `numerals.cyr` without a `_c_` cache, and it returns ~856 B of fresh
      bump memory per call for one of only 15 possible (script, method) results, so a
      caller in a loop grows the heap without bound. Callers would then share the pair
      records, as every other cached table already does. That aliasing change gets its
      own release note.
- [ ] **Microbenchmarks move with code layout.** `grammar_by_code_lookup` read 209 →
      ~241 ns across three toolchain pins, then fell back to ~213 ns when 2.5.2's JSON
      code was added elsewhere; the same change moved `lexicon_find_word` +10.5%. None of
      those functions changed. Find the mechanism (fn / loop alignment, which the cyrius
      backend may not do) before reading a row's movement as a code change; until then,
      interleave A/B against the previous build for any row that moves, as 2.4.3–2.5.2
      did. If alignment is the cause, file it with cyrius, with the repro.
- [ ] **Move `assert` and `bench` to `[test] stdlib`** (cyrius 6.7.6+), so they stop
      being prepended to the library build. Measure the binary before and after.
- [ ] **`-D LOGGING` through sakshi.** `src/logging.cyr` is a level-gated stderr logger;
      the `VARNA_LOG` filter and the sakshi audit trail CLAUDE.md calls for are still
      deferred.

## 2.6.0 — Coverage scale (P3)

- [ ] **Expanded allophone rules**: Mandarin, Spanish, Japanese, Russian, Arabic
      (`allophone_english` is the only rule set).
- [ ] **Expanded phonotactic profiles**: all core languages (3 today: English,
      Sanskrit, Japanese).
- [ ] **Transliteration tables**: Cyrillic–Latin, Arabic–Latin, Hebrew–Latin, Pinyin
      (2 today: Devanagari–IAST, Greek–Beta Code).
- [ ] **Numeral follow-ons** from the 2.4.0 review. None is a correction, since every
      shipped value is pinned per letter.
  - **Variant-glyph alias map**: a decode-side layer that normalises a glyph to its
    canonical letter before lookup, so Church Slavonic izhitsa ѵ (400), ot ѿ (800),
    monograph uk ꙋ and koppa ҁ (90, pre-1300) can be read without polluting the
    canonical tables. These must not go inline; see
    [architecture note 001](../architecture/001-numeral-mappings-are-the-ordinal-source.md).
    It is a prerequisite for any early-period Cyrillic material.
  - **Glagolitic numerals (`Glag`)**: a genuinely different system, numbered in its
    own alphabetical order (buky=2, zhivete=7, where Cyrillic gives both no value). A
    separate table, not an edit to the Cyrillic one.
  - **Mispar gadol**: the Hebrew method that gives the finals 500–900 instead of their
    base values. A fourth `NumericSystem`, and the first that needs per-character data
    the standard table does not already carry.
- [ ] **Source provenance**: track the bibliography / reference for each inventory.
      Today the source is named only in comments, as the 2.5.3 method requires.
- [ ] **Multiple inventories per language**: competing analyses, as PHOIBLE keeps them.
      `dialect` overlays (British English RP) are the nearest thing today.
- [ ] **Data-driven inventories**: load PHOIBLE for 2000+ languages, feature-gated.
      It needs 2.5.2's JSON for the JSON form, and a CSV load would take bayan's
      `bayan-csv` profile beside `bayan-json`.

## 2.7.0 — Typology and interchange (P4)

The verified typological working data behind 2.3.x is in
[typology-data.md](typology-data.md); read its caveats before extending that work.

- [ ] **PHOIBLE-compatible export**, built on 2.5.2's serializers.
- [ ] **WALS feature-code mapping**: tie the eight WALS-style `GrammarProfile`
      dimensions (2.3.x) to their WALS feature codes.
- [ ] **Cross-cutting typological queries** ("all SOV languages with ejectives").
      Single-axis filters exist (`registry_by_family`, `registry_by_macroarea`); this is
      the combination across grammar, phonology and the registry.
- [ ] **ISO 639 validation**: a static table of valid codes beyond the 51 in the corpus.

## 2.8.0+ — Linguistic depth (P4)

- [ ] Prosody patterns: intonation contours, and rhythm class (stress-, syllable- or
      mora-timed).
- [ ] Morphological analyzer: stemming and lemmatization per language.
- [ ] Historical phonology: sound-change rules, Proto-Indo-European reconstructions.
- [ ] Sign language phonology: handshape, location and movement features.
- [ ] `ScriptType.Featural`, to reclassify Hangul.

## Conditional

- [ ] **Maghrebi abjad as a second Arabic table.** It differs from the shipped Mashriqi
      reckoning on exactly six letters (sin, sad, shin, dad, zah, ghayn) and agrees on
      the other 22. Only worth building if North African or Andalusi sources enter
      the corpus.

## Downstream

Consumers checked at 2.5.1, from their manifests in `~/Repos`:

- **shabda** and **shabdakosh** fold `dist/varna.cyr` at varna **2.4.1** (`[deps.varna]`),
  which meets the old v1.0 criterion "shabda + shabdakosh consuming varna". Moving
  them to 2.5.x is their own release work, and 2.5.2's consumer hazard concerns these
  two.
- **sankhya** folds varna 2.4.1.
- **svara** is a named consumer with no `[deps.varna]` yet. **jnana** is not checked
  out here, so its status was not checked.
- **vidya** is not a consumer. It would need varna only if it gains speech support for
  the programming-language codex.
- [ ] **Cipher foundation** (carried out of 2.4.0): belongs to a downstream consumer, a
      crypto Cyrius project or sankhya, and varna already supplies everything it needs.
      Note for whoever builds it: varna's letter table is ONE-based while mod-26 cipher
      arithmetic is zero-based. The 25-letter i/j merge belongs to Polybius-square
      ciphers, where a letter's number is a row/column pair rather than a value in this
      table.

## Decided against — do not re-propose

These were rejections, so a reader who only sees a feature list will propose them
again. The CHANGELOG entries have the full reasoning.

- **2.2.0**: folding `Click` / `Implosive` / `Ejective` into `Manner`. They are
  airstream mechanisms, and folding them would have destroyed the manner of 19 of the
  38 affected phonemes. `Airstream` is a separate axis, as in PHOIBLE.
- **2.4.0**: an "Additive" fourth `NumericSystem` value. It describes how a system
  combines values across a string, not a per-character method, and that axis already
  exists as `NumeralSystemKind`.
- **2.4.1**: retranscribing Sanskrit's voiced aspirates to match its siblings. The split
  is legitimate: Sanskrit's inventory is a Pāṇinian varga listing in which all ten
  mahāprāṇa are one class. The inconsistency was fixed by asking (`phoneme_is_breathy`)
  rather than respelling.
- **2.5.1**: dropping bayan because nothing called it. Nothing called it because
  serialization was never ported. The fix is 2.5.2, on bayan's thin JSON profile.

> The v1.0 criteria checklist was retired at 2.1.3. Its "full JSON roundtrip tests for
> all public types" entry is 2.5.2's work.
