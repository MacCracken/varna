//! Adversarial tests — edge-case and boundary inputs for all public APIs.
//!
//! These tests probe the library with empty strings, invalid codes, Unicode
//! edge cases, and boundary values to ensure graceful handling.

use std::borrow::Cow;

// ---------------------------------------------------------------------------
// Registry — empty, invalid, and boundary inputs
// ---------------------------------------------------------------------------

#[test]
fn registry_info_empty_string() {
    assert!(varna::registry::info("").is_none());
}

#[test]
fn registry_info_very_long_code() {
    let long = "a".repeat(1000);
    assert!(varna::registry::info(&long).is_none());
}

#[test]
fn registry_info_unicode_garbage() {
    assert!(varna::registry::info("🔥💀").is_none());
    assert!(varna::registry::info("\0").is_none());
    assert!(varna::registry::info("\u{FEFF}").is_none()); // BOM
    assert!(varna::registry::info("en\0").is_none());
}

#[test]
fn registry_phonemes_empty_string() {
    assert!(varna::registry::phonemes("").is_none());
}

#[test]
fn registry_phonemes_unicode_garbage() {
    assert!(varna::registry::phonemes("🔥").is_none());
    assert!(varna::registry::phonemes("\0\0\0").is_none());
}

#[test]
fn registry_primary_script_empty() {
    assert!(varna::registry::primary_script("").is_none());
}

#[test]
fn registry_primary_script_invalid() {
    assert!(varna::registry::primary_script("zzzzz").is_none());
}

// ---------------------------------------------------------------------------
// Script — empty and invalid lookups
// ---------------------------------------------------------------------------

#[test]
fn script_by_code_empty() {
    assert!(varna::script::by_code("").is_none());
}

#[test]
fn script_by_code_lowercase() {
    // ISO 15924 codes are case-sensitive (Latn, not latn)
    assert!(varna::script::by_code("latn").is_none());
}

#[test]
fn script_by_code_unicode_garbage() {
    assert!(varna::script::by_code("🔥🔥🔥🔥").is_none());
}

#[test]
fn script_contains_codepoint_zero() {
    let s = varna::script::latin();
    assert!(!s.contains_codepoint(0));
}

#[test]
fn script_contains_codepoint_max() {
    let s = varna::script::latin();
    assert!(!s.contains_codepoint(u32::MAX));
}

#[test]
fn script_contains_codepoint_just_outside_ranges() {
    let s = varna::script::devanagari();
    // Devanagari range: 0x0900-0x097F
    assert!(!s.contains_codepoint(0x08FF));
    assert!(s.contains_codepoint(0x0900));
    assert!(s.contains_codepoint(0x097F));
    assert!(!s.contains_codepoint(0x0980));
}

// ---------------------------------------------------------------------------
// Phoneme inventory — edge-case lookups
// ---------------------------------------------------------------------------

#[test]
fn phoneme_find_empty_string() {
    let en = varna::phoneme::english();
    assert!(en.find("").is_none());
    assert!(!en.has(""));
}

#[test]
fn phoneme_find_unicode_garbage() {
    let en = varna::phoneme::english();
    assert!(en.find("🔥").is_none());
    assert!(en.find("\0").is_none());
    assert!(en.find("\u{200B}").is_none()); // zero-width space
}

#[test]
fn phoneme_find_very_long_string() {
    let en = varna::phoneme::english();
    let long = "ʃ".repeat(1000);
    assert!(en.find(&long).is_none());
}

#[test]
fn phoneme_find_similar_but_wrong() {
    let en = varna::phoneme::english();
    // Lookalike characters that are NOT the same as IPA symbols
    assert!(en.find("S").is_none()); // Latin S vs IPA
    assert!(en.find("g").is_none()); // ASCII g vs IPA ɡ
}

// ---------------------------------------------------------------------------
// Builder — edge-case construction
// ---------------------------------------------------------------------------

#[test]
fn builder_empty_inventory() {
    let inv = varna::phoneme::PhonemeInventoryBuilder::new("xx", "Empty").build();
    assert_eq!(inv.consonant_count(), 0);
    assert_eq!(inv.vowel_count(), 0);
    assert_eq!(inv.phonemes.len(), 0);
    assert!(inv.find("a").is_none());
    assert!(!inv.has("a"));
}

