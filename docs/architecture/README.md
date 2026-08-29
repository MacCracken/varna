# Architecture notes

Non-obvious constraints, quirks, and invariants that a reader cannot derive from the code alone. Numbered chronologically — never renumber.

Not decisions (those live in [`../adr/`](../adr/)) and not guides (those live in [`../guides/`](../guides/)). An item here describes *how the world is*, not *what we chose* or *how to do something*.

## Items

- [001 — A numeral system's mappings vec *is* its alphabet ordering](001-numeral-mappings-are-the-ordinal-source.md) — why the order of `_nsys_add` calls is data, and why variant glyphs must stay out of the canonical tables

_Add a numbered entry (`NNN-kebab-case-title.md`) when the code has a non-obvious invariant a reader can't derive. Do not write entries for decisions — those are ADRs._
