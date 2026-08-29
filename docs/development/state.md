# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures (durable);
> this file is **state** (volatile). It describes the tree as it stands *now* — the
> per-release history is [CHANGELOG.md](../../CHANGELOG.md) and open work is the
> [roadmap](roadmap.md).

## Version

**2.4.1** (2026-08-28) — data corrections. Thai had no falling tone, Vietnamese
huyền was Mandarin's third tone, Burmese's checked tone was low where every source
says high, and Wolof's vowel system was four qualities. `phoneme_is_breathy` closes
the Sanskrit voiced-aspirate item by deciding *not* to retranscribe.

Previous: **2.4.0** gematria and numeric letter values; **2.3.x** typological depth
(grammar, classification, vitality and geography); **2.2.x** phonological depth
(airstream, secondary features, distinctive features, structured tone); **2.1.x**
toolchain, hardening, the test-suite port and the `rust-old/` removal; **2.0.0** the
Rust → Cyrius port ([ADR 0001](../adr/0001-port-from-rust-to-cyrius.md)).

## Shape

| | |
|---|---|
| Languages | 51, each with a phoneme inventory and a resolving primary script |
| Scripts | 19 |
| Numeral systems | 9 — five of them letter-value (Hebrew, Arabic, Latin, Cyrillic, Greek) |
| Grammar profiles | 11 |
| Source modules | 22 — 18 core, 4 behind `-D` defines |
| Tests | 35 files, **2,442 assertions**, 0 failing |
| Benchmarks | 23, covering every domain |
| Reference coverage | 328/350 functions (93%) — a floor, not a correctness proof |

## Toolchain

- **Cyrius pin**: `6.5.36` (in `cyrius.cyml [package].cyrius`)
- **Lock**: `cyrius deps --verify` clean — 29 verified, 0 failed
- **Lock authority is `cyrius deps`**, which walks the include graph. `cyrius lib sync`
  name-matches `[deps].stdlib` instead and yields a *different* 29-module set (adds the
  unreferenced `hashmap_fast.cyr`, drops `atomic.cyr`, which `lib/alloc.cyr` includes),
  so locking its output breaks `--verify`. Sequence for a pin bump: edit the pin →
  `rm -rf lib && cyrius deps` → `cyrius deps --lock` → `cyrius deps --verify`.

## Modules

**Core (always compiled).** Every one was written against the v1.x Rust oracle during
the port and has since been extended past it.

| Module | Now holds |
|---|---|
| `error` | `VarnaError` codes |
| `util` | shared UTF-8 codepoint helpers |
| `phoneme` | phoneme record + builder, `Airstream`, a 14-bit feature mask, `phoneme_is_breathy` |
| `features` | 25 derived SPE/Hayes distinctive features, tri-state (+/−/unspecified) |
| `tone` | Chao tone-letter parsing → contour, register, endpoint levels, creak/checked |
| `inventories` | the 48 extended inventories behind the 51-language corpus |
| `allophone` | English allophone rules (flapping, aspiration, dark l) |
| `syllable` | syllable templates + phonotactics (English, Sanskrit, Japanese) |
| `script` | 19 writing systems — type, direction, status, Unicode ranges |
| `numerals` | 9 numeral systems + the gematria layer (standard / ordinal / reduced) |
| `transliteration` | Devanagari↔IAST, Greek↔Beta Code (greedy longest-match) |
| `grammar` | 11 typological profiles, eight WALS-style dimensions each |
| `lexicon` | `LexEntry`/`Lexicon`, `PartOfSpeech`, frequency ranking |
| `swadesh` | Swadesh-25 for 10 languages (250 entries) |
| `cognate` | cognate sets, etymology, borrowing type |
| `dialect` | variety overlays (British English RP) |
| `registry` | ISO 639 lookup across every module, plus family, endangerment, geography |

**`-D`-gated surfaces.** Each test `#define`s its own macro, so `cyrius tests` exercises
all four. Default `cyrius build` excludes them.

| Define | Module | Holds |
|---|---|---|
| `-D DAIMON` | `daimon` | agent registration payload (6 capabilities, 51 languages, 19 scripts) |
| `-D HOOSH` | `hoosh` | `LanguageQuery` + `answer_from_data` |
| `-D LOGGING` | `logging` | level-gated stderr logger; `sakshi` audit still deferred |
| `-D MCP` | `mcp` | 5 tools + `invoke`; hand-built JSON, no bote/bayan yet |

## Dependencies

Declared in `cyrius.cyml [deps].stdlib`: string, fmt, alloc, vec, str, slice, syscalls,
io, args, assert, hashmap, bayan, fnptr, tagged, result, bench.

Two are *planned* for `-D` surfaces but not yet wired, so they are absent from
`cyrius.lock` until a module actually resolves them: `sakshi` (for `-D LOGGING`) and
`bote` + `chrono` (for `-D MCP`).

Consumers pull one of two bundles, each with a generated sidecar naming the stdlib
leaves it needs: `dist/varna.cyr` (16 leaves) or `dist/varna-core.cyr` (5 — string,
alloc, vec, assert, hashmap), which drops the `-D` surfaces.

## Verification

`sh scripts/check.sh` is the local gate: `cyrius deps` + lock verify + `fmt --check` +
`lint` + build (default and full `-D`) + `cyrius tests`. `vet`, `deny` and `doc --check`
pass separately. CI runs the gate plus bench and distlib; `release.yml` bundles
`dist/varna.cyr`.

Two verification habits are load-bearing here and worth keeping:

- **Reintroduce the bug.** A test that passes both before and after a fix has not
  tested the fix. Several releases have shipped assertions that pinned the *defect*
  rather than the correction — 2.3.1 asserted a genus violation, 2.4.0 asserted an
  ordinal shift.
- **Interleave benchmark runs.** Min-of-N across separate sessions produced phantom
  regressions in four consecutive releases. Run old and new alternately in one session
  and report spreads alongside minima; if the spreads overlap, there is no result.

## Known gaps

Full detail in the [roadmap](roadmap.md); in brief —

- **Glottocodes** are the one untouched data item, and need a source rather than a
  decision. Generating them would produce fabrications indistinguishable from real
  identifiers.
- **Somali's vowel analysis** gates the `AdvancedTongueRoot` bit. Yoruba and Wolof are
  markable as of 2.4.1; marking two of three is worse than marking none.
- **Burmese and Lao tone rows** are under-sourced rather than wrong, and want a
  single-source pass instead of cell-by-cell patching.
- **Gematria follow-ons** carried out of 2.4.0: caching `script_alphabet_values`, a
  variant-glyph alias map, the Maghrebi abjad, Glagolitic, mispar gadol.
- **JSON round-trip tests did not survive the port.** The Rust `serde_roundtrip.rs`
  suite was never carried over and `src/mcp.cyr` still hand-builds its JSON.

Next feature tier is **2.5.0** (coverage scale).

## Consumers

shabda, shabdakosh, svara, sankhya, jnana, vidya. Planned: vansh, sahifa.
