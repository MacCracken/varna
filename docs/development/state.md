# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures (durable);
> this file is **state** (volatile). It describes the tree as it stands *now* — the
> per-release history is [CHANGELOG.md](../../CHANGELOG.md) and open work is the
> [roadmap](roadmap.md).

## Version

**2.5.1** (2026-11-05) — toolchain `6.7.9` → `6.7.18`; bayan moved from the stdlib
monolith to its thin `dist/bayan-json.cyr` profile (an optional `[deps.bayan]`, on by
default), which keeps the 6.7.18 monolith's ~353 KB of Unicode tables out and leaves
the live binary 37% smaller than at 2.5.0. bote recipe → 3.8.14. And the finding that
sets the next release: **varna does not serialize** — the port dropped v1.x's serde
surface, while the docs claimed it. That is roadmap **2.5.2 (P0)**.

Previous: **2.5.0** toolchain `6.6.20` → `6.7.9` (the language minor; semantic changes
swept, none touch varna; suite passes under `--bounds`); **2.4.3** toolchain `6.6.2` → `6.6.20`, dist bundles under 6.6.18+
`distlib` (sidecar 16 → 4 leaves), daimon version held to `VERSION`; **2.4.2** toolchain `6.5.36` → `6.6.2`; **2.4.1** data corrections (Thai
falling tone, Vietnamese huyền, Burmese checked tone, Wolof vowels,
`phoneme_is_breathy`); **2.4.0** gematria and numeric letter values; **2.3.x** typological depth
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
| Tests | 35 files, **2,409 assertions**, 0 failing |
| Benchmarks | 22, covering every domain |
| Reference coverage | 328/350 functions (93%) — a floor, not a correctness proof |

## Toolchain

- **Cyrius pin**: `6.7.18` (in `cyrius.cyml [package].cyrius`)
- **Lock**: `cyrius deps --verify` clean — 35 verified, 0 failed. The lock also
  records the toolchain (`cyrius 6.7.18`) as its last row, and a `commit` row pinning
  bayan 1.11.1 (`965298d`). `cyrius deps --locked`
  (6.7.6+) checks lib/ and the lock against the tags without writing anything.
- **Lock authority is `cyrius deps`**, which walks the include graph. `cyrius lib sync`
  name-matches `[deps].stdlib` instead and resolves a *different* set (at 6.5.x it added
  the unreferenced `hashmap_fast.cyr` and dropped `atomic.cyr`, which `lib/alloc.cyr`
  includes), so locking its output breaks `--verify`. Sequence for a pin bump: edit the
  pin → `rm -rf lib && cyrius deps` (from 6.6.20 this writes the lock itself) →
  `cyrius deps --verify` → `cyrius distlib && cyrius distlib core`. Read cyrius's
  `docs/development/ecosystem-migration.md` at the new tag first: it lists every
  change that reaches consumer code, newest first.
- **Benchmarks across a pin bump**: `lib/bench.cyr` changes with the toolchain, and
  `min` is not comparable across 6.6.9 (its "regime 6" stopped subtracting a mean clock
  floor from a minimum). To separate harness from codegen, build `tests/varna.bcyr`
  with the *old* harness under the *new* compiler — see the 2.4.3 CHANGELOG entry.
  `lib/bench.cyr` is byte-identical between 6.6.20 and 6.7.9.

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

**`-D`-gated surfaces.** Each test `#define`s its own macro, so `cyrius test tests` exercises
all four. Default `cyrius build` excludes them.

| Define | Module | Holds |
|---|---|---|
| `-D DAIMON` | `daimon` | agent registration payload (6 capabilities, 51 languages, 19 scripts) |
| `-D HOOSH` | `hoosh` | `LanguageQuery` + `answer_from_data` |
| `-D LOGGING` | `logging` | level-gated stderr logger; `sakshi` audit still deferred |
| `-D MCP` | `mcp` | 5 tools + `invoke`; hand-built JSON, no bote/bayan yet (2.5.2 moves it onto bayan) |

## Dependencies

Declared in `cyrius.cyml [deps].stdlib`: string, fmt, alloc, vec, str, slice, syscalls,
io, args, assert, hashmap, fnptr, tagged, result, bench.

bayan is a named dep, not a stdlib leaf: `[deps.bayan]` at tag 1.11.1 (what the 6.7.18
pin vendors) with `modules = ["dist/bayan-json.cyr"]`, `optional = true` and
`[features] default = ["bayan"]`. Features resolve from the root manifest only, so
varna's own builds get it and consumers do not — shabda and shabdakosh declare the
bayan monolith themselves, and vendoring both files double-defines 27 JSON symbols.
Nothing calls it yet; it is in place for 2.5.2.

Two are *planned* for `-D` surfaces but not yet wired, so they are absent from
`cyrius.lock` until a module actually resolves them: `sakshi` (for `-D LOGGING`) and
`bote` 3.8.14 (for `-D MCP`; its sidecar is `hashmap sys` and the rest arrive through
their requires blocks; its `log` module's sakshi arrives through bote's own libro dep). (`chrono` *is* in the lock now, but only as a
transitive include of `lib/hashseed.cyr`.)

Consumers pull one of two bundles, each with a compile-verified sidecar naming the
stdlib leaves it needs and a matching `# Requires` include block: `dist/varna.cyr` or
`dist/varna-core.cyr`, one leaf each at 2.5.1: `hashmap`, whose own requires block
brings alloc / string / vec / str (checked by building a scratch consumer that declares
only `hashmap`). It was 4 leaves at 2.4.3–2.5.0; before cyrius 6.6.18 the full sidecar
unioned in the whole `[deps].stdlib` (16 leaves).

## Verification

`sh scripts/check.sh` is the local gate: `cyrius deps` + lock verify + `fmt --check` +
`lint` + build (default and full `-D`) + `cyrius test tests`. `vet`, `deny` and `doc --check`
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
- **Serialization did not survive the port** — not just its tests. No varna type emits
  or reads JSON; v1.x derived serde on 47 types across 13 modules and round-tripped
  them in `serde_roundtrip.rs` (27 tests, never ported). `src/mcp.cyr` hand-builds its
  JSON. Roadmap **2.5.2 (P0)**.

Next release is **2.5.2** (bayan / serde integration, P0), ahead of the 2.5.0 coverage
tier.

## Consumers

shabda, shabdakosh, svara, sankhya, jnana, vidya. Planned: vansh, sahifa.