#[test]
fn builder_empty_strings() {
    let inv = varna::phoneme::PhonemeInventoryBuilder::new("", "")
        .consonant(
            "",
            varna::phoneme::Manner::Plosive,
            varna::phoneme::Place::Bilabial,
            false,
        )
        .build();
    assert_eq!(inv.language_code, "");
    assert_eq!(inv.consonant_count(), 1);
    // Can find the empty-string phoneme
    assert!(inv.has(""));
}

#[test]
fn builder_unicode_ipa_symbols() {
    let inv = varna::phoneme::PhonemeInventoryBuilder::new("xx", "Unicode Test")
        .consonant(
            "🔥",
            varna::phoneme::Manner::Plosive,
            varna::phoneme::Place::Bilabial,
            false,
        )
        .build();
    assert!(inv.has("🔥"));
}

#[test]
fn builder_with_zero_capacity() {
    let inv = varna::phoneme::PhonemeInventoryBuilder::with_capacity("xx", "Zero Cap", 0)
        .consonant(
            "p",
            varna::phoneme::Manner::Plosive,
            varna::phoneme::Place::Bilabial,
            false,
        )
        .build();
    assert_eq!(inv.consonant_count(), 1);
}

// ---------------------------------------------------------------------------
// Grammar — edge-case lookups
// ---------------------------------------------------------------------------

#[test]
fn grammar_by_code_empty() {
    assert!(varna::grammar::by_code("").is_none());
}

#[test]
fn grammar_by_code_unicode() {
    assert!(varna::grammar::by_code("中文").is_none());
}

// ---------------------------------------------------------------------------
// Lexicon — edge-case lookups
// ---------------------------------------------------------------------------

#[test]
fn lexicon_find_empty_string() {
    let lex = varna::lexicon::Lexicon {
        language_code: Cow::Borrowed("en"),
        entries: vec![],
    };
    assert!(lex.find("").is_none());
    assert!(lex.swadesh().is_empty());
    assert!(lex.most_frequent(100).is_empty());
}

#[test]
fn lexicon_most_frequent_zero() {
    let lex = varna::lexicon::swadesh::by_code("es").unwrap();
    assert!(lex.most_frequent(0).is_empty());
}

#[test]
fn lexicon_most_frequent_huge_n() {
    let lex = varna::lexicon::swadesh::by_code("es").unwrap();
    // Requesting more than available should just return all available
    let freq = lex.most_frequent(1_000_000);
    assert!(freq.len() <= lex.entries.len());
}

// ---------------------------------------------------------------------------
// Swadesh — edge-case lookups
// ---------------------------------------------------------------------------

#[test]
fn swadesh_by_code_empty() {
    assert!(varna::lexicon::swadesh::by_code("").is_none());
}

#[test]
fn swadesh_by_code_invalid() {
    assert!(varna::lexicon::swadesh::by_code("xx").is_none());
    assert!(varna::lexicon::swadesh::by_code("🔥").is_none());
}

// ---------------------------------------------------------------------------
// Cognate — edge-case lookups
// ---------------------------------------------------------------------------

#[test]
fn cognate_for_language_empty() {
    let cog = varna::lexicon::cognate::water_cognates();
    assert!(cog.for_language("").is_none());
}

#[test]
fn cognate_for_language_invalid() {
    let cog = varna::lexicon::cognate::water_cognates();
    assert!(cog.for_language("zz").is_none());
}

// ---------------------------------------------------------------------------
// Transliteration — edge-case inputs
// ---------------------------------------------------------------------------

#[test]
fn transliteration_empty_string() {
    let table = varna::script::transliteration::devanagari_iast();
    assert_eq!(table.transliterate(""), "");
}

#[test]
fn transliteration_ascii_passthrough() {
    let table = varna::script::transliteration::devanagari_iast();
    assert_eq!(table.transliterate("hello world"), "hello world");
}

#[test]
fn transliteration_emoji_passthrough() {
    let table = varna::script::transliteration::greek_beta_code();
    assert_eq!(table.transliterate("🔥"), "🔥");
}

#[test]
fn transliteration_null_bytes() {
    let table = varna::script::transliteration::greek_beta_code();
    assert_eq!(table.transliterate("\0"), "\0");
}

#[test]
fn transliterate_char_empty() {
    let table = varna::script::transliteration::devanagari_iast();
    assert!(table.transliterate_char("").is_none());
}

#[test]
fn transliteration_reverse_map_not_empty() {
    let table = varna::script::transliteration::devanagari_iast();
    let rev = table.reverse_map();
    assert!(!rev.is_empty());
}

// ---------------------------------------------------------------------------
// Numerals — edge-case inputs
// ---------------------------------------------------------------------------

