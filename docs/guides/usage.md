# Usage Guide

> Examples show the **ported Cyrius API shape**: the v1.x Rust module-path surface
> (`varna::phoneme::english()`) is re-exposed as snake-case free functions
> (`phoneme_english()`); `Option`/`unwrap` become sentinel-return checks (a pointer is
> `0` when absent); enums use `Enum.VARIANT` namespacing; tagged results are matched.
> Stdlib I/O helper names (`io`/`fmt`) are elided for brevity; `assert`/`streq` come from
> `lib/assert.cyr` / `lib/str.cyr`.

## Getting Started

Depend on varna from another Cyrius project via `cyrius.cyml`:

```cyml
[deps.varna]
git = "https://github.com/MacCracken/varna"
tag = "2.3.1"
modules = ["dist/varna.cyr"]
```

`cyrius deps` copies the bundle to `lib/varna.cyr`, included as:

```cyrius
include "lib/varna.cyr"
```

Optional surfaces compile in only when their `-D` define is passed to `cyrius build`:

| Define | Enables |
|--------|---------|
| _(none)_ | Core data types |
| `-D LOGGING` | `VARNA_LOG` logging init (`log` + `sakshi`) |
| `-D MCP` | MCP tool definitions and `mcp_invoke()` for AI-agent integration |
| `-D DAIMON` | AGNOS agent-framework registration |
| `-D HOOSH` | Structured LLM query types and `hoosh_answer_from_data()` |

```sh
# Core only
cyrius build src/main.cyr build/varna
# All optional integrations
cyrius build -D LOGGING -D MCP -D DAIMON -D HOOSH src/main.cyr build/varna
```

---

## Phoneme Inventories

Look up a language by ISO 639 code via the registry, then inspect its inventory:

```cyrius
var inv = registry_phonemes("sa");           # sentinel 0 if the code is unknown

var c = phoneme_consonant_count(inv);
var v = phoneme_vowel_count(inv);            # inv carries language_name + counts

# find returns a phoneme pointer, or 0 when absent
var p = phoneme_find(inv, "ʂ");
if p != 0 {
    # phoneme_ipa(p), phoneme_kind(p) available
}

# has is the existence check
assert(phoneme_has(inv, "ɖ"));               # retroflex stop
assert(!phoneme_has(inv, "θ"));              # no dental fricative in Sanskrit
```

All registered codes: `registry_all_codes()`.

---

## Builder Pattern

Construct a custom inventory with the inventory builder:

```cyrius
var b = phoneme_builder_new("xx", "Example Language");
phoneme_builder_stress(b, StressPattern.Fixed);
phoneme_builder_consonant(b, "p", Manner.Plosive, Place.Bilabial, false);
phoneme_builder_consonant(b, "t", Manner.Plosive, Place.Alveolar, false);
phoneme_builder_vowel(b, "a", Height.Open, Backness.Central, false);
phoneme_builder_vowel(b, "i", Height.Close, Backness.Front, false);
var inv = phoneme_builder_build(b);

assert(phoneme_consonant_count(inv) == 2);
assert(phoneme_vowel_count(inv) == 2);
```

`phoneme_builder_with_capacity(code, name, n)` is available when the approximate phoneme
count is known.

---

## Scripts and Writing Systems

Look up script metadata by ISO 15924 code:

```cyrius
var deva = script_by_code("Deva");           # sentinel 0 if unknown

# script_type(deva) -> ScriptType.Abugida
# script_direction(deva) -> Direction.LeftToRight
# script_status(deva) -> ScriptStatus.Living

# Test whether a Unicode code point belongs to the script
assert(script_contains_codepoint(deva, 0x0915));  # क (ka)
assert(!script_contains_codepoint(deva, 0x0041)); # 'A' is Latin
```

Registered codes: `"Latn"`, `"Arab"`, `"Deva"`, `"Hani"`, `"Cyrl"`, `"Hang"`, `"Kana"`, `"Grek"`, `"Xsux"`, `"Egyp"`.

`ScriptStatus` variants: `ScriptStatus.Living`, `ScriptStatus.Limited`, `ScriptStatus.Historical`.

---

## Transliteration

Use pre-built transliteration tables to convert between scripts:

```cyrius
# Devanagari -> IAST
var iast = translit_devanagari_iast();
assert(streq(translit_apply(iast, "क"), "ka"));
assert(streq(translit_apply(iast, "आ"), "ā"));

# Greek -> Beta Code
var beta = translit_greek_beta_code();
assert(streq(translit_apply(beta, "λογος"), "logos"));
assert(streq(translit_apply(beta, "Αθηνα"), "*aqhna"));

# Reverse map (target -> source)
var rev = translit_reverse_map(iast);
assert(streq(map_get(rev, "a"), "अ"));
```

