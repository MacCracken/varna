# Development Roadmap

> **Current**: 2.5.4 · **Open work only.** Shipped releases are in
> [CHANGELOG.md](../../CHANGELOG.md) and the git tags.
>
> Releases are listed in their intended order, which is not a commitment to ship
> in that order. `(P0)`–`(P4)` are priority. A large item lands in small bites,
> each verified before the next (CLAUDE.md, *Task Sizing*). Reorganized at 2.5.1:
> the old tiers were renumbered around the serde gap, and `2.5.0 — Coverage Scale`
> became 2.6.0 because 2.5.0 shipped as a toolchain release.

| Release | Theme | Priority |
|---|---|---|
| 2.6.0 | Coverage scale: allophony, phonotactics, transliteration, numerals, provenance, PHOIBLE | P3 |
| 2.7.0 | Typology and interchange: PHOIBLE export, WALS codes, cross-cutting queries, ISO 639 | P4 |
| 2.8.0+ | Linguistic depth: prosody, morphology, historical phonology, sign language, Hangul | P4 |

## Data items: the method

Changes to the corpus data are not coding tasks. Each one is a decision about a
language, and none should change silently mid-release. 2.4.1 set the method: every
value is researched independently, then adversarially refuted, and **most proposed
corrections did not survive that**. A reference corpus is damaged as much by churn as
by error, so a value defensible under any mainstream description stays, and the
variety it commits to is named in the source comment. 2.5.3 added a third step:
**read every quote and value against the primary text**. Two of the corpus's
citations turned out to be Wikipedia's text credited to the primary source (Burmese
"Wheatley 1987", Somali "Orwin 1994"), and both had survived an adversarial review
that checked only whether the claim was plausible. JIPA Illustrations set IPA in
custom fonts, so render the page and look rather than trusting extracted text.

## Carried over from 2.5.4 (not yet assigned to a release)

- [ ] **Drop `_json_text_ok` once bayan refuses raw control bytes and invalid UTF-8.**
      RFC 8259 requires both; bayan's parser accepts both, so `_json_parse` scans for
      them itself, at about 8% of a read. The fix is bayan's, filed there as
      `docs/development/issues/2026-11-11-json-strings-accept-raw-control-bytes-and-invalid-utf8.md`.
      After a bayan release refuses them, bump `[deps.bayan]` and delete the scan.
- [ ] **Osatananda (1997, p. 40)** reportedly gives Lao 33 / 35 / 13 / 52 / 31, closer to
      Enfield's prose than Yanagimura's isolation values. That figure is known only
      from Wikipedia. The dissertation (UMI 9801455) is image-only and was not
      reachable at 2.5.3 or 2.5.4. Read it before preferring it.
- [x] **Re-check the source comments that predate 2.5.3.** Only 7 of the 48 inventories
      cite a dated source; the two not already re-checked, Thai and Vietnamese, were
      Wikipedia's tone columns too, and are re-sourced (Teeranon 2007; Kirby 2011). The
      four explicit WALS claims hold against WALS's CLDF data. The other 41 inventories
      cite nothing, which is the provenance item below.

## 2.6.0 — Coverage scale (P3)

- [x] **Expanded allophone rules**: Spanish, Japanese, Russian, Mandarin and Arabic,
      each from its JIPA Illustration, with only the rules whose environment the model
      states exactly. The inventories were corrected to the same sources.
- [ ] **An allophone environment model that can state the rules left out.** Each new
      set's comment lists rules that are real but not encoded, because the model has
      no "elsewhere" (Spanish spirantization, /ɟ͡ʝ/), no "same syllable" (Mandarin
      [a̟ ʊ] before a nasal coda), no two conditions at once (Japanese devoicing, [ᵈʑ]
      word-initially before /i/), and classes that are labels nothing evaluates
      (Russian voicing assimilation needs "voiced obstruent other than /v vʲ/").
