# Usage Guide

## Getting Started

Add `lipi` to `Cargo.toml`:

```toml
[dependencies]
lipi = "0.6"
```

Feature flags:

| Flag | Enables |
|------|---------|
| `default` | Core data types, `std` support |
| `full` | All features below |
| `logging` | `tracing-subscriber` initialisation helpers |
| `mcp` | MCP tool definitions and `invoke()` for AI agent integration |
| `daimon` | AGNOS agent framework registration |
| `hoosh` | Structured LLM query types and `answer_from_data()` |

```toml
# All optional integrations
lipi = { version = "0.6", features = ["full"] }

# MCP only
lipi = { version = "0.6", features = ["mcp"] }
```

---

## Phoneme Inventories

Look up a language by ISO 639 code via the registry, then inspect its inventory:

```rust
let inv = lipi::registry::phonemes("sa").unwrap();

println!("{} — {}C + {}V", inv.language_name, inv.consonant_count(), inv.vowel_count());

// find returns Option<&Phoneme>
if let Some(p) = inv.find("ʂ") {
    println!("/{}/  {:?}", p.ipa, p.kind);
}

// has is the existence check
assert!(inv.has("ɖ"));   // retroflex stop
assert!(!inv.has("θ"));  // no dental fricative in Sanskrit
```

All registered codes: `lipi::registry::all_codes()`.

---

## Builder Pattern

Construct a custom inventory with `PhonemeInventoryBuilder`:

```rust
use lipi::phoneme::{PhonemeInventoryBuilder, Manner, Place, Height, Backness, StressPattern};

let inv = PhonemeInventoryBuilder::new("xx", "Example Language")
    .stress(StressPattern::Fixed)
    .consonant("p", Manner::Plosive, Place::Bilabial, false)
    .consonant("t", Manner::Plosive, Place::Alveolar, false)
    .vowel("a", Height::Open, Backness::Central, false)
    .vowel("i", Height::Close, Backness::Front, false)
    .build();

assert_eq!(inv.consonant_count(), 2);
assert_eq!(inv.vowel_count(), 2);
```

`with_capacity(code, name, n)` is available when the approximate phoneme count is known.

---

## Scripts and Writing Systems

Look up script metadata by ISO 15924 code:

```rust
let deva = lipi::script::by_code("Deva").unwrap();

println!("Type: {:?}", deva.script_type);   // Abugida
println!("Direction: {:?}", deva.direction); // LeftToRight
println!("Status: {:?}", deva.status);       // Living

// Test whether a Unicode code point belongs to the script
assert!(deva.contains_codepoint(0x0915)); // क (ka)
assert!(!deva.contains_codepoint(0x0041)); // 'A' is Latin
```

Registered codes: `"Latn"`, `"Arab"`, `"Deva"`, `"Hani"`, `"Cyrl"`, `"Hang"`, `"Kana"`, `"Grek"`, `"Xsux"`, `"Egyp"`.

`ScriptStatus` variants: `Living`, `Limited`, `Historical`.

---

## Transliteration

Use pre-built `TransliterationTable` instances to convert between scripts:

```rust
use lipi::script::transliteration::{devanagari_iast, greek_beta_code};

// Devanagari -> IAST
let iast = devanagari_iast();
assert_eq!(iast.transliterate("क"), "ka");
assert_eq!(iast.transliterate("आ"), "ā");

// Greek -> Beta Code
let beta = greek_beta_code();
assert_eq!(beta.transliterate("λογος"), "logos");
assert_eq!(beta.transliterate("Αθηνα"), "*aqhna");

// Reverse map (target -> source)
let rev = iast.reverse_map();
assert_eq!(rev.get("a"), Some(&"अ"));
```

`transliterate()` uses greedy longest-match; unmapped characters pass through unchanged.

---

## Numeral Systems

Query script-specific numeral mappings for isopsephy, digit conversion, and ancient notation:

```rust
use lipi::script::numerals::{greek_isopsephy, devanagari_digits, babylonian_sexagesimal};

// Greek isopsephy — additive letter values
let iso = greek_isopsephy();
assert_eq!(iso.value_of("α"), Some(1));
assert_eq!(iso.value_of("ω"), Some(800));
assert_eq!(iso.string_value("αω"), Some(801)); // 1 + 800

// Devanagari decimal digits
let deva = devanagari_digits();
assert_eq!(deva.value_of("५"), Some(5));
assert_eq!(deva.char_for(7), Some("७"));

// Babylonian sexagesimal (base-60 cuneiform)
let bab = babylonian_sexagesimal();
assert_eq!(bab.value_of("𒌋"), Some(10)); // u (ten)
assert_eq!(bab.char_for(30), Some("𒌍"));
```