`translit_apply()` uses greedy longest-match; unmapped characters pass through unchanged.

---

## Numeral Systems

Query script-specific numeral mappings for isopsephy, digit conversion, and ancient notation:

```cyrius
# Greek isopsephy — additive letter values
var iso = numerals_greek_isopsephy();
assert(numerals_value_of(iso, "α") == 1);
assert(numerals_value_of(iso, "ω") == 800);
assert(numerals_string_value(iso, "αω") == 801);   # 1 + 800

# Devanagari decimal digits
var deva = numerals_devanagari_digits();
assert(numerals_value_of(deva, "५") == 5);
assert(streq(numerals_char_for(deva, 7), "७"));

# Babylonian sexagesimal (base-60 cuneiform)
var bab = numerals_babylonian_sexagesimal();
assert(numerals_value_of(bab, "𒌋") == 10);          # u (ten)
assert(streq(numerals_char_for(bab, 30), "𒌍"));
```

Also available: `numerals_egyptian_hieroglyphic()`, `numerals_chinese_rod()`.

---

## Grammar Profiles

Look up typological data — morphology, word order, and case system — by ISO 639 code:

```cyrius
var de = grammar_by_code("de");

# grammar_morphology(de) -> Morphology.Fusional
# grammar_word_order(de)  -> WordOrder.SVO (V2)
# grammar_case_count(de)   == 4
# grammar_gender_count(de) == 3
# grammar_has_classifiers(de) == false

# Japanese for comparison
var ja = grammar_by_code("ja");
assert(grammar_morphology(ja) == Morphology.Agglutinative);
assert(grammar_word_order(ja) == WordOrder.SOV);
assert(grammar_has_classifiers(ja));
```

Covered languages: `en`, `ar`, `zh`, `hi`, `ja`, `es`, `fr`, `de`, `ru`, `ko`, `pt`.

---

## Swadesh Lists

Access 25-word core vocabulary per language for cross-language comparison:

```cyrius
# Fetch Spanish Swadesh-25
var es = swadesh_by_code("es");

# Iterate Swadesh entries in index order
var n = swadesh_count(es);
for i in 0..n {
    var e = swadesh_entry(es, i);
    # swadesh_entry_index(e), swadesh_entry_word(e),
    # swadesh_entry_ipa(e), swadesh_entry_gloss(e)
}

# Cross-language: find the word for "water" in every covered language
for code in swadesh_all_codes() {
    var lex = swadesh_by_code(code);
    var w = swadesh_find_gloss(lex, "water");   # sentinel 0 if absent
    if w != 0 {
        # swadesh_entry_word(w), swadesh_entry_ipa(w)
    }
}
```

Swadesh data covers: `ar`, `zh`, `hi`, `ja`, `es`, `fr`, `de`, `ru`, `ko`, `pt`.

---

## Allophone Rules

`allophone_realize()` returns the surface form of a phoneme in a given context:

```cyrius
var rules = allophone_english();

# /t/ -> [ɾ] (flap) between vowels: "water", "better"
assert(streq(allophone_realize(rules, "t", Environment.Intervocalic), "ɾ"));

# /t/ -> [tʰ] (aspirated) word-initially: "top"
assert(streq(allophone_realize(rules, "t", Environment.WordInitial), "tʰ"));

# /l/ -> [ɫ] (dark-l) in syllable coda: "feel", "milk"
assert(streq(allophone_realize(rules, "l", Environment.SyllableFinal), "ɫ"));

# No matching rule: returns the phoneme unchanged
assert(streq(allophone_realize(rules, "t", Environment.WordFinal), "t"));
```

Use `allophone_rules_for(rules, ipa)` to retrieve all rules for a given phoneme.

---

## Phonotactics

Query syllable templates and explicit onset/coda constraints:

```cyrius
var p = phonotactics_english();

# phonotactics_pattern(p) -> "(C)(C)(C)V(C)(C)(C)(C)"
assert(phonotactics_allows_onset_clusters(p));  # max_onset = 3
assert(phonotactics_allows_coda_clusters(p));   # max_coda = 4

# is_permitted is tri-state: 1 = allowed, 0 = forbidden, -1 = unspecified (sentinel)
assert(phonotactics_is_permitted(p, "str", SyllablePosition.Onset) == 1);
assert(phonotactics_is_permitted(p, "sr",  SyllablePosition.Onset) == 0);
assert(phonotactics_is_permitted(p, "br",  SyllablePosition.Onset) == -1);  # no explicit rule

# Japanese: (C)V(N) only
var ja = phonotactics_japanese();
assert(!phonotactics_allows_onset_clusters(ja));
assert(phonotactics_is_permitted(ja, "n", SyllablePosition.Coda) == 1);
```

