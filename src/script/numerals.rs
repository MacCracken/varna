//! Script-to-numeral mapping — numeric values for script characters.
//!
//! Many writing systems assign numeric values to their characters.
//! Greek letters have isopsephy values (α=1, β=2, ..., ω=800),
//! Devanagari has its own digit forms (०=0, १=1, ..., ९=9),
//! and Hebrew letters have gematria values.
//!
//! This module provides a generic interface for these mappings,
//! used by sankhya for script-aware numeral display.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// The kind of numeral system a script uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum NumeralSystemKind {
    /// Positional decimal digits (0-9), like Devanagari or Arabic-Indic.
    Decimal,
    /// Alphabetic/additive system where letters have fixed values (e.g., Greek isopsephy, Hebrew gematria).
    Alphabetic,
    /// Mixed or other system.
    Other,
}

/// A single character-to-value mapping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumeralMapping {
    /// The character in the native script.
    pub character: Cow<'static, str>,
    /// Its numeric value.
    pub value: u32,
}

/// A complete numeral system for a script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NumeralSystem {
    /// ISO 15924 script code.
    pub script_code: Cow<'static, str>,
    /// Human-readable name (e.g., "Greek Isopsephy", "Devanagari Digits").
    pub name: Cow<'static, str>,
    /// Kind of numeral system.
    pub kind: NumeralSystemKind,
    /// Character-to-value mappings.
    pub mappings: Vec<NumeralMapping>,
}

impl NumeralSystem {
    /// Look up the numeric value of a character.
    #[must_use]
    pub fn value_of(&self, ch: &str) -> Option<u32> {
        tracing::trace!(script = %self.script_code, ch, "numeral lookup");
        self.mappings
            .iter()
            .find(|m| m.character == ch)
            .map(|m| m.value)
    }

    /// Look up the character for a numeric value.
    #[must_use]
    pub fn char_for(&self, value: u32) -> Option<&str> {
        self.mappings
            .iter()
            .find(|m| m.value == value)
            .map(|m| m.character.as_ref())
    }

    /// Compute the additive numeric value of a string by summing individual
    /// character values. Returns `None` if any character has no mapping.
    ///
    /// This is additive (isopsephy/gematria style), not positional.
    /// For decimal digit strings, the caller must handle place values.
    #[must_use]
    pub fn string_value(&self, s: &str) -> Option<u32> {
        let mut total: u32 = 0;
        let mut buf = [0u8; 4];
        for ch in s.chars() {
            let ch_s = ch.encode_utf8(&mut buf);
            total = total.checked_add(self.value_of(ch_s)?)?;
        }
        Some(total)
    }
}

// ---------------------------------------------------------------------------
// Pre-built numeral systems
// ---------------------------------------------------------------------------

/// Devanagari decimal digits (०-९).
#[must_use]
pub fn devanagari_digits() -> NumeralSystem {
    NumeralSystem {
        script_code: Cow::Borrowed("Deva"),
        name: Cow::Borrowed("Devanagari Digits"),
        kind: NumeralSystemKind::Decimal,
        mappings: vec![
            NumeralMapping {
                character: Cow::Borrowed("०"),
                value: 0,
            },
            NumeralMapping {
                character: Cow::Borrowed("१"),
                value: 1,
            },
            NumeralMapping {
                character: Cow::Borrowed("२"),
                value: 2,
            },
            NumeralMapping {
                character: Cow::Borrowed("३"),
                value: 3,
            },
            NumeralMapping {
                character: Cow::Borrowed("४"),
                value: 4,
            },
            NumeralMapping {
                character: Cow::Borrowed("५"),
                value: 5,
            },
            NumeralMapping {
                character: Cow::Borrowed("६"),
                value: 6,
            },
            NumeralMapping {
                character: Cow::Borrowed("७"),
                value: 7,
            },
            NumeralMapping {
                character: Cow::Borrowed("८"),
                value: 8,
            },
            NumeralMapping {
                character: Cow::Borrowed("९"),
                value: 9,
            },
        ],
    }
}