Also available: `egyptian_hieroglyphic()`, `chinese_rod_numerals()`.

---

## Grammar Profiles

Look up typological data — morphology, word order, and case system — by ISO 639 code:

```rust
let de = lipi::grammar::by_code("de").unwrap();

println!("Morphology: {:?}", de.morphology);   // Fusional
println!("Word order: {:?}", de.word_order);    // SVO (V2)
println!("Cases: {}", de.case_count);           // 4
println!("Genders: {}", de.gender_count);       // 3
println!("Classifiers: {}", de.has_classifiers); // false

// Japanese for comparison
let ja = lipi::grammar::by_code("ja").unwrap();
assert_eq!(ja.morphology, lipi::grammar::Morphology::Agglutinative);
assert_eq!(ja.word_order, lipi::grammar::WordOrder::SOV);
assert!(ja.has_classifiers);
```

Covered languages: `en`, `ar`, `zh`, `hi`, `ja`, `es`, `fr`, `de`, `ru`, `ko`, `pt`.

---

## Swadesh Lists

Access 25-word core vocabulary per language for cross-language comparison:

```rust
// Fetch Spanish Swadesh-25
let es = lipi::lexicon::swadesh::by_code("es").unwrap();

// Iterate Swadesh entries in index order
for entry in es.swadesh() {
    println!("{:2}. {} ({}) — {}", entry.swadesh_index.unwrap(), entry.word, entry.ipa, entry.gloss);
}

// Cross-language: find the word for "water" in every covered language
for code in lipi::lexicon::swadesh::all_codes() {
    let lex = lipi::lexicon::swadesh::by_code(code).unwrap();
    if let Some(w) = lex.entries.iter().find(|e| e.gloss == "water") {
        println!("{code}: {} /{}/", w.word, w.ipa);
    }
}
```

Swadesh data covers: `ar`, `zh`, `hi`, `ja`, `es`, `fr`, `de`, `ru`, `ko`, `pt`.

---

## Allophone Rules

`AllophoneRuleSet::realize()` returns the surface form of a phoneme in a given context:

```rust
use lipi::phoneme::allophone::{english_allophones, Environment};

let rules = english_allophones();

// /t/ -> [ɾ] (flap) between vowels: "water", "better"
let flap = rules.realize("t", &Environment::Intervocalic);
assert_eq!(flap, "ɾ");

// /t/ -> [tʰ] (aspirated) word-initially: "top"
let asp = rules.realize("t", &Environment::WordInitial);
assert_eq!(asp, "tʰ");

// /l/ -> [ɫ] (dark-l) in syllable coda: "feel", "milk"
let dark_l = rules.realize("l", &Environment::SyllableFinal);
assert_eq!(dark_l, "ɫ");

// No matching rule: returns the phoneme unchanged
let unchanged = rules.realize("t", &Environment::WordFinal);
assert_eq!(unchanged, "t");
```

Use `rules_for(ipa)` to retrieve all rules for a given phoneme.

---

## Phonotactics

Query syllable templates and explicit onset/coda constraints:

```rust
use lipi::phoneme::syllable::{english_phonotactics, SyllablePosition};

let p = english_phonotactics();

// Syllable template
println!("Pattern: {}", p.syllable.pattern); // (C)(C)(C)V(C)(C)(C)(C)
assert!(p.syllable.allows_onset_clusters());  // max_onset = 3
assert!(p.syllable.allows_coda_clusters());   // max_coda = 4

// is_permitted: Some(true) = allowed, Some(false) = forbidden, None = unspecified
assert_eq!(p.is_permitted("str", SyllablePosition::Onset), Some(true));
assert_eq!(p.is_permitted("sr",  SyllablePosition::Onset), Some(false));
assert_eq!(p.is_permitted("br",  SyllablePosition::Onset), None); // no explicit rule

// Japanese: (C)V(N) only
let ja = lipi::phoneme::syllable::japanese_phonotactics();
assert!(!ja.syllable.allows_onset_clusters());
assert_eq!(ja.is_permitted("n", SyllablePosition::Coda), Some(true));
```

---

## Dialect Overlays

`LanguageVariety::apply()` derives a modified `PhonemeInventory` from a parent:

```rust
use lipi::dialect::british_english;

let rp = british_english();
assert_eq!(rp.parent, "en");
assert!(rp.adds("ɒ"));   // LOT vowel, absent in General American
assert!(rp.removes("ɹ")); // non-rhotic: no post-vocalic /r/

// Apply overlay to General American inventory
let ga = lipi::phoneme::english();
let rp_inv = rp.apply(&ga);

assert_eq!(rp_inv.language_code, "en-GB");
assert!(rp_inv.has("ɒ"));
assert!(!rp_inv.has("ɹ"));
```

`VarietyKind` variants: `Regional`, `NationalStandard`, `Historical`, `Sociolect`, `Creole`.

---

## Cognates and Etymology

`CognateSet` groups related words across languages with a reconstructed proto-form:

```rust
use lipi::lexicon::cognate::{water_cognates, Etymology, BorrowingType};
use std::borrow::Cow;

let cog = water_cognates();
println!("Proto-form: {}", cog.proto_form.as_deref().unwrap()); // *wódr̥
println!("Languages: {}", cog.language_count()); // 5

let en = cog.for_language("en").unwrap();
assert_eq!(en.word, "water");

let de = cog.for_language("de").unwrap();
assert_eq!(de.word, "Wasser");

// Record a loanword
let etym = Etymology {
    source_language: Cow::Borrowed("fr"),
    source_form: Cow::Borrowed("café"),
    borrowing_type: BorrowingType::Loanword,
    period: Some(Cow::Borrowed("18th century")),
};
```

`BorrowingType` variants: `Loanword`, `Calque`, `SemanticLoan`, `Inherited`.

---

## MCP Tools

Requires feature `mcp`. Exposes lipi data as structured tools for AI agent frameworks:

```rust
use lipi::mcp::{tool_definitions, invoke, ToolResult};
use std::collections::HashMap;

// Enumerate available tools
for tool in tool_definitions() {
    println!("{}: {}", tool.name, tool.description);
}
// lipi_phonemes, lipi_script, lipi_grammar, lipi_translate_ipa, lipi_compare

// Invoke a tool
let mut params = HashMap::new();
params.insert("language".to_string(), "ja".to_string());

match invoke("lipi_phonemes", &params) {
    ToolResult::Success(json) => println!("{}", json),
    ToolResult::Error(msg)   => eprintln!("error: {msg}"),
}

// Transliterate via MCP
let mut params = HashMap::new();
params.insert("text".to_string(),   "αβγ".to_string());
params.insert("scheme".to_string(), "greek_beta".to_string());
// -> ToolResult::Success("abg")
```

---

## Daimon Registration

Requires feature `daimon`. Produces the registration payload for the AGNOS agent framework:

```rust
let reg = lipi::daimon::registration();

println!("Agent: {} v{}", reg.name, reg.version);
println!("Languages: {}", reg.supported_languages.len()); // 27
println!("Scripts: {}",   reg.supported_scripts.len());   // 10
println!("Capabilities: {}", reg.capabilities.len());      // 6

for cap in &reg.capabilities {
    println!("  {} <- {:?}", cap.name, cap.inputs);
}
// phoneme_lookup, script_lookup, grammar_profile,
// transliterate, language_compare, numeral_value
```

Serialize with `serde_json::to_string(&reg)` to pass the payload to the daimon client.

---

## Hoosh Queries

Requires feature `hoosh`. Defines structured queries for LLM inference; `answer_from_data()` resolves queries that lipi can answer directly without calling an LLM:

```rust
use lipi::hoosh::{LanguageQuery, ComparisonAspect, answer_from_data, ResponseSource};

// Data-resolvable: phoneme existence check
let q = LanguageQuery::ExamplesForPhoneme {
    language: "en".into(),
    ipa: "θ".into(),
    count: 3,
};
let resp = answer_from_data(&q).unwrap();
assert_eq!(resp.source, ResponseSource::LipiData);
assert_eq!(resp.confidence, Some(1.0));
println!("{}", resp.content); // "/θ/ is present in English (24C + 12V inventory)"

// Data-resolvable: phonology comparison
let q = lipi::hoosh::compare(
    vec!["en".into(), "ja".into()],
    ComparisonAspect::Phonology,
);
let resp = answer_from_data(&q).unwrap();
println!("{}", resp.content); // "English has 24C+12V, Japanese has ..."

// Requires LLM inference — answer_from_data returns None
let q = lipi::hoosh::identify("bonjour le monde");
assert!(answer_from_data(&q).is_none());
```

`ResponseSource` variants: `LipiData`, `LlmGenerated`, `Hybrid`.
