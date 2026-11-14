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
tag = "2.5.4"
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

All registered codes: `registry_all_codes()`. Each entry also carries its Glottolog
identifier: `registry_info_glottocode(registry_info("lo"))` is `"laoo1244"`, and
`registry_by_glottocode("laoo1244")` finds the entry again.

---

## Provenance

An inventory names the works its values are taken from, each with what it covers. The
works themselves are in one bibliography, keyed:

```cyrius
var links = inventory_sources(phoneme_thai());
# [0] key "tingsabadh1993", covers "consonants, vowels and diphthongs"
# [1] key "teeranon2007",   covers "tones"
var w = source_by_key(source_link_key(vec_get(links, 0)));
# source_authors(w) -> "Tingsabadh, M. R. Kalaya; Abramson, Arthur S."
# source_year(w)    -> 1993
# source_venue(w)   -> "Journal of the International Phonetic Association 23(1)"
# source_doi(w)     -> "10.1017/s0025100300004746"

# Most inventories cite nothing yet, and say so.
assert(inventory_is_sourced(phoneme_finnish()) == 0);
```

A link goes to a work the values are taken from; works that only agree, or that could
not be read, stay in the inventory's comment. `source_all_keys()` lists the
bibliography. Every entry has a DOI or the URL its record was read from.

The other datasets link the same way: `phonotactics_sources(p)`,
`allophone_sources(set)`, `translit_sources(table)` and `numerals_sources(sys)`.
For example, `translit_sources(translit_zhuyin_pinyin())` names the 1958 Pinyin
scheme, The Unicode Standard 18.0.0 and the Taiwan Ministry of Education's manual.

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

Registered codes (`script_all_codes()`, 21): `"Latn"`, `"Arab"`, `"Deva"`, `"Hani"`, `"Cyrl"`,
`"Hang"`, `"Kana"`, `"Grek"`, `"Xsux"`, `"Egyp"`, `"Thai"`, `"Beng"`, `"Taml"`, `"Ethi"`, `"Hebr"`,
`"Geor"`, `"Mymr"`, `"Khmr"`, `"Laoo"`, `"Glag"`, `"Bopo"`.

`ScriptStatus` variants: `ScriptStatus.Living`, `ScriptStatus.Limited`, `ScriptStatus.Historical`.

---

## Transliteration

Six pre-built tables convert a script to Latin. Each is reachable by its own function
or by scheme name:

```cyrius
# Devanagari -> IAST: the inherent vowel a is written unless a vowel sign or virama follows
var iast = translit_devanagari_iast();
assert(streq(translit_apply(iast, "नमस्ते"), "namaste"));
assert(streq(translit_apply(iast, "क"), "ka"));
assert(streq(translit_char(iast, "क"), "k"));      # the pair's own target is bare

# Greek -> Beta Code, accents in the TLG's order
var beta = translit_greek_beta_code();
assert(streq(translit_apply(beta, "λόγος"), "lo/gos"));
assert(streq(translit_apply(beta, "Ἀθῆναι"), "*)aqh=nai"));

# ALA-LC Russian, Arabic and Hebrew (Arabic and Hebrew need vowelled text)
assert(streq(translit_apply(translit_russian_alalc(), "щука"), "shchuka"));
assert(streq(translit_apply(translit_by_scheme("arabic_alalc"), "كِتَاب"), "kitāb"));

# Zhuyin -> Pinyin, by syllable
assert(streq(translit_apply(translit_zhuyin_pinyin(), "ㄋㄧˇ ㄏㄠˇ"), "nǐ hǎo"));

# Every scheme name: devanagari_iast greek_beta russian_alalc arabic_alalc
# hebrew_alalc zhuyin_pinyin
var schemes = translit_all_schemes();

# Reverse map (target -> source)
var rev = translit_reverse_map(iast);
assert(streq(map_get(rev, "a"), "अ"));
```

`translit_apply()` uses greedy longest-match over up to four codepoints; unmapped
characters pass through unchanged. The Arabic and Hebrew tables are letter by letter:
unvowelled text gives the consonants (كتب → ktb), and the ALA-LC rules that need the
whole word are listed in each table's comment.

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

### Gematria — letter values across scripts

`char_value` is the unified lookup: pick a script and a method, get a number.

