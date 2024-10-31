use std::path::Path;

use transcribe::Witness;

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
    dbg!(&folios);
    // TODO: write a function to concat multiple folios into a vec of text + language
    // TODO: then write a function to take one text + language into the lex format output
    // TOOD: then write a function to take a lex file and read it into the internal format
}
