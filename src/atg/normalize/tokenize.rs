//! Tokenizing a stream of ATG into words.

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{anchor::Anchor, atg::Uncertain};

use super::{flatten::{UniquePart, UniqueText}, AtgDialect, UniqueSurfacePart, Word};


#[derive(Serialize, Deserialize, Debug)]
struct BoundedWordChain {
    left_boundary_divides: bool,
    word_chain: Vec<Word>,
    right_boundary_divides: bool,
}

struct WordSplitIterator<'a, D>
where D: AtgDialect,
{
    original: &'a str,
    /// the index, if the last char was a punctuation
    last_char_was_punctuation: Option<usize>,
    /// an iterator over the Chars in the original string
    characters: core::str::CharIndices<'a>,
    _dialect: PhantomData<D>,
}
impl<'a, D> WordSplitIterator<'a, D>
where D: AtgDialect,
{
    pub fn new(s: &'a str) -> WordSplitIterator<'a, D> {
        Self {
            original: s,
            last_char_was_punctuation: None,
            characters: s.char_indices(),
            _dialect: PhantomData::<D>,
        }
    }
}
/// The first return is the substring
/// the second return is true iff that substrings last character ends a word
impl<'a, D> Iterator for WordSplitIterator<'a, D> where D: AtgDialect, {
    type Item = (&'a str, bool);
    fn next(&mut self) -> Option<Self::Item> {
        match self.last_char_was_punctuation {
            None => {
                if let Some((start_idx, start_char)) = self.characters.next() {
                    if start_char == D::WORD_DIVISOR {
                        return Some(("", false));
                    };
                    if D::PUNCTUATION.contains(start_char) {
                        return Some((&self.original[start_idx..start_idx + start_char.len_utf8()], true));
                    };
                    while let Some((next_idx, next_char)) = self.characters.next() {
                        if next_char == D::WORD_DIVISOR {
                            let res = Some((&self.original[start_idx..next_idx], true));
                            return res;
                        } else if D::PUNCTUATION.contains(next_char) {
                            self.last_char_was_punctuation = Some(next_idx);
                            return Some((&self.original[start_idx..next_idx], true));
                        };
                    };
                    Some((&self.original[start_idx..], false))
                } else {
                    None
                }
            },
            Some(x) => {
                let res = Some((&self.original[x..=x], true));
                self.last_char_was_punctuation = None;
                res
            }
        }
    }
}
/// Given a raw stream in the natural language, split it along words.
fn split_native_stream<D>(s: &str) -> WordSplitIterator<D>
where D: AtgDialect,
{
    WordSplitIterator::<D>::new(s)
}


impl UniqueText {
    /// Split a [UniqueText] into words, adding anchor positions
    ///
    /// The return is:
    /// - a vec of words
    /// - a Map from Anchor to the index in the first return value directly after the Anchor in
    ///   logical order
    pub fn split_words<D>(self) -> AnchoredUniqueText
    where
        D: AtgDialect,
    {
        let mut words = Vec::<Word>::new();
        let mut anchors = Vec::<(Anchor, usize)>::new();
        let mut break_after_last = None::<bool>;
        for part in self.parts {
            match part {
                UniquePart::Anchor(x) => {
                    // an Anchor breaks the current word
                    // meaning that words last entry contains the correct word directly before the
                    // Anchor in logical ordering.
                    anchors.push((x, words.len()));
                    // the next word will have to be inserted as a new word
                    break_after_last = Some(true);
                }
                // Format breaks do not matter for word separation or normalisation
                UniquePart::FormatBreak(_) => {}
                x => {
                    let mut bounded_chain = x
                        .as_surface_part()
                        .expect("UniquePart that is not Anchor and not FormatBreak is UniqueSurfacePart")
                        .split_words::<D>();
                    if let Some(last_part_ended_word) = break_after_last {
                        // either this part starts a new word or the last part ended a word
                        if bounded_chain.left_boundary_divides || last_part_ended_word {
                            // simply add the next word
                            words.append(&mut bounded_chain.word_chain);
                        } else {
                            let mut first_word_of_bounded_chain =
                                bounded_chain.word_chain.remove(0);
                            match words.last_mut() {
                                // this is the first word; add it
                                None => {
                                    words.push(first_word_of_bounded_chain);
                                }
                                // this a word later in the text. We need to add the parts of the first
                                // word in this part to that word
                                Some(x) => x.parts.append(&mut first_word_of_bounded_chain.parts),
                            };
                            // all other words in this part need to be pushed
                            words.append(&mut bounded_chain.word_chain);
                        };
                        // the next iteration of the loop needs to know whether this part ended on a
                        // word boundary
                        break_after_last = Some(bounded_chain.right_boundary_divides);

                    // the text was empty so far
                    } else {
                        // add all words from this part
                        words.append(&mut bounded_chain.word_chain);
                        break_after_last = Some(bounded_chain.right_boundary_divides);
                    };
                }
            };
        }
        AnchoredUniqueText {
            text: words,
            anchor_positions: anchors,
        }
    }
}

/// A [Text], split into individual words, but:
/// - with no [Correction]s
/// - With [Anchor]s split off into a hash map into indices of words in the text.
#[derive(Debug)]
pub struct AnchoredUniqueText {
    text: Vec<Word>,
    anchor_positions: Vec<(Anchor, usize)>,
}
impl AnchoredUniqueText {
    pub fn into_anchored_normalised_text<D>(self: AnchoredUniqueText) -> AnchoredNormalisedText
    where
        D: AtgDialect,
    {
        AnchoredNormalisedText {
            text: self
                .text
                .into_iter()
                .map(|w| w.supply_uncertain::<D>())
                .collect(),
            anchor_positions: self.anchor_positions,
        }
    }
}

/// An [AnchoredUniqueText], but:
/// - to each word is appended the supplied surface form
#[derive(Debug, PartialEq, Eq)]
pub struct AnchoredNormalisedText {
    // TODO: remove inner pubs once critic and critic_core are merged
    pub text: Vec<(Word, String)>,
    pub anchor_positions: Vec<(Anchor, usize)>,
}


impl UniqueSurfacePart {
    /// Split a single ATG Part into its constituent words, without flattening [Uncertain] or
    /// [Correction]
    fn split_words<D>(self) -> BoundedWordChain
    where
        D: AtgDialect,
    {
        let mut res = Vec::<Word>::new();
        let mut left_boundary_divides = false;
        let mut right_boundary_divides = None;
        match self {
            Self::Native(x) => {
                // literally split along word divisors, spit out a list of Native
                'word: for (idx, (word, word_definitely_closed)) in split_native_stream::<D>(&x).enumerate() {
                    right_boundary_divides = Some(word_definitely_closed);
                    if word.is_empty() {
                        if idx == 0 {
                            left_boundary_divides = true;
                        };
                        continue 'word;
                    } else {
                        let word_as_obj = Word {
                            parts: vec![UniqueSurfacePart::Native(word.to_owned())],
                        };
                        res.push(word_as_obj);
                    };
                }
                BoundedWordChain {
                    left_boundary_divides,
                    word_chain: res,
                    right_boundary_divides: right_boundary_divides
                        .expect("Each iteration should set the right boundary correctly."),
                }
            }
            Self::Illegible(x) => {
                match x.proposal {
                    None => {
                        res.push(Word {
                            parts: vec![UniqueSurfacePart::Illegible(x)],
                        });
                        BoundedWordChain {
                            left_boundary_divides: true,
                            word_chain: res,
                            right_boundary_divides: true,
                        }
                    }
                    Some(prop) => {
                        // similar to native
                        'word: for (idx, (word, word_definitely_closed)) in split_native_stream::<D>(&prop).enumerate() {
                            right_boundary_divides = Some(word_definitely_closed);
                            if word.len() < 1 {
                                if idx == 0 {
                                    left_boundary_divides = true;
                                };
                                continue 'word;
                            } else {
                                let word_as_obj = Word {
                                    parts: vec![UniqueSurfacePart::Illegible(Uncertain::new(
                                        word.len().try_into().expect(
                                            "Uncertain Passages can never be longer then u8",
                                        ),
                                        Some(word.to_owned()),
                                    ))],
                                };
                                res.push(word_as_obj);
                            };
                        }
                        BoundedWordChain {
                            left_boundary_divides,
                            word_chain: res,
                            right_boundary_divides: right_boundary_divides
                                .expect("Each iteration should set the right boundary correctly."),
                        }
                    }
                }
            }
            // equal to Illegible
            Self::Lacuna(x) => {
                match x.proposal {
                    None => {
                        res.push(Word {
                            parts: vec![UniqueSurfacePart::Lacuna(x)],
                        });
                        BoundedWordChain {
                            left_boundary_divides: true,
                            word_chain: res,
                            right_boundary_divides: true,
                        }
                    }
                    Some(prop) => {
                        // similar to native
                        'word: for (idx, (word, word_definitely_closed)) in split_native_stream::<D>(&prop).enumerate() {
                            right_boundary_divides = Some(word_definitely_closed);
                            if word.len() < 1 {
                                if idx == 0 {
                                    left_boundary_divides = true;
                                };
                                continue 'word;
                            } else {
                                let word_as_obj = Word {
                                    parts: vec![UniqueSurfacePart::Lacuna(Uncertain::new(
                                        word.len().try_into().expect(
                                            "Uncertain Passages can never be longer then u8",
                                        ),
                                        Some(word.to_owned()),
                                    ))],
                                };
                                res.push(word_as_obj);
                            };
                        }
                        BoundedWordChain {
                            left_boundary_divides,
                            word_chain: res,
                            right_boundary_divides: right_boundary_divides
                                .expect("Each iteration should set the right boundary correctly."),
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(feature = "anchor_example")]
    fn word_split_atg() {
        use crate::{anchor::AnchorDialect, atg::{dialect::ExampleAtgDialect, normalize::UniqueText, Text}};

        let input = "This ^(1)(i)s /(line)so~(4)()ext.";
        let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 1).unwrap();
        let uniques: Vec<UniqueText>  = parsed.into();
        assert_eq!(uniques.len(), 1);
        let words_split = uniques
            .into_iter()
            .map(|x| x.split_words::<ExampleAtgDialect>())
            .collect::<Vec<_>>();
        assert_eq!(words_split.get(0).unwrap().text.len(), 6);

        let input = "some^(1)(x)text";
        let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 1).unwrap();
        let uniques: Vec<UniqueText> = parsed.into();
        assert_eq!(uniques.len(), 1);
        let words_split = uniques
            .into_iter()
            .map(|x| x.split_words::<ExampleAtgDialect>())
            .collect::<Vec<_>>();
        assert_eq!(words_split.get(0).unwrap().text.len(), 1);

        let input = "som§(1)e^(1)()te/(line)xt";
        let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 1).unwrap();
        let uniques: Vec<UniqueText> = parsed.into();
        assert_eq!(uniques.len(), 1);
        let words_split = uniques
            .into_iter()
            .map(|x| x.split_words::<ExampleAtgDialect>())
            .collect::<Vec<_>>();
        assert_eq!(words_split.get(0).unwrap().text.len(), 4);
    }
}
