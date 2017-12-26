use ::{FontobeneParser, Rule};

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

#[test]
fn definition() {
    parses_to! {
        parser: FontobeneParser,
        input: "[0041]\n@0040",
        rule: Rule::definition,
        tokens: [
            definition(0, 12, [
                declaration(0, 6, [
                    glyph(0, 6, [
                        codepoint(1, 5)
                    ])
                ]),
                reference(7, 12, [
                    codepoint(8, 12)
                ])
            ])
        ]
    }

    parses_to! {
        parser: FontobeneParser,
        input: "[0041] A\n@0040  \n 1,2;3,-4.1\n@0039",
        rule: Rule::definition,
        tokens: [
            definition(0, 34, [
                declaration(0, 8, [
                    glyph(0, 6, [
                        codepoint(1, 5)
                    ])
                ]),
                reference(9, 14, [
                    codepoint(10, 14)
                ]),
                polyline(18, 28, [
                    coord_pair(18, 21, [
                       number(18, 19),
                       number(20, 21)
                    ]),
                    coord_pair(22, 28, [
                       number(22, 23),
                       number(24, 28)
                    ])
                ]),
                reference(29, 34, [
                  codepoint(30, 34)
                ])
            ])
        ]
    }
}

#[test]
fn string() {
    // Should parse `hello`
    parses_to! {
        parser: FontobeneParser,
        input: "hello",
        rule: Rule::string,
        tokens: [
            string(0, 5)
        ]
    }

    // Should parse `"hello"`
    parses_to! {
        parser: FontobeneParser,
        input: "\"hello\"",
        rule: Rule::string,
        tokens: [
            string(0, 7)
        ]
    }

    // Should parse `a b`
    parses_to! {
        parser: FontobeneParser,
        input: "a b",
        rule: Rule::string,
        tokens: [
            string(0, 3)
        ]
    }

    // Should not parse an empty string
    fails_with! {
        parser: FontobeneParser,
        input: "",
        rule: Rule::string,
        positives: vec![Rule::string],
        negatives: vec![],
        pos: 0
    }

    // Should not parse whitespace
    fails_with! {
        parser: FontobeneParser,
        input: "  ",
        rule: Rule::string,
        positives: vec![Rule::string],
        negatives: vec![],
        pos: 0
    }

    // Should not parse leading whitespace
    fails_with! {
        parser: FontobeneParser,
        input: "  a",
        rule: Rule::string,
        positives: vec![Rule::string],
        negatives: vec![],
        pos: 0
    }
}

#[test]
fn header_section_simple() {
    // Should parse `[font]`
    parses_to! {
        parser: FontobeneParser,
        input: "[fo_nt]",
        rule: Rule::header_section,
        tokens: [
            header_section(0, 7, [
                ident(1, 6)
            ])
        ]
    }

    // Should not parse `[_font]`
    fails_with! {
        parser: FontobeneParser,
        input: "[_font]",
        rule: Rule::header_section,
        positives: vec![Rule::ident],
        negatives: vec![],
        pos: 1
    }
}

#[test]
fn header_key() {
    // Should parse `foo_bar`
    parses_to! {
        parser: FontobeneParser,
        input: "foo_bar",
        rule: Rule::header_key,
        tokens: [
            header_key(0, 7, [
                ident(0, 7)
            ])
        ]
    }
}

#[test]
fn header_value() {
    // Should parse `"foo bar"`
    parses_to! {
        parser: FontobeneParser,
        input: "foo bar",
        rule: Rule::header_value,
        tokens: [
            header_value(0, 7, [
                string(0, 7)
            ])
        ]
    }

    // Should parse `-123.4`
    parses_to! {
        parser: FontobeneParser,
        input: "-123.4",
        rule: Rule::header_value,
        tokens: [
            header_value(0, 6, [
                string(0, 6)
            ])
        ]
    }
}

#[test]
fn header_item_nowhitespace() {
    // Should parse `foo=-1`
    parses_to! {
        parser: FontobeneParser,
        input: "foo=-1",
        rule: Rule::header_item,
        tokens: [
            header_item(0, 6, [
                header_key(0, 3, [
                   ident(0, 3)
                ]),
                header_value(4, 6, [
                    string(4, 6)
                ])
            ])
        ]
    }
}

#[test]
fn header_item_whitespace() {
    // Should parse `foo =  a b c `
    parses_to! {
        parser: FontobeneParser,
        input: "foo =  a b c ",
        rule: Rule::header_item,
        tokens: [
            header_item(0, 13, [
                header_key(0, 3, [
                   ident(0, 3)
                ]),
                header_value(7, 13, [
                    string(7, 13)
                ])
            ])
        ]
    }
}

#[test]
fn header() {
    parses_to! {
        parser: FontobeneParser,
        input: "[font]\nversion = 1.0\nid=librepcb",
        rule: Rule::header,
        tokens: [
            header(0, 32, [
                header_section(0, 6, [
                    ident(1, 5)
                ]),
                header_item(7, 20, [
                    header_key(7, 14, [
                       ident(7, 14)
                    ]),
                    header_value(17, 20, [
                        string(17, 20)
                    ])
                ]),
                header_item(21, 32, [
                    header_key(21, 23, [
                       ident(21, 23)
                    ]),
                    header_value(24, 32, [
                        string(24, 32)
                    ])
                ])
            ])
        ]
    }
}
