//! Normalize an ATG Stream in a text with words in normal forms and Anchors pointing to certain
//! indices in that stream of words.

use serde::{Deserialize, Serialize};

use super::{AtgDialect, Illegible, Lacuna, Uncertain};

// TODO: make all these names consistent
/// Takes plain ATG and flattens out corrections into multiple Texts, each belonging to a separate
/// scribes hand
mod flatten;
pub use flatten::UniqueText;
/// Takes ATG without corrections and splits it into individual words, ignoring formatting
mod tokenize;
pub use tokenize::AnchoredNormalisedText;
/// Take tokenized ATG, supply with the proposed data
mod specialize;
pub use specialize::{NonAgnosticAnchoredText, NormalisedAtgBlock, WordNormalForm};

/// Like [Part]. but
/// - No [Correction]s
/// - Nothing that is not represented in the Surface Text of the transcribed natural language
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub enum UniqueSurfacePart {
    Native(String),
    Illegible(Uncertain<Illegible>),
    Lacuna(Uncertain<Lacuna>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Word {
    parts: Vec<UniqueSurfacePart>,
}
impl Word {
    fn supply_uncertain<D>(self) -> (Word, String)
    where
        D: AtgDialect,
    {
        let mut res = String::new();
        for part in &self.parts {
            res.push_str(&part.supply_uncertain::<D>())
        }
        (self, res)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        anchor::AnchorDialect,
        atg::{
            dialect::ExampleAtgDialect,
            normalize::{AnchoredNormalisedText, UniqueSurfacePart, Word},
            Text,
        },
    };

    #[test]
    #[cfg(feature = "anchor_example")]
    fn split_word_simple() {
        let input = " A B";
        let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 0).unwrap();
        let normalised = parsed
            .auto_normalise::<ExampleAtgDialect>()
            .collect::<Vec<_>>();
        let normalised = normalised.get(0).unwrap();
        let res = AnchoredNormalisedText {
            text: vec![
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("A".to_owned())],
                    },
                    "A".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("B".to_owned())],
                    },
                    "B".to_owned(),
                ),
            ],
            anchor_positions: vec![],
        };
        assert_eq!(*normalised, res);
    }

    #[test]
    #[cfg(feature = "anchor_example")]
    fn split_words_complicated() {
        let input = "A sentence. Another, sentence.without";
        let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 0).unwrap();
        let normalised = parsed
            .auto_normalise::<ExampleAtgDialect>()
            .collect::<Vec<_>>();
        let normalised = normalised.get(0).unwrap();
        let res = AnchoredNormalisedText {
            text: vec![
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("A".to_owned())],
                    },
                    "A".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("sentence".to_owned())],
                    },
                    "sentence".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native(".".to_owned())],
                    },
                    ".".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("Another".to_owned())],
                    },
                    "Another".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native(",".to_owned())],
                    },
                    ",".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("sentence".to_owned())],
                    },
                    "sentence".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native(".".to_owned())],
                    },
                    ".".to_owned(),
                ),
                (
                    Word {
                        parts: vec![UniqueSurfacePart::Native("without".to_owned())],
                    },
                    "without".to_owned(),
                ),
            ],
            anchor_positions: vec![],
        };
        assert_eq!(*normalised, res);
    }
}
