# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures
> (durable); this file is **state** (volatile).

## Version

**2.3.2** — vitality and geography (2026-08-28): `EndangermentLevel` (with an added
`END_HISTORICAL` for classical languages), `MacroArea` and coordinates on all 51 entries.
Four vitality values corrected by audit (haw, qu, nah, plus ko sourced separately).
Cyrius pin `6.5.35` → `6.5.36`. `LanguageInfo` 48 B → 80 B. Completes the 2.3.x line.
**2.3.1** — genealogical classification (2026-08-28): `family`/`subfamily`/`genus` on
all 51 registry entries, with `registry_by_family` for grouping. Contested nodes carry
the conventional label with the objection in a comment. `LanguageInfo` 24 B → 48 B.
**2.3.0** — typological depth, grammar expansion (2026-08-27): eight WALS-style
dimensions on `GrammarProfile` (alignment, adposition order, tense, future,
evidentiality, negation, articles, adjective and relative-clause order) across the 11
profiles. Data gathered and adversarially verified — 43 of 44 values unchanged by the
reviewer. `GrammarProfile` 64 B → 136 B.
**2.2.4** — structured tone (2026-08-27): new `src/tone.cyr` parsing Chao tone-letter
notation into records with contour, register, endpoint levels and features.
**Breaking**: `phoneme_tones` returns records, not strings (`tone_letters` recovers
them). Completes the 2.2.x Phonological Depth line.
**2.2.3** — distinctive features (2026-08-27): new `src/features.cyr` deriving 25
SPE/Hayes features from the existing axes rather than storing them, with a tri-state
reader (+/−/unspecified). No struct growth, memory unchanged.
**2.2.2** — vowel features (2026-08-27): `Nasalized`, `Syllabic` and `ExtraShort`
added to the feature mask (`Long` shared with the consonant side), 118 vowels marked.
No struct growth — the bits reuse the word 2.2.1 allocated. ATR deliberately deferred
as a data item.
**2.2.1** — consonant secondary features (2026-08-27): `PhonemeFeature`, an 11-bit
mask (aspirated, breathy, palatalized, labialized, pharyngealized, tense, devoiced,
prenasalized, long, raised, lateral) with 190 consonants marked. Lateral clicks moved
from `Manner.LateralFricative` to `Manner.Plosive` + the `Lateral` bit. Phoneme record
48 B → 56 B.
**2.2.0** — phonological depth, airstream axis (2026-08-27): `Airstream`
(Pulmonic/Ejective/Implosive/Click) added to the Phoneme record as an axis orthogonal
to manner, and 38 consonants across 7 languages reclassified. The roadmap's proposal to
fold these into `Manner` was rejected — it would have erased the manner of 19 of them.
Phoneme record 40 B → 48 B.
**2.1.6** — script registry completeness (2026-08-27): the nine ISO 15924 codes the
registry named but never defined (Thai, Beng, Taml, Ethi, Hebr, Geor, Mymr, Khmr, Laoo)
are now real Scripts, so `registry_primary_script` resolves for all 51 languages. The
`registry_script_codes_resolve` invariant is strict again.
**2.1.5** — deferred hardening items (2026-08-27): the 98 pre-built data constructors
are now shared singletons (leak gone — `registry_phonemes` handed out 2,400 B per call,
now 0), MCP JSON values are escaped, and `varna_translate_ipa` returns a JSON object
like the other four tools (**breaking**). New `phoneme_clone` for callers needing a
mutable inventory.
**2.1.4** — `rust-old/` removed (2026-08-27): the frozen v1.x Rust crate (35 files,
8,386 lines, 484K) deleted from the tree per [ADR 0002](../adr/0002-remove-the-rust-old-archive.md),
with its 43 provenance comments rewritten and the doc references swept. Recoverable from
git tags `1.0.0`–`2.1.3`.
**2.1.3** — test-suite port (2026-08-27): the four standalone Rust suites under
`rust-old/tests/` (147 tests) finally carried over as `tests/invariants.tcyr`,
`adversarial.tcyr`, `integration.tcyr` and `mcp_json.tcyr`. Test-only; 21 → 25 test
files, 652 → 1,005 assertions, coverage 89% → 94%. Unblocks `rust-old/` removal.
**2.1.2** — hardening release (2026-08-27) from a P(-1) scaffold sweep: six memory-safety
defects fixed (two heap overflows reachable from `-D MCP` / `-D HOOSH` with
caller-controlled length, two UTF-8 over-reads, one output-buffer overflow), the
transliteration/numeral hot paths de-allocated and `registry_info` indexed. No API or data
changes; `registry_all_codes` now returns a shared read-only vec.
**2.1.1** — toolchain-maintenance release (2026-08-27): Cyrius pin `6.4.69` → `6.5.35`,
vendored stdlib refreshed, `cyrius.lock` regenerated (and a wrong 2.1.0 hash fixed), the
commented `[deps.bote]` recipe refreshed to `3.3.7`. No API or data changes.
Originally ported from Rust to Cyrius at **2.0.0** (2026-06-16) via `cyrius port`; 8386
lines of Rust removed from the tree at 2.1.4 (recoverable from git tags). See
[ADR 0001](../adr/0001-port-from-rust-to-cyrius.md).

