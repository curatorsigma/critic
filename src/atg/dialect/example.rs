//! Example ATG dialect

use crate::atg::{AtgDialect, ControlPointDefinition};

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

pub struct ExampleAtgDialect {}
impl AtgDialect for ExampleAtgDialect {
    const NATIVE_POINTS: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,.'";
    const PUNCTUATION: &'static str = ",.";
    const ATG_CONTROL_POINTS: ControlPointDefinition = EXAMPLE_CONTROL_POINTS;
    const WORD_DIVISOR: char = ' ';
}

