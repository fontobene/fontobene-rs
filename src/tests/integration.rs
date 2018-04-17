use pest::Parser;

use ::{FontobeneParser, Rule};

static FONT_NEWSTROKE: &'static str = include_str!("fonts/newstroke.bene");

/// The parser should be able to parse an entire file without errors.
#[test]
fn parse_newstroke() {
    FontobeneParser::parse_str(Rule::main, &FONT_NEWSTROKE)
        .unwrap_or_else(|e| panic!("{}", e));
}
