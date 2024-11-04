//! IO Operations for the transcription phase

use std::{fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::{atg::{dialect::{AtgDialectList, AtgDialectUnknown}, normalize::UniqueText, AtgParseError, Text}, define::WitnessMetadata, language::Language};

use super::FolioTranscript;

pub struct TranscriptIterator<'a, 'b> {
    metadata: &'a WitnessMetadata,
    base_dir: &'b std::path::Path,
    current: usize,
}
impl<'a, 'b> TranscriptIterator<'a, 'b> {
    pub fn new(metadata: &'a WitnessMetadata, base_dir: &'b std::path::Path) -> Self {
        Self {
            metadata,
            base_dir,
            current: 0,
        }
    }
}
impl<'a, 'b> Iterator for TranscriptIterator<'a, 'b> {
    type Item = (String, Result<FolioTranscript, ReadFolioTranscriptError>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(folio_name) = self.metadata.folios().get(self.current) {
            let full_path = self.base_dir.join(folio_name).with_extension("toml");
            let folio_data = read_folio_transcript(&full_path, &self.metadata);
            self.current += 1;
            return Some((folio_name.to_owned(), folio_data));
        } else {
            return None;
        };
    }
}

#[derive(Debug)]
pub struct FolioTranscriptParseError {
    // The byte offset in the input at which parsing failed
    location: Option<usize>,
    reason: FolioTranscriptParseErrorReason,
}
impl FolioTranscriptParseError {
    pub fn new(reason: FolioTranscriptParseErrorReason, location: Option<usize>) -> Self {
        Self { reason, location, }
    }
}
impl core::fmt::Display for FolioTranscriptParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.location {
            None => {
                write!(f, "Error parsing folio transcript: {}.", self.reason)
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
impl From<AtgDialectUnknown> for FolioTranscriptParseError {
    fn from(value: AtgDialectUnknown) -> Self {
        FolioTranscriptParseError {
            location: None,
            reason: FolioTranscriptParseErrorReason::AtgDialectUnknown(value.name),
        }
    }
}

/// The reasons for which Folio parsing can fail.
#[derive(Debug)]
pub enum FolioTranscriptParseErrorReason {
    /// File is not valid toml
    Toml(toml::de::Error),
    /// Metadata block was missing
    NoMetadata,
    /// No ATG dialect was defined on either
    /// - the witness definition metadata block
    /// - the folio metadata block
    NoAtg,
    /// No anchor style was defined on either
    /// - the witness definition metadata block
    /// - the folio metadata block
    NoAnchor,
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
            Self::NoAtg => write!(
                f,
                "No ATG dialect was set either on the witness or on the folio."
            ),
            Self::NoAnchor => write!(
                f,
                "No anchor style was set either on the witness or on the folio."
            ),
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
/// Error that can occur while reading a single folio file from disk
#[derive(Debug)]
pub enum ReadFolioTranscriptError {
    /// Something went wrong while reading the file itself
    Io(std::io::Error, String),
    /// The file was read successfully, but something went wrong interpreting its content as a
    /// Folio Transcript
    Content(FolioTranscriptParseError),
}
impl core::fmt::Display for ReadFolioTranscriptError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Io(x, path) => write!(f, "Error reading from file \"{path}\": {x}."),
            Self::Content(x) => write!(f, "Error parsing the data in the file: {x}."),
        }
    }
}
impl From<FolioTranscriptParseError> for ReadFolioTranscriptError {
    fn from(value: FolioTranscriptParseError) -> Self {
        Self::Content(value)
    }
}
impl std::error::Error for ReadFolioTranscriptError {}

pub fn read_folio_transcript(
    path: &Path,
    meta: &WitnessMetadata,
) -> Result<FolioTranscript, ReadFolioTranscriptError> {
    let content = read_to_string(path)
        .map_err(|x| ReadFolioTranscriptError::Io(x, path.to_string_lossy().to_string()))?;
    Ok(FolioTranscript::from_folio_file_content(&content, meta)?)
}


