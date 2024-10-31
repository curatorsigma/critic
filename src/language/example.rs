//! The language used in examples.

use critic_core::atg::Word;

use super::WordNormalForm;

pub fn normalise(input: Vec<(Word, String)>) -> Vec<WordNormalForm> {
    input
        .into_iter()
        .map(|(w, s)| WordNormalForm::new(w, s, None))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use critic_core::{anchor::example::Example, atg::Text};

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