- [x] **Expanded phonotactic profiles**: the 11 core languages plus Sanskrit, each
      re-sourced to one cited work, spelled in its inventory's symbols, and checked
      against that inventory by a test. `phonotactics_by_code` / `phonotactics_all_codes`.
- [ ] **French phonotactics: re-check against Walker (2001) or Tranel (1987).** The
      profile is partial: no reachable source states a template, so `max_coda 1` is a
      view Côté (2000) reports and argues against, and the onset list is his minimum
      (it misses vʁ, *vrai*).
- [ ] **German permitted codas.** Schäfer (2018, §5.2) states them as prose rules, so
      only his exclusions are encoded and every other coda reads unspecified. A source
      that enumerates them (Wiese 1996, which could not be read) would allow a list.
- [ ] **Portuguese inventory: one variety.** It mixes Brazilian /t͡ʃ d͡ʒ/ into an
      otherwise European set; the phonotactic profile is European (Mateus & d'Andrade
      1998). Decide the variety, or split it under *Multiple inventories per language*.
- [ ] **Korean inventory: /ʌ/ is missing.** Lee (1993, JIPA, p. 29) lists it; the
      inventory has i e ɛ a o u ɯ.
- [x] **Transliteration tables**: ALA-LC Russian, Arabic and Hebrew, and Zhuyin →
      Pinyin, with `translit_by_scheme`; Devanagari's inherent vowel and Greek's
      accented letters fixed; a source index for lookup; Bopomofo registered.
- [ ] **Word-level romanization rules.** Arabic's initial hamzah, construct-state tāʼ
      marbūṭah, the article al- and final -iyy; Hebrew's word-initial alef and vocal
      shva; ALA-LC capitalization; Pinyin's apostrophe and word division. Each needs
      word position, which the table model lacks; the same gap as the allophone
      environment model above.
- [ ] **Marks a table can skip while matching.** A Hebrew cantillation mark sorts
      between a vowel point and its vowel letter, so בְּרֵאשִׁ֖ית reads breʼshiyt.
- [ ] **Devanagari → IAST, re-sourced.** v1.x's table cites nothing, and lacks the vowel
      signs ॄ ॢ ॣ (their independent vowels are in it), nukta, avagraha and
      chandrabindu.
- [ ] **Greek → Beta Code: the 16 derived letters.** The diaeresis, macron and breve
      have no place in the TLG's stated order; they follow an analogy (the owner's
      choice) until a TLG statement turns up. Digamma and lunate sigma, which the
      TLG lists, are not in the table.
- [ ] **More ALA-LC tables**: the other Cyrillic languages (ALA-LC has one per
      language) and the Yiddish column of the Hebrew table.
- [x] **Numeral follow-ons**: the variant-glyph alias layer (ѵ ѿ ҁ; not ꙋ, which has
      no numeric use), Glagolitic numerals (`Glag`, the 29 values every source agrees
      on), and mispar gadol (`NUM_GADOL`, Hebrew only).
- [x] **Source provenance**: a keyed bibliography (`src/sources.cyr`) and each
      inventory's links to the works its values come from (`inventory_sources`);
      12 inventories cite 18 works, every entry checkable by DOI or URL.
- [ ] **Provenance for the other datasets**: link the phonotactic profiles, allophone
      sets, transliteration tables, tone rows and numeral systems to the bibliography,
      as the inventories are. Their comments cite about 50 more works, each to be
      entered from its primary record.
- [ ] **Re-source the 39 unsourced inventories**, JIPA Illustrations first, correcting
      each to its source as 2.6.0 did for ja es zh. Each may change data.
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
  them to 2.5.x is their own release work.
- **sankhya** folds varna 2.4.1.
- From 2.5.2, a consumer that wants JSON folds `dist/varna-json.cyr` instead. shabda,
  shabdakosh and sankhya already declare the bayan monolith, so they change only the
  `modules` line. This was simulated with scratch consumers, not run in those repos.
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
> all public types" entry was met at 2.5.2.
