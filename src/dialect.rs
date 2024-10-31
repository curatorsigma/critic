//! Loading all the different defined dialects based on cargo features.

use std::str::FromStr;

use critic_core::{
    anchor::AnchorDialect,
    atg::{AtgParseError, Text},
};
use serde::Deserialize;

use crate::transcribe::FolioTranscriptParseError;

pub mod anchor;
pub mod atg;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct AtgDialectUnknown {
    pub(crate) name: String,
}
impl AtgDialectUnknown {
    pub fn new(s: String) -> Self {
        Self { name: s }
    }
}
impl core::fmt::Display for AtgDialectUnknown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "The ATG dialect \"{}\" does not exist. Is critic compiled with the correct features?",
            self.name
        )
    }
}
impl std::error::Error for AtgDialectUnknown {}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum AtgDialectList {
    #[cfg(feature = "atg_example")]
    Example,
}
impl FromStr for AtgDialectList {
    type Err = AtgDialectUnknown;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "atg_example")]
            "example" => Ok(Self::Example),
            _ => Err(AtgDialectUnknown::new(s.to_owned())),
        }
    }
}

pub fn parse_by_dialect_name(
    input: &str,
    name: &str,
    anchor_dialect: AnchorDialect,
) -> Option<Result<Text, AtgParseError>> {
    let dialect = name.parse();
    match &dialect {
        Ok(x) => Some(parse_by_dialect(input, x, anchor_dialect)),
        Err(_) => None,
    }
}

pub fn parse_by_dialect(input: &str, atg_dialect: &AtgDialectList, anchor_dialect: AnchorDialect) -> Result<Text, AtgParseError> {
    match atg_dialect {
        #[cfg(feature = "atg_example")]
        AtgDialectList::Example => Text::parse::<crate::dialect::atg::ExampleAtgDialect>(
            input,
            anchor_dialect,
        ),
        // this happens only if Language is empty (no language feature enabled)
        // but in this case, Language is the bottom type anyways
        _ => unreachable!(),
    }
}
