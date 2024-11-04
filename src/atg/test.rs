//! Tests for ATG parsing
use std::str::FromStr;

use crate::anchor::SuperAnchorDialect;

use crate::atg::dialect::ExampleAtgDialect;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[test]
#[cfg(feature = "anchor_example")]
fn known_good_stanza() {
    let stanza: Anchor = AnchorDialect::Example.parse("1").unwrap();
    assert_eq!(
        stanza,
        Anchor::Example(crate::anchor::example::Example::One)
    );

    let stanza: Anchor = AnchorDialect::Example.parse("2").unwrap();
    assert_eq!(
        stanza,
        Anchor::Example(crate::anchor::example::Example::Two)
    );
}

#[test]
#[cfg(feature = "anchor_example")]
fn render_text() {
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
    let mut text = Vec::<Part>::new();
    text.push(Part::Anchor(Anchor::Example(
        crate::anchor::example::Example::One,
    )));
    text.push(Part::Native("In twilight's glow, the sha".to_owned()));
    text.push(Part::Illegible(Uncertain::<Illegible>::new(
        2,
        Some("do".to_owned()),
    )));
    text.push(Part::Native("ws dance,".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native("Whispers of dre".to_owned()));
    text.push(Part::Illegible(Uncertain::<Illegible>::new(
        1,
        Some("a".to_owned()),
    )));
    text.push(Part::Native("ms in a fleeting trance,".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native("Stars awaken".to_owned()));
    text.push(Part::Illegible(Uncertain::<Illegible>::new(
        1,
        Some(",".to_owned()),
    )));
    text.push(Part::Native(" the night unfurls,".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native(
        "A canvas painted with secret pearls.".to_owned(),
    ));
    text.push(Part::FormatBreak(FormatBreak::Paragraph));
    text.push(Part::Anchor(Anchor::Example(
        crate::anchor::example::Example::Two,
    )));

    text.push(Part::Native("The moonlight ".to_owned()));
    text.push(Part::Correction(Correction {
        versions: vec![
            Present::Native("illuminates".to_owned()),
            Present::Native("bathes".to_owned()),
        ],
    }));
    text.push(Part::Native(" the quiet stream,".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native(
        "Where echoes linger of a distant dream,".to_owned(),
    ));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native("Soft breezes carry the ta".to_owned()));
    text.push(Part::Lacuna(Uncertain::<Lacuna>::new(8, None)));
    text.push(Part::Native("ld,".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Line));

    text.push(Part::Native("Of hearts entwined and lo".to_owned()));
    text.push(Part::Lacuna(Uncertain::<Lacuna>::new(8, None)));
    text.push(Part::Native("d.".to_owned()));
    text.push(Part::FormatBreak(FormatBreak::Folio));
    assert_eq!(Text { parts: text }.render::<ExampleAtgDialect>(), res);
}

#[test]
#[cfg(feature = "anchor_example")]
fn render_part() {
    let native = Part::Native("native".to_owned());
    assert_eq!(native.render::<ExampleAtgDialect>(), "native");
    let illeg = Part::Illegible(Uncertain::<Illegible>::new(2, Some("?s".to_owned())));
    assert_eq!(illeg.render::<ExampleAtgDialect>(), "^(2)(?s)");
    let lacuna = Part::Lacuna(Uncertain::<Lacuna>::new(4, None));
    assert_eq!(lacuna.render::<ExampleAtgDialect>(), "~(4)");
    let p1 = Present::Native("some text or smth...".to_owned());
    let p2 = Present::Illegible(Uncertain::<Illegible>::new(2, Some("os".to_owned())));
    let correction = Part::Correction(Correction {
        versions: vec![p1, p2],
    });
    assert_eq!(
        correction.render::<ExampleAtgDialect>(),
        "&(some text or smth...)(^(2)(os))"
    );
    let format_break = Part::FormatBreak(FormatBreak::Column);
    assert_eq!(format_break.render::<ExampleAtgDialect>(), "/(column)");
    let anchor = Part::Anchor(Anchor::Example(crate::anchor::example::Example::Two));
    assert_eq!(anchor.render::<ExampleAtgDialect>(), "§(2)");
}

#[test]
fn render_correction() {
    let p1 = Present::Native("some text or smth...".to_owned());
    let illeg = Present::Illegible(Uncertain::<Illegible>::new(2, Some("os".to_owned())));
    let correction = Correction {
        versions: vec![p1, illeg],
    };
    assert_eq!(
        correction.render::<ExampleAtgDialect>(),
        "&(some text or smth...)(^(2)(os))"
    );
}

#[test]
fn render_present() {
    let p1 = Present::Native("some text or smth...".to_owned());
    assert_eq!(
        p1.render::<ExampleAtgDialect>(),
        "some text or smth...".to_owned()
    );
    let illeg = Present::Illegible(Uncertain::<Illegible>::new(2, Some("os".to_owned())));
    assert_eq!(illeg.render::<ExampleAtgDialect>(), "^(2)(os)");
}

#[test]
fn render_uncertain_illegible() {
    let illeg = Uncertain::<Illegible>::new(2, Some("os".to_owned()));
    assert_eq!(illeg.render::<ExampleAtgDialect>(), "^(2)(os)");
}

#[test]
fn render_uncertain_lacuna() {
    let lacuna = Uncertain::<Lacuna>::new(2, Some("os".to_owned()));
    assert_eq!(lacuna.render::<ExampleAtgDialect>(), "~(2)(os)");
}

#[test]
fn render_format_break() {
    assert_eq!(FormatBreak::Line.render::<ExampleAtgDialect>(), "/(line)");
    assert_eq!(
        FormatBreak::Column.render::<ExampleAtgDialect>(),
        "/(column)"
    );
    assert_eq!(
        FormatBreak::Paragraph.render::<ExampleAtgDialect>(),
        "/(paragraph)"
    );
    assert_eq!(FormatBreak::Folio.render::<ExampleAtgDialect>(), "/(folio)");
}

#[test]
fn test_escape_until_control_point() {
    let input = "asd(";
    let parsed = escape_until_control_point::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(parsed, ("asd".to_owned(), Some('('), "(", 4));

    let input = "asd^(1)(c)";
    let parsed = escape_until_control_point::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(parsed, ("asd".to_owned(), Some('^'), "^(1)(c)", 4));

    let input = "a\\41d(";
    let parsed = escape_until_control_point::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(parsed, ("aAd".to_owned(), Some('('), "(", 6));
}

#[test]
#[cfg(feature = "anchor_example")]
fn parse_part() {
    let native = "abcdef";
    let parsed_native =
        Part::parse::<ExampleAtgDialect>(native, AnchorDialect::Example, 1).unwrap();
    assert_eq!(parsed_native, (Part::Native("abcdef".to_owned()), ""));

    let illeg = "^(3)(abc)";
    let parsed_illeg = Part::parse::<ExampleAtgDialect>(illeg, AnchorDialect::Example, 1).unwrap();
    assert_eq!(
        parsed_illeg,
        (
            Part::Illegible(Uncertain::<Illegible>::new(3, Some("abc".to_owned()))),
            ""
        )
    );
    let illeg = "^(2)";
    let parsed_illeg = Part::parse::<ExampleAtgDialect>(illeg, AnchorDialect::Example, 1).unwrap();
    assert_eq!(
        parsed_illeg,
        (Part::Illegible(Uncertain::<Illegible>::new(2, None)), "")
    );
    let illeg = "^(2)()";
    let parsed_illeg = Part::parse::<ExampleAtgDialect>(illeg, AnchorDialect::Example, 1).unwrap();
    assert_eq!(
        parsed_illeg,
        (Part::Illegible(Uncertain::<Illegible>::new(2, None)), "")
    );

    let lacuna = "~(3)(abc)";
    let parsed_lacuna =
        Part::parse::<ExampleAtgDialect>(lacuna, AnchorDialect::Example, 0).unwrap();
    assert_eq!(
        parsed_lacuna,
        (
            Part::Lacuna(Uncertain::<Lacuna>::new(3, Some("abc".to_owned()))),
            ""
        )
    );
    let lacuna = "~(2)some";
    let parsed_lacuna =
        Part::parse::<ExampleAtgDialect>(lacuna, AnchorDialect::Example, 0).unwrap();
    assert_eq!(
        parsed_lacuna,
        (Part::Lacuna(Uncertain::<Lacuna>::new(2, None)), "some")
    );
    let lacuna = "~(2)()";
    let parsed_lacuna =
        Part::parse::<ExampleAtgDialect>(lacuna, AnchorDialect::Example, 0).unwrap();
    assert_eq!(
        parsed_lacuna,
        (Part::Lacuna(Uncertain::<Lacuna>::new(2, None)), "")
    );

    let input = "/(line)";
    let parsed = Part::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 0);
    assert_eq!(parsed.unwrap(), (Part::FormatBreak(FormatBreak::Line), ""));

    let input = "#(comment)";
    let parsed = Part::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 0);
    assert_eq!(parsed.unwrap(), (Part::Native("".to_owned()), ""));

    let input = "&(optiona)(optionb)";
    let parsed = Part::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 2);
    assert_eq!(
        parsed.unwrap(),
        (
            Part::Correction(Correction {
                versions: vec![
                    Present::Native("optiona".to_owned()),
                    Present::Native("optionb".to_owned())
                ]
            }),
            ""
        )
    );

    let input = "&(optiona)no option";
    let parsed = Part::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 1);
    assert_eq!(
        parsed.unwrap(),
        (
            Part::Correction(Correction {
                versions: vec![Present::Native("optiona".to_owned())]
            }),
            "no option"
        )
    );

    let input = "&(optiona)(option b)(opti on c)";
    let parsed = Part::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 2);
    assert!(parsed.is_err());
}

