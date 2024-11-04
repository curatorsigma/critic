//! Flatten a Text with multiple corrections to several texts, one for each correcting hand

use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{anchor::Anchor, atg::{dialect::AtgDialectList, AtgBlock, AtgDialect, FormatBreak, Illegible, Lacuna, Part, Present, Text, Uncertain}, language::Language};

use super::UniqueSurfacePart;


/// Split a [Text] into multiple [UniqueText], each corresponding to one correctors hand.
impl From<Text> for Vec<UniqueText> {
    fn from(value: Text) -> Self {
        let num_of_corrections = value
            .parts
            .iter()
            .filter_map(|p| match p {
                Part::Correction(x) => Some(x.versions.len()),
                _ => None,
            })
            .max()
            .unwrap_or(1);
        let mut res = vec![UniqueText::new(); num_of_corrections];
        for part in value.parts.into_iter() {
            match part {
                Part::Correction(x) => {
                    if x.versions.len() < num_of_corrections {
                        panic!("A correction was encountered that is longer than the longest correction in the containing text.");
                    };
                    for (idx, unique_text) in res.iter_mut().enumerate() {
                        unique_text.add_part(
                            x.versions
                                .get(idx)
                                .map(|x| x.clone())
                                .unwrap_or(Present::Native("".to_owned()))
                                .into(),
                        );
                    }
                }
                Part::Native(x) => {
                    for unique_text in res.iter_mut() {
                        unique_text.add_part(Present::Native(x.clone()).into());
                    }
                }
                Part::Illegible(x) => {
                    for unique_text in res.iter_mut() {
                        unique_text.add_part(UniquePart::Illegible(x.clone()));
                    }
                }
                Part::Lacuna(x) => {
                    for unique_text in res.iter_mut() {
                        unique_text.add_part(UniquePart::Lacuna(x.clone()));
                    }
                }
                Part::FormatBreak(x) => {
                    for unique_text in res.iter_mut() {
                        unique_text.add_part(UniquePart::FormatBreak(x));
                    }
                }
                Part::Anchor(x) => {
                    for unique_text in res.iter_mut() {
                        unique_text.add_part(UniquePart::Anchor(x));
                    }
                }
            };
        }
        res
    }
}




/// Like [Part], but
/// - No [Correction]s
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub enum UniquePart {
    Native(String),
    Illegible(Uncertain<Illegible>),
    Lacuna(Uncertain<Lacuna>),
    FormatBreak(FormatBreak),
    Anchor(Anchor),
}
impl UniquePart {
    /// Return true iff this [UniquePart] represents a part of the natural language
    pub fn as_surface_part(self) -> Option<UniqueSurfacePart> {
        match self {
            Self::Native(x) => Some(UniqueSurfacePart::Native(x)),
            Self::Illegible(x) => Some(UniqueSurfacePart::Illegible(x)),
            Self::Lacuna(x) => Some(UniqueSurfacePart::Lacuna(x)),
            Self::FormatBreak(_) => None,
            Self::Anchor(_) => None,
        }
    }
}
impl From<Present> for UniquePart {
    fn from(value: Present) -> Self {
        match value {
            Present::Native(x) => UniquePart::Native(x),
            Present::Illegible(x) => UniquePart::Illegible(x),
        }
    }
}

impl UniqueSurfacePart {
    /// Supply all uncertain characters with their proposal or the replacement character.
    pub fn supply_uncertain<D>(&self) -> String
    where
        D: AtgDialect,
    {
        match self {
            Self::Native(x) => x.to_owned(),
            Self::Illegible(x) => match &x.proposal {
                None => std::iter::repeat(D::ATG_CONTROL_POINTS.illegible)
                    .take(x.len as usize)
                    .collect::<String>(),
                Some(x) => x.to_owned(),
            },
            Self::Lacuna(x) => match &x.proposal {
                None => std::iter::repeat(D::ATG_CONTROL_POINTS.lacuna)
                    .take(x.len as usize)
                    .collect::<String>(),
                Some(x) => x.to_owned(),
            },
        }
    }
}
impl From<UniqueSurfacePart> for UniquePart {
    fn from(value: UniqueSurfacePart) -> Self {
        match value {
            UniqueSurfacePart::Native(x) => UniquePart::Native(x),
            UniqueSurfacePart::Illegible(x) => UniquePart::Illegible(x),
            UniqueSurfacePart::Lacuna(x) => UniquePart::Lacuna(x),
        }
    }
}


/// Like [Text], but
/// - No [Correction]s
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Hash)]
pub struct UniqueText {
    pub(super) parts: Vec<UniquePart>,
}
impl UniqueText {
    pub fn new() -> Self {
        Self { parts: vec![] }
    }

    fn add_part(&mut self, p: UniquePart) {
        self.parts.push(p);
    }

}

/// A block of ATG text without corrections
pub struct UniqueAtgBlock {
    /// The text in this Block
    ///
    /// It does not have Corrections in it.
    pub(super) text: UniqueText,
    /// the language used in this block
    pub(super) language: Language,
    /// the atg dialect used in this block
    pub(super) atg_dialect: AtgDialectList,
}
impl UniqueAtgBlock {
    pub fn new(text: UniqueText, language: Language, atg_dialect: AtgDialectList) -> Self {
        Self {
            text,
            language,
            atg_dialect,
        }
    }
}

impl AtgBlock {
    /// flatten out different corrections in one AtgBlock
    pub fn into_unique_blocks(self) -> impl Iterator<Item = UniqueAtgBlock> {
        let language = self.language;
        let atg_dialect = self.atg_dialect;
        let texts: Vec<UniqueText> = self.text.into();
        texts
            .into_iter()
            .map(move |t| UniqueAtgBlock::new(t, language.clone(), atg_dialect.clone()))
    }
}
