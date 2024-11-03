//! Everything dealing with the normalisation of the surface form of texts
//!
//! The normalisation part of the pipeline does these things:
//! - split texts into words (based on the word divisor)
//! - supply uncertain passages
//! - do other normalisations like nomina-sacra expansion or unicode-mapping for each language

use critic_core::{
    anchor::Anchor,
    atg::{AtgDialect, UniqueText},
};

use crate::{
    dialect::AtgDialectList,
    language::{Language, WordNormalForm},
    transcribe::FolioTranscriptMetadata,
};

#[cfg(feature = "atg_example")]
use crate::dialect::atg::ExampleAtgDialect;

/// A text which was normalised with the method relying on the language
#[derive(Debug)]
pub struct NonAgnosticAnchoredText {
    text: Vec<WordNormalForm>,
    anchor_positions: Vec<(Anchor, usize)>,
}
impl NonAgnosticAnchoredText {
    pub fn new(text: Vec<WordNormalForm>, anchor_positions: Vec<(Anchor, usize)>) -> Self {
        Self {
            text,
            anchor_positions,
        }
    }

    /// Render this text into the lex file presented to a human
    ///
    /// as_block_nr MUST be one-based
    pub fn render_for_lex_file(&self, as_block_nr: usize) -> String {
        // a table in insert order with anchors and individual words
        let mut res = String::new();
        let mut word_idx = 0;
        let mut words_till_anchor = String::new();
        for (anchor, anchor_idx) in self.anchor_positions.iter() {
            // push the entire sentence as a comment directly
            res.push_str("# ");
            // push the actual words later on, cache them in this loop
            words_till_anchor.clear();
            loop {
                if word_idx >= *anchor_idx {
                    words_till_anchor.push('\n');
                    break;
                };
                let word = &self.text[word_idx];
                res.push_str(word.display_form());
                res.push(' ');
                words_till_anchor.push_str(&word.render_for_lex_file(as_block_nr, word_idx + 1));
                words_till_anchor.push('\n');
                word_idx += 1;
            };
            res.push('\n');
            res.push_str(&words_till_anchor);
            // print this anchor
            res.push_str(&format!("[anchor.{anchor}]\n"));
        };
        // print the remaining words after the last anchor
        words_till_anchor.clear();
        res.push_str("# ");
        while word_idx < self.text.len() {
            let word = &self.text[word_idx];
            res.push_str(word.display_form());
            res.push(' ');
            words_till_anchor.push_str(&word.render_for_lex_file(as_block_nr, word_idx + 1));
            words_till_anchor.push('\n');
            word_idx += 1;
        };
        res.push('\n');
        res.push_str(&words_till_anchor);
        res
    }
}

/// A Block of ATG, with versions flattened out and words normalised
#[derive(Debug)]
pub struct NormalisedAtgBlock {
    /// the actual text, normalised and with anchor positions
    text: NonAgnosticAnchoredText,
    /// the language used in this block
    language: Language,
    /// the atg dialect used in this block
    atg_dialect: AtgDialectList,
}
impl NormalisedAtgBlock {
    pub fn render_for_lex_file(&self, as_block_nr: usize) -> String {
        // the block header
        let mut res = format!("[{as_block_nr}]\n");
        res.push_str(&format!("language = \"{}\"\n", self.language));
        res.push_str(&format!("atg = \"{}\"\n\n", self.atg_dialect));
        res.push_str(&self.text.render_for_lex_file(as_block_nr));
        res
    }
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

    pub fn normalise(self, language: Language) -> NormalisedAtgBlock {
        match self.atg_dialect {
            #[cfg(feature = "atg_example")]
            AtgDialectList::Example => self.inner_normalise::<ExampleAtgDialect>(language),
            _ => unreachable!(),
        }
    }

    /// Replace the text in this [UniqueAtgBlock] with the normalised text
    fn inner_normalise<D>(self, language: Language) -> NormalisedAtgBlock
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
#[derive(Debug)]
pub struct NormalisedFolioTranscript {
    metadata: FolioTranscriptMetadata,
    blocks: Vec<NormalisedAtgBlock>,
}
impl NormalisedFolioTranscript {
    pub fn new(metadata: FolioTranscriptMetadata, blocks: Vec<NormalisedAtgBlock>) -> Self {
        Self { metadata, blocks }
    }

    /// Render the lex file shown to a human to add lex and morph information
    pub fn render_lex_file(&self) -> String {
        // render the metadata block
        let mut res = toml::to_string(&self.metadata).expect("Statically infallible Serialization");
        res.push('\n');
        // render the other blocks
        for (idx, block) in self.blocks.iter().enumerate() {
            res.push_str(&block.render_for_lex_file(idx + 1));
        };
        res
    }
}
