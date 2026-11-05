# 0003 — JSON wire format matches v1.x serde

**Status**: Accepted
**Date**: 2026-11-05

## Context

The v1.x Rust crate derived `Serialize` / `Deserialize` on its public types: 21 structs
and 23 enums, with no `#[serde(...)]` attribute anywhere. So every type used serde's
default representation:

- a struct is an object with its fields in declaration order;
- a unit enum variant is its name as a string (`"Plosive"`);
- a data-carrying variant is externally tagged, an object with one key
  (`{"Consonant":{...}}`);
- `Option::None` is `null`, and a `Vec` is an array.

Its MCP tools served that JSON to AI agents (`varna_phonemes` returned a whole inventory
through `serde_json::to_value`), and `tests/serde_roundtrip.rs` round-tripped every
public type in 27 tests.

The Rust → Cyrius port carried none of it (see ADR 0001's 2.5.1 amendment). Restoring
it is roadmap 2.5.2, and the first question is the wire format. A scratch probe on
cyrius 6.7.18 established what the toolchain offers:

- A `#derive(Serialize)` struct declared over an existing varna record matches its
  layout. The 56-byte phoneme record passes derive's build-time offset check, since
  `cstring`, untyped and `bool` fields are all 8-byte slots. Both generated decoders
  round-trip every field.
- **A struct derive cannot write an enum field by name.** An enum-typed field fails
  the layout check. An untyped one is written as its number (`"manner":8`).
- An enum's own `#derive(Serialize)` generates a by-name codec: `E_to_json(v, sb)`
  writes `"Affricate"`, and `E_from_json_str` returns `Ok(value)`, or `Err` for an
  unknown name.

So the choice was between names, the v1.x format, which needs a short hand-written
writer and reader per record that call the derived enum codecs, and numbers, which
are nearly pure derive. Names were chosen on 2026-11-05.

## Decision

**varna's JSON is the format v1.x serde produced, field for field, with names for
every enum value.** Fields varna gained after v1.x are added inside the same objects,
and a v1.x document still decodes.

- **Enums** get `#derive(Serialize)` on their declaration. That is the only enum codec
  varna has, so the names come from the enum constants themselves.
- **Records** get a hand-written `<module>_to_json(record, sb)` writer, which appends
  to a `str_builder`, and a `<module>_from_json(text)` reader, which returns a new
  record or 0. Both live in `src/json.cyr` and call the derived enum codecs. Readers
  parse with bayan's value tree (`bayan_json_v_*`, the `dist/bayan-json.cyr`
  profile).
- **Readers are as strict as serde's defaults.** A missing required field, a wrong
  JSON type (a number where a name or a `bool` belongs), an unknown enum name, or an
  externally tagged object with other than one key makes the reader return 0.
  Unknown extra keys are ignored, as serde ignores them by default. A field v1.x
  did not have is optional on read and takes the constructor's default, so v1.x
  documents decode.
- **A bit set** (`PhonemeFeature`) is written as an array of flag names in ascending
  bit order. An empty set is `[]`.

For a phoneme, v1.x wrote:

```json
{"ipa":"p","kind":{"Consonant":{"manner":"Plosive","place":"Bilabial","voiced":false}}}
```

varna writes the same, plus the two fields added in 2.2.x:

```json
{"ipa":"pʰ","kind":{"Consonant":{"manner":"Plosive","place":"Bilabial","voiced":false,"airstream":"Pulmonic","features":["Aspirated"]}}}
```

## Consequences

- **Positive**
  - v1.x consumers and stored v1.x JSON read unchanged.
  - The documents describe themselves, and AI agents get names, as the v1.x MCP tools
    gave them.
  - Bad input is caught: an unknown name is an error, not a silently stored integer.
- **Negative**
  - Each record type carries a hand-written writer and reader (~21 types across the
    modules), each of which has to be kept in step with its record's accessors.
  - Renaming an enum variant now breaks stored JSON. The names are the contract.
  - Documents are longer than the numeric form would be.
- **Neutral**
  - The struct derive is unused for records. It stays available if a record ever
    becomes a real `struct` with no enum fields.
  - The serializers live in `src/json.cyr`, which stays out of `dist/` until 2.5.2's
    consumer-hazard bite settles how bayan reaches consumers.

## Alternatives considered

- **Numbers, derive only.** Almost no hand-written code, and stable, because varna's
  enum values are explicit constants. Rejected because it breaks v1.x compatibility,
  the documents are unreadable without varna's source, and an out-of-range number
  would decode silently.
- **Names, flat** (`{"ipa":..,"kind":"Consonant","manner":..}`). The same hand-written
  cost as the decision, and no v1.x compatibility, so it was dominated.
- **Hand-written enum name tables** instead of the enum derive. Rejected because it
  duplicates the enum declarations, which the derive's generated code reads directly.
