//! Defines the LexSchema and relevant associated types
//! TODO: better docs


/// Implementors are types, the instances of which are unique Lexeme-IDs
///
/// An instance of an implementing Type MUST be a unique (for its type) ID that can be used to
/// uniquely identify a single lexeme in the Schema.
///
/// The [`NAME`] const MUST be a human readable identifier for this Schema, unique amongst all
/// names for all Schemas.
///
/// The [`Display`] impl of an implementing type MUST be a human readable rendering of the
/// ID, which defines it uniquely.
/// The [`FromStr`] impl of an implementing type MUST be pseudo-inverse to its [`Display`], that is
/// parsing the output of a print must return an Object which is Eq to the initial object.
///
/// [`NAME`]: LexSchema::NAME
/// [`Display`]: core::fmt::Display
/// [`FromStr`]: core::str::FromStr
pub trait LexSchema:
    core::fmt::Display + core::fmt::Debug + core::cmp::Eq + core::str::FromStr<Err = LexParseError>
{
    const NAME: &'static str;
}


#[derive(Debug)]
pub struct LexParseError {
    location: usize,
    reason: String,
}
impl LexParseError {
    pub fn new(location: usize, reason: String) -> Self {
        Self { location, reason }
    }
}
impl core::fmt::Display for LexParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Error parsing Lex at byte {}: {}.",
            self.location, self.reason
        )
    }
}
impl std::error::Error for LexParseError {}

