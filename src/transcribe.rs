//! Everything needed in the transcribe phase

use serde::{Deserialize, Serialize};

use crate::{anchor::AnchorDialect, atg::{dialect::{parse_by_dialect, AtgDialectList, AtgDialectUnknown}, normalize::NormalisedAtgBlock, AtgBlock}, define::WitnessMetadata, language::Language};

use self::io::{FolioTranscriptParseError, FolioTranscriptParseErrorReason};

pub mod io;

/// Metadata associated to a single folio.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FolioTranscriptMetadata {
    /// Name of the principal transcriber of this folio
    transcriber: String,
    /// List of editors / correctors / secondary transcribers
    editors: Vec<String>,
}
impl FolioTranscriptMetadata {
    pub fn new(transcriber: String, editors: Vec<String>) -> Self {
        Self {
            transcriber,
            editors,
        }
    }
}

/// A single block in a transcript file
///
/// This struct is used during parsing only.
#[derive(Deserialize, Debug)]
struct TranscriptBlock {
    /// ATG Dialect used in this block
    ///
    /// If not supplied, use the default set in the the witness metadata.
    ///
    /// See `Cargo.toml` - `features` - <the section about ATG> for available dialects.
    atg: Option<String>,
    /// Anchor Dialect used in this block
    ///
    /// If not supplied, use the default set in the the witness metadata.
    ///
    /// See `Cargo.toml` - `features` - <the section about anchors> for available dialects.
    anchor: Option<String>,
    /// Natural language that is transcribed
    ///
    /// Defaults to the value in [TranscriptBlock::atg].
    language: Option<String>,
    /// The text that is actually transcribed
    transcript: String,
}
impl TranscriptBlock {
    fn select_dialects(
        &self,
        meta: &WitnessMetadata,
    ) -> Result<(String, Language, AnchorDialect), FolioTranscriptParseError> {
        let atg = match &self.atg {
            None => match &meta.default_atg() {
                Some(x) => x,
                None => {
                    return Err(FolioTranscriptParseError::new(
                        FolioTranscriptParseErrorReason::NoAtg,
                        None,
                    ));
                }
            },
            Some(x) => x,
        };
        let language = match &self.language {
            None => match &meta.default_language() {
                Some(x) => x,
                None => atg,
            },
            Some(x) => x,
        };
        let language =
            crate::language::Language::from_name(language).ok_or(FolioTranscriptParseError::new(
                FolioTranscriptParseErrorReason::LanguageUnknown(language.to_owned()),
                None,
            ))?;

        let anchor = match &self.anchor {
            None => match &meta.default_anchor() {
                Some(x) => x,
                None => {
                    return Err(FolioTranscriptParseError::new(
                        FolioTranscriptParseErrorReason::NoAnchor,
                        None,
                    ));
                }
            },
            Some(x) => x,
        };
        let anchor_dialect = AnchorDialect::get_by_name(anchor).ok_or(
            FolioTranscriptParseError::new(
                FolioTranscriptParseErrorReason::AnchorDialectUnknown(anchor.to_owned()),
                None,
            ),
        )?;
        Ok((atg.to_owned(), language, anchor_dialect))
    }
}


/// A transcript of a single folio.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct FolioTranscript {
    /// The metadata associated specifically with this folio
    metadata: FolioTranscriptMetadata,
    /// The individual blocks of ATG
    ///
    /// Usually, each folio will have only one [AtgBlock] associated. However, when the language or
    /// script (or anchor style) changes in the middle of a Folio, we need to tag each individual
    /// section with the correct dialects, which is why this Vec exists.
    blocks: Vec<AtgBlock>,
}
impl FolioTranscript {
    pub fn new(metadata: FolioTranscriptMetadata, blocks: Vec<AtgBlock>) -> Self {
        Self { metadata, blocks }
    }

