mod parser;
mod generator;
mod blocks;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::FlowdownParser;

const INPUT: &str = r#"

@ conversation1
[end]
[end]
[end]


[end]

@ conversation2
[end]

"#;

fn main() {
    let mut parser = FlowdownParser::new();
    parser.parse(INPUT);

    println!("{:?}", parser);
}