```cyrius
# Standard values — the letter's own numeral
assert(char_value("Hebr", NumericSystem.NUM_STANDARD, "ת") == 400);
assert(char_value("Arab", NumericSystem.NUM_STANDARD, "غ") == 1000);
assert(char_value("Grek", NumericSystem.NUM_STANDARD, "ω") == 800);

# Whole words
assert(string_value_in("Hebr", NumericSystem.NUM_STANDARD, "חי") == 18);
assert(string_value_in("Grek", NumericSystem.NUM_STANDARD, "χξϛ") == 666);

# Ordinal is the letter's POSITION; reduced is the standard value's digital root
assert(char_value("Hebr", NumericSystem.NUM_ORDINAL, "ת") == 22);
assert(char_value("Hebr", NumericSystem.NUM_REDUCED, "ת") == 4);   # 400 -> 4

# The full table for a script, in alphabet order: 16-byte [0]=char [8]=value pairs
var tbl = script_alphabet_values("Latn", NumericSystem.NUM_STANDARD);
assert(vec_len(tbl) == 26);
```

Five scripts carry letter values — `gematria_all_scripts()` lists them: `Hebr`,
`Arab`, `Latn`, `Cyrl`, `Grek`. An unmapped character returns `(0 - 1)`, and
`string_value_in` propagates that for the whole string rather than silently
skipping it, so unnormalised input fails loudly.

Three things to know before wiring this to a cipher:

- **Ordinal is one-based**, while the mod-26 arithmetic of Caesar, shift and
  affine ciphers is zero-based. Subtract one, or every result is off by a letter.
- **Lookup is case-sensitive.** Uppercase returns the unmapped sentinel; lowercase
  first.
- **Not every mapped character is a letter.** Greek ϛ, ϟ and ϡ are numeral-only
  signs with a standard value but no alphabet position, so `NUM_ORDINAL` returns
  the sentinel for them; and an allograph such as Greek ς or the Hebrew finals
  folds to its base letter's position. `numerals_mapping_role()` tells them apart.

The tables encode one recension each — Arabic is Mashriqi, Cyrillic is the
standardised post-1300 Church Slavonic — and variant glyphs are deliberately
excluded. See the roadmap's `2.4.x` tier for the alias layer that will handle them.

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

# is_permitted is tri-state: 1 = allowed, 0 = forbidden, -1 = unspecified (sentinel).
# Sequences are spelled in the inventory's symbols: English /r/ is ɹ.
assert(phonotactics_is_permitted(p, "stɹ", SyllablePosition.Onset) == 1);
assert(phonotactics_is_permitted(p, "tl",  SyllablePosition.Onset) == 0);
assert(phonotactics_is_permitted(p, "bɹ",  SyllablePosition.Onset) == -1);  # no explicit rule

# Japanese (Kubozono 2015): (Cj)V(VC), so Cj onsets and a moraic nasal or obstruent coda
var ja = phonotactics_japanese();
assert(phonotactics_allows_onset_clusters(ja));  # max_onset = 2
assert(phonotactics_is_permitted(ja, "ɴ", SyllablePosition.Coda) == 1);

# By ISO 639 code: the 11 core languages and Sanskrit; 0 for any other code
var de = phonotactics_by_code("de");
assert(phonotactics_is_permitted(de, "b", SyllablePosition.Coda) == 0);  # final devoicing
assert(phonotactics_by_code("xx") == 0);
```

Unspecified is not a ban: a Permitted list is what its source enumerates, and a
Forbidden list is what it explicitly excludes. Each profile's comment in
`src/syllable.cyr` names its source; French is partial.

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

## JSON

varna reads and writes the format v1.x's serde did, field for field, with every enum by
name ([ADR 0003](../adr/0003-json-wire-format-matches-v1x-serde.md)). A writer appends one
value to a `str_builder` and returns 0, or -1 with nothing written. A reader takes
NUL-terminated text and returns a new record, or 0.

```cyrius
var sb = str_builder_new();
if (phoneme_inventory_to_json(phoneme_english(), sb) != 0) { return 1; }
var text = str_cstr(str_builder_build(sb));

