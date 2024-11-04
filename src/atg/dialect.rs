//! Different ATG dialects in critic and ways to switch between them at runtime

use serde::Deserialize;

mod example;
pub use example::ExampleAtgDialect;

use crate::anchor::AnchorDialect;

use super::{AtgParseError, Text};

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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub enum AtgDialectList {
    #[cfg(feature = "atg_example")]
    Example,
}
impl core::str::FromStr for AtgDialectList {
    type Err = AtgDialectUnknown;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "atg_example")]
            "example" => Ok(Self::Example),
            _ => Err(AtgDialectUnknown::new(s.to_owned())),
        }
    }
}
impl core::fmt::Display for AtgDialectList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            #[cfg(feature = "atg_example")]
            AtgDialectList::Example => {
                write!(f, "example")
            }
            #[allow(unreachable_patterns)]
            _ => unreachable!(),
        }
    }
}

pub fn parse_by_dialect(
    input: &str,
    atg_dialect: &AtgDialectList,
    anchor_dialect: AnchorDialect,
    number_of_corrections: usize,
) -> Result<Text, AtgParseError> {
    match atg_dialect {
        #[cfg(feature = "atg_example")]
        AtgDialectList::Example => {
            Text::parse::<ExampleAtgDialect>(input, anchor_dialect, number_of_corrections)
        }
        // this happens only if Language is empty (no language feature enabled)
        // but in this case, Language is the bottom type anyways
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    }
}
