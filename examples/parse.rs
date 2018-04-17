extern crate pest;
extern crate fontobene;

use std::env;
use std::process;
use std::rc::Rc;

use pest::Parser;
use pest::inputs::FileInput;
use fontobene::{FontobeneParser, Rule};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path/to/font.bene>", args[0]);
        process::exit(1);
    }

    let input = FileInput::new(&args[1]).expect("Could not open file");
    let _parsed = FontobeneParser::parse(Rule::main, Rc::new(input))
        .unwrap_or_else(|e| panic!("{0}: {0:?}", e));
    
    println!("Parsed");
}