/// Greek alphabetic numeral values (isopsephy).
///
/// Used for Greek numeral notation and isopsephy calculations.
/// Only includes the standard 24 letters with their traditional values.
#[must_use]
pub fn greek_isopsephy() -> NumeralSystem {
    NumeralSystem {
        script_code: Cow::Borrowed("Grek"),
        name: Cow::Borrowed("Greek Isopsephy"),
        kind: NumeralSystemKind::Alphabetic,
        mappings: vec![
            // Units
            NumeralMapping {
                character: Cow::Borrowed("α"),
                value: 1,
            },
            NumeralMapping {
                character: Cow::Borrowed("β"),
                value: 2,
            },
            NumeralMapping {
                character: Cow::Borrowed("γ"),
                value: 3,
            },
            NumeralMapping {
                character: Cow::Borrowed("δ"),
                value: 4,
            },
            NumeralMapping {
                character: Cow::Borrowed("ε"),
                value: 5,
            },
            // ϛ (stigma/digamma) = 6, archaic — omitted from modern alphabet
            NumeralMapping {
                character: Cow::Borrowed("ζ"),
                value: 7,
            },
            NumeralMapping {
                character: Cow::Borrowed("η"),
                value: 8,
            },
            NumeralMapping {
                character: Cow::Borrowed("θ"),
                value: 9,
            },
            // Tens
            NumeralMapping {
                character: Cow::Borrowed("ι"),
                value: 10,
            },
            NumeralMapping {
                character: Cow::Borrowed("κ"),
                value: 20,
            },
            NumeralMapping {
                character: Cow::Borrowed("λ"),
                value: 30,
            },
            NumeralMapping {
                character: Cow::Borrowed("μ"),
                value: 40,
            },
            NumeralMapping {
                character: Cow::Borrowed("ν"),
                value: 50,
            },
            NumeralMapping {
                character: Cow::Borrowed("ξ"),
                value: 60,
            },
            NumeralMapping {
                character: Cow::Borrowed("ο"),
                value: 70,
            },
            NumeralMapping {
                character: Cow::Borrowed("π"),
                value: 80,
            },
            // ϟ (koppa) = 90, archaic — omitted
            // Hundreds
            NumeralMapping {
                character: Cow::Borrowed("ρ"),
                value: 100,
            },
            NumeralMapping {
                character: Cow::Borrowed("σ"),
                value: 200,
            },
            NumeralMapping {
                character: Cow::Borrowed("ς"), // final sigma, same value
                value: 200,
            },
            NumeralMapping {
                character: Cow::Borrowed("τ"),
                value: 300,
            },
            NumeralMapping {
                character: Cow::Borrowed("υ"),
                value: 400,
            },
            NumeralMapping {
                character: Cow::Borrowed("φ"),
                value: 500,
            },
            NumeralMapping {
                character: Cow::Borrowed("χ"),
                value: 600,
            },
            NumeralMapping {
                character: Cow::Borrowed("ψ"),
                value: 700,
            },
            NumeralMapping {
                character: Cow::Borrowed("ω"),
                value: 800,
            },
        ],
    }
}

/// Babylonian sexagesimal (base-60) numeral system.
///
/// Uses two cuneiform signs: 𒐕 (diš, unit = 1) and 𒌋 (u, ten = 10).
/// Values 1-59 are composed additively. Position gives powers of 60.
/// Used by sankhya for Babylonian mathematical notation.
#[must_use]
pub fn babylonian_sexagesimal() -> NumeralSystem {
    NumeralSystem {
        script_code: Cow::Borrowed("Xsux"),
        name: Cow::Borrowed("Babylonian Sexagesimal"),
        kind: NumeralSystemKind::Other, // positional base-60 but additive within digits
        mappings: vec![
            NumeralMapping {
                character: Cow::Borrowed("𒐕"),
                value: 1,
            }, // diš
            NumeralMapping {
                character: Cow::Borrowed("𒐖"),
                value: 2,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐗"),
                value: 3,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐘"),
                value: 4,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐙"),
                value: 5,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐚"),
                value: 6,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐛"),
                value: 7,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐜"),
                value: 8,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒐝"),
                value: 9,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒌋"),
                value: 10,
            }, // u
            NumeralMapping {
                character: Cow::Borrowed("𒌋𒌋"),
                value: 20,
            },
            NumeralMapping {
                character: Cow::Borrowed("𒌍"),
                value: 30,
            },
        ],
    }
}

