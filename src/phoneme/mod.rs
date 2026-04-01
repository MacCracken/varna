//! Phoneme inventories — IPA phonemes per language, phonological features.
//!
//! Every language has a finite set of contrastive sounds (phonemes). This module
//! provides those inventories with IPA transcription, articulatory features
//! (manner, place, voicing), and language-specific allophone rules.

use serde::{Deserialize, Serialize};

/// Articulatory manner of a consonant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Manner {
    Plosive,
    Nasal,
    Trill,
    TapFlap,
    Fricative,
    LateralFricative,
    Approximant,
    LateralApproximant,
    Affricate,
}

/// Place of articulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
}

/// Vowel height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Height {
    Close,
    NearClose,
    CloseMid,
    Mid,
    OpenMid,
    NearOpen,
    Open,
}

/// Vowel backness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Backness {
    Front,
    Central,
    Back,
}

/// A phoneme with its IPA symbol and articulatory features.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Phoneme {
    /// IPA symbol (e.g., "p", "ʃ", "æ").
    pub ipa: String,
    /// Classification.
    pub kind: PhonemeKind,
}

/// Classification of a phoneme.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PhonemeKind {
    Consonant {
        manner: Manner,
        place: Place,
        voiced: bool,
    },
    Vowel {
        height: Height,
        backness: Backness,
        rounded: bool,
    },
}

/// A language's complete phoneme inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonemeInventory {
    /// ISO 639-1 or 639-3 language code.
    pub language_code: String,
    /// Language name in English.
    pub language_name: String,
    /// All phonemes in this language.
    pub phonemes: Vec<Phoneme>,
    /// Tone system (None for non-tonal languages).
    pub tones: Option<Vec<String>>,
    /// Stress pattern.
    pub stress: StressPattern,
}

/// How stress works in a language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StressPattern {
    /// Stress on a fixed syllable (e.g., French → final, Finnish → initial).
    Fixed,
    /// Stress is contrastive / unpredictable (e.g., English, Russian).
    Free,
    /// Pitch accent system (e.g., Japanese, Swedish).
    PitchAccent,
    /// Tonal — pitch distinguishes meaning (e.g., Mandarin, Yoruba).
    Tonal,
}

impl PhonemeInventory {
    /// Number of consonants in the inventory.
    #[must_use]
    pub fn consonant_count(&self) -> usize {
        self.phonemes
            .iter()
            .filter(|p| matches!(p.kind, PhonemeKind::Consonant { .. }))
            .count()
    }

    /// Number of vowels in the inventory.
    #[must_use]
    pub fn vowel_count(&self) -> usize {
        self.phonemes
            .iter()
            .filter(|p| matches!(p.kind, PhonemeKind::Vowel { .. }))
            .count()
    }

    /// Look up a phoneme by IPA symbol.
    #[must_use]
    pub fn find(&self, ipa: &str) -> Option<&Phoneme> {
        self.phonemes.iter().find(|p| p.ipa == ipa)
    }

    /// Check if a phoneme exists in this language.
    #[must_use]
    pub fn has(&self, ipa: &str) -> bool {
        self.find(ipa).is_some()
    }
}

