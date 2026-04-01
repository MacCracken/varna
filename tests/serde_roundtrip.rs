//! Serde roundtrip tests — systematic JSON serialization/deserialization
//! for every public type in varna.
//!
//! Each type is serialized to JSON and deserialized back, verifying equality.

use std::borrow::Cow;

// ---------------------------------------------------------------------------
// Phoneme types
// ---------------------------------------------------------------------------

#[test]
fn serde_phoneme_consonant() {
    let p = varna::phoneme::Phoneme::consonant(
        "p",
        varna::phoneme::Manner::Plosive,
        varna::phoneme::Place::Bilabial,
        false,
    );
    let json = serde_json::to_string(&p).unwrap();
    let back: varna::phoneme::Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn serde_phoneme_vowel() {
    let p = varna::phoneme::Phoneme::vowel(
        "æ",
        varna::phoneme::Height::NearOpen,
        varna::phoneme::Backness::Front,
        false,
    );
    let json = serde_json::to_string(&p).unwrap();
    let back: varna::phoneme::Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn serde_phoneme_inventory_all_languages() {
    for code in varna::registry::all_codes() {
        let inv = varna::registry::phonemes(code).unwrap();
        let json = serde_json::to_string(&inv).unwrap();
        let back: varna::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
        assert_eq!(inv, back, "roundtrip failed for {code}");
    }
}

#[test]
fn serde_phoneme_inventory_with_tones() {
    // Mandarin has tones
    let inv = varna::registry::phonemes("zh").unwrap();
    assert!(inv.tones.is_some());
    let json = serde_json::to_string(&inv).unwrap();
    let back: varna::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(inv, back);
    assert_eq!(inv.tones, back.tones);
}

#[test]
fn serde_phoneme_inventory_without_tones() {
    let inv = varna::registry::phonemes("en").unwrap();
    assert!(inv.tones.is_none());
    let json = serde_json::to_string(&inv).unwrap();
    let back: varna::phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(inv.tones, back.tones);
}

// ---------------------------------------------------------------------------
// Script types
// ---------------------------------------------------------------------------

#[test]
fn serde_script_all() {
    for code in varna::script::all_codes() {
        let script = varna::script::by_code(code).unwrap();
        let json = serde_json::to_string(&script).unwrap();
        let back: varna::script::Script = serde_json::from_str(&json).unwrap();
        assert_eq!(script, back, "roundtrip failed for script {code}");
    }
}

#[test]
fn serde_script_with_attestation() {
    // Cuneiform and Egyptian have attestation fields
    let script = varna::script::cuneiform();
    assert!(script.attestation.is_some());
    let json = serde_json::to_string(&script).unwrap();
    let back: varna::script::Script = serde_json::from_str(&json).unwrap();
    assert_eq!(script.attestation, back.attestation);
}

#[test]
fn serde_script_without_attestation() {
    let script = varna::script::latin();
    assert!(script.attestation.is_none());
    let json = serde_json::to_string(&script).unwrap();
    let back: varna::script::Script = serde_json::from_str(&json).unwrap();
    assert_eq!(script.attestation, back.attestation);
}

// ---------------------------------------------------------------------------
// Transliteration types
// ---------------------------------------------------------------------------

#[test]
fn serde_transliteration_devanagari_iast() {
    let table = varna::script::transliteration::devanagari_iast();
    let json = serde_json::to_string(&table).unwrap();
    let back: varna::script::transliteration::TransliterationTable =
        serde_json::from_str(&json).unwrap();
    assert_eq!(table, back);
}

#[test]
fn serde_transliteration_greek_beta_code() {
    let table = varna::script::transliteration::greek_beta_code();
    let json = serde_json::to_string(&table).unwrap();
    let back: varna::script::transliteration::TransliterationTable =
        serde_json::from_str(&json).unwrap();
    assert_eq!(table, back);
}

// ---------------------------------------------------------------------------
// Numeral types
// ---------------------------------------------------------------------------

#[test]
fn serde_numeral_system_all() {
    let systems = [
        varna::script::numerals::devanagari_digits(),
        varna::script::numerals::greek_isopsephy(),
        varna::script::numerals::babylonian_sexagesimal(),
        varna::script::numerals::egyptian_hieroglyphic(),
        varna::script::numerals::chinese_rod_numerals(),
    ];
    for sys in &systems {
        let json = serde_json::to_string(sys).unwrap();
        let back: varna::script::numerals::NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(*sys, back, "roundtrip failed for {}", sys.name);
    }
}

#[test]
fn serde_numeral_mapping() {
    let mapping = varna::script::numerals::NumeralMapping {
        character: Cow::Borrowed("α"),
        value: 1,
    };
    let json = serde_json::to_string(&mapping).unwrap();
    let back: varna::script::numerals::NumeralMapping = serde_json::from_str(&json).unwrap();
    assert_eq!(mapping, back);
}

// ---------------------------------------------------------------------------
// Grammar types
// ---------------------------------------------------------------------------

#[test]
fn serde_grammar_profile_all() {
    for code in varna::grammar::all_codes() {
        let g = varna::grammar::by_code(code).unwrap();
        let json = serde_json::to_string(&g).unwrap();
        let back: varna::grammar::GrammarProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back, "roundtrip failed for grammar {code}");
    }
}

// ---------------------------------------------------------------------------
// Lexicon types
// ---------------------------------------------------------------------------

#[test]
fn serde_lex_entry() {
    let entry = varna::lexicon::LexEntry {
        word: Cow::Borrowed("water"),
        ipa: Cow::Borrowed("ˈwɔːtər"),
        gloss: Cow::Borrowed("water"),
        pos: varna::lexicon::PartOfSpeech::Noun,
        frequency_rank: Some(250),
        swadesh_index: Some(23),
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: varna::lexicon::LexEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_lex_entry_no_optionals() {
    let entry = varna::lexicon::LexEntry {
        word: Cow::Borrowed("xyz"),
        ipa: Cow::Borrowed("xyz"),
        gloss: Cow::Borrowed("test"),
        pos: varna::lexicon::PartOfSpeech::Verb,
        frequency_rank: None,
        swadesh_index: None,
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: varna::lexicon::LexEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_lexicon() {
    let lex = varna::lexicon::Lexicon {
        language_code: Cow::Borrowed("en"),
        entries: vec![varna::lexicon::LexEntry {
            word: Cow::Borrowed("fire"),
            ipa: Cow::Borrowed("ˈfaɪər"),
            gloss: Cow::Borrowed("fire"),
            pos: varna::lexicon::PartOfSpeech::Noun,
            frequency_rank: Some(800),
            swadesh_index: Some(24),
        }],
    };
    let json = serde_json::to_string(&lex).unwrap();
    let back: varna::lexicon::Lexicon = serde_json::from_str(&json).unwrap();
    assert_eq!(lex, back);
}

#[test]
fn serde_swadesh_all() {
    for code in varna::lexicon::swadesh::all_codes() {
        let lex = varna::lexicon::swadesh::by_code(code).unwrap();
        let json = serde_json::to_string(&lex).unwrap();
        let back: varna::lexicon::Lexicon = serde_json::from_str(&json).unwrap();
        assert_eq!(lex, back, "roundtrip failed for swadesh {code}");
    }
}

// ---------------------------------------------------------------------------
// Cognate & etymology types
// ---------------------------------------------------------------------------

#[test]
fn serde_cognate_set() {
    let cog = varna::lexicon::cognate::water_cognates();
    let json = serde_json::to_string(&cog).unwrap();
    let back: varna::lexicon::cognate::CognateSet = serde_json::from_str(&json).unwrap();
    assert_eq!(cog, back);
}

#[test]
fn serde_cognate_entry() {
    let entry = varna::lexicon::cognate::CognateEntry {
        language: Cow::Borrowed("en"),
        word: Cow::Borrowed("water"),
        ipa: Cow::Borrowed("ˈwɔːtər"),
    };
    let json = serde_json::to_string(&entry).unwrap();
    let back: varna::lexicon::cognate::CognateEntry = serde_json::from_str(&json).unwrap();
    assert_eq!(entry, back);
}

#[test]
fn serde_etymology() {
    let etym = varna::lexicon::cognate::Etymology {
        source_language: Cow::Borrowed("fr"),
        source_form: Cow::Borrowed("café"),
        borrowing_type: varna::lexicon::cognate::BorrowingType::Loanword,
        period: Some(Cow::Borrowed("18th century")),
    };
    let json = serde_json::to_string(&etym).unwrap();
    let back: varna::lexicon::cognate::Etymology = serde_json::from_str(&json).unwrap();
    assert_eq!(etym, back);
}

#[test]
fn serde_etymology_no_period() {
    let etym = varna::lexicon::cognate::Etymology {
        source_language: Cow::Borrowed("la"),
        source_form: Cow::Borrowed("aqua"),
        borrowing_type: varna::lexicon::cognate::BorrowingType::Inherited,
        period: None,
    };
    let json = serde_json::to_string(&etym).unwrap();
    let back: varna::lexicon::cognate::Etymology = serde_json::from_str(&json).unwrap();
    assert_eq!(etym, back);
}

// ---------------------------------------------------------------------------
// Allophone types
// ---------------------------------------------------------------------------

#[test]
fn serde_allophone_rule_set() {
    let rules = varna::phoneme::allophone::english_allophones();
    let json = serde_json::to_string(&rules).unwrap();
    let back: varna::phoneme::allophone::AllophoneRuleSet = serde_json::from_str(&json).unwrap();
    assert_eq!(rules, back);
}

#[test]
fn serde_allophone_rule() {
    let rule = varna::phoneme::allophone::AllophoneRule {
        phoneme: Cow::Borrowed("t"),
        allophone: Cow::Borrowed("ɾ"),
        environment: varna::phoneme::allophone::Environment::Intervocalic,
        obligatory: false,
    };
    let json = serde_json::to_string(&rule).unwrap();
    let back: varna::phoneme::allophone::AllophoneRule = serde_json::from_str(&json).unwrap();
    assert_eq!(rule, back);
}

// ---------------------------------------------------------------------------
// Syllable/Phonotactics types
// ---------------------------------------------------------------------------

#[test]
fn serde_phonotactics_all() {
    let profiles = [
        varna::phoneme::syllable::english_phonotactics(),
        varna::phoneme::syllable::sanskrit_phonotactics(),
        varna::phoneme::syllable::japanese_phonotactics(),
    ];
    for p in &profiles {
        let json = serde_json::to_string(p).unwrap();
        let back: varna::phoneme::syllable::Phonotactics = serde_json::from_str(&json).unwrap();
        assert_eq!(*p, back, "roundtrip failed for {}", p.language_code);
    }
}

#[test]
fn serde_syllable_template() {
    let tmpl = varna::phoneme::syllable::SyllableTemplate {
        max_onset: 3,
        max_coda: 4,
        complex_nucleus: true,
        pattern: Cow::Borrowed("(C)(C)(C)V(C)(C)(C)(C)"),
    };
    let json = serde_json::to_string(&tmpl).unwrap();
    let back: varna::phoneme::syllable::SyllableTemplate = serde_json::from_str(&json).unwrap();
    assert_eq!(tmpl, back);
}

// ---------------------------------------------------------------------------
// Dialect types
// ---------------------------------------------------------------------------

#[test]
fn serde_language_variety() {
    let rp = varna::dialect::british_english();
    let json = serde_json::to_string(&rp).unwrap();
    let back: varna::dialect::LanguageVariety = serde_json::from_str(&json).unwrap();
    assert_eq!(rp, back);
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[test]
fn error_debug_format() {
    // VarnaError derives Debug — verify it doesn't panic
    let errors = vec![
        varna::VarnaError::UnknownLanguage("xx".into()),
        varna::VarnaError::UnknownScript("Xxxx".into()),
        varna::VarnaError::PhonemeNotInInventory {
            phoneme: "ʀ".into(),
            language: "en".into(),
        },
        varna::VarnaError::InvalidIpa("???".into()),
        varna::VarnaError::WordNotFound {
            word: "foo".into(),
            language: "en".into(),
        },
    ];
    for err in &errors {
        let debug = format!("{err:?}");
        assert!(!debug.is_empty());
        let display = format!("{err}");
        assert!(!display.is_empty());
    }
}
