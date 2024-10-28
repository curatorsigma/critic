//! Everything to do with defining natural languages

/// A natural language which has an associated lexeme- and morphological system.
pub trait Language: core::fmt::Debug {}

#[derive(Debug)]
struct Example {}
impl Language for Example {}
#[derive(Debug)]
struct Example2 {}
impl Language for Example2 {}

pub fn select(s: &str) -> Option<Box<dyn Language>> {
    match s {
        "example" => Some(Box::new(Example {})),
        "example2" => Some(Box::new(Example2 {})),
        _ => None,
    }
}
