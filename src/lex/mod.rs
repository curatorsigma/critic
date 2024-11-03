//! Everything about lexing a transcribed text
//!
//! This module handles lexem-assignment and morphological tagging.

use critic_core::{anchor::Anchor, atg::Word};
use serde::{Deserialize, Serialize};

use crate::language::{Example, SuperLanguage, WordNormalForm};

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

/// The error type for parsing a MorphPoint
#[derive(Debug)]
pub struct MorphPointParseError {
    location: usize,
    reason: String,
}
impl MorphPointParseError {
    pub fn new(location: usize, reason: String) -> Self {
        Self { location, reason }
    }
}
impl core::fmt::Display for MorphPointParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Error parsing MorphPoint at byte {}: {}.",
            self.location, self.reason
        )
    }
}
impl std::error::Error for MorphPointParseError {}

/// Implementors are types, the instances of which are complete Morphological Tags for words
///
/// An instance of an implementing type MUST be a complete morphological tag for a word.
///
/// The [`NAME`] MUST return a human readable identifier for this Schema, unique amongst all
/// names for all Schemas. It SHOULD NOT contain the word Point - this name will be used
/// for both the [MorphPointSchema] and the [MorphRangeSchema].
///
/// The [`Display`] impl of an implementing type MUST be a human readable rendering of the
/// MorphPoint, which defines it uniquely.
/// It SHOULD be easy to machine-read as well. For a good
/// example of how the [`Display`] may look, consider [openscripture's hebrew morph
/// codes](https://hb.openscriptures.org/parsing/HebrewMorphologyCodes.html).
/// The [`FromStr`] impl of an implementing type MUST be pseudo-inverse to its [`Display`], that is
/// parsing the output of a print must return an Object which is Eq to the initial object.
///
/// [`NAME`]: MorphPointSchema::NAME
/// [`Display`]: core::fmt::Display
/// [`FromStr`]: core::str::FromStr
pub trait MorphPointSchema:
    core::fmt::Display
    + core::fmt::Debug
    + core::cmp::Eq
    + core::str::FromStr<Err = MorphPointParseError>
{
    /// A unique name for this [MorphPointSchema]
    const NAME: &'static str;

    /// The type of Range that one of these Points can be in
    type Range: MorphRangeSchema<Point = Self>;
    /// true iff [`self`] is in `r`
    ///
    /// Calls the Ranges contains method on [self]
    fn is_in<R>(&self, r: &R) -> bool
    where
        R: MorphRangeSchema<Point = Self>,
    {
        r.contains(self)
    }
}

/// The error type for parsing a MorphRange
#[derive(Debug)]
pub struct MorphRangeParseError {
    location: usize,
    reason: String,
}
impl MorphRangeParseError {
    pub fn new(location: usize, reason: String) -> Self {
        Self { location, reason }
    }
}
impl core::fmt::Display for MorphRangeParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Error parsing MorphRange at byte {}: {}.",
            self.location, self.reason
        )
    }
}
impl std::error::Error for MorphRangeParseError {}

/// Implementors are types, the instances of which are sets of Morphological Tags
///
/// The [`Display`] impl of an implementing type MUST be a human readable rendering of the
/// morph range, which defines it uniquely.
/// It SHOULD be easy to machine-read as well.
/// The [`FromStr`] impl of an implementing type MUST be pseudo-inverse to its [`Display`], that is
/// parsing the output of a print must return an Object which is Eq to the initial object.
///
/// [`Display`]: core::fmt::Display
/// [`FromStr`]: core::str::FromStr
///
/// TODO: what is a good way to actually implement this?
pub trait MorphRangeSchema:
    core::fmt::Display
    + core::fmt::Debug
    + core::cmp::Eq
    + core::str::FromStr<Err = MorphRangeParseError>
{
    /// The type of Point that one of these Ranges contains
    type Point: MorphPointSchema<Range = Self>;
    /// true iff `p` is contained in [`self`]
    fn contains(&self, p: &Self::Point) -> bool;
}

/// A single lexed word
#[derive(Debug)]
pub struct LexWord<L>
where
    L: SuperLanguage,
{
    word: WordNormalForm,
    lexeme_id: L::Lex,
    morph: L::Morph,
}

/// A single block of text in a lex file, generic over the language
struct InnerLexBlock<L>
where
    L: SuperLanguage,
{
    /// The words in this block
    words: Vec<LexWord<L>>,
    /// A list of the encountered Anchors and the index of the word immediately following that
    /// anchor in [`words`](Self::words).
    anchors: Vec<(Anchor, usize)>,
}

/// A non-generic type containing the [InnerLexBlock] for each known language
enum LexBlock {
    #[cfg(feature = "language_example")]
    Example(Vec<InnerLexBlock<Example>>),
}

