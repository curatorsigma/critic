//! Everything dealing with the normalisation of the surface form of texts
//!
//! The normalisation part of the pipeline does these things:
//! - split texts into words (based on the word divisor)
//! - supply uncertain passages
//! - do other normalisations like nomina-sacra expansion or unicode-mapping for each language

use std::collections::HashMap;

use critic_core::{anchor::Anchor, atg::{AtgDialect, Text, UniqueText}};

use crate::{dialect::AtgDialectList, language::{Language, WordNormalForm}, transcribe::AtgBlock};

pub fn normalise<D>(text: Text, language: Language) -> Vec<Vec<WordNormalForm>>
where
    D: AtgDialect,
{
    text.auto_normalise::<D>()
        .into_iter()
        .map(|w| language.normalise(w))
        .collect::<Vec<_>>()
}

/// A Block of ATG, with versions flattened out and words normalised
pub struct NormalisedAtgBlock {
    /// the actual text in this block
    text: Vec<WordNormalForm>,
    /// The positions for individual anchors
    anchor_positions: HashMap<Anchor, usize>,
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
            text, language, atg_dialect,
        }
    }

    pub fn normalise(self) -> NormalisedAtgBlock {
        // iterate over the text, creating the anchor positions and word normal forms
        todo!();
    }
}

pub fn normalise_atg_block(block: AtgBlock) -> impl Iterator<Item = NormalisedAtgBlock> {
    block.into_unique_blocks().map(|b| b.normalise())
}
