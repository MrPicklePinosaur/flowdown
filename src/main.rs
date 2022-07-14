use pest::{Parser, iterators::Pairs};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

const INPUT: &str = r#"
hello world
[code myfile.js]

how do you do
[end]
"#;

fn main() {
    let parse = Lexer::parse(Rule::lines, INPUT).expect("unsuccessful parse").next().unwrap();

    for line in parse.into_inner() {
        match line.as_rule() {
            Rule::command_block => {
                println!("command_block");
                parse_command_block(line.into_inner());
            },
            Rule::utterance_block => {
                println!("utterance_block"); 
            },
            _ => {}
        }
    }
}

fn parse_command_block(pairs: Pairs<Rule>) {
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::end_command_body => { println!("> end command"); },
                Rule::code_command_body => { println!("> code command"); },
                _ => {}
            }
        }
    }
}
