//! Error types for lipi.

use thiserror::Error;

/// Errors that can occur in language processing.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LipiError {
    /// Language not found in the registry.
    #[error("unknown language: {0}")]
    UnknownLanguage(String),

    /// Script/writing system not found.
    #[error("unknown script: {0}")]
    UnknownScript(String),

    /// Phoneme not in the language's inventory.
    #[error("phoneme {phoneme} not in {language} inventory")]
    PhonemeNotInInventory { phoneme: String, language: String },

    /// Invalid IPA symbol.
    #[error("invalid IPA symbol: {0}")]
    InvalidIpa(String),

    /// Lexicon entry not found.
    #[error("word not found: {word} in {language}")]
    WordNotFound { word: String, language: String },
}
