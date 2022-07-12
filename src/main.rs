
use pest::Parser;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

const INPUT: &str = r#"[end]"#;

fn main() {
    let parse = Lexer::parse(Rule::command_block, INPUT).expect("unsuccessful parse").next().unwrap();

    for pair in parse.into_inner() {
        println!("Rule: {:?}", pair.as_rule());
    }
}