#[test]
fn parse_native() {
    let native = "a";
    let (parsed_native, _) = Part::parse_native::<ExampleAtgDialect>(native).unwrap();
    assert_eq!(parsed_native, Part::Native(native.to_owned()));

    let native = "a^(1)(b)";
    let (parsed_native, remainder) = Part::parse_native::<ExampleAtgDialect>(native).unwrap();
    assert_eq!(parsed_native, Part::Native("a".to_owned()));
    assert_eq!(remainder, "^(1)(b)");
}

#[test]
fn test_escape_one_if_required() {
    let input = "a";
    let (char, remainder, offset) = escape_one_if_required::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(char, 'a');
    assert_eq!(remainder, "");
    assert_eq!(offset, 1);

    let input = "^";
    let (char, remainder, offset) = escape_one_if_required::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(char, '^');
    assert_eq!(remainder, "");
    assert_eq!(offset, 1);

    let input = "\\000041somestuff";
    let (char, remainder, offset) = escape_one_if_required::<ExampleAtgDialect>(input).unwrap();
    assert_eq!(char, '\u{41}');
    assert_eq!(remainder, "somestuff");
    assert_eq!(offset, 7);

    let input = "";
    let res = escape_one_if_required::<ExampleAtgDialect>(input);
    assert_eq!(res, Err("".to_owned()));
}

