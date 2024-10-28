//! Everything directly concerning transcription of Manuscripts.

use critic_core::{anchor::AnchorDialect, atg::Text};
use serde::Deserialize;

use crate::language::Language;

/// Metadata associated to a single folio.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FolioTranscriptMetadata {
    /// Name of the principal transcriber of this folio
    transcriber: String,
    /// List of editors / correctors / secondary transcribers
    editors: Vec<String>,
}
impl FolioTranscriptMetadata {
    pub fn new(transcriber: String, editors: Vec<String>) -> Self  {
        Self {
            transcriber, editors,
        }
    }
}

/// A transcript of a single folio.
#[derive(Debug)]
pub struct FolioTranscript
{
    metadata: FolioTranscriptMetadata,
    dialect_blocks: Vec<AtgBlock>,
}
impl FolioTranscript
{
    pub fn new(metadata: FolioTranscriptMetadata, dialect_blocks: Vec<AtgBlock>) -> Self {
        Self {
            metadata,
            dialect_blocks,
        }
    }
}

#[derive(Debug)]
pub struct AtgBlock
{
    text: Text,
    language: Box<dyn Language>,
}
impl AtgBlock {
    pub fn new(text: Text, language: Box<dyn Language>) -> Self {
        Self {
            text,
            language,
        }
    }
}
