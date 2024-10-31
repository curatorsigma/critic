//! Everything dealing with the normalisation of the surface form of texts
//!
//! The normalisation part of the pipeline does these things:
//! - split texts into words (based on the word divisor)
//! - supply uncertain passages
//! - do other normalisations like nomina-sacra expansion or unicode-mapping for each language

use critic_core::atg::{AtgDialect, Text};

use crate::language::{Language, WordNormalForm};

pub fn normalise<D>(text: Text, language: Language) -> Vec<Vec<WordNormalForm>>
where
    D: AtgDialect,
{
    text.auto_normalise::<D>()
        .into_iter()
        .map(|w| language.normalise(w))
        .collect::<Vec<_>>()
}