## Toolchain

- **Cyrius pin**: `6.5.36` (in `cyrius.cyml [package].cyrius`)
- **Lock authority**: `cyrius deps` — it walks the include graph. 6.5.35's `cyrius lib sync`
  name-matches `[deps].stdlib` instead and yields a different 29-module set (adds the
  unreferenced `hashmap_fast.cyr`, drops `atomic.cyr` which `lib/alloc.cyr` includes), so
  locking its output breaks `cyrius deps --verify`. Sequence for a pin bump: edit the pin →
  `rm -rf lib && cyrius deps` → `cyrius deps --lock` → `cyrius deps --verify`.

## Source

- Rust reference: removed at 2.1.4. The port was verified complete at the API level
  first (19 Rust modules → 19 Cyrius modules, 150/150 `pub fn` covered, 48/48
  inventories matching by name) and its four standalone test suites ported at 2.1.3.
  Recover any v1.x file with `git show 2.1.3:rust-old/<path>`.
- Cyrius port (complete; each module was written against the v1.x Rust oracle):
  - ✅ `src/error.cyr` — VarnaError codes
  - ✅ `src/phoneme.cyr` — phoneme types, builder, `english`/`sanskrit`/`greek`
  - ✅ `src/tone.cyr` — Chao tone-letter parsing (contour/register/features)
  - ✅ `src/features.cyr` — 25 derived SPE/Hayes distinctive features (tri-state)
  - ✅ `src/inventories.cyr` — 48 extended language inventories (51 languages total)
  - ✅ `src/registry.cyr` — ISO 639 lookup: `info`/`phonemes`/`all_codes`/`primary_script[_code]`
  - ✅ `src/script.cyr` — 19 writing systems (type/direction/status/Unicode ranges,
    `by_code`/`contains_codepoint`); unblocked `registry_primary_script`
  - ✅ `src/numerals.cyr` — 5 numeral systems (Deva digits, Greek isopsephy, Babylonian,
    Egyptian, Chinese rod); UTF-8 `string_value`
  - ✅ `src/transliteration.cyr` — Devanagari↔IAST, Greek↔Beta Code (greedy longest-match, reverse map)
  - ✅ `src/util.cyr` — shared `_utf8_len` codepoint helper
  - ✅ `src/allophone.cyr` — English allophone rules (flapping/aspiration/dark-l); `realize`/`rules_for`
  - ✅ `src/syllable.cyr` — syllable templates + phonotactics (English/Sanskrit/Japanese); tri-state `is_permitted`
  - ✅ `src/grammar.cyr` — 11 typological profiles (morphology/word-order/case/gender/classifiers); `by_code`
  - ✅ `src/lexicon.cyr` — LexEntry/Lexicon (`find`/`swadesh`/`most_frequent` with sort), PartOfSpeech
  - ✅ `src/swadesh.cyr` — Swadesh-25 lists for 10 languages (250 entries)
  - ✅ `src/cognate.cyr` — water cognates + CognateSet/Etymology/BorrowingType
  - ✅ `src/dialect.cyr` — variety overlays (British English RP); `adds`/`removes`/`apply`
  - **Core data engine complete — 15 modules.** `cyrius distlib` → `dist/varna.cyr` bundles + compiles.
  - ✅ `src/daimon.cyr` (`-D DAIMON`) — agent registration (6 capabilities, 51 langs, 19 scripts)
  - ✅ `src/hoosh.cyr` (`-D HOOSH`) — `LanguageQuery` + `answer_from_data` (string-built content;
    confidence per-mille, structured_data deferred)
  - ✅ `src/logging.cyr` (`-D LOGGING`) — level-gated stderr logger (`io`); env-filter/`sakshi` deferred
  - ✅ `src/mcp.cyr` (`-D MCP`) — 5 tools + `invoke`; lightweight ToolDef + hand-built JSON (no bote/bayan)
  - **Port complete — 19 modules (15 core + 4 `-D` surfaces).** Default `cyrius build` excludes
    surfaces (973 unreachable); `-D LOGGING -D MCP -D DAIMON -D HOOSH` includes all (1023).
    `cyrius distlib` → `dist/varna.cyr`. Gating: `#ifdef NAME` body; each test `#define`s its
    macro so `cyrius tests` exercises it.

