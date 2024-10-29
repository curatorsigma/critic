//! The language used in examples.

pub fn normalise(input: Vec<String>) -> Vec<String> {
    input
}

#[cfg(test)]
mod test {
    use critic_core::{anchor::example::Example, atg::Text};

    use crate::{dialect::atg::ExampleAtgDialect, normalise::normalise};

    #[test]
    fn normalise_example() {
        let input = "This &(word)(sword) ~(3)^(2)(st)rong.";
        let parsed = Text::parse::<ExampleAtgDialect>(input, critic_core::anchor::AnchorDialect::Example).unwrap();
        let normalised = normalise::<ExampleAtgDialect>(parsed, crate::language::Language::Example);
        assert_eq!(normalised, vec![
            vec!["This".to_owned(), "word".to_owned(), "~~~".to_owned(), "strong.".to_owned()],
            vec!["This".to_owned(), "sword".to_owned(), "~~~".to_owned(), "strong.".to_owned()],
        ]);
    }
}