/// Egyptian hieroglyphic numeral system (additive decimal).
///
/// Each power of 10 has a distinct symbol. Values are composed
/// by repeating symbols additively. Used by sankhya for Egyptian
/// mathematical notation and unit fraction display.
#[must_use]
pub fn egyptian_hieroglyphic() -> NumeralSystem {
    NumeralSystem {
        script_code: Cow::Borrowed("Egyp"),
        name: Cow::Borrowed("Egyptian Hieroglyphic Numerals"),
        kind: NumeralSystemKind::Other, // additive decimal
        mappings: vec![
            NumeralMapping {
                character: Cow::Borrowed("𓏺"),
                value: 1,
            }, // stroke
            NumeralMapping {
                character: Cow::Borrowed("𓎆"),
                value: 10,
            }, // heel bone
            NumeralMapping {
                character: Cow::Borrowed("𓍢"),
                value: 100,
            }, // coil of rope
            NumeralMapping {
                character: Cow::Borrowed("𓆼"),
                value: 1000,
            }, // lotus flower
            NumeralMapping {
                character: Cow::Borrowed("𓂭"),
                value: 10000,
            }, // bent finger
            NumeralMapping {
                character: Cow::Borrowed("𓆐"),
                value: 100000,
            }, // tadpole
            NumeralMapping {
                character: Cow::Borrowed("𓁨"),
                value: 1000000,
            }, // god Heh
        ],
    }
}

