# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [2.4.0] - 2026-08-28

Gematria and numeric letter values: a unified character-to-number layer across five
scripts, and the fix for two defects it uncovered in the Greek table that has been
shipping since the port. Additive, plus a large speedup on numeral lookup.

### Added

- **numerals** — `NumericSystem` (`NUM_STANDARD` / `NUM_ORDINAL` / `NUM_REDUCED`)
  and a unified lookup across scripts: `char_value(script, system, ch)`,
  `string_value_in(script, system, s)`, `script_alphabet_values(script, system)`,
  `gematria_by_script(code)` and `gematria_all_scripts()`. 136 mapped characters
  over Hebrew, Arabic, Latin, Church Slavonic and Greek.

- **Four alphabet tables** — Hebrew mispar hechrachi (22 letters + 5 finals),
  Arabic abjad in **Mashriqi** reckoning (28, abjadi order, not hija'i), Latin
  simple gematria (a=1..z=26), and Church Slavonic (27, standardised recension).
  Greek **reuses** `numerals_greek_isopsephy` rather than copying it — a test
  asserts the identity, so the two can never drift.

- Only `NUM_STANDARD` is stored. Ordinal is derived from alphabet position and
  reduced is the standard value's digital root, so neither can drift from the
  values they describe. The roadmap also listed an "Additive" `NumericSystem`
  value; it was **not added**, because it describes how a system combines values
  across a string rather than a per-character method, and that axis already
  exists as `NumeralSystemKind`.

- **`LetterRole`** on every `NumeralMapping` — `ROLE_LETTER`, `ROLE_ALLOGRAPH`
  (a positional variant such as Greek ς or the Hebrew finals) and
  `ROLE_NUMERAL_SIGN` (a value-bearing sign that is not a letter). Accessors
  `numerals_mapping_char` / `_len` / `_value` / `_role` / `_base`.

- **tests/gematria.tcyr** — 265 assertions. Every one of the 136 values is pinned
  individually against the canonical sources, because a structural sweep cannot
  see two letters swapping values: the multiset is unchanged and every rung still
  resolves. Plus the derivation rules, the absences, the Greek reuse identity,
  the accessors at each role, and a corpus sweep that pins each alphabet's size
  as its **maximum** position rather than its last letter's.

- **An allocation assertion**, not just a benchmark — `string_value_in` and
  `char_value` are measured with `alloc_used()` deltas over 1,000 calls and must
  come back at exactly 0 bytes on the success path, the unmapped-character path,
  and the rescanning ordinal path. Assertions cannot see a leak and this release
  shipped one during development (below); a measurement is the only thing that
  catches it.

- **Four benchmark rows** for the new surface — `gematria_char_value`,
  `gematria_char_value_ordinal`, `gematria_string_value_in` and
  `gematria_alphabet_table`. None of 2.4.0's public API had a row, so the
  benchmark gate could not have seen a regression in it.

### Fixed

- **Greek ordinals were shifted from tau to omega.** Final sigma sat inline in
  the isopsephy table, and the ordinal was derived from a mapping's raw vec
  index — so ς claimed a position of its own and every letter after it read one
  too high (τ=20, ω=25). ς is a positional allograph of σ, not a 25th letter.
  Now τ=19 and ω=24, and ς folds to σ's position.

- **Greek could not compute 666.** The letter run skips 6, 90 and 900 because
  those values belong to three numeral-only signs — stigma ϛ, koppa ϟ, sampi ϡ —
  which were absent entirely, so `χξϛ` returned the unmapped sentinel. All three
  are now mapped as `ROLE_NUMERAL_SIGN`, giving a complete 1..900 run.

- **The root cause of both**, rather than the two symptoms: deriving an ordinal
  from a raw table index assumes every entry occupies an alphabet position.
  `_ordinal_at` now counts only `ROLE_LETTER` entries. A consequence is that the
  Hebrew finals fold to their base letter's position (final mem is 13, like mem),
  which is what classical mispar siduri does; they previously read 23-27.

- **The architecture overview's numeral table listed systems that do not exist**,
  predating this release: "Arabic-Indic Digits" and "Egyptian Fractions" were never
  in the code (`Arab` is the abjad; `Egyp` is Hieroglyphic Numerals), and Chinese
  rod numerals were missing entirely. Corrected against the constructors, and the
  usage guide's new gematria examples are executed as assertions rather than
  written by hand.

- **A per-character allocation leak in `string_value_in`**, introduced earlier in
  this release and caught before it shipped. It cut a NUL-terminated copy of every
  codepoint purely to call `char_value`, burning 8 B per character per call — on
  the failure path too, since the copies were made before the unmapped character
  was reached. The bump allocator never frees, so that is permanent, unbounded in
  string length, and the exact pattern `_numerals_value_of_bytes` already existed
  to remove. `_char_value_bytes` now compares in place, and `string_value_in`
  resolves the script once instead of once per character.

- **`_ordinal_at` would have dereferenced a null base.** No constructor produces
  an allograph without one, but 0 is the not-found sentinel in that slot and
  `strlen` would have followed it. Guarded.

- **`script_alphabet_values` allocated before checking its sentinel**, leaking a
  vec on every not-found call.

- **Church Slavonic rationale was wrong in a way that would propagate.** The
  comment claimed only letters with a Greek ancestor carry a value. Its own table
  refutes that: ц descends from Hebrew tsade and carries 900, and ч is Slavic and
  carries 90. The rule is positional — values follow the Greek numeral *sequence*
  slots, including those held by stigma, koppa and sampi. The wrong rule had been
  mirrored into the tests, so it is corrected in both places and both
  counterexamples are now asserted. No value changed.

### Changed

- `NumeralMapping` grew 16 B → 40 B: `[0]=char [8]=len [16]=value [24]=role
  [32]=base`. All five fields have accessors; nothing outside `numerals.cyr`
  decodes the record by offset any more.

- The Arabic and Church Slavonic records now **name their recension**
  ("Arabic Abjad (Mashriqi)", "Church Slavonic Numerals (standardised
  recension)"). "Abjadi" alone is ambiguous: the Maghrebi tradition moves exactly
  six letters, and scoring a Maghrebi source against a Mashriqi table is silently
  wrong on those six and only those. Documented, not encoded — variant glyphs and
  period alternatives (izhitsa, ot, koppa-for-90, Glagolitic) stay out of the
  canonical tables, because the mappings vec is also the ordinal source and an
  inline variant corrupts every position after it. That is not hypothetical: it
  is exactly what final sigma did to Greek.

### Performance

- **Numeral lookup is ~40% faster.** The scan called `strlen()` on each stored
  character once per entry per lookup — recomputing a constant. The byte length
  is now cached at construction, so the scan compares an integer before touching
  memory. Measured over 4 interleaved rounds, non-overlapping spreads:
  - `numeral_value_of_char` 279ns → 168ns (**-39.8%**), spreads 279-295 / 168-179
  - `numeral_string_value_word` 892ns → 555ns (**-37.8%**), spreads 892-939 / 555-588

  This is a net win despite the Greek table growing by three entries, which adds
  a comparison to every lookup that scans past stigma or koppa.

- **The string walk is 25.8% faster** than the leaking version it replaced, since
  removing the per-character copy also collapsed a per-character script lookup to
  once per string. Interleaved, 3 rounds, non-overlapping spreads:
  `gematria_string_value_in` 1012ns → 751ns, spreads 1012-1049 / 751-764.
  `gematria_char_value` is unchanged at 262-266ns, as expected — its path did not
  move.

- **The ordinal table build is no longer quadratic.** `script_alphabet_values`
  called `_ordinal_at`, itself a full rescan, once per entry. Letters now take a
  running counter and only allographs and numeral signs — at most a handful per
  table — still rescan.

## [2.3.2] - 2026-08-28

Final slice of the 2.3.x Typological Depth line: vitality and geography on every
registry entry. Also bumps the Cyrius pin to 6.5.36. Additive.

### Added

- **registry** — `EndangermentLevel`, `MacroArea`, and latitude/longitude on all 51
  languages, with `registry_info_endangerment` / `_macroarea` /
  `_latitude_mdeg` / `_longitude_mdeg`, plus `registry_by_macroarea(area)` and
  `registry_at_least_endangered(level)`.

- **`END_HISTORICAL`**, an eighth vitality value: no native-speaker community, but
  the language remains in active scholarly, liturgical or literary use. Latin,
  Ancient Greek, Literary Chinese, Sanskrit and Classical Nahuatl take it.
  Collapsing those into `END_EXTINCT` would conflate them with a minority language
  whose last speakers died. **No corpus language is `END_EXTINCT`**, which a test
  pins so the distinction stays visible. A threshold query deliberately excludes
  the historical languages rather than sweeping them in, since `END_HISTORICAL`
  sorts last numerically but is not "worse than extinct".

- **tests/vitality.tcyr** (65 assertions), including three checks a per-row review
  cannot do: every coordinate is validated against its own macro-area's bounding
  box, every coordinate must be a multiple of 100 mdeg so the storage cannot
  outrun the one-decimal source, and `ko` is pinned present by name.

### Fixed

Four vitality values, from an audit aimed at the low-confidence and
indigenous-language rows:

- **`haw`** Moribund → **Threatened**. Moribund means only the grandparent
  generation speaks it, which is false: Ni'ihau transmission has never broken and
  the Pūnana Leo / Kula Kaiapuni immersion system has run since 1983. The old value
  traces to UNESCO's Atlas, **frozen in 2010, before the revitalization**.
- **`qu`** Shifting → **Vulnerable**. Downstream of varna's own 2.3.1 rename: the
  entry was scoped to *Southern Quechua* (~5–7M speakers, intact rural
  transmission, co-official) while the level still described the whole
  macrolanguage. Shifting is right for Central and northern Quechua, which the
  entry excludes by name.
- **`nah`** Shifting → **`END_HISTORICAL`**. Same cause. The 2.3.1 rename to
  *Classical Nahuatl* was correct — the inventory has no voiced stops, the
  four-vowel system with contrastive length, /t͡ɬ/, /kʷ/ and the saltillo — so it
  was the level that described a different language.
- **`ko`** was missing entirely from the gathered data (50 of 51 rows) and is
  sourced here: Safe, Eurasia, ~80M speakers, official in both Koreas.

- **docs/development/typology-data.md** synced to the code; it had drifted on all
  four historical languages and omitted `ko`.

### Changed

- **Toolchain** — Cyrius pin `6.5.35` → **`6.5.36`**. The 6.5.35 stdlib snapshot had
  been rewritten in place locally (`io.cyr` plus four `syscalls_*` files in varna's
  closure), which `cyrius deps --verify` caught in the gate — the check added at
  2.1.1 for exactly this. The lock now matches a version that actually changed:
  29 verified, 0 failed. The five files in the re-lock are precisely those five.
- **LanguageInfo** is 80 bytes, up from 48.
- Every row whose ISO code is broader than the entry (`ar`, `zh`, `gn`, `nah`, `qu`)
  now carries an explicit SCOPE comment naming the variety its values describe.

### Notes

- **Correction to the 2.3.2 working notes: Cyrius does have f64.** An earlier
  comment in this work claimed it did not, and justified the integer coordinate
  encoding on that basis. That was wrong — `lib/math.cyr` and `lib/ganita.cyr`
  carry a full float surface (`f64_pow`, `f64_hypot`, trig, `f64v2`/`f64v4` SIMD)
  and bayan parses JSON floats. The claim came from misreading a design note in
  `hoosh.cyr` ("per-mille … to avoid f64") as a statement about the language.

  Milli-degrees are **kept**, on the real reasons: the source is rounded to one
  decimal degree so a float would advertise precision the data lacks, and integers
  compare exactly so the bounding-box and precision tests need no epsilon. The
  comments now say that instead.

- **The failure pattern behind three of the four fixes** is one thing: a level
  assigned to a different language than the entry names. A macrolanguage or
  collection ISO code paired with a single vitality value is structurally risky,
  which is why the SCOPE comments and their tests exist.

### Performance

- **Flat.** `registry_all_codes_iter` -0.3%, `registry_phonemes_lookup` 0.0%. Two
  rows tripped a 10% threshold and both are min-of-N artifacts at the harness floor
  (`english_phoneme_inventory` 5→3 ns on a new spread of 3-5; `greek` 5→6 on 5-6).


## [2.3.1] - 2026-08-28

Second slice of the 2.3.x Typological Depth line: genealogical classification on
every registry entry. Additive — the three original `LanguageInfo` fields and
their accessors are untouched.

### Added

- **registry** — `family`, `subfamily` and `genus` on all 51 languages, with
  `registry_info_family` / `_subfamily` / `_genus`, the `registry_family(code)`
  shortcut, and `registry_by_family(family)` returning every code in a family.
  An entry built without a classification reads back `0`, which is
  distinguishable from any placeholder string.

- **registry** — a written **rank rule** in the module header, which is what the
  file lacked and what let the defects below through: family is the conventional
  top-level stock, subfamily is always a *primary* (depth-1) branch, genus is the
  smallest conventionally named clade the language's close relatives still share.
  Two consequences are documented as correct rather than left to look like
  oversights: genus depth varies between families because family diversity does,
  and `genus == subfamily` is the right answer for a low-diversity family (seven
  rows do this). `la` is recorded as the one intentional exception to non-nesting.

- **tests/classification.tcyr** (176 assertions), including two groups that exist
  because of the audit: `non_nesting`, which checks the invariant a rank column
  must satisfy — no value may be a proper ancestor of another in the same column —
  and `subfamily_is_always_a_primary_branch`. Plus `rejected_groupings`, scanning
  all three fields of all 51 rows for **Altaic, Ural-Altaic, Nostratic,
  Hamito-Semitic, Mon-Khmer and Quechumaran**.

### Fixed

Three **partition violations**, found by a systematic audit after the row-by-row
review had passed them, and fixed before release:

- **`ar`** genus was `Central Semitic`, which strictly contains `he`'s
  `Northwest Semitic` — so grouping by genus asserted Hebrew sits outside Central
  Semitic. Now `Arabic`, the actual sister node.
- **`sa`** genus was `Old Indo-Aryan`, a stratum *inside* the `Indo-Aryan` that
  `hi`/`ur`/`bn` carry, not a sister of it. Now `Indo-Aryan`, with the stage in a
  comment — the same treatment `grc` and `lzh` already had. This value also
  contradicted the table it was derived from, which had the stage as a
  parenthetical.
- **`haw`** subfamily was `Oceanic`, four nodes inside the `Malayo-Polynesian`
  that `id` and `tl` carry. It was the only row in all 51 whose subfamily was not
  a primary branch.

- **tests/classification.tcyr** had *locked one of these in*: it asserted
  "Sanskrit and Hindi are different genera", which is only true while the nesting
  bug exists. Inverted to `_same_genus("sa", "hi")`.

### Changed

- **LanguageInfo** is 48 bytes, up from 24.
- **`qu`** is named *Southern Quechua* and **`nah`** *Classical Nahuatl*, in both
  the registry and the inventory. Both ISO codes have wider scope than the entry
  describes — `qu` is a macrolanguage spanning both Quechua branches and `nah` a
  639-3 collection code — so the previous names made the subfamily false for part
  of that scope. Naming the entry for the variety it actually models is the
  pattern `zh` already used. The codes are untouched.
- Comment pass on the four Niger-Congo rows, which asserted `Niger-Congo` in the
  field while using `Atlantic-Congo` in their own prose, and on `am`, whose
  "South Semitic" node is superseded. Fields unchanged — only the marked set is
  now complete rather than arbitrary.

### Notes

- **Caveats live in comments, not in fields.** The source table carried values like
  `Ugric (contested; Glottolog rejects the node)` — prose inside a queryable field.
  The encoded value is the clean conventional label with the objection beside it.
  Four subfamilies are contested this way (`hu`, `wo`, `my`, `qu`), each pinned by a
  test so the choice stays visible in code.

- **The audit is why this release is correct.** Four independent lenses over the
  whole table at once — granularity, cross-row coherence, rejected nodes,
  macrolanguage treatment — after a row-by-row review had returned **zero
  disputes**. Every one of the three bad values was individually defensible, which
  is exactly why per-row review could not see them: the defect was only visible in
  the column read as a whole. The reintroduced-defect check confirms the new
  invariant catches all three (9 assertions fail, including 5 genus nestings and 2
  subfamily nestings from the generic scan).

- **Re-levelling the genus column was considered and rejected.** Two auditors
  flagged the same depth variance and prescribed opposite fixes — one to flatten to
  the WALS tier, one to deepen. When careful reviewers reach opposite conclusions
  from the same column, there is no single right depth to reach for; and the churn
  would touch all 51 rows, the assertions, both bundles and the doc table for no
  consumer-visible gain, since no `registry_by_genus` exists yet. The rank rule is
  written down instead, to be revisited if that accessor lands.

### Performance

- **Flat.** The rows this release could touch are unmoved: `registry_all_codes_iter`
  -0.4%, `registry_phonemes_lookup` 0.0%. Three other rows tripped a 10% threshold
  and all three are min-of-N artifacts — `allophone_realize` showed "-78.8%" on a new
  spread of 7-33 ns, i.e. one outlier sample against a stable 33. Reporting spreads
  alongside minima is now routine here precisely because this statistic has produced
  a phantom result in three consecutive releases.

## [2.3.0] - 2026-08-27

First slice of the 2.3.x Typological Depth line: eight WALS-style dimensions on
`GrammarProfile`. Additive — the eight original fields and their accessors are
untouched.

### Added

- **grammar** — eight dimensions across the 11 language profiles, with accessors
  `grammar_alignment`, `grammar_adposition_order`, `grammar_tense_marking`,
  `grammar_has_future`, `grammar_evidentiality`, `grammar_negation`,
  `grammar_article_type`, `grammar_adjective_order`, `grammar_relative_order`,
  and five new enums.

  Every dimension carries an explicit "no dominant value" variant, and three
  profiles use one: Mandarin's adposition order (WALS 85A), Arabic's
  tense-vs-aspect question, and Korean negation. `grammar_has_future` is a
  tri-state (1 / 0 / -1) because Korean's `-gess-` is genuinely disputed.

- **tests/typology.tcyr** (73 assertions) pinning the well-established cases —
  Hindi split-ergative, French discontinuous negation, Mandarin and Japanese
  prenominal relatives, the five article-less profiles — so a regression reads as a
  linguistic error rather than a diff. Plus range checks and a guard that all
  eleven profiles moved off the constructor defaults.

- **docs/development/typology-data.md** — the verified classification and
  endangerment/geography tables for **2.3.1** and **2.3.2**, so that verification
  is not repeated. Carries its own caveats.

### Changed

- **GrammarProfile** is 136 bytes, up from 64. Only 11 instances exist.
- **roadmap** — the "Missing script entries" sub-item is marked shipped: all nine
  scripts landed at **2.1.6**, verified against `script_by_code` and
  `script_all_codes` rather than taken on trust. It was listed in error.

### Notes

- **The data was gathered and adversarially verified.** Twelve agents in a
  gather-then-verify pipeline; each verifier saw only the claims, not the
  reasoning, and was told to refute rather than confirm. **43 of 44 grammar values
  survived unchanged.** The single dispute was about how to gloss English `-n't` in
  a comment — the verifier confirmed the encoded value (`NEG_PARTICLE`) was right.

- **The verifier caught a real error elsewhere**: Guarani was proposed as
  `Vulnerable` and corrected to `Safe` (~6.5M speakers, co-official in Paraguay,
  full intergenerational transmission). That correction is applied in the 2.3.2
  reference table.

- **The verification design has a hole, and it bit.** The endangerment agent
  returned 50 of 51 languages, silently omitting Korean, and the verifier did not
  notice — a reviewer checking claims only sees what is present, never what is
  absent. Caught by comparing the returned codes against the registry. Recorded in
  `typology-data.md`; any future gather/verify pass needs a completeness check
  independent of the reviewer.

- **Glottocode support is deliberately not implemented.** Glottocodes are opaque
  identifiers (`stan1293`) that can only be looked up, never derived. Generating 51
  from memory would produce plausible-looking fabrications indistinguishable from
  real ones — the worst failure mode for a reference corpus. Filed as a data item
  needing an authoritative source.

- Alignment records **syntactic** alignment. WALS 98A (case marking on full NPs)
  is a separate feature that would code both English and Mandarin as *Neutral*;
  it is not encoded here, and the profiles say so at the point of decision.

### Performance

- **Flat.** A first two-round A/B suggested `phoneme_lookup_ipa` had regressed
  10.5%, which would have been unexplainable — nothing in this release touches
  phoneme lookup. Five rounds dissolved it: the old build's spread is 306-346 ns
  and the new one's 338-341, overlapping distributions where the min-of-N statistic
  had latched onto one lucky low sample. `grammar_by_code_lookup`, the row that
  would actually move, is +0.9%.


## [2.2.4] - 2026-08-27

Last slice of the 2.2.x Phonological Depth line: tone becomes structured data.
Includes one **breaking** change to `phoneme_tones`.

### Breaking

- **phoneme** — `phoneme_tones(inv)` now returns a vec of **Tone records** rather
  than a vec of Chao strings.

  **Migration**: wrap element reads in `tone_letters(t)` to get the old string
  back. Nothing the previous representation carried is lost — the letters are
  preserved verbatim, and the record adds contour, register, endpoint levels and
  features alongside them.

  `phoneme_builder_tones` still *takes* strings and parses them, so the corpus
  stays declarative and the letters remain the single source of truth.

### Added

- **src/tone.cyr** — a new module (wired into `src/main.cyr` and both
  `cyrius.cyml` profiles) parsing Chao tone-letter notation into records:

  ```
  tone_parse(letters)   -> Tone
  tone_contour(t)       -> level / rising / falling / dipping / peaking
  tone_register(t)      -> high / mid / low  (from the mean pitch level)
  tone_start_level(t), tone_end_level(t)     -> the 1-5 Chao scale
  tone_features(t)      -> glottalized (ˀ) | checked (final ʔ)
  tone_letters(t)       -> the original string
  ```

  U+02E5..U+02E9 normalise to 5..1; ˀ (U+02C0) and a final ʔ (U+0294) are recorded
  as features rather than pitch. Unmodelled bytes are skipped rather than rejected,
  so an unknown diacritic degrades the record instead of discarding the tone.

- **tests/tone.tcyr** (163 assertions). Expectations come from each language's
  tonology, not from re-running the parser: Mandarin's 55/35/214/51 must come out
  level/rising/dipping/falling, Vietnamese ngã and nặng must be the glottalised
  pair, the Literary Chinese entering tone must be checked. Plus corpus-wide
  structure — every level on the 1-5 scale, every contour consistent with its own
  endpoints — and a round-trip check that every record returns its original letters.

### Notes

- **Nine languages carry tones, not the five the roadmap listed.** It named `zh`,
  `yo`, `th`, `vi`, `lzh`; **Hausa, Burmese, Somali and Lao** also have tone
  inventories, for 37 tones in total. The existing tonal/non-tonal test in
  `tests/integration.tcyr` had not caught this because those four appeared in
  neither of its two hard-coded lists. The corpus-wide assertion here pins 9 and 37,
  so a tenth cannot be added silently.

- **Parsed, not hand-transcribed** — the same choice `src/features.cyr` made at
  2.2.3, for the same reason: two hand-maintained encodings of one fact drift apart.

- **A data error surfaced and was left alone.** Both Thai and Vietnamese list a
  `˨˩˦` tone, which is the shape of *Mandarin's* third tone. Thai's inventory is
  missing its falling tone `˥˩` and has `˨˩˦` where that should be; Vietnamese's
  huyền should be `˨˩`, not `˨˩˦`. The same wrong value in two unrelated languages
  points at a copy-paste from the Mandarin entry. Correcting tone transcriptions is
  a data change beyond this release, so it is filed in the roadmap rather than made
  silently — the structured records are what made it visible.

- **Adding a module costs 19 test files.** Every test including `src/phoneme.cyr`
  needed `src/tone.cyr` ahead of it, since the builder now calls into it. Worth
  knowing before the next module lands.

### Performance

- **Flat** — no benchmark row moves more than run-to-run noise. Memory for all 51
  inventories goes 139,152 → 147,920 bytes (+8,768) for the 37 tone records and
  their vecs, paid once at build since the constructors have been cached since
  2.1.5.


## [2.2.3] - 2026-08-27

Fourth slice of the 2.2.x Phonological Depth line: the SPE/Hayes distinctive-feature
system, **derived** from the axes 2.2.0-2.2.2 put in place rather than stored. No
struct growth, no breaking changes.

### Added

- **src/features.cyr** — a new module (wired into `src/main.cyr` and both
  `cyrius.cyml` bundle profiles) providing `DistinctiveFeature`, 25 features across
  major class, manner, laryngeal, place and vowel quality:

  ```
  phoneme_df(p, f)        -> 1 (+), 0 (-), (0-1) unspecified
  phoneme_df_present(p)   -> bitmask of features valued +
  phoneme_df_defined(p)   -> bitmask of features specified at all
  df_name(f)              -> "sonorant", "spread glottis", …
  phoneme_minimal_contrast(a, b) -> the single feature two phonemes differ on, or 0
  ```

- **tests/distinctive.tcyr** (195 assertions). The expectations are taken from the
  phonetics literature for segments whose values are not in dispute — /p/ is
  [−sonorant, −continuant, +labial], /s/ is [+strident, +anterior], /θ/ is
  [−strident, +distributed] — rather than re-derived from the same inputs, which
  would have proved nothing. Plus corpus-wide structural checks over all 1,649
  phonemes: `present` is always a subset of `defined`, no obstruent is [+sonorant],
  every nasal is, every consonant is [−syllabic].

### Notes

- **Derived, not stored.** Almost every distinctive feature is a function of manner,
  place, voicing, height, backness, airstream and the secondary-articulation mask.
  Storing them again would double the per-phoneme cost and let the two
  representations drift — an inventory edited to change a manner would keep a stale
  [±sonorant]. Derivation makes that impossible. This is why the roadmap sequenced
  2.2.3 after 2.2.1 and 2.2.2.

- **Three values, not two.** A feature can be +, −, or genuinely unspecified:
  [±anterior] is coronal-only, [±delayed release] applies to stops and affricates,
  and [±ATR] is untranscribed corpus-wide. Collapsing unspecified into − would
  assert things the data does not support. Two cases are left unspecified because
  the literature genuinely splits, not because the data is missing: **trills and
  taps** for [±continuant] (intermittent closure), and **central vowels** for
  [±back] (a two-valued feature cannot carry a three-way contrast, and the
  `Backness` axis already records it exactly).

- **Secondary articulation feeds place.** Palatalized consonants come out [+dorsal,
  +high], labialized ones [+labial, +round], and the Arabic emphatics [+low, +back]
  without being pharyngeal by place — all falling out of 2.2.1's mask rather than
  needing their own data.

- **The derivation allocated, and now does not.** The first implementation used a
  16-byte accumulator per call, which the bump allocator would never reclaim —
  measured at 16,000 bytes per 1,000 reads. That is exactly the defect 2.1.5 swept
  out of the rest of the tree, so it was replaced with a single reused scratch
  record before release. `tests/distinctive.tcyr` asserts that 1,500 consecutive
  feature reads allocate zero bytes.

### Performance

- **Flat**, and **memory unchanged** at 139,152 bytes for all 51 inventories — the
  features are computed on demand and the Phoneme record stays 56 bytes.


## [2.2.2] - 2026-08-27

Third slice of the 2.2.x Phonological Depth line: vowel length, nasalization and
syllabicity become queryable. No struct growth and no breaking changes — the vowel
features reuse the `PhonemeFeature` word 2.2.1 added.

### Added

- **phoneme** — three vowel-side bits on the existing mask: `Nasalized` (2048),
  `Syllabic` (4096), `ExtraShort` (8192). `Long` (256) is **shared** with the
  consonant side rather than duplicated. Construct with `phoneme_vowel_full(...)`
  or `phoneme_builder_vowel_full(...)`; `phoneme_vowel` keeps its arity and
  defaults to no features.

- **inventories** — **118 vowels marked**, carrying 120 feature bits: 110 long
  across 20 languages, 5 nasal (French /ɛ̃ ɑ̃ œ̃ ɔ̃/, Guarani /ɨ̃/), 4 syllabic
  (Sanskrit /r̩ l̩ r̩ː l̩ː/) and 1 extra-short (Vietnamese /ɤ̆/). Sanskrit's /r̩ː l̩ː/
  are the only segments in the corpus carrying two vowel features at once.

- **tests/vowel_features.tcyr** (72 assertions) — per-language cases, corpus
  totals, a `symbol_vowel_feature_agreement` guard matching the consonant one, and
  a `long_is_shared_across_kinds` group that separates the 110 long vowels from
  the 1 long consonant.

### Changed

- **tests/features.tcyr** — `_count_feature` is now kind-aware. Sharing the `Long`
  bit meant an unfiltered count of long segments swept the new vowels into the
  consonant total; the existing assertion caught it (111 where 1 was expected).
  The "no vowel carries a consonant feature" assertion, which 2.2.2 makes false by
  design, is replaced by the stronger claim that no vowel carries a
  *consonant-only* feature — aspiration, laterality and the rest.

### Notes

- **ATR was named in the roadmap and is deliberately not implemented.**
  `tests/vowel_features.tcyr` has an `atr_is_absent` group that pins the reasons so
  the omission is not read as an oversight:
  1. **No corpus segment is transcribed with an ATR/RTR diacritic** (U+0318 /
     U+0319) — the assertion counts zero. Adding an `AdvancedTongueRoot` bit would
     ship a feature nothing sets, the same mistake `Velarized` was kept out of
     2.2.1 to avoid.
  2. **It is not a one-bit change.** Three corpus languages have ATR harmony and
     two have incomplete vowel systems for it: Yoruba's /i e ɛ a ɔ o u/ is the
     classic 7-vowel ATR set and could be marked as it stands, but Wolof lists only
     /i e a o/ and Somali /i e a o u/ — both missing the vowels the contrast needs.
     Marking Yoruba alone would leave ATR present in one language and silently
     absent in two that have it, which is worse than uniformly absent.

  Filed in the roadmap as a data item naming the three languages and what each
  needs, rather than shipped half-done.

### Performance

- **Flat**, and **memory unchanged**: no row moves more than run-to-run noise, and
  building all 51 inventories still takes 139,152 bytes. The vowel features occupy
  bits in the word 2.2.1 already allocated, so unlike the previous two releases
  this one widens nothing.


## [2.2.1] - 2026-08-27

Second slice of the 2.2.x Phonological Depth line: secondary articulation and
phonation as a queryable bitmask, and the lateral-click manner fix 2.2.0 deferred.
No breaking changes — the existing constructors keep their arity.

### Added

- **phoneme** — `PhonemeFeature`, an 11-bit mask on the phoneme record:
  `Aspirated`, `Breathy`, `Palatalized`, `Labialized`, `Pharyngealized`, `Tense`,
  `Devoiced`, `Prenasalized`, `Long`, `Raised`, `Lateral`. Read with
  `phoneme_features(p)` / `phoneme_has_feature(p, f)`; construct with
  `phoneme_consonant_full(...)` or `phoneme_builder_consonant_full(...)`.

  A bitmask rather than a field per feature because they combine freely (Hindi
  /d̪ʱ/ is breathy *and* dental, Zulu /ǁʰ/ aspirated *and* lateral *and* a click),
  several are rare enough that a slot each would be mostly zeroes, and 2.2.3's
  distinctive-feature bundle can extend the same word.

- **inventories** — **190 consonants marked** across the corpus: 76 aspirated,
  16 palatalized, 15 breathy, 6 devoiced, 5 tense, 4 pharyngealized, 4
  prenasalized, 3 labialized, 1 long, 1 raised, plus the laterals.

- **tests/features.tcyr** (104 assertions) — per-language cases written out by
  hand, a `lateral_consistency` check that the `Lateral` bit and the lateral
  manners agree in both directions, corpus totals per feature, and
  `symbol_feature_agreement`, which requires every secondary-articulation
  diacritic in all 1,649 phonemes to match its bit and no vowel to carry a
  consonant feature yet.

### Changed

- **Lateral clicks** — /ǁ ǁʰ ɡǁ/ move from `Manner.LateralFricative` to
  `Manner.Plosive`, with laterality now carried by `PhonemeFeature.Lateral`. A
  click is a stop, not a fricative; the wrong manner was inherited from before
  2.2.0 and left in place because "lateral" had nowhere else to live. It does now.
  /ŋǁ/ keeps `Manner.Nasal` — a nasal click is still nasal — and gains the bit.
- **Phoneme record** is 56 bytes, up from 48.

### Notes

- **The roadmap named five features; the corpus needs nine.** A scan of all 1,193
  consonants found four more that are real and were unrepresentable: **breathy
  voice** (15 segments — the entire Hindi/Bengali/Urdu voiced-aspirate series),
  **pharyngealization** (4 — the Arabic emphatic contrast), **tense/fortis** (5 —
  Korean), and **devoiced sonorants** (6 — Burmese, Icelandic), plus `Raised`
  (Czech /r̝/) and `Lateral`. `Velarized` is deliberately absent: nothing in the
  corpus needs it, and bits are additive.
- **A transcription inconsistency surfaced and was left alone.** Sanskrit spells
  its voiced aspirates /ɡʰ d͡ʑʰ ɖʰ d̪ʰ bʰ/ with U+02B0, so they derive as
  `Aspirated`; Hindi, Bengali and Urdu spell the same historical series with
  U+02B1 and derive as `Breathy`. Phonetically they are breathy in all four.
  Retranscribing Sanskrit is a data change beyond this release, so the corpus is
  marked as written and the discrepancy is documented at the assertion that pins
  it. Filed for a later data pass.
- **The marks were derived mechanically from the IPA symbols**, so
  `symbol_feature_agreement` cannot independently confirm the conversion — it
  would be checking the derivation against itself. Its value is forward-looking:
  an inventory added later with /pʰ/ and no `Aspirated` bit fails. The
  per-language groups are the ones that pin today's data, and those are hand-written.

### Performance

- **Flat.** Interleaved A/B; the only row moving more than 10% is
  `english_phoneme_inventory` at 3 → 5 ns, which is a 2 ns delta on a row already
  at the harness floor — noise, not a regression. The constructors have been cached
  singletons since 2.1.5, so the wider record is paid once at build.
- **Memory**: +8 bytes per phoneme. Building all 51 inventories moves 125,960 →
  139,152 bytes — exactly 1,649 × 8 — and being cached, that is the whole cost for
  a process.


## [2.2.0] - 2026-08-27

First slice of the 2.2.x Phonological Depth line: an **airstream** axis on the
phoneme record, and the corpus reclassified against it. No breaking changes — the
existing constructors keep their arity and default to pulmonic.

### Added

- **phoneme** — `Airstream` enum (`Pulmonic`, `Ejective`, `Implosive`, `Click`) as a
  field on the Phoneme record, with `phoneme_airstream(p)` to read it,
  `phoneme_consonant_airstream(...)` to construct one, and
  `phoneme_builder_consonant_airstream(...)` on the builder. `phoneme_consonant` and
  `phoneme_vowel` are unchanged and set `Pulmonic` implicitly.

- **inventories** — **38 consonants across 7 languages** reclassified with their
  airstream, manner intact:

  | | count | languages |
  |---|---|---|
  | Click | 12 | Zulu |
  | Implosive | 5 | Zulu, Hausa, **Khmer** |
  | Ejective | 21 | Georgian, **Yucatec Maya**, **Amharic**, **Quechua**, Hausa |

  The roadmap named three languages; a scan of all 51 inventories found seven. Khmer,
  Yucatec Maya, Amharic and Quechua were carrying unmarked implosives and ejectives.

- **tests/airstream.tcyr** (119 assertions). Every check asserts manner *and*
  airstream together — a test that only checked the airstream would pass equally
  under the single-enum model this release rejected. Beyond the per-language cases it
  pins corpus totals (12/5/21 across 7 languages), that no vowel is non-pulmonic,
  that every value is inside the enum, and that `phoneme_clone` carries the axis.

  The load-bearing one is `symbol_airstream_agreement`: it walks all 1,649 phonemes
  and requires that any IPA symbol containing a click letter, the ejective apostrophe
  or an implosive hook is marked accordingly — **and that nothing else claims a
  non-pulmonic airstream**. The totals pin today's corpus; this pins the rule, so a
  future inventory added with an unmarked /ɓ/ fails.

### Changed

- **Phoneme record** is 48 bytes, up from 40.

### Notes

- **The roadmap's `Manner` proposal was rejected, deliberately.** It read "Add
  `Click`, `Implosive`, `Ejective` to `Manner`", but those are airstream mechanisms,
  not manners of articulation, and the corpus shows the collision directly: /ŋǀ/ is a
  nasal click, /t͡sʼ/ an ejective affricate, /ǁ/ a lateral click. A single enum records
  one or the other, so the literal change would have erased the manner of 19 of the 38
  phonemes. A separate axis loses nothing and matches how PHOIBLE models it — which
  was the bullet's own stated goal. Roadmap amended.
- **Lateral clicks keep `Manner.LateralFricative`**, which is wrong — a click is a
  stop, not a fricative. It is pre-existing, and correcting it means moving "lateral"
  somewhere else, which is 2.2.1's job. Left alone rather than silently re-adjudicated
  here, and written into the roadmap so it is not forgotten.
- Aspiration is *not* an airstream: /pʰ/, /ǀʰ/ and /t͡sʰ/ are pulmonic, and the tests
  assert that explicitly. Aspiration becomes queryable at 2.2.1.

### Performance

- **Flat.** Interleaved A/B, every row within run-to-run noise. The inventory
  constructors have been cached singletons since 2.1.5, so the wider record is paid
  once at build rather than per call; the near-zero construction rows move by a few ns
  in both directions, which is noise at that magnitude, not a win.
- **Memory**: +8 bytes per phoneme. Building all 51 inventories moves 112,768 →
  125,960 bytes — exactly 1,649 phonemes × 8 — and, being cached, that is the total
  cost for a process, not a per-call one.


## [2.1.6] - 2026-08-27

Roadmap item 2.1.6: define the nine scripts the registry had been naming without
them. This empties the `2.1.x` carry-over list.

### Added

- **script** — nine ISO 15924 scripts that the registry referenced but `script.cyr`
  never defined, so `registry_primary_script` silently returned 0 for nine of the 51
  languages: **Thai** (`Thai`), **Bengali** (`Beng`), **Tamil** (`Taml`),
  **Ethiopic** (`Ethi`), **Hebrew** (`Hebr`), **Georgian** (`Geor`), **Myanmar**
  (`Mymr`), **Khmer** (`Khmr`) and **Lao** (`Laoo`). Each carries its type,
  direction, status, Unicode blocks and languages, and is reachable through
  `script_by_code` and `script_all_codes`. The registry now resolves a primary script
  for all 51 languages; `script_all_codes()` returns 19, up from 10.

  Blocks follow Unicode, main block first, including the supplementary ones a
  single-range definition would have missed: Tamil Supplement (`11FC0`–`11FFF`),
  Ethiopic Supplement / Extended / Extended-A, Hebrew presentation forms
  (`FB1D`–`FB4F`), Georgian Extended and Supplement, Myanmar Extended-A and -B, and
  Khmer Symbols. Hebrew is the only right-to-left addition; Georgian is the only
  alphabet, the other seven are abugidas.

- **tests/script.tcyr** — a `scripts_added_at_2_1_6` group (52 assertions) covering
  each new script's metadata, dispatch reachability, and codepoint boundaries on both
  sides of every block — including the adjacent-block confusions the ranges invite
  (Thai vs Lao at `0E7F`/`0E80`, Bengali vs Devanagari at `097F`).

### Changed

- **tests/invariants.tcyr** — `registry_script_codes_resolve` is **strict** again.
  The Rust original was deliberately tolerant ("Some scripts may not be registered yet
  (Thai, Beng, etc.)") and the 2.1.3 port carried that allowance, pinning the gap at
  exactly 9 unresolved codes. With the nine defined, the assertion now requires every
  registered language to resolve a primary script — a new language whose script is
  undefined fails here rather than returning 0 at runtime.
- **tests/integration.tcyr** — `registry_script_consistency` now expects all 51
  languages to resolve, up from 42.
- **tests/daimon.tcyr** — the agent registration advertises 19 supported scripts.

### Notes

- No performance change: `script_by_code` grew nine `streq` branches on a path that
  has been a cached singleton lookup since 2.1.5, and `script_by_code_lookup` is flat.
- Suite totals: 1,066 → 1,120 assertions across 26 test files.


## [2.1.5] - 2026-08-27

Roadmap item 2.1.5: the three items deferred out of the 2.1.2 hardening sweep.
Includes one **breaking** change to the `-D MCP` surface.

### Breaking

- **mcp** — `varna_translate_ipa` now returns a JSON object,
  `{"scheme":...,"input":...,"output":...}`, where it previously returned the
  transliterated text as a bare string. The other four tools already returned JSON
  objects, so a consumer parsing every success payload got malformed data from
  exactly one tool.

  **Migration**: read the `"output"` field instead of using the payload directly.
  `mcp_result_payload(r)` → parse → `output`.

  The contract is now stated at the top of `src/mcp.cyr` and pinned by
  `tests/mcp_json.tcyr`: **success payloads are JSON objects for all five tools;
  error payloads are plain diagnostic messages, not JSON.** Errors stay unstructured
  deliberately — the result is already tagged, so a caller never has to parse one to
  discover it is an error.

### Changed

- **Pre-built data constructors are now shared singletons.** All 98 zero-argument
  constructors — 51 `phoneme_*` inventories, 10 `script_*`, 11 `grammar_*`,
  10 `swadesh_*`, 2 `translit_*`, 5 `numerals_*`, plus `allophone_english`,
  the three `phonotactics_*`, `dialect_british_english`, `cognate_water` and the
  four `*_all_codes` list builders — build once and return the same pointer to
  every later caller.

  **This is a public-API semantic change.** Do not mutate a returned inventory
  through the `phoneme_builder_*` functions: you would be editing what every other
  caller sees. Use the new `phoneme_clone` for a mutable copy. `registry_all_codes`
  took this same change at 2.1.2; the contract is now uniform and documented at the
  head of the pre-built section in `src/phoneme.cyr`.

- **mcp** — every value written inside a JSON string literal now goes through
  `_mcp_append_json`, which escapes `"`, `\`, the five short-form control escapes
  and any other C0 byte as `\u00XX` (RFC 8259). Nothing in the current data needs
  an escape, which was exactly the problem: the invariant was unenforced and one new
  field away from emitting malformed JSON.

### Added

- **phoneme** — `phoneme_clone(inv)` returns an independent inventory that is safe
  to mutate. The copy is shallow in the same sense `dialect_apply` has always been:
  the vec, stress and tone slots are fresh, the immutable Phoneme records are shared.
- **tests/caching.tcyr** (54 assertions) — pins the singleton contract: pointer
  identity across repeat calls and through `registry_phonemes` / `script_by_code` /
  `grammar_by_code` / `swadesh_by_code` / `registry_primary_script`, that distinct
  constructors stay distinct objects, that the data is unchanged and does not
  accumulate on re-read, that `phoneme_clone` isolates mutations from the shared
  original, and that `dialect_apply` still leaves its parent untouched now that the
  parent is shared.
- **tests/mcp_json.tcyr** — a `json_escaping` group feeding a quote, a backslash, a
  newline, a tab and a raw `0x01` through `varna_translate_ipa` and checking the
  payload is still valid JSON with each byte escaped correctly.

### Performance

Interleaved A/B in one session (`cyrius bench`, min ns):

- **english_phoneme_inventory** 1,252 → 0 ns, **sanskrit** 1,664 → 6 ns, **greek**
  858 → 6 ns. The 0 is below the harness floor rather than literally free — the call
  is now a load and a compare, which the optimizer can hoist out of the loop.
- **registry_phonemes_lookup** 1,712 → 52 ns (**-97%**);
  **swadesh_by_code_lookup** 1,688 → 110 ns (**-93%**);
  **script_by_code_lookup** 287 → 102 ns (**-65%**).
- **numeral_value_of_char** 302 → 198 ns. Not attributable — the benchmark pre-builds
  its table, so nothing in this change should touch it; most likely allocation
  locality. Reported, not claimed.
- All other rows flat within noise.

**The leak is gone.** Measured directly off the bump allocator's `_heap_used`
counter over 1,000 warm calls, bytes handed out per call:

| call | before | after |
|---|---|---|
| `registry_phonemes("en")` | 2,400 B | 0 B |
| `phoneme_english()` | 2,400 B | 0 B |
| `swadesh_by_code("es")` | 1,624 B | 0 B |
| `script_by_code("Deva")` | 400 B | 0 B |

The allocator never frees, so those were permanent: a consumer polling the registry
once a second leaked ~200 MB a day.


## [2.1.4] - 2026-08-27

Roadmap item 2.1.4: remove the frozen v1.x Rust crate. The tree is single-language
again. No source behaviour changed — the only edits to `src/` are comment rewrites —
so benchmarks are unmoved and no new `bench-history.csv` row was recorded.

### Removed

- **`rust-old/`** — the frozen v1.x Rust crate: 35 files, 8,386 lines, 484K. Deleted per
  [ADR 0002](docs/adr/0002-remove-the-rust-old-archive.md), which supersedes the
  "freeze the Rust source" half of [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md).

  It was kept as the parity oracle because the 2.0.0 port was source-level
  reimplementation, not translation. Two things made removal decidable: the API port was
  verified complete at 2.1.3 (19 modules 1:1, 150/150 `pub fn`, 48/48 inventories matching
  by name, and nothing under `src/` or `tests/` ever compiled against it), and the four
  standalone suites under `rust-old/tests/` — the real dependency, 147 tests — were ported
  in the same release. Recoverable from git tags `1.0.0`–`2.1.3`:
  `git show 2.1.3:rust-old/src/lib.rs`. Recoverability was checked against tags `2.0.0`
  and `2.1.3` before deleting.
- **`.gitignore`** — the `/rust-old/target/` entry, and the stale "Cyrius port" heading
  above it.

### Changed

- **Provenance comments** — 43 references across 42 files in `src/` and `tests/` rewritten
  from `rust-old/src/X.rs` to `v1.x Rust src/X.rs`. The provenance is worth keeping; a path
  that no longer resolves is not. Three bare-form references (`rust-old/` with no file,
  `rust-old inventory_test!`, `rust-old mod.rs sample_lexicon`) were reworded individually
  rather than pattern-matched.
- **docs/adr/0001-port-from-rust-to-cyrius.md** — status amended to note the supersession,
  with a dated amendment block. The body is left as written: an ADR records what was
  decided at the time, so its `rust-old/` references stay.
- **README.md**, **SECURITY.md**, **CLAUDE.md**, **docs/guides/getting-started.md**,
  **docs/development/state.md** — `rust-old/` mentions replaced with the tag-recovery
  route. `getting-started.md` lost its "cross-check parity against `rust-old/`" step
  (the oracle it named is gone; the behaviour now lives in the ported suites) and gained
  `dist/` in the layout list, which was never described.
- **docs/benchmarks-rust-vs-cyrius.md** — **kept**, deliberately. Its criterion figures
  are the only surviving record of v1.x performance and are not reproducible from anything
  still in the tree. Annotated with where the source went and how to recover it.

### Notes

- `git rm` was used, so the 35 deletions are staged. Nothing is committed.
- The "93 references across 49 files" figure quoted in the 2.1.3 roadmap counted the
  generated `dist/*.cyr` bundles alongside their sources. The real hand-edited count was
  43 across 42 files; the bundles picked up the rewrite when `cyrius distlib` regenerated.


## [2.1.3] - 2026-08-27

Roadmap item 2.1.3: port the four standalone Rust test suites the 2.0.0 port left
behind. Test-only — no source changes, so benchmarks are unmoved and no new
`bench-history.csv` row was recorded.

The 2.0.0 port carried every module's public API and its in-module unit tests, but
not the 147 tests under `rust-old/tests/`. That gap is why `rust-old/` could not be
deleted: removing it would have destroyed the record of what those tests asserted.
It is now closed, which unblocks 2.1.4.

### Added

- **tests/invariants.tcyr** (39 assertions) — port of `rust-old/tests/invariants.rs`
  (33 tests). Structural guarantees across the whole dataset: code uniqueness for
  languages/scripts/grammar/Swadesh, consonant+vowel totals, no duplicate or empty
  IPA symbols, inventory codes matching their registry key, tones and `Tonal` stress
  implying each other, well-formed Unicode ranges, 25-entry Swadesh lists covering
  indices 1-25, numeral tables without duplicates, decimal digit runs, and
  registered dialect parents and cognate languages.
- **tests/adversarial.tcyr** (74 assertions) — port of `rust-old/tests/adversarial.rs`
  (54 tests): empty strings, 1000-character codes, emoji, BOM, zero-width space,
  IPA lookalikes (`S` vs `ʃ`, ASCII `g` vs `ɡ`), case-sensitive ISO 15924 codes,
  codepoint boundaries either side of every script range, `u32::MAX`, empty and
  zero-capacity builders, `most_frequent(0)` and `most_frequent(1_000_000)`,
  unmapped and mixed-validity numeral strings, and dialect idempotence. **This is
  the suite whose absence let the 2.1.2 heap overflows survive** — it already
  contained `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`.
- **tests/integration.tcyr** (35 assertions) — the 18 behavioural tests from
  `rust-old/tests/integration.rs`: phoneme-kind classification, constructor field
  round-trips, registry/script consistency, the allophone/inventory boundary
  (`/t/` is an English phoneme, its intervocalic tap `[ɾ]` is not), Greek isopsephy
  of θεος, the RP overlay, cognate/registry cross-checks, Swadesh index 23 glossing
  as "water" in every list, and the tonal/non-tonal split.
- **tests/mcp_json.tcyr** (28 assertions) — stands in for
  `rust-old/tests/serde_roundtrip.rs` (27 tests), which cannot port directly: there
  is no serde in Cyrius and no deserializer to round-trip against. It checks the
  property the roundtrips were protecting — that every payload the library emits is
  structurally valid JSON — across 51 phoneme payloads, every script and grammar
  payload, and 204 compare payloads, using a structural validator that is itself
  self-checked against eleven malformed inputs.

### Changed

- **docs/development/roadmap.md** — the pre-port `1.1.0`-`1.5.0` tiers are
  renumbered `2.2.0`-`2.6.0` and the section retitled; the old numbers predated the
  2.0.0 port and no longer tracked the shipping version. 2.1.3 is removed from the
  carry-over list and 2.1.4 (`rust-old/` removal) is no longer gated.
- **docs/development/roadmap.md** — reduced to open work only. The `Completed`
  section (2.1.2 back through the 0.x Rust-crate line) is dropped; release history
  lives in this file and the git tags. Added a `2.1.x — Carry-over` section for
  work identified but not taken: 2.1.3 port the Rust integration suites, 2.1.4
  remove `rust-old/` (gated on 2.1.3), 2.1.5 the deferments from the 2.1.2
  hardening sweep, 2.1.6 script-registry completeness. The retired `v1.0 Criteria`
  checklist left one unmet item (shabda/shabdakosh consumption), now under
  `Downstream`; its "full JSON roundtrip tests for all public types" line was
  inaccurate post-port and is corrected there.
- **docs/development/state.md**, **CLAUDE.md** — record the port-completeness
  verdict, and correct CLAUDE.md's claim that `rust-old/` is "gitignored from CI"
  (it is tracked; only `rust-old/target/` is ignored).

### Verified

- **Rust → Cyrius port is complete at the API level.** 19 Rust modules map 1:1 to
  19 Cyrius modules (plus `util.cyr`); all 150 Rust `pub fn` have Cyrius
  counterparts, checked name by name; the 48 language inventories match exactly.
  Nothing in `src/` or `tests/` compiles against `rust-old/` — the 93 references
  across 49 files are provenance comments.
- **The gap is test coverage, not code.** The four suites under `rust-old/tests/`
  — `invariants.rs` (33), `adversarial.rs` (54), `integration.rs` (33),
  `serde_roundtrip.rs` (27) — were never ported. `adversarial.rs` already contains
  `registry_info_very_long_code`, `phoneme_find_very_long_string` and
  `error_display_very_long`: the class of test that would have caught the 2.1.2
  heap overflows. `rust-old/` is therefore **not yet safe to delete** — porting
  those suites first is scheduled as 2.1.3.
- **26 of 27 checkable Rust invariants hold** against current Cyrius data
  (uniqueness, consonant+vowel totals, no duplicate/empty IPA symbols, well-formed
  Unicode ranges, 25-entry Swadesh lists with indices 1-25, numeral tables,
  registered dialect parents and cognate languages). The 27th,
  `registry_script_codes_resolve`, finds 9 languages whose primary script is
  unregistered — **not a port regression**: the Rust invariant explicitly tolerated
  the same gap ("Some scripts may not be registered yet (Thai, Beng, etc.)").
  Tracked as 2.1.6.


### Notes

- **Four Rust tests do not port, and are documented inline rather than dropped
  silently.** `registry_info("en\0")` and `transliterate("\0")` tested that a Rust
  `String` may carry an interior NUL; a Cyrius cstring ends at the first NUL, so the
  property does not exist here. The three `error_display_*` tests formatted a
  payload into `VarnaError`, which in Cyrius is a plain integer code with static
  messages — the concern behind them (an unbounded caller string reaching a
  formatter) lives in `_tool_err2` and is covered by `tests/hardening_surfaces.tcyr`.
- **All 26 checkable invariants hold.** The 27th, `registry_script_codes_resolve`,
  is ported in the tolerant form the Rust original used and pins the current gap
  exactly: 9 of 51 languages name an unregistered primary script (Thai, Bengali,
  Tamil, Amharic, Hebrew, Georgian, Burmese, Khmer, Lao). Not a port regression —
  the Rust invariant carried the same allowance in a comment. Roadmap 2.1.6 adds the
  scripts, at which point the assertion can be tightened.
- Suite totals: 21 → 25 test files, 652 → 1,005 assertions, reference coverage
  89% → 94%.


## [2.1.2] - 2026-08-27

Hardening release from a P(-1) scaffold sweep. Six memory-safety defects fixed —
two of them heap overflows reachable from the `-D MCP` and `-D HOOSH` surfaces
with caller-controlled length — plus the allocation-churn and lookup-complexity
work the audit turned up. No public API or linguistic data changed.

### Security

- **mcp** — `_tool_err2` rendered `"<prefix><value>"` into a fixed `alloc(256)`
  with no bound on `value`, which is an unvalidated tool parameter. A 400-byte
  `language` / `code` / `scheme` / `lang1` / tool name wrote ~418 bytes into the
  256-byte allocation — a heap overflow with both length and contents chosen by
  the caller. Reachable from all five tools plus the unknown-tool path
  (6 call sites). Every append in the file now takes the buffer capacity and
  truncates; `tests/hardening_surfaces.tcyr` fails on the pre-fix code at all six.
- **hoosh** — `hoosh_answer_from_data` interpolated the caller's `ipa` string into
  a fixed `alloc(256)`. On the not-in-inventory branch that string is never matched
  against anything first, so its length was entirely the caller's choice — the same
  overflow. Bounded the same way.
- **transliteration** — `translit_apply` walked the input with `_utf8_len`, which
  classifies a lead byte without checking that its continuation bytes are present.
  A truncated codepoint at the tail (a lone `0xF0`) advanced the cursor 4 bytes
  with 1 byte left, so the following `memcpy` read up to 3 bytes past the end of
  the input. Reachable through the `varna_translate_ipa` MCP tool.
- **numerals** — `numerals_string_value` had the identical over-read, and is
  reachable through daimon's `numeral_value` capability.
- **util** — new `_utf8_step(s, p, n)` clamps every stride to the bytes actually
  present and never returns 0; it is now the only way the tree walks UTF-8.
  `_utf8_len` is kept and documented as lead-byte classification only.
- **transliteration** — `translit_apply` sized its output at `inlen * 4`, which
  held only because both bundled tables happen to contract. A table mapping a
  short source to a longer target overflowed the buffer, and nothing
  bounds-checked the writes. Output is now sized from the table's longest target
  (`_translit_max_target`, memoised and invalidated by `_tt_add`) and every write
  is checked against the capacity regardless.

### Changed

- **registry** — `registry_all_codes` caches its vec instead of rebuilding a
  51-element one per call. **The result is now shared and must be treated as
  read-only.** The old behaviour leaked a vec per call, since the bump allocator
  never frees.
- **phoneme** — documented that `phoneme_builder_with_capacity`'s `cap` argument
  is ignored: `lib/vec.cyr` exposes no capacity-taking constructor. Kept in the
  signature because the bundle is a published consumer API.

### Added

- **tests** — `tests/hardening.tcyr` (26 assertions) and
  `tests/hardening_surfaces.tcyr` (28) pin every fix above. Both were checked
  against the pre-fix code: the surface tests fail 7 assertions and the
  expanding-table test fails 2, so they are regression guards rather than
  restatements of current behaviour.
- **tests** — `registry_table_consistency` ties `registry_phonemes` to
  `_registry_build`. The two list the same 51 codes in different forms with
  nothing connecting them, so a language could be registered with no inventory
  and only surface downstream.
- **tests** — coverage sweep over the accessor surface `cyrius coverage` reported as
  unreferenced. Reference coverage 204/278 fns (73%) → 249/278 (89%), clearing the 80%
  floor in CLAUDE.md; `swadesh`, `syllable`, `lexicon`, `allophone`, `numerals`,
  `dialect` and `error` are now at 100%, and `error.cyr` went from no referenced fn at
  all to covered. 222 new assertions, 17 → 19 test files (21 with the two above).
  - **swadesh** — `tests/swadesh.tcyr` (+100): all ten list builders called directly
    instead of only through the `swadesh_by_code` dispatch table, each checked for
    code, 25 entries and the first/last entry's word, IPA, gloss and Swadesh index;
    `swadesh_entry` / `_word` / `_ipa` / `_gloss` pinned against the shared gloss set.
  - **syllable** — `tests/syllable.tcyr` (+42): `SyllableTemplate` accessors read
    directly and cross-checked against the delegating `phonotactics_*` ones;
    `phonotactics_language_code` / `_syllable` / `_constraints`; `constraint_kind` /
    `_position` / `_sequences` / `_description` over the English, Japanese and
    Sanskrit profiles; and an open-syllable (`max_coda 0`) template for the coda
    thresholds no pre-built profile reaches.
  - **lexicon** — new `tests/lexicon.tcyr` (24): `LexEntry` field accessors, the
    `(0-1)` None sentinel for `frequency_rank` / `swadesh_index`, `lexicon_entries`
    insertion order, and `lexicon_find_gloss` vs `lexicon_find` on a non-English
    lexicon where native word and English gloss cannot agree.
  - **numerals** — `tests/numerals.tcyr` (+19): `numerals_script_code` / `_name` /
    `_mappings` for all five systems, pinning each Unicode script tag and table size.
  - **allophone** — `tests/allophone.tcyr` (+11): `allophone_rule_phoneme` /
    `_allophone` / `_environment` / `_obligatory` — the environment and obligatoriness
    that `allophone_realize` collapses away.
  - **dialect** — `tests/dialect.tcyr` (+11): `dialect_code` / `_name` / `_added` /
    `_removed`, and the RP intervocalic-/t/ override read through the rule accessors.
  - **error** — new `tests/error.tcyr` (15): `varna_error_str` for every `VarnaError`
    code plus the out-of-range fallback, so an unknown sentinel still renders.

### Performance

Measured by interleaving old and new builds in one session (`cyrius bench`, min
ns) — a straight before/after against an earlier baseline showed a phantom
15-25% regression across untouched rows that was pure machine drift.

- **registry_all_codes_iter** — 42,499 → 3,310 ns (**-92%**, 12.8x).
  `registry_info` was a linear `streq` scan over 51 entries, so iterating the
  registry cost ~1,300 string compares; it is now a lazily built `map_get`.
- **transliterate_greek_word** — 39,636 → 19,351 ns (**-51%**). `translit_apply`
  allocated a `lens` vec plus up to four NUL-terminated candidate buffers *per
  input grapheme*; matching is now done in place against the table literals.
- **numeral_string_value_word** — 1,591 → 969 ns (**-39%**); the per-character
  allocation is gone the same way.
- **numeral_value_of_char** — 502 → 310 ns (**-38%**) and
  **transliterate_devanagari_char** — 916 → 575 ns (**-37%**): one `strlen` of the
  needle per call instead of one per table entry.
- Every other row is flat within its run-to-run noise. `script_contains_codepoint`
  reads 16 → 14 ns but that is inside its own ±14% spread and nothing in
  `script.cyr` changed — not claimed as a win.


## [2.1.1] - 2026-08-27

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.4.69` → `6.5.35` (`cyrius.cyml [package].cyrius`);
  CI and the release workflow derive their install version from the pin, so the whole
  pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.5.35 snapshot.
  The closure is the same 29 modules as at 2.1.0, but 20 of them changed content —
  most substantially `bayan` (TOML/JSON parser, ~10.8k changed lines), `syscalls_x86_64_agnos`,
  `alloc`, `bench`, `io`, `vec`, and the per-arch `syscalls_*` set. Upstream added
  `async_macos`/`thread_macos` to the snapshot; neither is in varna's closure.
- **cyrius.lock** — regenerated at 6.5.35; `cyrius deps --verify` is clean (29 verified,
  0 failed). Resolved with `cyrius deps`, whose include-graph walk is the authority:
  6.5.35's new `cyrius lib sync` matches on declared `[deps].stdlib` names instead and
  produces a *different* 29-module set — it adds the unreferenced `hashmap_fast.cyr`
  and drops `atomic.cyr`, which `lib/alloc.cyr` includes transitively and the build
  needs. Locking that set would break a fresh `cyrius deps --verify`.
- **bote** — the commented-out `[deps.bote]` recipe in `cyrius.cyml` (inert; `-D MCP`
  hand-builds its JSON today) refreshed from the stale `tag = "2.7.6"` to `3.3.7`,
  which pins the same Cyrius 6.5.35 toolchain. Its companion notes were corrected
  against the real `dist/bote-core.deps` sidecar: the profile is 12 modules (not 9),
  and the only stdlib leaf varna does not already declare is `chrono` — not the
  `ct`/`keccak`/`random`/`thread`/`thread_local`/`sigil`/`freelist` set the old
  comment listed.
- **daimon** — agent-registration version string bumped to `2.1.1` (`src/daimon.cyr`).
- **dist** — bundles regenerated at `2.1.1`. `cyrius distlib` now also emits a `.deps`
  stdlib-leaf sidecar per profile, and `cyrius distlib --all` writes the `core` profile
  bundle, so three files join the tracked `dist/`: `varna.deps` (16 leaves),
  `varna-core.cyr`, and `varna-core.deps` (4 leaves — `string`, `alloc`, `vec`,
  `hashmap`). `[lib]` and `[lib.core]` declare identical module lists, so the two
  bundle bodies are byte-identical; the sidecar is what distinguishes them.
- **scripts/check.sh** — the gate now runs `cyrius deps --verify` after `cyrius deps`,
  so a lock that does not match the snapshot it claims to pin fails locally and in CI
  instead of shipping (see *Fixed* below).
- **scripts/bench-history.sh** — now records the toolchain version and the harness's
  measured timer floor in `BENCHMARKS.md`, plus a note marking the pre-/post-6.5.19
  measurement discontinuity. On a parse failure it echoes the captured output instead
  of failing silently.

### Fixed

- **cyrius.lock** — the lock shipped at 2.1.0 recorded a wrong hash for
  `lib/syscalls_x86_64_agnos.cyr` (`c5e63ab…` against the 6.4.69 snapshot's actual
  `e53e129…`), so `cyrius deps --verify` reported `28 verified, 1 failed` on a clean
  checkout. CI never caught it because `scripts/check.sh` ran plain `cyrius deps`,
  not `--verify`. Regenerating at 6.5.35 clears it, and the gate now runs `--verify`
  so a stale lock cannot ship again.
- **Build warnings** — the gate is now warning-free. 6.4.69's vendored `lib/bayan.cyr`
  emitted three `assigning non-pointer to typed pointer` warnings on *every* compile
  (`_toml_parse_str`/`_toml_parse_multiline_q` were declared `: i64` but assigned into
  a `Str`); 6.5.35 declares them `: Str`. The `toolchain drift` warning is gone too,
  now that the pin matches the installed `cycc`.

### Performance

- **Stdlib refresh is performance-neutral.** Holding the benchmark harness constant at
  6.5.35 and swapping only the other 28 vendored modules, every benchmark lands within
  run-to-run noise (min ns, 6.4.69 → 6.5.35 stdlib): `english_phoneme_inventory`
  1272 → 1341, `registry_all_codes_iter` 42306 → 42820, `transliterate_greek_word`
  39686 → 40632, `dialect_apply_overlay` 2906 → 3215. Binary size is unchanged at
  597,976 bytes for the default build.
- **The recorded numbers moved, but the instrument moved — not the code.** Cyrius
  6.5.19 rewrote `lib/bench.cyr`: `bench_run` used to wrap a clock pair around every
  iteration, flooring all 18 varna benchmarks at roughly two clock reads (the
  ~419ns/489ns/907ns plateaus visible throughout the old history). It now sizes its own
  batches and subtracts a *measured* per-host clock cost (1.409us on the recording
  host). Sub-microsecond rows finally resolve — `script_contains_codepoint` reads 14ns
  and `allophone_realize` 34ns where both previously bottomed out at 419ns — and the
  heavier rows shift because they are no longer inflated by an unsubtracted floor.
  Rows in `bench-history.csv` on either side of 2026-08-27 are **not** comparable;
  the floor is a property of the host clocksource and moves between reboots. See
  `BENCHMARKS.md`.

## [2.1.0] - 2026-07-21

Toolchain-maintenance release: Cyrius upgrade and dependency refresh. No changes to
the public API or linguistic data — the default build, the full
`-D LOGGING -D MCP -D DAIMON -D HOOSH` build, and every test stay green against the
newer stdlib.

### Changed

- **Toolchain** — Cyrius pin bumped `6.2.12` → `6.4.69` (`cyrius.cyml [package].cyrius`);
  CI derives its install version from the pin, so the whole pipeline moves with it.
- **Dependencies** — vendored Cyrius stdlib re-resolved against the 6.4.69 snapshot. Upstream
  dropped `agnosys`; `async_win`/`protobuf`/`yantra` are new (none used by varna).
- **cyrius.lock** — regenerated at 6.4.69. Now pins the exact 29-module dependency closure a
  fresh `cyrius deps` resolves, so `cyrius deps --verify` is clean (29 verified, 0 failed).
  Removed 8 stale entries that were never actually resolved or linked by any build config
  (`agnosys`, `bote-core`, `libro`, `majra`, `log`, `sakshi`, `sigil`, `patra`) and added the
  benchmark-harness dependency `bench`.
- **daimon** — agent-registration version string bumped to `2.1.0` (`src/daimon.cyr`).
- **dist** — `dist/varna.cyr` bundle regenerated (header `# Version: 2.1.0`).

### Fixed

- **scripts/bench-history.sh** — `to_ns()` now parses the fractional-unit benchmark
  output Cyrius ≥6.3 emits (`1.396us`); the old converter only stripped leading integer
  digits, so decimal timings were written to `bench-history.csv` unscaled (`1.396` instead
  of `1396`). Results are rounded to integer nanoseconds, matching the existing history.

### Performance

- No regressions from the toolchain move (min ns, 6.2.12 → 6.4.69): `english_phoneme_inventory`
  2000 → 1396, `transliterate_greek_word` 38000 → 36527, `registry_all_codes_iter` 37000 → 36666;
  all other benchmarks within noise. See `bench-history.csv` / `BENCHMARKS.md`.

## [2.0.0] - 2026-06-16

Ported from a Rust crate to the [Cyrius](https://github.com/MacCracken/cyrius)
systems language. See [ADR 0001](docs/adr/0001-port-from-rust-to-cyrius.md) for the
full rationale. The `content`/data model and the public surface are preserved at
parity; the implementation language, build system, and dependency stack changed.

### Breaking

- **Language** — varna is now a Cyrius library (`dist/varna.cyr`), not a Rust crate.
  The v1.x Rust source is frozen in `rust-old/`. Consumers depend on varna via a
  `[deps.varna]` block in `cyrius.cyml` (`modules = ["dist/varna.cyr"]`), not Cargo.
- **API shape** — the Rust module-path API (`varna::phoneme::english()`) is re-exposed
  as snake-case free functions (`phoneme_english()`); `Option`/`unwrap` access becomes
  sentinel-return checks; `Result` errors use native tagged `enum`s.
- **Feature flags → build defines** — the Cargo `std`/`logging`/`mcp`/`daimon`/`hoosh`/`full`
  features are replaced by `-D LOGGING`/`-D MCP`/`-D DAIMON`/`-D HOOSH` passed to
  `cyrius build`. There is no `std`/`no_std` split.
- **Renames for project-name consistency** — `LIPI_LOG` → `VARNA_LOG`; `LipiError` →
  `VarnaError`; MCP tools `lipi_*` → `varna_*` (`varna_phonemes`, `varna_script`,
  `varna_grammar`, `varna_translate_ipa`, `varna_compare`); `ResponseSource::LipiData`
  → `VarnaData`.

### Changed

- **Dependency stack** — every external Rust dependency now resolves to the Cyrius
  stdlib or a language builtin:
  - `serde` + `serde_json` → `bayan` (`#derive(Serialize)` + `json_parse`/`json_build`)
  - `thiserror` → native `enum` + `Result<T,E>` (`lib/result.cyr`, `lib/tagged.cyr`)
  - `tracing` + `tracing-subscriber` → `lib/log.cyr` + `lib/sakshi.cyr` (env-filter wired by hand)
  - `criterion` → `cyrius bench` + `lib/bench.cyr`
  - Optional `bote` (MCP) → consumed as the `dist/bote-core.cyr` git dep under `-D MCP`
- **String model** — `Cow<'static, str>` fields become `'static` `str` literals (Cyrius
  has no `Cow`); the builder still constructs inventories at runtime.
- **Enums** — `#[non_exhaustive]` enums become Cyrius tagged enums (additive variants).
- **Build + test layout** — `Cargo.toml`/`Cargo.lock` → `cyrius.cyml`/`cyrius.lock`;
  `benches/benchmarks.rs` (criterion) → `benches/*.bcyr`; `tests/integration.rs` →
  `tests/tcyr/*.tcyr`; `make check` (cargo fmt/clippy/test/audit) → `cyrius audit`.

### Removed

- The Rust toolchain config (`rust-toolchain.toml`), `deny.toml`, and the Cargo
  feature system. `cargo doc` browsable API reference (varna's surface is a folded
  `.cyr` bundle, navigable in `src/`).
- The `std`/`no_std`/`alloc` feature distinction — Cyrius emits a single artifact.

### Performance

- Baselined 18 benchmarks via `cyrius bench tests/varna.bcyr` (`bench-history.csv` /
  `BENCHMARKS.md`). Representative per-op `min` figures: phoneme lookup 419ns,
  `script_by_code` 838ns, grammar lookup 419ns, English inventory build 2µs, registry
  iteration over 51 languages 40µs.

### Note

The source-level port is **complete**: all 19 modules (15 core + 4 `-D` surfaces) are
reimplemented as `src/*.cyr`, verified by **526 parity assertions** (`cyrius tests`)
against the frozen `rust-old/` oracle plus a 7-agent parity audit (0 unported public
functions). Documented divergences from the oracle: sentinel returns for `Option`/`Result`,
integer-tagged enums, hand-built JSON (`mcp` uses a lightweight `ToolDef`, no `bote`/`bayan`),
`hoosh` confidence as per-mille, and the name-colliding enum variants `WordOrder.Free` /
`VarietyKind.Historical` / `PhonemeClass` omitted. `daimon`/`hoosh` remain Cyrius binaries
with no consumable library bundle, so those define-gated surfaces only toggle JSON output.

## [1.0.0] - 2026-03-31

### Added

- **phoneme::inventories** — 24 additional language inventories reaching 51 total:
  - European: Italian (23C+7V), Dutch (19C+13V), Polish (29C+6V), Czech (25C+10V), Hungarian (25C+14V), Romanian (21C+7V), Icelandic (22C+8V)
  - South Asian: Bengali (30C+7V), Tamil (19C+10V), Urdu (37C+10V)
  - African: Amharic (27C+7V), Hausa (25C+10V), Somali (22C+10V), Wolof (25C+8V)
  - Southeast Asian: Indonesian (18C+6V), Burmese (30C+7V), Khmer (24C+18V), Lao (20C+9V)
  - Middle East/Caucasus: Persian (23C+6V), Hebrew (23C+5V), Georgian (28C+5V)
  - Americas: Quechua (24C+3V), Guarani (16C+12V)
  - Central Asian: Mongolian (21C+14V)
- **grammar** — English grammar profile added (`grammar::by_code("en")`)
- **docs** — Usage guide (docs/guides/usage.md) with 15 sections and code examples
- **docs** — Architecture overview fully updated for all modules and 51 languages

### Changed

- **registry** — 51 languages registered (up from 27)
- v1.0 criteria: test coverage measured at 98.53%, all documentation complete

## [0.6.0] - 2026-03-31

### Added

- **mcp** — MCP tool definitions (feature-gated `mcp`): `lipi_phonemes`, `lipi_script`, `lipi_grammar`, `lipi_translate_ipa`, `lipi_compare`. Tool registry with `tool_definitions()`, `invoke()` dispatcher, JSON-serializable `ToolResult`. Language comparison computes shared/unique phonemes and grammar differences
- **daimon** — Agent registration for AGNOS framework (feature-gated `daimon`): `AgentRegistration` with 6 capabilities, dynamic language/script coverage from registry. Version auto-synced from Cargo.toml
- **hoosh** — LLM query interface (feature-gated `hoosh`): `LanguageQuery` enum (5 query types), `QueryResponse`, `ResponseSource`. `answer_from_data()` handles phoneme/comparison queries without LLM, returns `None` for queries requiring inference
- Feature gates: `mcp`, `daimon`, `hoosh` (all included in `full`)

## [0.5.0] - 2026-03-31

### Added

- **phoneme::inventories** — 9 core language inventories: Mandarin (21C+7V), Hindi (34C+10V), Japanese (20C+5V), Spanish (23C+5V), French (21C+16V), German (23C+16V), Russian (36C+6V), Korean (19C+7V), Portuguese (23C+14V)
- **grammar** — Pre-built grammar profiles for 10 core languages with `by_code()` lookup and `all_codes()`. Covers morphology, word order, case count, gender, dual number, classifiers
- **lexicon::swadesh** — Swadesh-25 starter word lists for 10 core languages (250 entries total) with IPA transcription, part of speech, and Swadesh index. `by_code()` lookup
- **registry** — 27 languages registered (up from 18)

## [0.4.1] - 2026-03-31

### Added

- **script** — `[S]` Cuneiform script metadata (Xsux) with Sumerian/Akkadian language tags and Unicode ranges
- **script** — `[S]` Egyptian hieroglyphic script metadata (Egyp) with Unicode ranges
- **script** — `ScriptStatus` enum (Living, Limited, Historical) and `attestation` period field on all scripts
- **script::numerals** — `[S]` Babylonian sexagesimal numeral system (base-60, cuneiform digits)
- **script::numerals** — `[S]` Egyptian hieroglyphic numeral system (additive decimal, powers of 10)
- **script::numerals** — `[S]` Chinese rod numeral system (positional decimal, vertical forms)
- **phoneme::inventories** — 4 classical/liturgical language inventories:
  - **Latin** (la) — 18C + 10V, labialized velars
  - **Classical Arabic** (ar) — 28C + 6V, pharyngeals, emphatics, uvular
  - **Koine Greek** (grc) — 17C + 5V, pitch accent
  - **Literary Chinese** (lzh) — 27C + 12V, 4-tone Middle Chinese reconstruction
- **registry** — 18 languages registered (up from 14), 10 scripts (up from 8)

## [0.4.0] - 2026-03-31

### Added

- **dialect** — Language variety support: `LanguageVariety`, `VarietyKind` (Regional, NationalStandard, Historical, Sociolect, Creole). Phoneme add/remove overlays and allophone overrides. British English (RP) and Egyptian Arabic pre-built
- **lexicon::cognate** — Cognate detection types: `CognateSet` with proto-form and cross-language entries, `Etymology` with `BorrowingType` (Loanword, Calque, SemanticLoan, Inherited). PIE "water" cognate set pre-built
- **phoneme::inventories** — 11 new language inventories across 6 language families:
  - `[S]` **Yucatec Maya** (yua) — 21C + 10V, ejective consonants for Mayan calendar validation
  - **Swahili** (sw) — 26C + 5V, **Yoruba** (yo) — 18C + 7V (3-tone), **Zulu** (zu) — 42C + 5V (clicks)
  - **Thai** (th) — 21C + 9V (5-tone), **Vietnamese** (vi) — 22C + 11V (6-tone), **Tagalog** (tl) — 18C + 5V
  - **Turkish** (tr) — 20C + 8V (vowel harmony), **Finnish** (fi) — 17C + 16V (short+long)
  - **Hawaiian** (haw) — 8C + 10V (minimal inventory), **Nahuatl** (nah) — 16C + 8V (lateral affricate)
- **registry** — 14 languages registered (up from 3)

## [0.3.0] - 2026-03-31

### Added

- **phoneme::allophone** — Allophone rule system: `AllophoneRuleSet`, `AllophoneRule`, `Environment`, `PhonemeClass`. Context-dependent sound variation with `rules_for()` and `realize()` lookup. English (GA) rules included (aspiration, flapping, dark-l)
- **phoneme::syllable** — Syllable structure templates: `SyllableTemplate` with max onset/coda, `Phonotactics` with `PhonotacticConstraint`. English, Sanskrit, and Japanese profiles included
- **script::transliteration** — `[S]` Bidirectional transliteration tables: `TransliterationTable` with `transliterate()`, `transliterate_char()`, and `reverse_map()`. Devanagari↔IAST and Greek↔Beta Code tables included
- **script::numerals** — `[S]` Script-to-numeral mapping: `NumeralSystem` with `value_of()`, `char_for()`, and `string_value()`. Devanagari decimal digits and Greek isopsephy included
- `Hash` derive on `Phoneme`, `PhonemeKind`, `Morphology`, `WordOrder`, `Direction`
- `debug_assert` for duplicate IPA detection in `PhonemeInventoryBuilder::build()`

### Changed

- **script** — Kana script name corrected from "Katakana" to "Kana (Hiragana + Katakana)"
- **registry** — `all_codes()` returns `&'static [&'static str]` instead of allocating `Vec`
- **lib.rs** — Crate documentation updated to reflect five modules (added registry)
- **docs** — Architecture overview rewritten with registry, script/language tables

## [0.2.0] - 2026-03-31

### Added

- **phoneme** — `PhonemeInventoryBuilder` for ergonomic inventory construction with `consonant()`, `vowel()`, `stress()`, `tones()`, and `with_capacity()` methods
- **phoneme** — Sanskrit (Classical) phoneme inventory: 36 consonants + 15 vowels, organized by 5 vargas (consonant groups for Katapayadi encoding) `[S]`
- **phoneme** — Greek (Modern Standard) phoneme inventory: 20 consonants + 5 vowels `[S]`
- **script** — Pre-built metadata for 8 writing systems: Latin, Arabic, Devanagari, CJK, Cyrillic, Hangul, Kana, Greek `[S]`
- **script** — `by_code()` lookup by ISO 15924 code, `all_codes()` enumeration, `Script::contains_codepoint()` for Unicode range checking
- **registry** — Language registry with ISO 639 lookup: `info()`, `phonemes()`, `primary_script()`, `all_codes()`
- **phoneme** — `LabialVelar` place of articulation variant
- **phoneme** — `Phoneme::consonant()` and `Phoneme::vowel()` constructors for external use
- **phoneme** — `PartialEq`/`Eq` derives on `Phoneme`, `PhonemeKind`, `PhonemeInventory`
- **script** — `PartialEq`/`Eq` derives on `Script`
- **grammar** — `PartialEq`/`Eq` derives on `GrammarProfile`
- **lexicon** — `PartialEq`/`Eq` derives on `LexEntry`, `Lexicon`
- 6 criterion benchmarks (3 inventories, phoneme lookup, registry lookup, script lookup)

### Changed

- **phoneme** — All string fields migrated to `Cow<'static, str>` for zero-alloc static inventories
- **phoneme** — English `/w/` reclassified from `Bilabial` to `LabialVelar` (linguistically accurate)
- **phoneme** — `english()` refactored to use `PhonemeInventoryBuilder`
- **phoneme** — `#[non_exhaustive]` added to `PhonemeKind::Consonant` and `PhonemeKind::Vowel` variants
- **script** — All string fields migrated to `Cow<'static, str>`
- **grammar** — `language_code` migrated to `Cow<'static, str>`
- **lexicon** — All string fields migrated to `Cow<'static, str>`
- Tracing instrumentation added to all public lookup methods

### Performance

- `english_phoneme_inventory`: 146ns (builder with pre-alloc)
- `sanskrit_phoneme_inventory`: 197ns
- `greek_phoneme_inventory`: 90ns
- `phoneme_lookup_ipa`: 14ns
- `registry_phonemes_lookup`: 200ns
- `script_by_code_lookup`: 19ns

## [0.1.0] - 2026-03-30

### Added

- **phoneme** — IPA phoneme inventories with articulatory features (manner, place, voicing, height, backness, rounding), stress patterns, tone systems. English (General American) inventory included
- **script** — Writing system metadata: alphabet, syllabary, logographic, abjad, abugida, mixed. Unicode ranges, directionality (LTR, RTL, TTB, bidi)
- **grammar** — Morphological typology (isolating, agglutinative, fusional, polysynthetic), word order (SVO/SOV/VSO/VOS/OVS/OSV/Free), case systems, gender, dual number, classifiers
- **lexicon** — Lexical entries with IPA transcription, part of speech, frequency ranking, Swadesh list indexing. Lookup, Swadesh extraction, frequency ranking
- **error** — `LipiError` with variants for unknown language/script, missing phonemes, invalid IPA, word-not-found
- **logging** — Optional structured logging via `LIPI_LOG` env var (feature-gated)
- Initial criterion benchmarks for phoneme inventory construction and lookup
