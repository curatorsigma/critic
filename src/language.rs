//! Everything to do with defining natural languages

use serde::Deserialize;

pub mod dialect;

mod lex;
pub use lex::LexSchema;

mod morph;
pub use morph::{MorphPointSchema, MorphRangeSchema};

use crate::atg::normalize::{AnchoredNormalisedText, NonAgnosticAnchoredText};

/// Supertrait for natural Languages in critic
/// TODO: better docs
pub trait SuperLanguage {
    type Morph: MorphPointSchema;
    type Lex: LexSchema;

    fn normalise(input: AnchoredNormalisedText) -> NonAgnosticAnchoredText;
}

/// A natural language which has an associated lexeme- and morphological system.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Example Language
    #[cfg(feature = "language_example")]
    Example,
}

impl Language {
    /// Select the correct Language, given its name
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            #[cfg(feature = "language_example")]
            "example" => Some(Language::Example),
            _ => None,
        }
    }

    /// Do the normalisation steps which depend on the language
    pub fn normalise(&self, text: AnchoredNormalisedText) -> NonAgnosticAnchoredText {
        match self {
            #[cfg(feature = "language_example")]
            Self::Example => crate::language::dialect::Example::normalise(text),
            // this happens only if Language is empty (no language feature enabled)
            // but in this case, Language is the bottom type anyways
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            #[cfg(feature = "language_example")]
            Self::Example => write!(f, "example"),
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}
