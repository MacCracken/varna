# 001 — A numeral system's mappings vec *is* its alphabet ordering

**Applies to**: `src/numerals.cyr`

## The constraint

`NumeralSystem` stores one vec of `NumeralMapping` records. That vec carries two facts
at once, and only one of them is visible in the record:

1. **The values** — each mapping's character and its numeral value. Explicit.
2. **The alphabet ordering** — the *position* of each letter, which `NUM_ORDINAL`
   reports. Implicit, and reconstructed by walking the vec.

Nothing in a `NumeralMapping` says "I am the 19th letter." `_ordinal_at` derives that by
scanning from the start and counting. So **the order of the `_nsys_add` calls is data**,
as load-bearing as the values themselves, and an entry inserted anywhere in a table
shifts the reported position of every letter after it.

## Why this is worth writing down

Because the obvious edit is the wrong one, and it has already been made twice.

Greek isopsephy carried final sigma ς inline between σ and τ, where it belongs
alphabetically. Ordinal was then derived from the raw vec index, so ς claimed a position
of its own and **every letter from tau to omega read one too high** — τ came back 20
against a classical 19, ω 25 against 24. The values were all correct; only the ordering
was wrong, and nothing in the table looked wrong.

The same table was missing stigma ϛ, koppa ϟ and sampi ϡ, so its value run had holes at
6, 90 and 900 and `χξϛ` — 666 — returned the unmapped sentinel. The natural fix, adding
three entries in numeral sequence, would have shifted the ordinals again.

Both were fixed at 2.4.0 by making the second fact explicit rather than positional. Every
mapping now carries a `LetterRole`:

- `ROLE_LETTER` — occupies an alphabet position, and is the only role `_ordinal_at`
  counts.
- `ROLE_ALLOGRAPH` — a positional variant (Greek ς, the five Hebrew finals). Carries a
  base pointer and folds to that letter's position, which is also what classical mispar
  siduri does for Hebrew.
- `ROLE_NUMERAL_SIGN` — carries a value but is not a letter (ϛ, ϟ, ϡ). Has no position
  at all, and `NUM_ORDINAL` returns the unmapped sentinel for it rather than inventing
  one.

## What this means for anyone editing these tables

- **Adding a plain letter mid-table renumbers everything after it.** That is usually
  correct — it is a real alphabet change — but it is never local.
- **Never add a variant glyph as a plain letter.** Church Slavonic izhitsa ѵ (400), ot ѿ
  (800), monograph uk ꙋ and pre-1300 koppa ҁ (90) are all attested and all excluded on
  purpose. Appended, a variant gets a nonsense ordinal; inserted, it corrupts every
  position after it. The roadmap's variant-glyph alias map exists precisely because
  these belong in a decode-side normalisation layer, not in a canonical alphabet table.
- **A structural test will not catch a transposition.** Two letters swapping values
  leaves the multiset unchanged and every value still resolving. `tests/gematria.tcyr`
  pins all 136 values individually for that reason, and pins each alphabet's size as its
  **maximum** position rather than its last letter's — a last-letter check passes even
  when a spurious position is appended after it.

## Related

- [ADR 0001](../adr/0001-port-from-rust-to-cyrius.md) — the sentinel-return convention
  `_ordinal_at` follows.
- `docs/development/roadmap.md`, `2.4.x` — the alias map this constraint requires.
