use std::str::FromStr;

use critic::{
    anchor::{AnchorDialect, SuperAnchorDialect},
    atg::{AtgDialect, ControlPointDefinition, Text},
};

const EXAMPLE_CONTROL_POINTS: ControlPointDefinition = ControlPointDefinition {
    escape: '\\',
    start_param: '(',
    stop_param: ')',
    illegible: '^',
    lacuna: '~',
    anchor: '§',
    format_break: '/',
    correction: '&',
    non_semantic: "\t\n",
    comment: '#',
};

struct ExampleAtgDialect {}
impl AtgDialect for ExampleAtgDialect {
    const NATIVE_POINTS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.'";
    const PUNCTUATION: &'static str = ",.";
    const ATG_CONTROL_POINTS: ControlPointDefinition = EXAMPLE_CONTROL_POINTS;
    const WORD_DIVISOR: char = ' ';
}

#[derive(Debug, PartialEq)]
enum ParseStanzaError {
    EmptyString,
    TooManyChars,
    NotInRange,
    NotANumber,
}
impl core::fmt::Display for ParseStanzaError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EmptyString => write!(f, "Empty String is no valid stanza number."),
            Self::TooManyChars => write!(f, "More then one character is no valid stanza number."),
            Self::NotANumber => write!(f, "The Argument is not a number."),
            Self::NotInRange => write!(f, "The Argument is not either 1 or 2."),
        }
    }
}
impl std::error::Error for ParseStanzaError {}

#[derive(Debug, PartialEq)]
enum Stanza {
    One,
    Two,
}
impl core::fmt::Display for Stanza {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::One => {
                write!(f, "1")
            }
            Self::Two => {
                write!(f, "2")
            }
        }
    }
}
impl FromStr for Stanza {
    type Err = ParseStanzaError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(ParseStanzaError::EmptyString)
        } else if s.len() >= 2 {
            Err(ParseStanzaError::TooManyChars)
        } else {
            let nr = s.parse::<u8>().map_err(|_| ParseStanzaError::NotANumber)?;
            match nr {
                1 => Ok(Stanza::One),
                2 => Ok(Stanza::Two),
                _ => Err(ParseStanzaError::NotInRange),
            }
        }
    }
}
impl SuperAnchorDialect for Stanza {
    type ParseError = ParseStanzaError;
}

fn main() {
    let res = "§(1)\
    In twilight's glow, the sha^(2)(do)ws dance,/(line)\
    Whispers of dre^(1)(a)ms in a fleeting trance,/(line)\
    Stars awaken^(1)(,) the night unfurls,/(line)\
    A canvas painted with secret pearls./(paragraph)\
    §(2)\
    The moonlight &(illuminates)(bathes) the quiet stream,/(line)\
    Where echoes linger of a distant dream,/(line)\
    Soft breezes carry the ta~(8)ld,/(line)\
    Of hearts entwined and lo~(8)d./(folio)\
    ";
    let parsed = Text::parse::<ExampleAtgDialect>(res, AnchorDialect::Example, 2).unwrap();
    assert_eq!(parsed.render::<ExampleAtgDialect>(), res);
}
