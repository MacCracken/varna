//! Integration tests for varna — cross-module behavior.

use std::borrow::Cow;

use varna::grammar::{GrammarProfile, Morphology, WordOrder};
use varna::lexicon::{LexEntry, Lexicon, PartOfSpeech};
use varna::phoneme::{self, Backness, Height, Manner, Phoneme, PhonemeKind, Place};
use varna::script::{Direction, Script, ScriptStatus, ScriptType};

// ---------------------------------------------------------------------------
// Serde roundtrips
// ---------------------------------------------------------------------------

#[test]
fn test_english_phoneme_serde_roundtrip() {
    let en = phoneme::english();
    let json = serde_json::to_string(&en).unwrap();
    let deserialized: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, en);
}

#[test]
fn test_sanskrit_phoneme_serde_roundtrip() {
    let sa = phoneme::sanskrit();
    let json = serde_json::to_string(&sa).unwrap();
    let deserialized: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, sa);
}

#[test]
fn test_greek_phoneme_serde_roundtrip() {
    let el = phoneme::greek();
    let json = serde_json::to_string(&el).unwrap();
    let deserialized: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, el);
}

#[test]
fn test_script_serde_roundtrip() {
    let latin = Script {
        code: Cow::Borrowed("Latn"),
        name: Cow::Borrowed("Latin"),
        script_type: ScriptType::Alphabet,
        direction: Direction::LeftToRight,
        status: ScriptStatus::Living,
        attestation: None,
        unicode_ranges: vec![(0x0041, 0x005A), (0x0061, 0x007A)],
        languages: vec![Cow::Borrowed("en"), Cow::Borrowed("fr")],
    };
    let json = serde_json::to_string(&latin).unwrap();
    let deserialized: Script = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, latin);
}

#[test]
fn test_grammar_serde_roundtrip() {
    let en = GrammarProfile {
        language_code: Cow::Borrowed("en"),
        morphology: Morphology::Fusional,
        word_order: WordOrder::SVO,
        case_count: 2,
        has_gender: false,
        gender_count: 0,
        has_dual: false,
        has_classifiers: false,
    };
    let json = serde_json::to_string(&en).unwrap();
    let deserialized: GrammarProfile = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, en);
}

#[test]
fn test_lexicon_serde_roundtrip() {
    let lex = Lexicon {
        language_code: Cow::Borrowed("en"),
        entries: vec![LexEntry {
            word: Cow::Borrowed("water"),
            ipa: Cow::Borrowed("ˈwɔːtər"),
            gloss: Cow::Borrowed("water"),
            pos: PartOfSpeech::Noun,
            frequency_rank: Some(250),
            swadesh_index: Some(1),
        }],
    };
    let json = serde_json::to_string(&lex).unwrap();
    let deserialized: Lexicon = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized, lex);
}

// ---------------------------------------------------------------------------
// Cross-module
// ---------------------------------------------------------------------------

#[test]
fn test_english_consonant_vowel_split() {
    let en = phoneme::english();
    let total = en.phonemes.len();
    assert_eq!(en.consonant_count() + en.vowel_count(), total);
}