#[test]
#[cfg(feature = "anchor_example")]
fn test_parse_anchor() {
    let input = "(1)asdf";
    let parsed = Part::parse_anchor::<ExampleAtgDialect>(input, AnchorDialect::Example).unwrap();
    assert_eq!(
        parsed,
        (
            Anchor::Example(crate::anchor::example::Example::One),
            "asdf"
        )
    )
}

#[test]
fn parse_uncertain() {
    let input = "(2)(abc)";
    let parsed = Uncertain::<Illegible>::parse::<ExampleAtgDialect>(&input).unwrap();
    assert_eq!(
        parsed,
        (Uncertain::<Illegible>::new(2, Some("abc".to_owned())), "")
    );

    let input = "(2)(\\g)";
    let parsed = Uncertain::<Illegible>::parse::<ExampleAtgDialect>(&input);
    assert_eq!(parsed.unwrap_err().location, 4);
}

#[test]
#[cfg(feature = "anchor_example")]
fn parse_render() {
    let input = "^(1)(a)";
    let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 0).unwrap();
    assert_eq!(parsed.render::<ExampleAtgDialect>(), input);

    let res = "§(1)\
    lo~(8)d.\
    ";
    let parsed = Text::parse::<ExampleAtgDialect>(res, AnchorDialect::Example, 1).unwrap();
    assert_eq!(parsed.render::<ExampleAtgDialect>(), res);

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

#[test]
#[cfg(feature = "anchor_example")]
fn parse_with_trailing_newline() {
    let input = "some string i ges\n";
    let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 1).unwrap();
    assert_eq!(parsed.render::<ExampleAtgDialect>(), "some string i ges");
}

#[test]
#[cfg(feature = "anchor_example")]
fn test_auto_normalise() {
    let input = "This &(word)(sword) ~(3)^(2)(st)rong.";
    let parsed = Text::parse::<ExampleAtgDialect>(input, AnchorDialect::Example, 2).unwrap();
    let uniques: Vec<UniqueText> = parsed.clone().into();
    assert_eq!(uniques.len(), 2);
    let normalised = parsed.auto_normalise::<ExampleAtgDialect>();
    let surface_only = normalised
        .into_iter()
        .map(|ant| {
            ant.text
                .into_iter()
                .map(|(_word, surface)| surface)
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
                "strong".to_owned(),
                ".".to_owned(),
            ],
            vec![
                "This".to_owned(),
                "sword".to_owned(),
                "~~~".to_owned(),
                "strong".to_owned(),
                ".".to_owned(),
            ],
        ]
    );
}