/// An error that can occur while parsing the morph and lex information contained in a [LexWordData]
#[derive(Debug)]
pub enum IntoLexWordError {
    LexParsing(LexParseError),
    MorphParsing(MorphPointParseError),
}
impl core::fmt::Display for IntoLexWordError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::LexParsing(x) => write!(f, "{x}"),
            Self::MorphParsing(x) => write!(f, "{x}"),
        }
    }
}
impl std::error::Error for IntoLexWordError {}
impl From<LexParseError> for IntoLexWordError {
    fn from(value: LexParseError) -> Self {
        Self::LexParsing(value)
    }
}
impl From<MorphPointParseError> for IntoLexWordError {
    fn from(value: MorphPointParseError) -> Self {
        Self::MorphParsing(value)
    }
}

/// This struct is used only when serializing LexWordData
#[derive(Serialize)]
struct Helper {
    surface_form: Word,
}

/// The data for a single word as presented to humans in lex files
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LexWordDataHumanReadable {
    display_form: String,
    compare_form: Option<String>,
    lexeme_id: String,
    morph: String,
}
impl LexWordDataHumanReadable {
    /// Add a surface form
    ///
    /// The surface form is not output into the human readable lex file, because its structure is
    /// unnecessarily complicated (and its content is also unnecessary while manually lexing).
    /// This function is then used to add the surface form back in.
    pub fn enrich_to_lex_word_data(self, surface_form: Word) -> LexWordData {
        LexWordData {
            surface_form,
            display_form: self.display_form,
            compare_form: self.compare_form,
            lexeme_id: self.lexeme_id,
            morph: self.morph,
        }
    }
}

/// The full data for a single word before Lex and Morph are parsed
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LexWordData {
    surface_form: Word,
    display_form: String,
    compare_form: Option<String>,
    lexeme_id: String,
    morph: String,
}
impl LexWordData {
    pub fn new(
        surface_form: Word,
        display_form: String,
        compare_form: Option<String>,
        lexeme_id: String,
        morph: String,
    ) -> Self {
        Self {
            surface_form,
            display_form,
            compare_form,
            lexeme_id,
            morph,
        }
    }

    /// Output this [LexWordData] in the format used for lex files
    ///
    /// Note that this function is NOT a pseudo-inverse to Deserialization and is NOT equivalent to
    /// Serialization.
    /// Instead, this function creates the output that will be presented to humans while lexing.
    ///
    /// When the output is parsed as [LexWordDataHumanReadable] and enriced with the correct word via
    /// [LexWordDataHumanReadable::enrich_to_lex_word_data], you get the same word back.
    pub fn to_toml_str(&self) -> String {
        let mut res = String::new();

        res.push_str(&"display_form = \"");
        res.push_str(&self.display_form);
        res.push_str("\"\n");

        if let Some(cmp_form) = &self.compare_form {
            res.push_str(&"compare_form = \"");
            res.push_str(cmp_form);
            res.push_str("\"\n");
        };

        res.push_str(&"lexeme_id = \"");
        res.push_str(&self.lexeme_id);
        res.push_str("\"\n");

        res.push_str(&"morph = \"");
        res.push_str(&self.morph);
        res.push_str("\"\n");

        res
    }

    /// Try to parse the user supplied lex and morph data into their internal representation
    pub fn into_lex_word<L>(self) -> Result<LexWord<L>, IntoLexWordError>
    where
        L: SuperLanguage,
    {
        let word = WordNormalForm::new(self.surface_form, self.display_form, self.compare_form);
        let lexeme_id = self.lexeme_id.parse::<L::Lex>()?;
        let morph = self.morph.parse::<L::Morph>()?;
        Ok(LexWord {
            word,
            lexeme_id,
            morph,
        })
    }
}

struct LexMetadata {
    lexer: String,
    editors: Vec<String>,
}

/// A single Folio, with lex information added
///
/// This struct will be created once human-supplied lex data is added.
struct LexedFolioTranscript {
    metadata: LexMetadata,
    blocks: Vec<LexBlock>,
}

#[cfg(test)]
mod test {
    use critic_core::atg::Word;

    use crate::lex::{LexWordData, LexWordDataHumanReadable};

    /// Ensure that Serialization and Deserialization for [LexWordData] work as intended
    #[test]
    fn ser_de_lex_word_data() {
        let word: Word = toml::de::from_str("[[parts]]\nNative = \"some\"\n").unwrap();
        let lexworddata = LexWordData::new(
            word.clone(),
            "some".to_owned(),
            None,
            "1".to_owned(),
            "N".to_owned(),
        );
        let ser = lexworddata.to_toml_str();
        let deser: LexWordDataHumanReadable = toml::from_str(&ser).unwrap();
        let enriched = deser.enrich_to_lex_word_data(word);
        assert_eq!(enriched, lexworddata);
    }
}
