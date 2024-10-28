//! Ways to interact with data in flat files.
//!
//! The main way to enter actual data is via flat files, which makes version tracking via git much
//! simpler then it would be if data were immediately entered as SQL.

use core::str::FromStr;

use critic_core::atg::AtgParseError;
use serde::Deserialize;

use crate::{dialect::parse_by_dialect_name, transcribe::{AtgBlock, FolioTranscript, FolioTranscriptMetadata}};

/// A single block in a transcript file
///
/// This struct is used during parsing only.
#[derive(Deserialize, Debug)]
struct TranscriptBlock {
    /// ATG Dialect used in this block
    ///
    /// See `Cargo.toml` - `features` - <the section about ATG> for available dialects.
    atg: String,
    /// Anchor Dialect used in this block
    ///
    /// See `Cargo.toml` - `features` - <the section about anchors> for available dialects.
    anchor: String,
    /// Natural language that is transcribed
    ///
    /// Defaults to the value in [TranscriptBlock::atg].
    language: Option<String>,
    /// The text that is actually transcribed
    transcript: String,
}

#[derive(Debug)]
pub struct FolioTranscriptParseError {
    // The byte offset in the input at which parsing failed
    location: Option<usize>,
    reason: FolioTranscriptParseErrorReason,
}
impl core::fmt::Display for FolioTranscriptParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.location {
            None => {
                write!(
                    f,
                    "Error parsing folio transcript: {}.",
                    self.reason
                )
            }
            Some(x) => {
                write!(
                    f,
                    "Error parsing folio transcript at {}: {}.",
                    x, self.reason
                )
            }
        }
    }
}
impl std::error::Error for FolioTranscriptParseError {}
impl From<toml::de::Error> for FolioTranscriptParseError {
    fn from(value: toml::de::Error) -> Self {
        Self {
            location: value.span().map(|x| x.start),
            reason: FolioTranscriptParseErrorReason::Toml(value),
        }
    }
}

/// The reasons for which Folio parsing can fail.
#[derive(Debug)]
enum FolioTranscriptParseErrorReason {
    /// File is not valid toml
    Toml(toml::de::Error),
    /// Metadata block was missing
    NoMetadata,
    /// A block was encountered, that is neither called metadata, not a decimal digit
    BlockNameNotDecimal(String),
    /// A block with a decibal name was encountered, but it was not given in ascending order
    BlockNameNotInAscendingOrder(u8),
    /// The given Language is not known
    LanguageUnknown(String),
    /// Anchor Dialect is not known
    AnchorDialectUnknown(String),
    /// The Transcript data itself is not parsable
    ///
    /// Values:
    /// - Name of the block where parsing failed
    /// - The error that occured
    TranscriptUnparsable(String, AtgParseError),
    /// The given Dialect did not exist
    AtgDialectUnknown(String),
}
impl core::fmt::Display for FolioTranscriptParseErrorReason {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Toml(inner) => {
                write!(f, "Error parsing file as toml: {inner}.")
            }
            Self::NoMetadata => {
                write!(f, "The file did not contain a \"metadata\" block.")
            }
            Self::BlockNameNotDecimal(name) => {
                write!(f, "The blockname {name} must be a decimal.")
            }
            Self::BlockNameNotInAscendingOrder(block_number) => {
                write!(f, "The blockname {block_number} needs to be exactly one higher then the last block name.")
            }
            Self::AnchorDialectUnknown(x) => {
                write!(f, "The anchor dialect \"{x}\" is not known. Is critic compiled with the correct features?")
            }
            Self::TranscriptUnparsable(block, e) => {
                write!(f, "While parsing transcript data for block {block}, the following error occured: {e}.")
            }
            Self::AtgDialectUnknown(x) => {
                write!(f, "The ATG Dialect \"{x}\" does not exist. Is critic compiled with the correct features?")
            }
            Self::LanguageUnknown(x) => {
                write!(f, "The language \"{x}\" is not known. Is critic compiled with the correct features?")
            }
        }
    }
}

impl FromStr for FolioTranscript
{
    type Err = FolioTranscriptParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // interpret s as toml object
        let as_toml: toml::Table = toml::from_str(s)?;
        // parse table entry by table entry
        let mut metadata = None;
        let mut dialect_blocks = Vec::<AtgBlock>::new();
        // each other block must have as a name decimals in ascending order and be AtgBlock format
        for (key, value) in as_toml {
            if key == "metadata" {
                metadata = value.try_into()?;
            } else {
                // check that key is a digit
                let num = key.parse::<u8>().map_err(|_| FolioTranscriptParseError {
                    location: None,
                    reason: FolioTranscriptParseErrorReason::BlockNameNotDecimal(key.clone()),
                })?;
                // The blocks are sorted in lexical order (by [toml]).
                // We need to make sure the names were actually given in ascending order.
                if num as usize != dialect_blocks.len() + 1 {
                    return Err(FolioTranscriptParseError {
                        location: None,
                        reason: FolioTranscriptParseErrorReason::BlockNameNotInAscendingOrder(num),
                    });
                };
                let trans_block: TranscriptBlock = value.try_into()?;
                dbg!(&trans_block);
                let language_name = &trans_block.language.unwrap_or(trans_block.atg.clone());
                let language = crate::language::select(language_name)
                    .ok_or(FolioTranscriptParseError { location: None, reason: FolioTranscriptParseErrorReason::LanguageUnknown(language_name.to_owned())})?;
                let anchor_dialect = critic_core::anchor::AnchorDialect::get_by_name(&trans_block.anchor).ok_or(
                    FolioTranscriptParseError {
                        location: None,
                        reason: FolioTranscriptParseErrorReason::AnchorDialectUnknown(trans_block.anchor),
                    })?;
                let text = match parse_by_dialect_name(&trans_block.transcript, &trans_block.atg, anchor_dialect) {
                    None => {
                        return Err(FolioTranscriptParseError {
                            location: None,
                            reason: FolioTranscriptParseErrorReason::AtgDialectUnknown(trans_block.atg),
                        });
                    }
                    Some(Err(parse_error)) => {
                        return Err(FolioTranscriptParseError {
                            location: None,
                            reason: FolioTranscriptParseErrorReason::TranscriptUnparsable(key, parse_error),
                        });
                    }
                    Some(Ok(x)) => { x }
                };
                dialect_blocks.push(AtgBlock::new(text, language));
            };
        };
        Ok(FolioTranscript::new(
            metadata.ok_or(FolioTranscriptParseError {
                location: None,
                reason: FolioTranscriptParseErrorReason::NoMetadata,
            })?,
            dialect_blocks,
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::transcribe::{FolioTranscript, FolioTranscriptMetadata};

    #[test]
    fn folio_parse() {
        let input = r#"
[metadata]
transcriber = "John Doe"
editors = ["Alice", "Bob"]

[1]
atg = "example"
anchor = "example"
language = "example"
transcript = '''
this is §(1) my transcript'''

[2]
atg = "example"
anchor = "example"
transcript = '''
some other t^(2)(ra)nscript
'''
"#;
        let res = input.parse::<FolioTranscript>().unwrap();
        // TODO: check that the data is correct - problem: AtgBlock can never be PartialEq, so this
        // is very very annoying
    }
}
