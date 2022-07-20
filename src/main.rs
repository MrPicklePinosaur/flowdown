mod blocks;
mod generator;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::generator::voiceflow::*;
use crate::parser::FlowdownParser;

const INPUT: &str = r#"

@ conversation1

    > hello world # yue-HK-standard-B
    [set $counter '1']
    [capture $firstName]
    > goodbye world


"#;

fn main() {
    let mut parser = FlowdownParser::new();
    parser.parse(INPUT);

    // println!("{:?}", parser);

    let config = VFConfig {
        project_name: "flowdown".into(),
    };

    let conv = parser.cur_conv();
    println!("{}", serialize_vf_file(&config, conv, parser.variables()));
}
