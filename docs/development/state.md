# varna — Current State

> Refreshed every release. CLAUDE.md is preferences/process/procedures (durable);
> this file is **state** (volatile). It describes the tree as it stands *now* — the
> per-release history is [CHANGELOG.md](../../CHANGELOG.md) and open work is the
> [roadmap](roadmap.md).

## Version

**2.5.2** (2026-11-05) — **varna serializes again.** Every type v1.x serialized has JSON in
v1.x's format ([ADR 0003](../adr/0003-json-wire-format-matches-v1x-serde.md)), with v1.x's
dropped variants restored, and the `-D` surfaces on the same serializers.
`dist/varna-json.cyr` is how a consumer opts in, and `dist/varna.cyr` still needs no bayan.

Previous: **2.5.1** toolchain `6.7.9` → `6.7.18`, bayan moved to its thin
`dist/bayan-json.cyr` profile, and the finding that set 2.5.2: the port had dropped
serialization while the docs claimed it.
**2.5.0** toolchain `6.6.20` → `6.7.9` (the language minor; semantic changes
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
| Tests | 37 files, **2,964 assertions**, 0 failing, plus a fuzz harness (2.5.1 shipped 35 / 2,409) |
| Benchmarks | 42, covering every domain and every module's JSON |
| Reference coverage | 411/417 functions (98%) — a floor, not a correctness proof |

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
| `json` | JSON in the v1.x serde format (2.5.2, ADR 0003) for every type v1.x serialized, plus `registry_info`; in `dist/varna-json.cyr` only |

**`-D`-gated surfaces.** Each test `#define`s its own macro, so `cyrius test tests` exercises
all four. Default `cyrius build` excludes them.

| Define | Module | Holds |
|---|---|---|
| `-D DAIMON` | `daimon` | agent registration payload (6 capabilities, 51 languages, 19 scripts), with JSON |
| `-D HOOSH` | `hoosh` | `LanguageQuery` + `answer_from_data`, with JSON |
| `-D LOGGING` | `logging` | level-gated stderr logger; `sakshi` audit still deferred |
| `-D MCP` | `mcp` | 5 tools with v1.x's input schemas + `invoke`; payloads from `src/json.cyr`, as v1.x's `to_value` gave them; no bote |

## Dependencies

Declared in `cyrius.cyml [deps].stdlib`: string, fmt, alloc, vec, str, slice, syscalls,
io, args, assert, hashmap, fnptr, tagged, result, bench.

bayan is a named dep, not a stdlib leaf: `[deps.bayan]` at tag 1.11.1 (what the 6.7.18
pin vendors) with `modules = ["dist/bayan-json.cyr"]`, `optional = true` and
`[features] default = ["bayan"]`. Features resolve from the root manifest only, so
varna's own builds get it and consumers do not — shabda and shabdakosh declare the
bayan monolith themselves, and vendoring both files double-defines 27 JSON symbols.
`src/json.cyr` calls it, and so `dist/varna-json.cyr` does; `dist/varna.cyr` does not.

Two are *planned* for `-D` surfaces but not yet wired, so they are absent from
`cyrius.lock` until a module actually resolves them: `sakshi` (for `-D LOGGING`) and
`bote` 3.8.14 (for `-D MCP`; its sidecar is `hashmap sys` and the rest arrive through
their requires blocks; its `log` module's sakshi arrives through bote's own libro dep). (`chrono` *is* in the lock now, but only as a
transitive include of `lib/hashseed.cyr`.)

Consumers pull one of three bundles, each with a compile-verified sidecar naming the
stdlib leaves it needs and a matching `# Requires` include block: `dist/varna.cyr` or
`dist/varna-core.cyr`, `result hashmap` each in the working tree (`hashmap`'s own
requires block brings alloc / string / vec / str; `result` is for the enums' JSON codecs,
and src/phoneme.cyr includes it so distlib records it), or `dist/varna-json.cyr`,
`result fnptr hashmap`, the engine plus the serializers. That third one needs a bayan the
consumer declares, the thin profile or the monolith but never both, which is why the
sidecar leaves it out. Checked by building scratch consumers each way. The sidecar was
`hashmap` alone at 2.5.1 and 4 leaves at 2.4.3–2.5.0; before cyrius 6.6.18 the full
sidecar unioned in the whole `[deps].stdlib` (16 leaves).

compile-verify records only what reachable code needs, so a sidecar proves nothing about
code nobody calls. `fnptr` and `result` are listed because the modules include them.

## Verification

`sh scripts/check.sh` is the local gate: `cyrius deps` + lock verify + `fmt --check` +
`lint` + build (default and full `-D`) + `cyrius test tests`. `vet`, `deny` and `doc --check`
pass separately. CI runs the gate plus bench, fuzz and all three distlib bundles;
`release.yml` bundles `dist/varna.cyr` and `dist/varna-json.cyr`.

Two verification habits are load-bearing here and worth keeping:

- **Reintroduce the bug.** A test that passes both before and after a fix has not
  tested the fix. Several releases have shipped assertions that pinned the *defect*
  rather than the correction — 2.3.1 asserted a genus violation, 2.4.0 asserted an
  ordinal shift.
- **Interleave benchmark runs.** Min-of-N across separate sessions produced phantom
  regressions in four consecutive releases. Run old and new alternately in one session
  and report spreads alongside minima; if the spreads overlap, there is no result.

## Known gaps

The [roadmap](roadmap.md) is organized into releases (reorganized at 2.5.1). In brief:

- **2.5.3, data accuracy:** Somali's vowel analysis, which gates the
  `AdvancedTongueRoot` bit; the Burmese and Lao tone rows, which are under-sourced rather
  than wrong; Glottocodes, which need a source rather than a decision (generating them
  would produce fabrications).
- **2.5.4, hygiene:** caching `script_alphabet_values`; `grammar_by_code_lookup`, which
  slowed at three straight toolchain pins (209 → ~241 ns); `assert` / `bench` into
  `[test] stdlib`; `-D LOGGING` through sakshi.
- **2.6.0+** is feature work: coverage scale, typology and interchange, linguistic
  depth.

## Consumers

Folding varna today, per their manifests in `~/Repos`: **shabda**, **shabdakosh** and
**sankhya**, all at 2.4.1. **svara** is a named consumer with no `[deps.varna]` yet, and
**jnana** is not checked out here. Planned: vansh, sahifa. vidya is not a consumer; it
would only need varna if it gains speech support for the programming-language codex.
