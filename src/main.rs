mod blocks;
pub mod error;
mod generator;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::generator::voiceflow::*;
use crate::parser::ConversationBuilder;
use log::{debug, info};

const INPUT: &str = r#"

> hello world # yue-HK-standard-B
[set $counter '1']
[capture $firstName]
-> bookmark1

= bookmark1

> goodbye world

"#;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let mut conv_builder = ConversationBuilder::new();
    conv_builder.parse(INPUT).unwrap();
    let conv = conv_builder.build().unwrap();

    debug!("{:?}", conv.dialog_table);

    // println!("{:?}", parser);

    let config = VFConfig {
        project_name: "flowdown".into(),
    };

    let dialog = conv.dialog_table.get("main").unwrap();
    println!("{}", serialize_vf_file(&config, dialog, &conv.variables));
}
