//! Things needed for defining Witnesses

use serde::Deserialize;

use crate::transcribe::io::TranscriptIterator;

use self::io::file::{read_witness_metadata, ReadWitnessDefinitionError};

mod io;

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

    pub fn default_atg(&self) -> &Option<String> {
        &self.default_atg
    }

    pub fn default_anchor(&self) -> &Option<String> {
        &self.default_anchor
    }

    pub fn default_language(&self) -> &Option<String> {
        &self.default_language
    }

    pub fn number_of_corrections(&self) -> usize {
        self.corrections.len()
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

