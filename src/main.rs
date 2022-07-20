mod parser;
mod generator;
mod blocks;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::parser::FlowdownParser;
use crate::generator::voiceflow::*;

const INPUT: &str = r#"

@ conversation1
> hello world
[end]


"#;

fn main() {
    let mut parser = FlowdownParser::new();
    parser.parse(INPUT);

    // println!("{:?}", parser);

    let conv = parser.cur_conv();
    println!("{}", serialize_vf_file(conv));
}

