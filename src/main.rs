mod blocks;
pub mod error;
mod generator;
mod parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;

use crate::{error::CliError, generator::voiceflow::*, parser::ConversationBuilder};
use argparse::{Cli, Command, Flag, FlagParse};
use log::{debug, info};

const HELP_MSG: &'static str = r#"
fdc - the flowdown compiler

USAGE: fdc [INPUT]

OPTIONS:
    -h, --help                print this help message
    -p, --pretty              format output file nicely
    -o, --output <FILE>       specify file to output to
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
                Flag::new('h').long("help"),
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

    use std::io::*;
    let args = std::env::args().collect();
    if let Err(boxed) = cli.run(&args) {
        write!(stderr(), "{}", boxed).unwrap();
    }
}

fn handle_compile(flagparse: FlagParse) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::*;
    use std::io::*;
    use std::path::*;

    if flagparse.get_flag('h') {
        println!("{}", HELP_MSG);
        return Ok(());
    }

    let output_file = flagparse.get_flag_value::<String>('o');

    debug!("{:?}", flagparse.args);
    if flagparse.args.len() < 1 {
        return Err(Box::new(CliError::MissingInputFile));
    }

    let input_file = flagparse.args.get(0).unwrap();
    let body = std::fs::read_to_string(input_file)
        .map_err(|_| Box::new(CliError::CannotReadFlowdownFile(input_file.into())))?;

    let mut conv_builder = ConversationBuilder::new();
    conv_builder.parse(&body).unwrap();
    let conv = conv_builder.build().unwrap();

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
