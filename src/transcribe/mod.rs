//! Everything directly concerning transcription of Manuscripts.

use critic_core::{
    anchor::AnchorDialect,
    atg::{AtgDialect, AtgParseError, Text},
};
use serde::{Deserialize, Serialize};

use crate::{
    dialect::{parse_by_dialect, AtgDialectList, AtgDialectUnknown},
    io::file::{read_witness_metadata, ReadWitnessDefinitionError, TranscriptIterator},
    language::Language,
    normalise::{NormalisedAtgBlock, NormalisedFolioTranscript, UniqueAtgBlock},
};

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
enum FolioTranscriptParseErrorReason {
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
            None => match &meta.default_atg {
                Some(x) => x,
                None => {
                    return Err(FolioTranscriptParseError {
                        location: None,
                        reason: FolioTranscriptParseErrorReason::NoAtg,
                    });
                }
            },
            Some(x) => x,
        };
        let language = match &self.language {
            None => match &meta.default_language {
                Some(x) => x,
                None => atg,
            },
            Some(x) => x,
        };
        let language =
            crate::language::Language::from_name(language).ok_or(FolioTranscriptParseError {
                location: None,
                reason: FolioTranscriptParseErrorReason::LanguageUnknown(language.to_owned()),
            })?;

        let anchor = match &self.anchor {
            None => match &meta.default_anchor {
                Some(x) => x,
                None => {
                    return Err(FolioTranscriptParseError {
                        location: None,
                        reason: FolioTranscriptParseErrorReason::NoAnchor,
                    });
                }
            },
            Some(x) => x,
        };
        let anchor_dialect = critic_core::anchor::AnchorDialect::get_by_name(anchor).ok_or(
            FolioTranscriptParseError {
                location: None,
                reason: FolioTranscriptParseErrorReason::AnchorDialectUnknown(anchor.to_owned()),
            },
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
                let num = key.parse::<u8>().map_err(|_| FolioTranscriptParseError {
                    location: None,
                    reason: FolioTranscriptParseErrorReason::BlockNameNotDecimal(key.clone()),
                })?;
                // The blocks are sorted in lexical order (by [toml]).
                // We need to make sure the names were actually given in ascending order.
                if num as usize != blocks.len() + 1 {
                    return Err(FolioTranscriptParseError {
                        location: None,
                        reason: FolioTranscriptParseErrorReason::BlockNameNotInAscendingOrder(num),
                    });
                };
                let trans_block: TranscriptBlock = value.try_into()?;
                let (atg, language, anchor_dialect) =
                    trans_block.select_dialects(&witness_metadata)?;
                let atg_dialect =
                    atg.parse::<AtgDialectList>()
                        .map_err(|AtgDialectUnknown { name: x }| FolioTranscriptParseError {
                            location: None,
                            reason: FolioTranscriptParseErrorReason::AtgDialectUnknown(x),
                        })?;

                let number_of_corrections = witness_metadata.corrections.len();
                let text =
                    match parse_by_dialect(&trans_block.transcript, &atg_dialect, anchor_dialect, number_of_corrections) {
                        Err(parse_error) => {
                            return Err(FolioTranscriptParseError {
                                location: None,
                                reason: FolioTranscriptParseErrorReason::TranscriptUnparsable(
                                    key,
                                    parse_error,
                                ),
                            });
                        }
                        Ok(x) => x,
                    };
                blocks.push(AtgBlock::new(text, language, atg_dialect));
            };
        }
        Ok(FolioTranscript::new(
            metadata.ok_or(FolioTranscriptParseError {
                location: None,
                reason: FolioTranscriptParseErrorReason::NoMetadata,
            })?,
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

/// A single block of ATG, together with the language and ATG dialect
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct AtgBlock {
    /// the actual text in this block
    text: Text,
    /// the language used in this block
    language: Language,
    /// the atg dialect used in this block
    atg_dialect: AtgDialectList,
}
impl AtgBlock {
    pub fn new(text: Text, language: Language, atg_dialect: AtgDialectList) -> Self {
        // make sure the atg dialect exists
        Self {
            text,
            language,
            atg_dialect,
        }
    }

    pub fn from_dialect_name(
        text: Text,
        language: Language,
        atg_dialect: String,
    ) -> Result<Self, AtgDialectUnknown> {
        // make sure the atg dialect exists
        let dialect = atg_dialect.parse()?;
        Ok(Self {
            text,
            language,
            atg_dialect: dialect,
        })
    }

    /// flatten out different corrections in one AtgBlock
    pub fn into_unique_blocks(self) -> impl Iterator<Item = UniqueAtgBlock> {
        let language = self.language;
        let atg_dialect = self.atg_dialect;
        let texts = self.text.into_unique_texts();
        texts
            .into_iter()
            .map(move |t| UniqueAtgBlock::new(t, language.clone(), atg_dialect.clone()))
    }

    pub fn into_normalised_blocks(self) -> impl Iterator<Item = NormalisedAtgBlock>
    {
        let lang = self.language;
        self.into_unique_blocks()
            .map(move |b| b.normalise(lang))
    }
}

#[derive(Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct WitnessMetadata {
    /// A name for this Witness
    ///
    /// This should be unique amongst all witnesses
    name: String,
    /// The Folios making up this witness, by name
    ///
    /// Their names must match the names of the folio files on disk
    folios: Vec<String>,
    /// The human readable Names for each correctors hand that was active in this
    /// Witness. The length of all correction sequences must be the same as the length of this
    /// Vector.
    ///
    /// When a specific correction needs to be refered to, this name will be used instead of the
    /// name of the entire Witness
    corrections: Vec<String>,
    /// For blocks which have no ATG dialect specified, use this default instead
    ///
    /// Since the ATG dialect, Anchor style and Language usually does not change in the middle of a
    /// Witness, you should always aim to supply these default options.
    default_atg: Option<String>,
    /// For blocks which have no anchor style specified, use this default instead
    default_anchor: Option<String>,
    /// For blocks which have no natural language specified, use this default instead
    default_language: Option<String>,
}
impl WitnessMetadata {
    pub fn folios(&self) -> &Vec<String> {
        &self.folios
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Witness {
    metadata: WitnessMetadata,
}
impl Witness {
    pub fn from_path(path: &std::path::Path) -> Result<Self, ReadWitnessDefinitionError> {
        let metadata = read_witness_metadata(path)?;
        Ok(Self { metadata })
    }

    pub fn folio_names(&self) -> core::slice::Iter<String> {
        self.metadata.folios.iter()
    }

    pub fn get_folios<'a, 'b>(
        &'a self,
        base_dir: &'b std::path::Path,
    ) -> TranscriptIterator<'a, 'b> {
        // return the correct iterator here
        TranscriptIterator::new(&self.metadata, base_dir)
    }
}

/// The structure FolioTranscript files have on disk
struct FolioTranscriptData {
    metadata: FolioTranscriptMetadata,
    blocks: Vec<AtgBlock>,
}
