#[cfg(not(test))]
pub extern crate pest;
#[cfg(test)]
#[macro_use]
pub extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../fontobene.pest");

#[derive(Parser)]
#[grammar = "../fontobene.pest"]
/// The main parser.
///
/// Use it like this:
///
/// ```should_panic
/// extern crate fontobene;
///
/// use fontobene::{FontobeneParser, Rule};
/// use fontobene::pest::{Parser};
///
/// fn main() {
///     let input = "..."; // The font file contents to parse
///
///     let _parsed = FontobeneParser::parse_str(Rule::main, &input)
///         .unwrap_or_else(|e| panic!("{}", e));
/// }
/// ```

pub struct FontobeneParser;