    pub fn from_folio_file_content(
        s: &str,
        witness_metadata: &WitnessMetadata,
    ) -> Result<Self, FolioTranscriptParseError> {
        // interpret s as toml object
        let as_toml: toml::Table = toml::from_str(s)?;
        // parse table entry by table entry
        let mut metadata = None;
        let mut blocks = Vec::<AtgBlock>::new();
        // each other block must have as a name decimals in ascending order and be AtgBlock format
        for (key, value) in as_toml {
            if key == "metadata" {
                metadata = value.try_into()?;
            } else {
                // check that key is a digit
                let num = key.parse::<u8>().map_err(|_| FolioTranscriptParseError::new(
                    FolioTranscriptParseErrorReason::BlockNameNotDecimal(key.clone()),
                    None,
                ))?;
                // The blocks are sorted in lexical order (by [toml]).
                // We need to make sure the names were actually given in ascending order.
                if num as usize != blocks.len() + 1 {
                    return Err(FolioTranscriptParseError::new(
                        FolioTranscriptParseErrorReason::BlockNameNotInAscendingOrder(num),
                        None,
                    ));
                };
                let trans_block: TranscriptBlock = value.try_into()?;
                let (atg, language, anchor_dialect) =
                    trans_block.select_dialects(&witness_metadata)?;
                let atg_dialect =
                    atg.parse::<AtgDialectList>()
                        .map_err(|AtgDialectUnknown { name: x }| FolioTranscriptParseError::new(
                            FolioTranscriptParseErrorReason::AtgDialectUnknown(x),
                            None,
                        ))?;

                let number_of_corrections = witness_metadata.number_of_corrections();
                let text =
                    match parse_by_dialect(&trans_block.transcript, &atg_dialect, anchor_dialect, number_of_corrections) {
                        Err(parse_error) => {
                            return Err(FolioTranscriptParseError::new(
                                FolioTranscriptParseErrorReason::TranscriptUnparsable(
                                    key,
                                    parse_error,
                                ),
                                None,
                            ));
                        }
                        Ok(x) => x,
                    };
                blocks.push(AtgBlock::new(text, language, atg_dialect));
            };
        }
        Ok(FolioTranscript::new(
            metadata.ok_or(FolioTranscriptParseError::new(
                FolioTranscriptParseErrorReason::NoMetadata,
                None,
            ))?,
            blocks,
        ))
    }

    /// Normalise all AtgBlocks in this Folio, creating a Vector over the different
    /// Corrections contained within.
    pub fn normalise(self) -> Vec<NormalisedFolioTranscript>
    {
        let metadata = self.metadata;
        // this is
        // - a vec over blocks
        //   - a vec over versions in that block
        let blocks = self
            .blocks
            .into_iter()
            .map(|b| b.into_normalised_blocks().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if blocks.is_empty() {
            return vec![NormalisedFolioTranscript::new(metadata, vec![])];
        };
        // transpose these blocks to
        // - a vec over versions
        //   - a vec over blocks in this version
        let correction_number = blocks[0].len();
        let mut block_iter: Vec<_> = blocks.into_iter().map(|n| n.into_iter()).collect();
        (0..correction_number)
            .map(|_| {
                block_iter
                    .iter_mut()
                    .map(|n| {
                        n.next()
                         .expect("All Blocks should have equal number of corrections because the parsing machinery asserts that.")
                    })
                    .collect::<Vec<_>>()
            })
            .map(|blocks_of_correction| {
                NormalisedFolioTranscript::new(metadata.clone(), blocks_of_correction)
            })
            .collect()
    }
}


/// A transcribed Folio, with the text completely normalized
#[derive(Debug)]
pub struct NormalisedFolioTranscript {
    metadata: FolioTranscriptMetadata,
    blocks: Vec<NormalisedAtgBlock>,
}
impl NormalisedFolioTranscript {
    pub fn new(metadata: FolioTranscriptMetadata, blocks: Vec<NormalisedAtgBlock>) -> Self {
        Self { metadata, blocks }
    }

    /// Render the lex file shown to a human to add lex and morph information
    pub fn render_lex_file(&self) -> String {
        // render the metadata block
        let mut res = toml::to_string(&self.metadata).expect("Statically infallible Serialization");
        res.push('\n');
        // render the other blocks
        for (idx, block) in self.blocks.iter().enumerate() {
            res.push_str(&block.render_for_lex_file(idx + 1));
        };
        res
    }
}

