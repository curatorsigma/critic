use std::path::Path;

use io::file::output_lex_file;
use transcribe::Witness;

mod dialect;
mod language;

pub mod io;

mod lex;
mod normalise;
mod transcribe;

fn main() {
    let wit = Witness::from_path(Path::new(".data/witness.toml")).unwrap();
    let mut folios = wit
        .get_folios(Path::new(".data/ExampleWitness/"))
        .collect::<Vec<_>>();
    let (_, result) = folios.remove(0);
    let folio = result.unwrap();
    let mut versions = folio.normalise();
    let normalised = versions.remove(0);
    let write_to_file = output_lex_file(Path::new(".data/Lex/example.toml"), normalised);
    dbg!(&write_to_file);

    // TODO:
    // - merge critic_core into critic

    // TODO:
    // - there could be words split over two folios
    // - if this happens: mark the part of the word on the first folio with the lex data for the
    //   entire word and mark the part of the word on the second folio with
    //   "second_half_of_cross_folio_break = true"
    //      - words with this marker that appear as the first word of a folio will be ignored
    //        completely when reading in lex
    //      - words with this marker anywhere else will raise an error
    // - rationale: We want to lex based on folio, not on witness, to keep file sizes small. Since
    //   we have to faithfully represent each folio that means we will have partial words on some
    //   occasions. We can ignore them while reading the filled lex file back


    // TODO:
    // - Add expandability to have the lex output make proposals for lex and morph data
    // - automatically suggest the lex and morph for punctuation

    // TODO: reading lex files from disk into a LexedFolioTranscript

    // TODO: write a function to concat multiple folios into a vec of text + language
    // TODO: then write a function to take one text + language into the lex format output
    // TOOD: then write a function to take a lex file and read it into the internal format
}