/// Chinese rod numeral system (positional decimal).
///
/// Rod numerals alternate between vertical (units, hundreds, ten-thousands)
/// and horizontal (tens, thousands) orientations for adjacent positions.
/// Used by sankhya for displaying ancient Chinese mathematical notation.
#[must_use]
pub fn chinese_rod_numerals() -> NumeralSystem {
    NumeralSystem {
        script_code: Cow::Borrowed("Hani"),
        name: Cow::Borrowed("Chinese Rod Numerals"),
        kind: NumeralSystemKind::Decimal,
        mappings: vec![
            // Vertical forms (units position)
            NumeralMapping {
                character: Cow::Borrowed("𝍠"),
                value: 1,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍡"),
                value: 2,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍢"),
                value: 3,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍣"),
                value: 4,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍤"),
                value: 5,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍥"),
                value: 6,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍦"),
                value: 7,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍧"),
                value: 8,
            },
            NumeralMapping {
                character: Cow::Borrowed("𝍨"),
                value: 9,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Devanagari digits --

    #[test]
    fn test_devanagari_digit_lookup() {
        let sys = devanagari_digits();
        assert_eq!(sys.value_of("०"), Some(0));
        assert_eq!(sys.value_of("५"), Some(5));
        assert_eq!(sys.value_of("९"), Some(9));
    }

    #[test]
    fn test_devanagari_char_for_value() {
        let sys = devanagari_digits();
        assert_eq!(sys.char_for(0), Some("०"));
        assert_eq!(sys.char_for(7), Some("७"));
        assert_eq!(sys.char_for(10), None); // no single digit for 10
    }

    #[test]
    fn test_devanagari_string_value() {
        let sys = devanagari_digits();
        // "१२३" = 1+2+3 = 6 (additive, not positional — that's the caller's job)
        assert_eq!(sys.string_value("१२३"), Some(6));
    }

    #[test]
    fn test_devanagari_unknown_char() {
        let sys = devanagari_digits();
        assert_eq!(sys.value_of("X"), None);
        assert_eq!(sys.string_value("X"), None);
    }

    // -- Greek isopsephy --

    #[test]
    fn test_greek_isopsephy_units() {
        let sys = greek_isopsephy();
        assert_eq!(sys.value_of("α"), Some(1));
        assert_eq!(sys.value_of("ε"), Some(5));
        assert_eq!(sys.value_of("θ"), Some(9));
    }

    #[test]
    fn test_greek_isopsephy_tens() {
        let sys = greek_isopsephy();
        assert_eq!(sys.value_of("ι"), Some(10));
        assert_eq!(sys.value_of("π"), Some(80));
    }

    #[test]
    fn test_greek_isopsephy_hundreds() {
        let sys = greek_isopsephy();
        assert_eq!(sys.value_of("ρ"), Some(100));
        assert_eq!(sys.value_of("ω"), Some(800));
    }

    #[test]
    fn test_greek_string_value() {
        let sys = greek_isopsephy();
        // "αω" = 1 + 800 = 801
        assert_eq!(sys.string_value("αω"), Some(801));
        // "πι" = 80 + 10 = 90
        assert_eq!(sys.string_value("πι"), Some(90));
    }

    #[test]
    fn test_greek_char_for() {
        let sys = greek_isopsephy();
        assert_eq!(sys.char_for(1), Some("α"));
        assert_eq!(sys.char_for(80), Some("π"));
        assert_eq!(sys.char_for(999), None);
    }

    #[test]
    fn test_numeral_system_serde_roundtrip() {
        let sys = devanagari_digits();
        let json = serde_json::to_string(&sys).unwrap();
        let back: NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys, back);
    }

    #[test]
    fn test_greek_isopsephy_serde_roundtrip() {
        let sys = greek_isopsephy();
        let json = serde_json::to_string(&sys).unwrap();
        let back: NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys, back);
    }

    // -- Babylonian sexagesimal --

    #[test]
    fn test_babylonian_units() {
        let sys = babylonian_sexagesimal();
        assert_eq!(sys.value_of("𒐕"), Some(1));
        assert_eq!(sys.value_of("𒐝"), Some(9));
        assert_eq!(sys.value_of("𒌋"), Some(10));
    }

    #[test]
    fn test_babylonian_char_for() {
        let sys = babylonian_sexagesimal();
        assert_eq!(sys.char_for(1), Some("𒐕"));
        assert_eq!(sys.char_for(30), Some("𒌍"));
    }

    #[test]
    fn test_babylonian_serde_roundtrip() {
        let sys = babylonian_sexagesimal();
        let json = serde_json::to_string(&sys).unwrap();
        let back: NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys, back);
    }

    // -- Egyptian hieroglyphic --

    #[test]
    fn test_egyptian_powers_of_ten() {
        let sys = egyptian_hieroglyphic();
        assert_eq!(sys.value_of("𓏺"), Some(1));
        assert_eq!(sys.value_of("𓎆"), Some(10));
        assert_eq!(sys.value_of("𓍢"), Some(100));
        assert_eq!(sys.value_of("𓆼"), Some(1000));
        assert_eq!(sys.value_of("𓁨"), Some(1000000));
    }

    #[test]
    fn test_egyptian_serde_roundtrip() {
        let sys = egyptian_hieroglyphic();
        let json = serde_json::to_string(&sys).unwrap();
        let back: NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys, back);
    }

    // -- Chinese rod numerals --

    #[test]
    fn test_rod_numeral_digits() {
        let sys = chinese_rod_numerals();
        assert_eq!(sys.value_of("𝍠"), Some(1));
        assert_eq!(sys.value_of("𝍨"), Some(9));
        assert_eq!(sys.kind, NumeralSystemKind::Decimal);
    }

    #[test]
    fn test_rod_serde_roundtrip() {
        let sys = chinese_rod_numerals();
        let json = serde_json::to_string(&sys).unwrap();
        let back: NumeralSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(sys, back);
    }
}
