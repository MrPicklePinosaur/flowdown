mod blocks;
pub mod error;
mod generator;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use crate::generator::voiceflow::*;
use crate::parser::ConversationBuilder;
use argparse::{Cli, Command, Flag, FlagParse};
use log::{debug, info};

const INPUT: &str = r#"

line 1
-> end
line 2
line 3
line 4
= end
line 5


"#;

fn main() {
    env_logger::builder().format_timestamp(None).init();

    let cli = Cli {
        program_name: "fdc",
        synopsis: "",
        root_command: Command {
            command_name: "compile",
            desc: "",
            handler: handle_compile,
            flags: vec![
                Flag::new('o')
                    .long("output")
                    .desc("file to output to")
                    .parameter(),
                Flag::new('p')
                    .long("pretty")
                    .desc("pretty format the output"),
            ],
        },
        ..Default::default()
    };

    let args = std::env::args().collect();
    cli.run(&args).unwrap();
}

fn handle_compile(flagparse: FlagParse) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::*;
    use std::io::*;
    use std::path::*;

    let output_file = flagparse.get_flag_value::<String>('o');

    let mut conv_builder = ConversationBuilder::new();
    conv_builder.parse(INPUT).unwrap();
    let conv = conv_builder.build().unwrap();

    // debug!("{:?}", conv.dialog_table);

    // println!("{:?}", parser);

    let config = VFConfig {
        project_name: "flowdown".into(),
        ..Default::default()
    };

    let mut compiler = VFCompiler::new(config);
    let output_raw = compiler.compile(&conv, &conv.variables);

    let output_str = if flagparse.get_flag('p') {
        serde_json::to_string_pretty(&output_raw).unwrap()
    } else {
        serde_json::to_string(&output_raw).unwrap()
    };

    if let Some(path) = output_file {
        let mut handle = File::create(Path::new(&(path as String)))?;
        write!(handle, "{}", output_str)?;
    } else {
        write!(stdout(), "{}", output_str)?;
    };

    Ok(())
}
