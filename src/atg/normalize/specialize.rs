//! Normalize a tokenized ATG stream dependent on a specific Language

#[cfg(feature = "atg_example")]
use crate::atg::dialect::ExampleAtgDialect;
use crate::{
    anchor::Anchor,
    atg::{dialect::AtgDialectList, AtgBlock, AtgDialect, Word},
    language::Language,
};

use super::flatten::UniqueAtgBlock;

/// Normal form of a word
#[derive(Debug)]
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
            annotated_form,
            display_form,
            compare_form,
        }
    }

    pub fn display_form(&self) -> &str {
        &self.display_form
    }

    /// Render this word as part of a lex file presented to a human
    ///
    /// as_block_nr and word_idx MUST be one-based
    pub fn render_for_lex_file(&self, as_block_nr: usize, word_idx: usize) -> String {
        let mut res = format!("[{as_block_nr}.word{word_idx}]\n");
        res.push_str(&format!("display_form = \"{}\"\n", self.display_form));
        if let Some(cmp_form) = &self.compare_form {
            res.push_str(&format!("compare_form = \"{}\"\n", cmp_form));
        };
        // TODO: allow critic to automatically lex here
        // instead define render_for_lex_file for a type which Option<Lex> and
        // Option<Morph>, and if Some(x) is defined there, output the string representation
        // instead of --TODO--
        res.push_str("lex = \"--TODO--\"\n");
        res.push_str("morph = \"--TODO--\"\n");
        res
    }
}

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
            }
            res.push('\n');
            res.push_str(&words_till_anchor);
            // print this anchor
            res.push_str(&format!("[anchor.{anchor}]\n"));
        }
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
        }
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
impl UniqueAtgBlock {
    pub fn normalise(self, language: Language) -> NormalisedAtgBlock {
        match self.atg_dialect {
            #[cfg(feature = "atg_example")]
            AtgDialectList::Example => self.inner_normalise::<ExampleAtgDialect>(language),
            #[allow(unreachable_patterns)]
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

impl AtgBlock {
    /// Do the entire noramlisation, including specialization
    pub fn into_normalised_blocks(self) -> impl Iterator<Item = NormalisedAtgBlock> {
        let lang = self.language;
        self.into_unique_blocks().map(move |b| b.normalise(lang))
    }
}
