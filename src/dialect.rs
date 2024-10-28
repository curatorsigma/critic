//! Loading all the different defined dialects based on cargo features.

use critic_core::{anchor::AnchorDialect, atg::{AtgParseError, Text}};

mod anchor;
mod atg;

pub fn parse_by_dialect_name(input: &str, name: &str, anchor_dialect: AnchorDialect) -> Option<Result<Text, AtgParseError>> {
    match name {
        #[cfg(feature = "atg_example")]
        "example" => {
            Some(Text::parse::<crate::dialect::atg::ExampleAtgDialect>(input, anchor_dialect))
        }
        _ => { None }
    }
}
