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

    // For information about the parses_to! and fails_with! macros,
    // see https://docs.rs/pest/1.0.0-beta/pest/macro.parses_to.html
    // and https://docs.rs/pest/1.0.0-beta/pest/macro.fails_with.html

    #[test]
    fn codepoint() {
        // Should parse `004A`
        parses_to! {
            parser: FontobeneParser,
            input: "004A",
            rule: Rule::codepoint,
            tokens: [
                codepoint(0, 4)
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
                codepoint(0, 6)
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
            ]
        }
    }

    #[test]
    fn reference() {
        // Should parse `@0041`
        parses_to! {
            parser: FontobeneParser,
            input: "@0041",
            rule: Rule::reference,
            tokens: [
                reference(0, 5, [
                    codepoint(1, 5)
                ])
            ]
        }

        // Should not parse `@ 0041`
        fails_with! {
            parser: FontobeneParser,
            input: "@ 0041",
            rule: Rule::reference,
            positives: vec![Rule::codepoint],
            negatives: vec![],
            pos: 1
        }
    }

    #[test]
    fn number() {
        // Should parse `123`
        parses_to! {
            parser: FontobeneParser,
            input: "123",
            rule: Rule::number,
            tokens: [
                number(0, 3)
            ]
        }

        // Should parse 0
        parses_to! {
            parser: FontobeneParser,
            input: "0",
            rule: Rule::number,
            tokens: [
                number(0, 1)
            ]
        }

        // Should parse `1.23`
        parses_to! {
            parser: FontobeneParser,
            input: "1.23",
            rule: Rule::number,
            tokens: [
                number(0, 4)
            ]
        }

        // Should parse 0.0
        parses_to! {
            parser: FontobeneParser,
            input: "0.0",
            rule: Rule::number,
            tokens: [
                number(0, 3)
            ]
        }

        // Should parse -12.345
        parses_to! {
            parser: FontobeneParser,
            input: "-12.345",
            rule: Rule::number,
            tokens: [
                number(0, 7)
            ]
        }
    }

    #[test]
    fn coord_pair() {
        // Should parse `1,2`
        parses_to! {
            parser: FontobeneParser,
            input: "1,2",
            rule: Rule::coord_pair,
            tokens: [
                coord_pair(0, 3, [
                    number(0, 1),
                    number(2, 3)
                ])
            ]
        }

        // Should parse `0,-12.345`
        parses_to! {
            parser: FontobeneParser,
            input: "0,-12.345",
            rule: Rule::coord_pair,
            tokens: [
                coord_pair(0, 9, [
                    number(0, 1),
                    number(2, 9)
                ])
            ]
        }

        // Should not parse `1, 2`
        fails_with! {
            parser: FontobeneParser,
            input: "1, 2",
            rule: Rule::coord_pair,
            positives: vec![Rule::number],
            negatives: vec![],
            pos: 2
        }
    }

    #[test]
    fn polyline() {
        // Should parse `1,2`
        parses_to! {
            parser: FontobeneParser,
            input: "1,2",
            rule: Rule::polyline,
            tokens: [
                polyline(0, 3, [
                    coord_pair(0, 3, [
                        number(0, 1),
                        number(2, 3)
                    ])
                ])
            ]
        }

        // Should parse `0,-12.345;-1,0;2,0.1`
        parses_to! {
            parser: FontobeneParser,
            input: "0,-12.345;-1,0;2,0.1",
            rule: Rule::polyline,
            tokens: [
                polyline(0, 20, [
                    coord_pair(0, 9, [
                        number(0, 1),
                        number(2, 9)
                    ]),
                    coord_pair(10, 14, [
                        number(10, 12),
                        number(13, 14)
                    ]),
                    coord_pair(15, 20, [
                        number(15, 16),
                        number(17, 20)
                    ])
                ])
            ]
        }
    }

}
