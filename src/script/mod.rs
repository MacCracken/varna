//! Writing systems — alphabet, syllabary, logographic, abjad, abugida.

use serde::{Deserialize, Serialize};

/// Writing system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ScriptType {
    /// Each symbol = one phoneme (e.g., Latin, Cyrillic, Greek).
    Alphabet,
    /// Each symbol = consonant + inherent vowel, modified by diacritics (e.g., Devanagari, Thai).
    Abugida,
    /// Each symbol = consonant only, vowels optional/diacritical (e.g., Arabic, Hebrew).
    Abjad,
    /// Each symbol = one syllable (e.g., Japanese kana, Cherokee).
    Syllabary,
    /// Each symbol = one morpheme/word (e.g., Chinese hanzi, Japanese kanji).
    Logographic,
    /// Mixed system (e.g., Japanese uses all three).
    Mixed,
}

/// Text directionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    /// Bidirectional (e.g., mixed Arabic + Latin text).
    Bidirectional,
}

/// A writing system's metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// ISO 15924 code (e.g., "Latn", "Arab", "Deva").
    pub code: String,
    /// Human name (e.g., "Latin", "Arabic", "Devanagari").
    pub name: String,
    /// Classification.
    pub script_type: ScriptType,
    /// Primary text direction.
    pub direction: Direction,
    /// Unicode block ranges (start, end) for this script.
    pub unicode_ranges: Vec<(u32, u32)>,
    /// Languages that use this script.
    pub languages: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_types() {
        // Smoke test — types are constructible
        let latin = Script {
            code: "Latn".into(),
            name: "Latin".into(),
            script_type: ScriptType::Alphabet,
            direction: Direction::LeftToRight,
            unicode_ranges: vec![(0x0041, 0x005A), (0x0061, 0x007A)],
            languages: vec!["en".into(), "fr".into(), "es".into(), "de".into()],
        };
        assert_eq!(latin.script_type, ScriptType::Alphabet);
        assert_eq!(latin.direction, Direction::LeftToRight);
    }

    #[test]
    fn test_arabic_rtl() {
        let arabic = Script {
            code: "Arab".into(),
            name: "Arabic".into(),
            script_type: ScriptType::Abjad,
            direction: Direction::RightToLeft,
            unicode_ranges: vec![(0x0600, 0x06FF)],
            languages: vec!["ar".into(), "fa".into(), "ur".into()],
        };
        assert_eq!(arabic.direction, Direction::RightToLeft);
        assert_eq!(arabic.script_type, ScriptType::Abjad);
    }
}
