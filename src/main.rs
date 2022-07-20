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
> goodbye world

"#;

fn main() {
    let mut parser = FlowdownParser::new();
    parser.parse(INPUT);

    // println!("{:?}", parser);

    let config = VFConfig {
        project_name: "flowdown".into()
    };

    let conv = parser.cur_conv();
    println!("{}", serialize_vf_file(&config, conv));
}

