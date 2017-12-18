#[cfg(not(test))]
extern crate pest;
#[cfg(test)]
#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(test)]
mod tests;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../fontobene.pest");

#[derive(Parser)]
#[grammar = "../fontobene.pest"]
/// The main parser.
pub struct FontobeneParser;
