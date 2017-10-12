#[cfg(not(test))]
extern crate pest;
#[cfg(test)]
#[macro_use]
extern crate pest;
#[macro_use]
extern crate pest_derive;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../fontobene.pest");

#[derive(Parser)]
#[grammar = "../fontobene.pest"]
struct FontobeneParser;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_nozero() {
        // Should parse digits 1-9
        parses_to! {
            parser: FontobeneParser,
            input: "1",
            rule: Rule::hex_nozero,
            tokens: [
                hex_nozero(0, 1) // name_of_rule(start_pos, end_pos): no children
            ]
        };

        // Should not parse 0
        fails_with! {
            parser: FontobeneParser,
            input: "0",
            rule: Rule::hex_nozero,
            positives: vec![Rule::hex_nozero],
            negatives: vec![],
            pos: 0
        };

        // Should parse A-F
        parses_to! {
            parser: FontobeneParser,
            input: "F",
            rule: Rule::hex_nozero,
            tokens: [
                hex_nozero(0, 1) // name_of_rule(start_pos, end_pos): no children
            ]
        };

        // Should not parse lowercase letters
        fails_with! {
            parser: FontobeneParser,
            input: "f",
            rule: Rule::hex_nozero,
            positives: vec![Rule::hex_nozero],
            negatives: vec![],
            pos: 0
        };

        // Should not parse G
        fails_with! {
            parser: FontobeneParser,
            input: "G",
            rule: Rule::hex_nozero,
            positives: vec![Rule::hex_nozero],
            negatives: vec![],
            pos: 0
        };
    }

    #[test]
    fn hex() {
        // Should parse 0
        parses_to! {
            parser: FontobeneParser,
            input: "0",
            rule: Rule::hex,
            tokens: [
                hex(0, 1) // name_of_rule(start_pos, end_pos): no children
            ]
        };
    }

    #[test]
    fn codepoint() {
        // Should parse 0041
        parses_to! {
            parser: FontobeneParser,
            input: "0041",
            rule: Rule::codepoint,
            tokens: [
                codepoint(0, 3)
            ]
        }


        // TODO: 10000
    }
}
