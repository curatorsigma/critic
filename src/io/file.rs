//! Ways to interact with data in flat files.
//!
//! The main way to enter actual data is via flat files, which makes version tracking via git much
//! simpler then it would be if data were immediately entered as SQL.

use std::{fs::read_to_string, path::Path};

use crate::transcribe::{FolioTranscript, FolioTranscriptParseError, WitnessMetadata};

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

/// An Error that can occur while reading a witness definition from disk
#[derive(Debug)]
pub enum ReadWitnessDefinitionError {
    /// Something went wrong while reading the file itself
    Io(std::io::Error, String),
    /// The file was read successfully, but something went wrong interpreting its content as a
    /// Witness Definition
    Toml(toml::de::Error),
}
impl From<toml::de::Error> for ReadWitnessDefinitionError {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value)
    }
}
impl core::fmt::Display for ReadWitnessDefinitionError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Io(x, path) => {
                write!(f, "Error reading from file \"{path}\": {x}")
            }
            Self::Toml(x) => {
                write!(
                    f,
                    "Error parsing file as toml defining WitnessMetadata: {x}"
                )
            }
        }
    }
}
impl std::error::Error for ReadWitnessDefinitionError {}

pub fn read_witness_metadata(path: &Path) -> Result<WitnessMetadata, ReadWitnessDefinitionError> {
    let content = read_to_string(path)
        .map_err(|x| ReadWitnessDefinitionError::Io(x, path.to_string_lossy().to_string()))?;
    Ok(toml::from_str(&content)?)
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use critic_core::atg::Text;

    use crate::{
        io::file::read_folio_transcript,
        transcribe::{AtgBlock, FolioTranscript, FolioTranscriptMetadata, WitnessMetadata},
    };

    use super::read_witness_metadata;

    #[test]
    #[cfg(all(
        feature = "language_example",
        feature = "anchor_example",
        feature = "atg_example"
    ))]
    fn folio_parse() {
        use crate::dialect::atg::ExampleAtgDialect;
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
        let witness_metadata_content = r#"
name = "example witness"
folios = ["name1"]
"#;
        let witness_metadata = toml::from_str(witness_metadata_content).unwrap();
        let res = FolioTranscript::from_folio_file_content(input, &witness_metadata).unwrap();
        let metadata = FolioTranscriptMetadata::new(
            "John Doe".to_owned(),
            vec!["Alice".to_owned(), "Bob".to_owned()],
        );
        let dialect_blocks = vec![
            AtgBlock::new(
                Text::parse::<ExampleAtgDialect>(
                    "this is §(1) my transcript",
                    critic_core::anchor::AnchorDialect::Example,
                )
                .unwrap(),
                crate::language::Language::Example,
                crate::dialect::AtgDialectList::Example,
            ),
            AtgBlock::new(
                Text::parse::<ExampleAtgDialect>(
                    "some other t^(2)(ra)nscript",
                    critic_core::anchor::AnchorDialect::Example,
                )
                .unwrap(),
                crate::language::Language::Example,
                crate::dialect::AtgDialectList::Example,
            ),
        ];
        assert_eq!(res, FolioTranscript::new(metadata, dialect_blocks));
    }

    #[test]
    fn read_witness_definition_file_nonex() {
        let path = Path::new("does/not/exist.toml");
        let error = read_witness_metadata(path).unwrap_err();
        let filename = match error {
            super::ReadWitnessDefinitionError::Io(_, x) => x,
            _ => panic!(),
        };
        assert_eq!(filename, "does/not/exist.toml".to_owned());
    }

    #[test]
    fn read_folio_transcript_file_nonex() {
        let path = Path::new("does/not/exist.toml");
        let witness_metadata_content = r#"
name = "example witness"
folios = ["name1"]
"#;
        let witness_metadata = toml::from_str(witness_metadata_content).unwrap();
        let error = read_folio_transcript(path, &witness_metadata).unwrap_err();
        let filename = match error {
            super::ReadFolioTranscriptError::Io(_, x) => x,
            _ => panic!(),
        };
        assert_eq!(filename, "does/not/exist.toml".to_owned());
    }
}