#[test]
fn numeral_value_of_empty() {
    let sys = varna::script::numerals::devanagari_digits();
    assert!(sys.value_of("").is_none());
}

#[test]
fn numeral_value_of_unicode_garbage() {
    let sys = varna::script::numerals::greek_isopsephy();
    assert!(sys.value_of("🔥").is_none());
}

#[test]
fn numeral_char_for_zero_greek() {
    // Greek isopsephy has no zero
    let sys = varna::script::numerals::greek_isopsephy();
    assert!(sys.char_for(0).is_none());
}

#[test]
fn numeral_char_for_u32_max() {
    let sys = varna::script::numerals::devanagari_digits();
    assert!(sys.char_for(u32::MAX).is_none());
}

#[test]
fn numeral_string_value_empty() {
    let sys = varna::script::numerals::greek_isopsephy();
    assert_eq!(sys.string_value(""), Some(0)); // sum of nothing = 0
}

#[test]
fn numeral_string_value_unmapped_char() {
    let sys = varna::script::numerals::greek_isopsephy();
    assert!(sys.string_value("X").is_none()); // Latin X, not Greek
}

#[test]
fn numeral_string_value_mixed_valid_invalid() {
    let sys = varna::script::numerals::greek_isopsephy();
    // α (valid) + X (invalid) → None because X has no mapping
    assert!(sys.string_value("αX").is_none());
}

// ---------------------------------------------------------------------------
// Allophone — edge-case inputs
// ---------------------------------------------------------------------------

#[test]
fn allophone_rules_for_empty() {
    let rules = varna::phoneme::allophone::english_allophones();
    assert!(rules.rules_for("").is_empty());
}

#[test]
fn allophone_realize_unmapped_phoneme() {
    let rules = varna::phoneme::allophone::english_allophones();
    // Phoneme not in English — should return itself unchanged
    let result = rules.realize("ʀ", &varna::phoneme::allophone::Environment::WordInitial);
    assert_eq!(result, "ʀ");
}

#[test]
fn allophone_realize_empty_phoneme() {
    let rules = varna::phoneme::allophone::english_allophones();
    let result = rules.realize("", &varna::phoneme::allophone::Environment::WordInitial);
    assert_eq!(result, "");
}

// ---------------------------------------------------------------------------
// Syllable/Phonotactics — edge-case inputs
// ---------------------------------------------------------------------------

#[test]
fn phonotactics_is_permitted_empty_sequence() {
    let p = varna::phoneme::syllable::english_phonotactics();
    assert_eq!(
        p.is_permitted("", varna::phoneme::syllable::SyllablePosition::Onset),
        None
    );
}

#[test]
fn phonotactics_constraints_at_unused_position() {
    let p = varna::phoneme::syllable::english_phonotactics();
    let nucleus = p.constraints_at(varna::phoneme::syllable::SyllablePosition::Nucleus);
    assert!(nucleus.is_empty());
}

// ---------------------------------------------------------------------------
// Dialect — edge-case inputs
// ---------------------------------------------------------------------------

#[test]
fn dialect_adds_empty() {
    let rp = varna::dialect::british_english();
    assert!(!rp.adds(""));
}

#[test]
fn dialect_removes_empty() {
    let rp = varna::dialect::british_english();
    assert!(!rp.removes(""));
}

#[test]
fn dialect_apply_idempotent() {
    // Applying the same overlay twice should not change the result
    let en = varna::phoneme::english();
    let rp = varna::dialect::british_english();
    let first = rp.apply(&en);
    let second = rp.apply(&first);
    // After second apply, should still have same phoneme set
    // (added phonemes already present, removed already gone)
    assert_eq!(first.consonant_count(), second.consonant_count());
    assert_eq!(first.vowel_count(), second.vowel_count());
}

// ---------------------------------------------------------------------------
// Error type — edge cases
// ---------------------------------------------------------------------------

#[test]
fn error_display_empty_strings() {
    let err = varna::VarnaError::UnknownLanguage(String::new());
    assert_eq!(err.to_string(), "unknown language: ");

    let err = varna::VarnaError::UnknownScript(String::new());
    assert_eq!(err.to_string(), "unknown script: ");
}

#[test]
fn error_display_unicode() {
    let err = varna::VarnaError::UnknownLanguage("🔥💀".into());
    assert!(err.to_string().contains("🔥💀"));
}

#[test]
fn error_display_very_long() {
    let long = "x".repeat(10000);
    let err = varna::VarnaError::UnknownLanguage(long.clone());
    assert!(err.to_string().contains(&long));
}
