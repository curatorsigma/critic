use std::path::Path;

use critic_core::atg::Word;
use transcribe::Witness;

use crate::lex::LexWordData;

mod dialect;
mod language;

pub mod io;

mod lex;
mod normalise;
mod transcribe;

fn main() {
    let wit = Witness::from_path(Path::new(".data/witness.toml")).unwrap();
    let folios = wit
        .get_folios(Path::new(".data/ExampleWitness/"))
        .collect::<Vec<_>>();
    // dbg!(&folios);

    // let input = r#""#;
    let word: Word = toml::de::from_str("[[parts]]\nNative = \"some\"\n").unwrap();
    dbg!(&word);
    let lexworddata = LexWordData::new(
        word,
        "some".to_owned(),
        None,
        "1".to_owned(),
        "N".to_owned(),
    );
    dbg!(&lexworddata);
    let straight = toml::to_string(&lexworddata);
    dbg!(&straight);
    let manual = lexworddata.to_toml_str();
    dbg!(&manual);

    // TODO: add Version/Correction metadata to witness definition
    // use this to create a new type of error in parsing when the number of corrections is not
    // correct

    // FolioTranscript -> Vec<NormalisedFolioTranscript>
    // NormalisedFolioTranscript:: (metadata, Vec<NormalisedAtgBlock>)
    // serialization for NormalisedAtgBlock into lex files

    // LexBlockData + dialects -> LexBlock
    // TODO: LexBlockData - 1:1 das Format, das in Lex-Dateien steht (pro AtgBlock)
    // TODO: LexFileData - 1:1 das Format, das in Lex-Dateien steht (gesamt)
    //
    // TODO: write a function to concat multiple folios into a vec of text + language
    // TODO: then write a function to take one text + language into the lex format output
    // TOOD: then write a function to take a lex file and read it into the internal format
}