## Tests

- `cyrius tests` — green: 526 parity assertions + smoke — phoneme (32) + inventories (159) +
  registry (15) + script (37) + numerals (34) + transliteration (14) + allophone (8) + syllable (21) +
  grammar (47) + swadesh (78) + cognate (13) + dialect (11) + daimon (9) + hoosh (13) +
  logging (5) + mcp (30) + `tests/varna.tcyr` (smoke).
- Full parity audit (7-agent workflow, 2026-06-16): **0 unported public functions**; every
  divergence documented (sentinels-for-Option, tagged enums, hand-built JSON, omitted
  name-colliding enum variants). One fidelity fix applied (mcp compare `unique_to_*` fields).
- `sh scripts/check.sh` — local gate green: `cyrius deps` + `fmt --check` + `lint` (0 warnings)
  + build (default + `-D` full) + `cyrius tests`. Also `cyrius vet`/`deny`/`doc --check` pass.
- `cyrius bench tests/varna.bcyr` / `./scripts/bench-history.sh` — 18 benchmarks baselined
  (`bench-history.csv` + `BENCHMARKS.md`); covers every domain.
- **Release-ready (2.3.2):** version synced (`VERSION` / `cyrius.cyml` `${file:VERSION}` /
  daimon string / `CHANGELOG [2.3.2]`); `cyrius.lock` regenerated at 6.5.35 and
  `cyrius deps --verify` clean (29 verified, 0 failed); CI runs `check.sh` + bench + distlib;
  release.yml bundles `dist/varna.cyr`.
- **Gaps not closed by 2.4.1:** Glottocodes are the only untouched data item and still
  need an authoritative source rather than generation. Two tone rows are under-sourced
  rather than wrong and want a single-source pass — Burmese (fixed in one cell at 2.4.1,
  unsourced in three) and Lao (no source prints its [135] verbatim). Somali's vowels
  stay at Orwin's five qualities, which is what still gates the `AdvancedTongueRoot`
  bit: Yoruba and Wolof are markable now, Somali is not, and marking two of three is
  worse than marking none. 2.4.x also carries the gematria follow-ons — caching
  `script_alphabet_values`, a variant-glyph alias map, Maghrebi abjad, Glagolitic,
  mispar gadol. Next feature tier is 2.5.0.

## Dependencies

Direct (declared in `cyrius.cyml [deps].stdlib`):

- string, fmt, alloc, vec, str, slice, syscalls, io, args, assert, hashmap, bayan,
  fnptr, tagged, result, bench

Deferred to their `-D`-gated surfaces (planned, not yet wired — the surfaces
currently self-contain, so these are absent from `cyrius.lock` until a module
actually resolves them):

- `-D LOGGING` → `sakshi` (logging.cyr is a self-contained level logger today)
- `-D MCP` → `bote` (`dist/bote-core.cyr`, tag 3.3.7 — a 12-module profile) plus `chrono`,
  the only stdlib leaf in its `dist/bote-core.deps` sidecar not already declared
  (mcp.cyr hand-builds JSON today, no bote/bayan)

## Consumers

shabda, shabdakosh, svara, sankhya, jnana, vidya (planned: vansh, sahifa).

## Next

The port shipped as 2.0.0; 2.1.1 was toolchain maintenance (Cyrius 6.5.35), 2.1.2 a hardening sweep, 2.1.3 the test-suite port, 2.1.4 the `rust-old/` removal, 2.1.5 the deferred hardening items, 2.1.6 the script registry completion, 2.2.0 the airstream axis, 2.2.1 consonant secondary features, 2.2.2 vowel features, 2.2.3 distinctive features, 2.2.4 structured tone, 2.3.0 grammar expansion, 2.3.1 classification, 2.3.2 vitality and geography, 2.4.0 gematria and numeric letter values, 2.4.1 the tone and vowel data corrections.

See the [roadmap](roadmap.md) `2.1.x — Carry-over` section for the scheduled items
(2.1.6 script-registry completeness). In brief:

- **`rust-old/` was removed at 2.1.4** — see [ADR 0002](../adr/0002-remove-the-rust-old-archive.md).
  The tree is single-language again.
- **Optimization**: done at 2.1.5 — the pre-built inventories/scripts/profiles are
  build-once singletons and the registry lookups no longer allocate.
