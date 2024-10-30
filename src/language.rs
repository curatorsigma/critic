//! Everything to do with defining natural languages

use critic_core::atg::Word;
use serde::Deserialize;

mod example;

/// A natural language which has an associated lexeme- and morphological system.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Example Language
    #[cfg(feature = "language_example")]
    Example,
}

impl Language {
    /// Select the correct Language, given its name
    pub fn from_name(s: &str)-> Option<Self> {
        match s {
            #[cfg(feature = "language_example")]
            "example" => Some(Language::Example),
            _ => None,
        }
    }

    /// Do the normalisation steps which depend on the language
    pub fn normalise(&self, text: Vec<(Word, String)>) -> Vec<WordNormalForm> {
        match self {
            Self::Example => {
                crate::language::example::normalise(text)
            }
        }
    }
}

/// Normal form of a word
pub struct WordNormalForm {
    annotated_form: Word,
    /// Form used for displaying the word when displayed without ATG annotations
    display_form: String,
    /// Form for comparing this word to other words
    ///
    /// This is mainly useful for languages which have skeletal forms which naturally compare,
    /// while the display forms vary.
    /// When not given, comparison will happen on the display_form itself.
    compare_form: Option<String>,
}
impl WordNormalForm {
    pub fn new(annotated_form: Word, display_form: String, compare_form: Option<String>) -> Self {
        Self {
            annotated_form, display_form, compare_form,
        }
    }

    pub fn display_form(self) -> String {
        self.display_form
    }
}
