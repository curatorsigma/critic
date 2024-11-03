//! Everything dealing with the normalisation of the surface form of texts
//!
//! The normalisation part of the pipeline does these things:
//! - split texts into words (based on the word divisor)
//! - supply uncertain passages
//! - do other normalisations like nomina-sacra expansion or unicode-mapping for each language

use std::collections::HashMap;

use critic_core::{
    anchor::Anchor,
    atg::{AtgDialect, UniqueText},
};

use crate::{
    dialect::AtgDialectList,
    language::{Language, WordNormalForm},
    transcribe::FolioTranscriptMetadata,
};

// pub fn normalise<D>(text: Text, language: Language) -> Vec<Vec<WordNormalForm>>
// where
//     D: AtgDialect,
// {
//     text.auto_normalise::<D>()
//         .into_iter()
//         .map(|w| language.normalise(w))
//         .collect::<Vec<_>>()
// }

pub struct NonAgnosticAnchoredText {
    text: Vec<WordNormalForm>,
    anchor_positions: HashMap<Anchor, usize>,
}
impl NonAgnosticAnchoredText {
    pub fn new(text: Vec<WordNormalForm>, anchor_positions: HashMap<Anchor, usize>) -> Self {
        Self {
            text,
            anchor_positions,
        }
    }
}

/// A Block of ATG, with versions flattened out and words normalised
pub struct NormalisedAtgBlock {
    /// the actual text, normalised and with anchor positions
    text: NonAgnosticAnchoredText,
    /// the language used in this block
    language: Language,
    /// the atg dialect used in this block
    atg_dialect: AtgDialectList,
}

/// A block of ATG text without corrections
pub struct UniqueAtgBlock {
    /// The text in this Block
    ///
    /// It does not have Corrections in it.
    text: UniqueText,
    /// the language used in this block
    language: Language,
    /// the atg dialect used in this block
    atg_dialect: AtgDialectList,
}
impl UniqueAtgBlock {
    pub fn new(text: UniqueText, language: Language, atg_dialect: AtgDialectList) -> Self {
        Self {
            text,
            language,
            atg_dialect,
        }
    }

    /// Replace the text in this [UniqueAtgBlock] with the normalised text
    pub fn normalise<D>(self, language: Language) -> NormalisedAtgBlock
    where
        D: AtgDialect,
    {
        let text_agnostic = self
            .text
            .split_words::<D>()
            .into_anchored_normalised_text::<D>();
        NormalisedAtgBlock {
            text: language.normalise(text_agnostic),
            language: self.language,
            atg_dialect: self.atg_dialect,
        }
    }
}

/// A transcribed Folio,
pub struct NormalisedFolioTranscript {
    metadata: FolioTranscriptMetadata,
    blocks: Vec<NormalisedAtgBlock>,
}
impl NormalisedFolioTranscript {
    pub fn new(metadata: FolioTranscriptMetadata, blocks: Vec<NormalisedAtgBlock>) -> Self {
        Self { metadata, blocks }
    }
}
