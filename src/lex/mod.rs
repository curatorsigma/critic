//! Everything about lexing a transcribed text
//!
//! This module handles lexem-assignment and morphological tagging.

use std::{collections::HashMap, str::FromStr};

use critic_core::{anchor::Anchor, atg::Word};
use serde::Deserialize;

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
struct LexWord<L>
where
    L: SuperLanguage,
{
    word: WordNormalForm,
    lexeme_id: L::Lex,
    morph: L::Morph,
}

struct InnerLexBlock<L>
where
    L: SuperLanguage,
{
    versions: Vec<LexWord<L>>,
    anchors: HashMap<Anchor, usize>,
}

enum LexBlock {
    #[cfg(feature = "language_example")]
    Example(InnerLexBlock<Example>),
}

#[derive(Debug)]
enum IntoLexWordError {
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

#[derive(Debug, Deserialize)]
struct LexWordData {
    surface_form: Word,
    display_form: String,
    compare_form: Option<String>,
    lexeme_id: String,
    morph: String,
}
impl LexWordData {
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