/// Build the English (General American) phoneme inventory.
#[must_use]
pub fn english() -> PhonemeInventory {
    PhonemeInventory {
        language_code: "en".to_string(),
        language_name: "English".to_string(),
        phonemes: vec![
            // Plosives
            Phoneme { ipa: "p".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Bilabial, voiced: false } },
            Phoneme { ipa: "b".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Bilabial, voiced: true } },
            Phoneme { ipa: "t".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Alveolar, voiced: false } },
            Phoneme { ipa: "d".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Alveolar, voiced: true } },
            Phoneme { ipa: "k".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Velar, voiced: false } },
            Phoneme { ipa: "ɡ".into(), kind: PhonemeKind::Consonant { manner: Manner::Plosive, place: Place::Velar, voiced: true } },
            // Fricatives
            Phoneme { ipa: "f".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Labiodental, voiced: false } },
            Phoneme { ipa: "v".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Labiodental, voiced: true } },
            Phoneme { ipa: "θ".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Dental, voiced: false } },
            Phoneme { ipa: "ð".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Dental, voiced: true } },
            Phoneme { ipa: "s".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Alveolar, voiced: false } },
            Phoneme { ipa: "z".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Alveolar, voiced: true } },
            Phoneme { ipa: "ʃ".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Postalveolar, voiced: false } },
            Phoneme { ipa: "ʒ".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Postalveolar, voiced: true } },
            Phoneme { ipa: "h".into(), kind: PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Glottal, voiced: false } },
            // Nasals
            Phoneme { ipa: "m".into(), kind: PhonemeKind::Consonant { manner: Manner::Nasal, place: Place::Bilabial, voiced: true } },
            Phoneme { ipa: "n".into(), kind: PhonemeKind::Consonant { manner: Manner::Nasal, place: Place::Alveolar, voiced: true } },
            Phoneme { ipa: "ŋ".into(), kind: PhonemeKind::Consonant { manner: Manner::Nasal, place: Place::Velar, voiced: true } },
            // Approximants
            Phoneme { ipa: "ɹ".into(), kind: PhonemeKind::Consonant { manner: Manner::Approximant, place: Place::Alveolar, voiced: true } },
            Phoneme { ipa: "l".into(), kind: PhonemeKind::Consonant { manner: Manner::LateralApproximant, place: Place::Alveolar, voiced: true } },
            Phoneme { ipa: "w".into(), kind: PhonemeKind::Consonant { manner: Manner::Approximant, place: Place::Bilabial, voiced: true } },
            Phoneme { ipa: "j".into(), kind: PhonemeKind::Consonant { manner: Manner::Approximant, place: Place::Palatal, voiced: true } },
            // Affricates
            Phoneme { ipa: "t͡ʃ".into(), kind: PhonemeKind::Consonant { manner: Manner::Affricate, place: Place::Postalveolar, voiced: false } },
            Phoneme { ipa: "d͡ʒ".into(), kind: PhonemeKind::Consonant { manner: Manner::Affricate, place: Place::Postalveolar, voiced: true } },
            // Vowels (General American)
            Phoneme { ipa: "iː".into(), kind: PhonemeKind::Vowel { height: Height::Close, backness: Backness::Front, rounded: false } },
            Phoneme { ipa: "ɪ".into(), kind: PhonemeKind::Vowel { height: Height::NearClose, backness: Backness::Front, rounded: false } },
            Phoneme { ipa: "eɪ".into(), kind: PhonemeKind::Vowel { height: Height::CloseMid, backness: Backness::Front, rounded: false } },
            Phoneme { ipa: "ɛ".into(), kind: PhonemeKind::Vowel { height: Height::OpenMid, backness: Backness::Front, rounded: false } },
            Phoneme { ipa: "æ".into(), kind: PhonemeKind::Vowel { height: Height::NearOpen, backness: Backness::Front, rounded: false } },
            Phoneme { ipa: "ɑː".into(), kind: PhonemeKind::Vowel { height: Height::Open, backness: Backness::Back, rounded: false } },
            Phoneme { ipa: "ɔː".into(), kind: PhonemeKind::Vowel { height: Height::OpenMid, backness: Backness::Back, rounded: true } },
            Phoneme { ipa: "oʊ".into(), kind: PhonemeKind::Vowel { height: Height::CloseMid, backness: Backness::Back, rounded: true } },
            Phoneme { ipa: "ʊ".into(), kind: PhonemeKind::Vowel { height: Height::NearClose, backness: Backness::Back, rounded: true } },
            Phoneme { ipa: "uː".into(), kind: PhonemeKind::Vowel { height: Height::Close, backness: Backness::Back, rounded: true } },
            Phoneme { ipa: "ʌ".into(), kind: PhonemeKind::Vowel { height: Height::OpenMid, backness: Backness::Central, rounded: false } },
            Phoneme { ipa: "ə".into(), kind: PhonemeKind::Vowel { height: Height::Mid, backness: Backness::Central, rounded: false } },
        ],
        tones: None,
        stress: StressPattern::Free,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_inventory_size() {
        let en = english();
        // English: ~24 consonants, ~11-15 vowels (GA)
        assert!(en.consonant_count() >= 22);
        assert!(en.vowel_count() >= 10);
    }

    #[test]
    fn test_english_has_th() {
        let en = english();
        assert!(en.has("θ")); // voiceless dental fricative (think)
        assert!(en.has("ð")); // voiced dental fricative (this)
    }

    #[test]
    fn test_english_no_tones() {
        let en = english();
        assert!(en.tones.is_none());
        assert_eq!(en.stress, StressPattern::Free);
    }

    #[test]
    fn test_find_phoneme() {
        let en = english();
        let p = en.find("ʃ").unwrap();
        assert!(matches!(p.kind, PhonemeKind::Consonant { manner: Manner::Fricative, place: Place::Postalveolar, voiced: false }));
    }

    #[test]
    fn test_missing_phoneme() {
        let en = english();
        // English doesn't have the uvular trill
        assert!(!en.has("ʀ"));
    }
}