var inv = phoneme_inventory_from_json(text);
if (inv == 0) { return 1; }
```

| Type | Writer and reader, each `_to_json` / `_from_json` |
|---|---|
| Phonemes | `phoneme`, `phoneme_inventory` |
| Phonology | `allophone_rule`, `allophone_rule_set`, `syllable_template`, `phonotactic_constraint`, `phonotactics` |
| Script | `script`, `numeral_mapping`, `numeral_system`, `transliteration_table` |
| Grammar | `grammar_profile` |
| Lexicon | `lex_entry`, `lexicon` (also every Swadesh list), `cognate_entry`, `cognate_set`, `etymology` |
| Dialect and registry | `language_variety`, `registry_info` |
| `-D MCP` | `tool_result`, `tool_def`, `tool_definitions` |
| `-D DAIMON` | `capability`, `agent_registration` |
| `-D HOOSH` | `language_query`, `query_response` |

Consumers get these from `dist/varna-json.cyr`, which needs a bayan declared alongside it.
The README's quick start shows both ways.

A document from v1.x decodes, since a field varna gained later is optional on read:

```json
{"ipa":"p","kind":{"Consonant":{"manner":"Plosive","place":"Bilabial","voiced":false}}}
```

Readers are as strict as serde's defaults. A missing field, a wrong JSON type, an unknown
name, a duplicate key, invalid UTF-8 or a raw control byte in a string refuses the whole
document, and unknown extra keys are ignored. Two things serde would hold and varna cannot
are also refused: a string containing U+0000, and a `confidence` outside 0.0–1.0.

v1.x's two parametrized environments carry a class, which is a label matched by equality:

```cyrius
var e = EnvBefore(ClassSpecific("k"));              # {"Before":{"Specific":"k"}}
var f = EnvAfter(PhonemeClass.ClassVowel);          # {"After":"Vowel"}
environment_eq(e, EnvBefore(ClassSpecific("k")));   # 1: compared by value
```

---

## MCP Tools

Requires `-D MCP`. Exposes varna data as structured tools for AI agent frameworks:

```cyrius
# Enumerate available tools
var tools = mcp_tool_definitions();
# varna_phonemes, varna_script, varna_grammar, varna_translate_ipa, varna_compare

# Invoke a tool — params is a lib/hashmap.cyr map
var params = map_new();
map_set(params, "language", "ja");

var res = mcp_invoke("varna_phonemes", params);
if (mcp_result_is_ok(res) == 1) {
    # A JSON object: here, the whole inventory, as phoneme_inventory_to_json writes it.
    println(mcp_result_payload(res));
} else {
    # A plain message, not JSON.
    println(mcp_result_payload(res));
}

# Transliterate via MCP
var tp = map_new();
map_set(tp, "text",   "αβγ");
map_set(tp, "scheme", "greek_beta");
# mcp_invoke("varna_translate_ipa", tp) -> {"scheme":"greek_beta","input":"αβγ","output":"abg"}
```

`tool_result_to_json(res, sb)` writes the result as v1.x did, `{"Success":..}` or
`{"Error":".."}`.

Each definition carries its input schema, so an agent can see what to pass:

```cyrius
var t = vec_get(mcp_tool_definitions(), 1);       # varna_script
vec_get(tool_def_required(t), 0);                 # "code"
tool_param_name(vec_get(tool_def_properties(t), 0));   # "code"

# A host answering tools/list wants the schema alone, under its own `inputSchema` key:
var sb = str_builder_new();
tool_input_schema_to_json(t, sb);
# {"type":"object","properties":{"code":{"description":"..","type":"string"}},"required":["code"]}
```

`tool_definitions_to_json(tools, sb)` writes the whole list in v1.x's serde form, where the
key is `input_schema`.

---

## Daimon Registration

Requires `-D DAIMON`. Produces the registration payload for the AGNOS agent framework:

```cyrius
var reg = daimon_registration();

# daimon_name(reg), daimon_version(reg)
# daimon_supported_languages(reg) -> 51
# daimon_supported_scripts(reg)   -> 19
# daimon_capabilities(reg)        -> 6
# capabilities: phoneme_lookup, script_lookup, grammar_profile,
#               transliterate, language_compare, numeral_value
```

`agent_registration_to_json(reg, sb)` encodes it in the v1.x format.

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
