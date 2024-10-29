//! Everything to do with defining natural languages

/// A natural language which has an associated lexeme- and morphological system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Example Language
    #[cfg(feature = "language_example")]
    Example,
}

impl Language {
    pub fn from_name(s: &str)-> Option<Self> {
        match s {
            #[cfg(feature = "language_example")]
            "example" => Some(Language::Example),
            _ => None,
        }
    }
}
