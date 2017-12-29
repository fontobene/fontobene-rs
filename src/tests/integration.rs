use pest::Parser;

use ::{FontobeneParser, Rule};

static FONT_STANDARD: &'static str = include_str!("fonts/standard.bene");

/// The parser should be able to parse an entire file without errors.
#[test]
fn parse_standard() {
    FontobeneParser::parse_str(Rule::main, &FONT_STANDARD)
        .unwrap_or_else(|e| panic!("{}", e));
}