---

## Dialect Overlays

`dialect_apply()` derives a modified phoneme inventory from a parent:

```cyrius
var rp = dialect_british_english();
# dialect_parent(rp) == "en"
assert(dialect_adds(rp, "ɒ"));    # LOT vowel, absent in General American
assert(dialect_removes(rp, "ɹ")); # non-rhotic: no post-vocalic /r/

# Apply overlay to the General American inventory
var ga = phoneme_english();
var rp_inv = dialect_apply(rp, ga);

# phoneme_language_code(rp_inv) == "en-GB"
assert(phoneme_has(rp_inv, "ɒ"));
assert(!phoneme_has(rp_inv, "ɹ"));
```

`VarietyKind` variants: `VarietyKind.Regional`, `VarietyKind.NationalStandard`,
`VarietyKind.Sociolect`, `VarietyKind.Creole`. (The v1.x `Historical` variant is
omitted — its name collides with `ScriptStatus.Historical` in Cyrius's flat enum
namespace.)

---

## Cognates and Etymology

`cognate_water()` groups related words across languages with a reconstructed proto-form:

```cyrius
var cog = cognate_water();
# cognate_proto_form(cog) -> "*wódr̥"
# cognate_language_count(cog) == 5

var en = cognate_for_language(cog, "en");
# cognate_word(en) == "water"

var de = cognate_for_language(cog, "de");
# cognate_word(de) == "Wasser"

# Record a loanword (struct literal; plain str fields — no Cow)
var etym = Etymology {
    "fr",                    # source_language
    "café",                  # source_form
    BorrowingType.Loanword,  # borrowing_type
    "18th century",          # period (empty str when none)
};
```

`BorrowingType` variants: `BorrowingType.Loanword`, `BorrowingType.Calque`,
`BorrowingType.SemanticLoan`, `BorrowingType.Inherited`.

---

## MCP Tools

Requires `-D MCP`. Exposes varna data as structured tools for AI agent frameworks (backed
by `bote-core`):

```cyrius
# Enumerate available tools
var tools = mcp_tool_definitions();
# varna_phonemes, varna_script, varna_grammar, varna_translate_ipa, varna_compare

# Invoke a tool — params is a lib/hashmap.cyr map
var params = map_new();
map_set(params, "language", "ja");

var res = mcp_invoke("varna_phonemes", params);   # res: ToolResult tagged enum
match res {
    ToolResult.Success(json) => { /* emit json */ }
    ToolResult.Error(msg)    => { /* report msg */ }
}

# Transliterate via MCP
var tp = map_new();
map_set(tp, "text",   "αβγ");
map_set(tp, "scheme", "greek_beta");
# mcp_invoke("varna_translate_ipa", tp) -> ToolResult.Success("abg")
```

---

## Daimon Registration

Requires `-D DAIMON`. Produces the registration payload for the AGNOS agent framework:

```cyrius
var reg = daimon_registration();

# daimon_name(reg), daimon_version(reg)
# daimon_supported_languages(reg) -> 51
# daimon_supported_scripts(reg)   -> 10
# daimon_capabilities(reg)        -> 6
# capabilities: phoneme_lookup, script_lookup, grammar_profile,
#               transliterate, language_compare, numeral_value
```

Serialize with the `bayan` JSON encoder — `var json = json_build(reg);` (the payload type
carries `#derive(Serialize)`) — to pass to the daimon client.

---

## Hoosh Queries

Requires `-D HOOSH`. Defines structured queries for LLM inference;
`hoosh_answer_from_data()` resolves queries varna can answer directly without an LLM:

```cyrius
# Data-resolvable: phoneme existence check (LanguageQuery is a tagged enum)
var q = LanguageQuery.ExamplesForPhoneme { "en", "θ", 3 };
var resp = hoosh_answer_from_data(q);            # sentinel 0 when not data-resolvable
if resp != 0 {
    assert(hoosh_response_source(resp) == ResponseSource.VarnaData);
    # hoosh_response_confidence(resp) == 1.0
    # hoosh_response_content(resp) -> "/θ/ is present in English (24C + 12V inventory)"
}

# Data-resolvable: phonology comparison
var q2 = hoosh_compare("en", "ja", ComparisonAspect.Phonology);
var r2 = hoosh_answer_from_data(q2);
# hoosh_response_content(r2) -> "English has 24C+12V, Japanese has ..."

# Requires LLM inference — answer_from_data returns the sentinel 0
var q3 = hoosh_identify("bonjour le monde");
assert(hoosh_answer_from_data(q3) == 0);
```

`ResponseSource` variants: `ResponseSource.VarnaData`, `ResponseSource.LlmGenerated`,
`ResponseSource.Hybrid`.