#[test]
fn test_phoneme_kind_classification() {
    for code in varna::registry::all_codes() {
        let inv = varna::registry::phonemes(code).unwrap();
        for p in &inv.phonemes {
            match &p.kind {
                PhonemeKind::Consonant { .. } => {}
                PhonemeKind::Vowel { .. } => {}
                _ => panic!("unexpected phoneme kind /{}/  in {}", p.ipa, code),
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Error display
// ---------------------------------------------------------------------------

#[test]
fn test_error_display() {
    let err = varna::VarnaError::UnknownLanguage("xx".into());
    assert_eq!(err.to_string(), "unknown language: xx");

    let err = varna::VarnaError::PhonemeNotInInventory {
        phoneme: "ʀ".into(),
        language: "en".into(),
    };
    assert!(err.to_string().contains("ʀ"));
    assert!(err.to_string().contains("en"));
}

#[test]
fn test_all_error_variants_display() {
    let cases: Vec<(varna::VarnaError, &str)> = vec![
        (
            varna::VarnaError::UnknownLanguage("zz".into()),
            "unknown language: zz",
        ),
        (
            varna::VarnaError::UnknownScript("Xxxx".into()),
            "unknown script: Xxxx",
        ),
        (
            varna::VarnaError::InvalidIpa("???".into()),
            "invalid IPA symbol: ???",
        ),
        (
            varna::VarnaError::WordNotFound {
                word: "foo".into(),
                language: "en".into(),
            },
            "word not found: foo in en",
        ),
    ];
    for (err, expected) in cases {
        assert_eq!(err.to_string(), expected);
    }
}

// ---------------------------------------------------------------------------
// Phoneme constructors & serde
// ---------------------------------------------------------------------------

#[test]
fn test_phoneme_kind_serde_consonant() {
    let p = Phoneme::consonant("p", Manner::Plosive, Place::Bilabial, false);
    let json = serde_json::to_string(&p).unwrap();
    let back: Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn test_phoneme_kind_serde_vowel() {
    let p = Phoneme::vowel("æ", Height::NearOpen, Backness::Front, false);
    let json = serde_json::to_string(&p).unwrap();
    let back: Phoneme = serde_json::from_str(&json).unwrap();
    assert_eq!(p, back);
}

#[test]
fn test_phoneme_constructors() {
    let c = Phoneme::consonant("t", Manner::Plosive, Place::Alveolar, false);
    assert_eq!(c.ipa, "t");
    assert!(matches!(
        c.kind,
        PhonemeKind::Consonant { voiced: false, .. }
    ));

    let v = Phoneme::vowel("iː", Height::Close, Backness::Front, false);
    assert_eq!(v.ipa, "iː");
    assert!(matches!(v.kind, PhonemeKind::Vowel { rounded: false, .. }));
}

#[test]
fn test_phoneme_constructor_with_owned_string() {
    let ipa = String::from("custom");
    let p = Phoneme::consonant(ipa, Manner::Fricative, Place::Glottal, false);
    assert_eq!(p.ipa, "custom");
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

#[test]
fn test_registry_roundtrip_all_languages() {
    for code in varna::registry::all_codes() {
        let inv = varna::registry::phonemes(code).unwrap();
        let json = serde_json::to_string(&inv).unwrap();
        let back: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
        assert_eq!(inv, back, "serde roundtrip failed for {code}");
    }
}

#[test]
fn test_registry_script_consistency() {
    for code in varna::registry::all_codes() {
        let info = varna::registry::info(code).unwrap();
        if let Some(script) = varna::registry::primary_script(code) {
            assert_eq!(
                script.code, info.script_codes[0],
                "script mismatch for {code}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Script metadata
// ---------------------------------------------------------------------------

#[test]
fn test_all_scripts_serde_roundtrip() {
    for code in varna::script::all_codes() {
        let script = varna::script::by_code(code).unwrap();
        let json = serde_json::to_string(&script).unwrap();
        let back: Script = serde_json::from_str(&json).unwrap();
        assert_eq!(script, back, "serde roundtrip failed for script {code}");
    }
}

// ---------------------------------------------------------------------------
// Allophone rules
// ---------------------------------------------------------------------------

#[test]
fn test_allophone_realize_cross_module() {
    use varna::phoneme::allophone;
    let rules = allophone::english_allophones();
    // Verify allophone realization agrees with phoneme inventory
    let en = phoneme::english();
    assert!(en.has("t")); // phoneme exists
    let realized = rules.realize("t", &allophone::Environment::Intervocalic);
    assert_eq!(realized, "ɾ");
    assert!(!en.has("ɾ")); // allophone is NOT in the phoneme inventory
}

// ---------------------------------------------------------------------------
// Phonotactics
// ---------------------------------------------------------------------------

#[test]
fn test_phonotactics_serde_roundtrip_all() {
    use varna::phoneme::syllable;
    let profiles = [
        syllable::english_phonotactics(),
        syllable::sanskrit_phonotactics(),
        syllable::japanese_phonotactics(),
    ];
    for p in &profiles {
        let json = serde_json::to_string(p).unwrap();
        let back: syllable::Phonotactics = serde_json::from_str(&json).unwrap();
        assert_eq!(*p, back, "serde roundtrip failed for {}", p.language_code);
    }
}

// ---------------------------------------------------------------------------
// Transliteration
// ---------------------------------------------------------------------------

#[test]
fn test_transliteration_greek_roundtrip_chars() {
    use varna::script::transliteration;
    let table = transliteration::greek_beta_code();
    // Every char in forward map should transliterate correctly
    for (src, tgt) in &table.forward {
        assert_eq!(
            table.transliterate(src),
            tgt.as_ref(),
            "failed for {src} → {tgt}"
        );
    }
}

// ---------------------------------------------------------------------------
// Numerals
// ---------------------------------------------------------------------------

#[test]
fn test_numeral_systems_serde_roundtrip() {
    use varna::script::numerals;
    let systems = [numerals::devanagari_digits(), numerals::greek_isopsephy()];
    for sys in &systems {
        let json = serde_json::to_string(sys).unwrap();
        let back: numerals::NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(*sys, back, "serde roundtrip failed for {}", sys.name);
    }
}

#[test]
fn test_greek_isopsephy_word_value() {
    use varna::script::numerals;
    let sys = numerals::greek_isopsephy();
    // "θεος" (theos) = 9 + 5 + 70 + 200 = 284
    assert_eq!(sys.string_value("θεος"), Some(284));
}

// ---------------------------------------------------------------------------
// Dialect/variety
// ---------------------------------------------------------------------------

#[test]
fn test_dialect_apply_overlay() {
    let en = phoneme::english();
    let rp = varna::dialect::british_english();
    let rp_inv = rp.apply(&en);

    // RP adds /ɒ/ and removes /ɹ/
    assert!(rp_inv.has("ɒ"));
    assert!(!rp_inv.has("ɹ"));
    // But still has everything else
    assert!(rp_inv.has("p"));
    assert!(rp_inv.has("θ"));
    assert_eq!(rp_inv.language_code, "en-GB");
}

#[test]
fn test_dialect_serde_roundtrip() {
    let rp = varna::dialect::british_english();
    let json = serde_json::to_string(&rp).unwrap();
    let back: varna::dialect::LanguageVariety = serde_json::from_str(&json).unwrap();
    assert_eq!(rp, back);
}

// ---------------------------------------------------------------------------
// Cognate & etymology
// ---------------------------------------------------------------------------

#[test]
fn test_cognate_cross_registry() {
    use varna::lexicon::cognate;
    let cog = cognate::water_cognates();
    // Verify cognate languages are registered
    for entry in &cog.entries {
        if varna::registry::info(&entry.language).is_some() {
            let inv = varna::registry::phonemes(&entry.language).unwrap();
            assert!(
                !inv.phonemes.is_empty(),
                "empty inventory for {}",
                entry.language
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Extended inventories
// ---------------------------------------------------------------------------

#[test]
fn test_all_inventories_serde_roundtrip() {
    for code in varna::registry::all_codes() {
        let inv = varna::registry::phonemes(code).unwrap();
        let json = serde_json::to_string(&inv).unwrap();
        let back: phoneme::PhonemeInventory = serde_json::from_str(&json).unwrap();
        assert_eq!(inv, back, "serde roundtrip failed for {code}");
    }
}

#[test]
fn test_all_inventories_consonant_vowel_split() {
    for code in varna::registry::all_codes() {
        let inv = varna::registry::phonemes(code).unwrap();
        assert_eq!(
            inv.consonant_count() + inv.vowel_count(),
            inv.phonemes.len(),
            "consonant+vowel mismatch for {code}"
        );
    }
}

// ---------------------------------------------------------------------------
// Grammar profiles
// ---------------------------------------------------------------------------

#[test]
fn test_grammar_all_serde_roundtrip() {
    for code in varna::grammar::all_codes() {
        let g = varna::grammar::by_code(code).unwrap();
        let json = serde_json::to_string(&g).unwrap();
        let back: varna::grammar::GrammarProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back, "grammar roundtrip failed for {code}");
    }
}

#[test]
fn test_grammar_gender_consistency() {
    for code in varna::grammar::all_codes() {
        let g = varna::grammar::by_code(code).unwrap();
        if g.has_gender {
            assert!(g.gender_count > 0, "{code} has_gender but gender_count=0");
        } else {
            assert_eq!(g.gender_count, 0, "{code} !has_gender but gender_count>0");
        }
    }
}

// ---------------------------------------------------------------------------
// Swadesh lists
// ---------------------------------------------------------------------------

#[test]
fn test_swadesh_all_serde_roundtrip() {
    for code in varna::lexicon::swadesh::all_codes() {
        let lex = varna::lexicon::swadesh::by_code(code).unwrap();
        let json = serde_json::to_string(&lex).unwrap();
        let back: varna::lexicon::Lexicon = serde_json::from_str(&json).unwrap();
        assert_eq!(lex, back, "swadesh roundtrip failed for {code}");
    }
}

#[test]
fn test_swadesh_cross_language_water() {
    for code in varna::lexicon::swadesh::all_codes() {
        let lex = varna::lexicon::swadesh::by_code(code).unwrap();
        let water = lex.entries.iter().find(|e| e.swadesh_index == Some(23));
        assert!(water.is_some(), "missing water (index 23) in {code}");
        assert_eq!(water.unwrap().gloss, "water");
    }
}

// ---------------------------------------------------------------------------
// Tonal language features
// ---------------------------------------------------------------------------

#[test]
fn test_tonal_languages_have_tones() {
    let tonal = ["zh", "yo", "th", "vi", "lzh"];
    for code in tonal {
        let inv = varna::registry::phonemes(code).unwrap();
        assert!(
            inv.tones.is_some(),
            "{code} should be tonal but has no tones"
        );
        assert_eq!(
            inv.stress,
            phoneme::StressPattern::Tonal,
            "{code} stress should be Tonal"
        );
    }
}

#[test]
fn test_non_tonal_languages_no_tones() {
    let non_tonal = ["en", "es", "fr", "de", "ru", "pt", "la", "fi", "tr"];
    for code in non_tonal {
        let inv = varna::registry::phonemes(code).unwrap();
        assert!(inv.tones.is_none(), "{code} should not have tones");
    }
}
