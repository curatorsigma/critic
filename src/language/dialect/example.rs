//! The language used in examples.

use std::str::FromStr;

use crate::{
    atg::normalize::{AnchoredNormalisedText, NonAgnosticAnchoredText, WordNormalForm},
    language::{
        lex::{LexParseError, LexSchema},
        morph::{MorphPointParseError, MorphRangeParseError},
        MorphPointSchema, MorphRangeSchema, SuperLanguage,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct ExampleLex {
    id: u16,
}
impl core::fmt::Display for ExampleLex {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.id)
    }
}
impl FromStr for ExampleLex {
    type Err = LexParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .parse::<u16>()
            .map_err(|_| LexParseError::new(0, "Not a Number.".to_owned()))?;
        Ok(Self { id })
    }
}
impl LexSchema for ExampleLex {
    const NAME: &'static str = "lex_example";
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExampleMorph {
    Verb,
    Noun,
}
impl core::fmt::Display for ExampleMorph {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Verb => write!(f, "V"),
            Self::Noun => write!(f, "N"),
        }
    }
}
impl FromStr for ExampleMorph {
    type Err = MorphPointParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "V" => Ok(Self::Verb),
            "N" => Ok(Self::Noun),
            _ => Err(MorphPointParseError::new(0, "Not either V or N".to_owned())),
        }
    }
}
impl MorphPointSchema for ExampleMorph {
    type Range = ExampleMorphRange;
    const NAME: &'static str = "morph_example";
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExampleMorphRange {
    None,
    Verb,
    Noun,
    Both,
}
impl core::fmt::Display for ExampleMorphRange {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::None => write!(f, ""),
            Self::Verb => write!(f, "V"),
            Self::Noun => write!(f, "N"),
            Self::Both => write!(f, "B"),
        }
    }
}
impl FromStr for ExampleMorphRange {
    type Err = MorphRangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Ok(Self::None),
            "V" => Ok(Self::Verb),
            "N" => Ok(Self::Noun),
            "B" => Ok(Self::Both),
            _ => Err(MorphRangeParseError::new(0, "Not either V or N".to_owned())),
        }
    }
}
impl MorphRangeSchema for ExampleMorphRange {
    type Point = ExampleMorph;

    fn contains(&self, p: &Self::Point) -> bool {
        match self {
            Self::None => false,
            Self::Both => true,
            Self::Verb => *p == Self::Point::Verb,
            Self::Noun => *p == Self::Point::Noun,
        }
    }
}

pub struct Example {}
impl SuperLanguage for Example {
    type Lex = ExampleLex;
    type Morph = ExampleMorph;

    fn normalise(input: AnchoredNormalisedText) -> NonAgnosticAnchoredText {
        NonAgnosticAnchoredText::new(
            input
                .text
                .into_iter()
                .map(|(w, s)| WordNormalForm::new(w, s, None))
                .collect::<Vec<_>>(),
            input.anchor_positions,
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    #[cfg(all(feature = "language_example", feature = "atg_example"))]
    fn normalise_example() {
        use crate::atg::{dialect::ExampleAtgDialect, Text};

        let input = "This &(word)(sword) ~(3)^(2)(st)rong.";
        let parsed =
            Text::parse::<ExampleAtgDialect>(input, crate::anchor::AnchorDialect::Example, 2)
                .unwrap();
        let normalised = parsed
            .auto_normalise::<ExampleAtgDialect>()
            .collect::<Vec<_>>();
        assert_eq!(normalised.len(), 2);
    }
}
