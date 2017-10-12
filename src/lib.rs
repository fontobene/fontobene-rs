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
    fn codepoint() {
        // Should parse `004A`
        parses_to! {
            parser: FontobeneParser,
            input: "004A",
            rule: Rule::codepoint,
            tokens: [
                codepoint(0, 4) // name_of_rule(start_pos, end_pos): no children
            ]
        }

        // Should not parse `00 4a`
        fails_with! {
            parser: FontobeneParser,
            input: "00 4a",
            rule: Rule::codepoint,
            positives: vec![Rule::codepoint],
            negatives: vec![],
            pos: 0
        }

        // Should not parse `004A`
        fails_with! {
            parser: FontobeneParser,
            input: "00 4A",
            rule: Rule::codepoint,
            positives: vec![Rule::codepoint],
            negatives: vec![],
            pos: 0
        }

        // Should parse `100000`
        parses_to! {
            parser: FontobeneParser,
            input: "100000",
            rule: Rule::codepoint,
            tokens: [
                codepoint(0, 6) // name_of_rule(start_pos, end_pos): no children
            ]
        }
    }

    #[test]
    fn declaration_simple() {
        // Should parse `[0041]`
        parses_to! {
            parser: FontobeneParser,
            input: "[0041]",
            rule: Rule::declaration,
            tokens: [
                declaration(0, 6, [
                    glyph(0, 6, [
                        codepoint(1, 5)
                    ])
                ])
                // name_of_rule(start_pos, end_pos, [children])
            ]
        }

        // Should not parse `[ 0041  ]`
        fails_with! {
            parser: FontobeneParser,
            input: "[ 0041  ]",
            rule: Rule::declaration,
            positives: vec![Rule::codepoint],
            negatives: vec![],
            pos: 1
        }
    }

    #[test]
    fn declaration_with_comment() {
        // Should parse `[1F4A9] 💩`
        parses_to! {
            parser: FontobeneParser,
            input: "[1F4A9] 💩",
            rule: Rule::declaration,
            tokens: [
                declaration(0, 12, [
                    glyph(0, 7, [
                        codepoint(1, 6)
                    ])
                ])
                // name_of_rule(start_pos, end_pos, [children])
            ]
        }
    }

}
