//! Tools for working with work-agnostic positional anchors (e.g. versification) in critic

use core::str::FromStr;

use serde::{Deserialize, Serialize};

#[cfg(feature = "anchor_example")]
pub mod example;

/// A dialect for Positional Anchors.
///
/// Notice that [`SuperAnchorDialect`] is just a convenience Trait to remember what an anchor dialect
/// has to satisfy. It is never actually used by critic. Instead, you must add your new Anchor
/// dialect to [`Anchor`].
///
/// By example, a positional Anchor could look like the following.
/// Here we model a short work, which is a poem consisting of only two Stanzas.
/// The positional Anchors are the starts of the two Stanzas.
/// ```
/// use core::str::FromStr;
/// use critic::anchor::SuperAnchorDialect;
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum ParseStanzaError {
///     EmptyString,
///     TooManyChars,
///     NotInRange,
///     NotANumber,
/// }
/// impl core::fmt::Display for ParseStanzaError {
///     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
///         match self {
///             Self::EmptyString => write!(f, "Empty String is no valid stanza number."),
///             Self::TooManyChars => write!(f, "More then one character is no valid stanza number."),
///             Self::NotANumber => write!(f, "The Argument is not a number."),
///             Self::NotInRange => write!(f, "The Argument is not either 1 or 2."),
///         }
///     }
/// }
/// impl std::error::Error for ParseStanzaError {}
///
/// #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
/// enum Stanza {
///     One,
///     Two,
/// }
/// impl core::fmt::Display for Stanza {
///     fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
///         match self {
///             Self::One => {
///                 write!(f, "1")
///             }
///             Self::Two => {
///                 write!(f, "2")
///             }
///         }
///     }
/// }
/// impl FromStr for Stanza {
///     type Err = ParseStanzaError;
///     fn from_str(s: &str) -> Result<Self, Self::Err> {
///         if s.is_empty() {
///             Err(ParseStanzaError::EmptyString)
///         } else if s.len() >= 2 {
///             Err(ParseStanzaError::TooManyChars)
///         } else {
///             let nr = s.parse::<u8>().map_err(|_| ParseStanzaError::NotANumber)?;
///             match nr {
///                 1 => Ok(Stanza::One),
///                 2 => Ok(Stanza::Two),
///                 _ => Err(ParseStanzaError::NotInRange),
///             }
///         }
///     }
/// }
/// impl SuperAnchorDialect for Stanza {
///     type ParseError = ParseStanzaError;
/// }
/// ```
///
/// A more interesting example for an [SuperAnchorDialect] could be a versification scheme for a
/// classical work.
pub trait SuperAnchorDialect:
    FromStr<Err = Self::ParseError> + core::fmt::Display + core::fmt::Debug + PartialEq
{
    type ParseError: std::error::Error + PartialEq;
}

/// The list of all supported anchors
///
/// This enum also contains the Values, not just the Types of dialect.
///
/// See also [AnchorDialect] for the enum which contains only the anchor dialect, but no value.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Anchor {
    /// An example anchor
    #[cfg(feature = "anchor_example")]
    Example(example::Example),
}
impl core::fmt::Display for Anchor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            #[cfg(feature = "anchor_example")]
            Self::Example(x) => {
                write!(f, "{x}")
            }
            #[cfg_attr(feature = "anchor_example", allow(unreachable_patterns))]
            _ => unreachable!(),
        }
    }
}
impl Anchor {
    /// Forget the value
    ///
    /// This is the forgetful functor from [Anchor] to [AnchorDialect].
    pub fn forgetful_functor(self) -> AnchorDialect {
        match self {
            #[cfg(feature = "anchor_example")]
            Self::Example(_) => AnchorDialect::Example,
        }
    }
}

/// The list of all supported Anchor Dialects
///
/// This enum contains no Values, only the Types of dialect.
///
/// See also [Anchor] for the enum which contains also the actual values.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AnchorDialect {
    /// An example anchor
    #[cfg(feature = "anchor_example")]
    Example,
}
impl AnchorDialect {
    /// Parse a string as the given type of [AnchorDialect], returning the corresponding [Anchor]
    pub fn parse(&self, s: &str) -> Result<Anchor, Box<dyn std::error::Error>> {
        match self {
            #[cfg(feature = "anchor_example")]
            Self::Example => Ok(Anchor::Example(s.parse::<example::Example>()?)),
            #[cfg_attr(feature = "anchor_example", allow(unreachable_patterns))]
            _ => unreachable!(),
        }
    }

    pub fn get_by_name(s: &str) -> Option<Self> {
        match s {
            #[cfg(feature = "anchor_example")]
            "example" => Some(Self::Example),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_by_name() {
        #[cfg(feature = "anchor_example")]
        assert_eq!(
            super::AnchorDialect::get_by_name("example"),
            Some(super::AnchorDialect::Example)
        );
        assert_eq!(super::AnchorDialect::get_by_name("does not exist"), None);
    }
}
