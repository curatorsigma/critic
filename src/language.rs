//! Everything to do with defining natural languages

mod example;

/// A natural language which has an associated lexeme- and morphological system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    pub fn normalise(&self, text: Vec<String>) -> Vec<String> {
        match self {
            Self::Example => {
                crate::language::example::normalise(text)
            }
        }
    }
}
