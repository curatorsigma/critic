//! The language used in examples.

use std::str::FromStr;

use critic_core::atg::AnchoredNormalisedText;

use crate::lex::{
    LexParseError, LexSchema, MorphPointParseError, MorphPointSchema, MorphRangeParseError,
    MorphRangeSchema,
};

use super::{SuperLanguage, WordNormalForm};

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

    fn normalise(input: AnchoredNormalisedText) -> Vec<WordNormalForm> {
        input
            .text()
            .into_iter()
            .map(|(w, s)| WordNormalForm::new(w, s, None))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod test {
    use critic_core::atg::Text;

    use crate::normalise::normalise;

    #[test]
    #[cfg(all(feature = "language_example", feature = "atg_example"))]
    fn normalise_example() {
        use crate::dialect::atg::ExampleAtgDialect;

        let input = "This &(word)(sword) ~(3)^(2)(st)rong.";
        let parsed =
            Text::parse::<ExampleAtgDialect>(input, critic_core::anchor::AnchorDialect::Example)
                .unwrap();
        let normalised = normalise::<ExampleAtgDialect>(parsed, crate::language::Language::Example);
        let surface_only = normalised
            .into_iter()
            .map(|text| {
                text.into_iter()
                    .map(|wnf| wnf.display_form())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            surface_only,
            vec![
                vec![
                    "This".to_owned(),
                    "word".to_owned(),
                    "~~~".to_owned(),
                    "strong.".to_owned()
                ],
                vec![
                    "This".to_owned(),
                    "sword".to_owned(),
                    "~~~".to_owned(),
                    "strong.".to_owned()
                ],
            ]
        );
    }
}
