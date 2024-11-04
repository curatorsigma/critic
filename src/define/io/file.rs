//! File IO for defining Witnesses

use std::{fs::read_to_string, path::Path};

use crate::define::WitnessMetadata;

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

