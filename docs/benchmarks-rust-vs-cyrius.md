# Benchmarks: Rust → Cyrius

A like-for-like comparison of the same micro-benchmarks before and after the
v2.0 port from Rust to [Cyrius](https://github.com/MacCracken/cyrius). Both
suites benchmark the *same operations* (the Cyrius `.bcyr` was ported 1:1 from
the Rust `benches/benchmarks.rs`).

> **Read this section first.** The two numbers are **not** measured the same way,
> so the ratios are indicative, not precise. See [Methodology](#methodology).
>
> **This is a historical document, frozen at v2.0.0.** It compares the last Rust
> commit against the first Cyrius one and is not re-run; for current numbers see
> [`../BENCHMARKS.md`](../BENCHMARKS.md).
>
> **The Cyrius side here is an unoptimized parity port** — a faithful 1:1
> translation with no optimization passes at the time it was measured: naive O(n)
> linear-scan lookups, one `alloc()` per record, and pure constructors rebuilt on
> every call. That low-hanging fruit has since been picked, so these numbers
> understate current performance considerably. 2.1.2 de-allocated the
> transliteration and numeral hot paths and indexed `registry_info`; 2.1.5 made the
> 98 pre-built constructors shared singletons (removing a 2,400 B-per-call leak);
> 2.4.0 cached each mapping's byte length, cutting numeral lookup ~40%.

- **Rust** — `criterion` statistical **median**, final pre-port commit `aadcd67`
  (2026-04-01). Source: the v1.x crate's `bench-history.csv`.
- **Cyrius** — `lib/bench.cyr` wall-clock **min** over N iterations, v2.0.0 commit
  `faa6580` (2026-06-16). Source: [`../bench-history.csv`](../bench-history.csv) /
  [`../BENCHMARKS.md`](../BENCHMARKS.md).

Both on the same x86_64 Linux host.

## Directly comparable (captured in both eras)

The Rust suite only ever logged history for these six; the other twelve
benchmarks were added to `benchmarks.rs` but never baselined under criterion.

| Benchmark | Rust (criterion median) | Cyrius (bench min) | Cyrius / Rust |
|-----------|------------------------:|-------------------:|--------------:|
| `greek_phoneme_inventory`    |  89.8 ns | 1 µs   | ~11× |
| `english_phoneme_inventory`  | 143.5 ns | 2 µs   | ~14× |
| `sanskrit_phoneme_inventory` | 196.1 ns | 3 µs   | ~15× |
| `registry_phonemes_lookup`   | 209.7 ns | 4 µs   | ~19× |
| `phoneme_lookup_ipa`         |  14.5 ns | 419 ns | ~29× |
| `script_by_code_lookup`      |  19.6 ns | 838 ns | ~43× |

## Full Cyrius suite as of v2.0.0 (18 benchmarks)

`min` is the most representative per-op figure; `avg` carries the bump
allocator's accumulating overhead (see below). Rust column shown where a
criterion baseline exists.

| Benchmark | Cyrius min | Cyrius avg | Rust median |
|-----------|-----------:|-----------:|------------:|
| `phoneme_lookup_ipa`            | 419 ns | 1 µs  | 14.5 ns |
| `script_contains_codepoint`     | 419 ns | 1 µs  | — |
| `numeral_value_of_char`         | 419 ns | 1 µs  | — |
| `grammar_by_code_lookup`        | 419 ns | 1 µs  | — |
| `allophone_realize`             | 419 ns | 1 µs  | — |
| `phonotactics_is_permitted`     | 419 ns | 1 µs  | — |
| `script_by_code_lookup`         | 838 ns | 2 µs  | 19.6 ns |
| `transliterate_devanagari_char` | 907 ns | 2 µs  | — |
| `lexicon_find_word`             | 907 ns | 2 µs  | — |
| `greek_phoneme_inventory`       | 1 µs   | 3 µs  | 89.8 ns |
| `numeral_string_value_word`     | 1 µs   | 3 µs  | — |
| `english_phoneme_inventory`     | 2 µs   | 5 µs  | 143.5 ns |
| `swadesh_by_code_lookup`        | 2 µs   | 4 µs  | — |
| `sanskrit_phoneme_inventory`    | 3 µs   | 6 µs  | 196.1 ns |
| `dialect_apply_overlay`         | 3 µs   | 5 µs  | — |
| `registry_phonemes_lookup`      | 4 µs   | 6 µs  | 209.7 ns |
| `registry_all_codes_iter`       | 40 µs  | 45 µs | — |
| `transliterate_greek_word`      | 41 µs  | 47 µs | — |

## Methodology

The headline ratios overstate the real logic-only gap. Three things differ:

1. **Statistical median vs wall-clock min.** Criterion runs adaptive sampling
   with warm-up and outlier rejection and reports a median with confidence
   intervals. `lib/bench.cyr` times a fixed N-iteration loop and reports
   min/avg/max — no outlier rejection, no warm-up modelling.

2. **The bump allocator never frees.** Cyrius's allocator is a bump arena: every
   `alloc()` advances a pointer and nothing is reclaimed within a run. The
   construction benchmarks (`*_inventory`, `registry_phonemes_lookup`,
   `dialect_apply_overlay`, `*_by_code_lookup` which *build* a fresh object) do
   ~30–50 small allocations per iteration, so their Cyrius times are dominated by
   allocation, not by the linguistic logic. Rust's criterion runs reuse/free, so
   they measure mostly the logic. Even the Cyrius `min` includes one operation's
   worth of bump-alloc; `avg` includes the accumulating drift, which is why
   `avg > min` here is allocator noise, not variance in the work.

3. **Resolution.** Cyrius bench rounds to `µs` for everything but the fastest
   ops, so a reported `2 µs` is really somewhere in `1.5–2.5 µs`. The Rust
   figures are sub-nanosecond-precise. Ratios are therefore ±1 bucket.

The fairest single comparison is a **pure lookup over pre-built state**:
`phoneme_lookup_ipa` (a linear `find` over ~36 phonemes, no allocation in either
language) — Rust 14.5 ns vs Cyrius 419 ns. That ~29× is the closest thing to a
true codegen gap, and it is exactly what you would expect comparing a mature
LLVM-backed optimizing compiler against a young, self-hosting, assembly-up
compiler with no LLVM.

## What this means

The port traded raw speed for **sovereignty, parity, and a single toolchain**
(zero non-Cyrius dependencies — see [ADR 0001](adr/0001-port-from-rust-to-cyrius.md)).
That trade is sound for what varna is:

- **The operations are already cheap in absolute terms.** Every lookup is
  sub-microsecond to a few microseconds. Consumers (shabda, shabdakosh, svara,
  sankhya, …) query language data occasionally and cache results; none of these
  sit in a per-sample audio or text hot loop.
- **The slow outliers are honest, not surprising.** `registry_all_codes_iter`
  (40 µs) walks all 51 languages, and `transliterate_greek_word` (41 µs) builds a
  result string codepoint-by-codepoint with per-append allocation — both are
  allocation-bound and would shrink with an arena/pool allocator. Both were
  substantially improved after this snapshot by the 2.1.2 and 2.1.5 work.
- **Correctness held at parity.** 526 assertions reproduced the Rust oracle's
  behaviour exactly at the time of the port; the numbers above measure the same
  work, not a reduced surface. (The suite has since grown well past that — see
  [`development/state.md`](development/state.md) for the current figure.)

Where Cyrius would *win* — simpler algorithms over small data, no FFI marshalling
for downstream Cyrius consumers folding `dist/varna.cyr` directly — is not
captured by these micro-benchmarks. The realistic end-to-end win for a Cyrius
consumer is removing the Rust↔Cyrius language boundary entirely.

---

Regenerate the Cyrius side with `./scripts/bench-history.sh`. The frozen Rust
suite lived at `benches/benchmarks.rs` and `bench-history.csv` in the v1.x Rust crate,
which was removed from the tree at 2.1.4 ([ADR 0002](adr/0002-remove-the-rust-old-archive.md)).
This document is kept because the criterion figures below are not reproducible from
anything still present — recover the source with `git show 2.1.3:rust-old/benches/benchmarks.rs`.
