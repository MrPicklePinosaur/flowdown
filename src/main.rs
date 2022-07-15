mod generator;

use std::collections::HashMap;
use pest::{Parser, iterators::{Pair, Pairs}};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

const INPUT: &str = r#"

@ conversation 1

    -> bookmark 1
    -> bookmark 2
    -> @conversation 1
    -> @ conversation 2

    = bookmark 1

    = bookmark 2

@ conversation 2

    = bookmark 2

    = bookmark 3
"#;

fn main() {
    let parse = Lexer::parse(Rule::lines, INPUT).expect("unsuccessful parse").next().unwrap();
    let compiler = Compiler::default();
    compiler.parse_sequence(parse);
}

#[derive(Default)]
struct Bookmark {

}
#[derive(Default)]
struct Conversation {
    bookmark_table: HashMap<String, Bookmark>,
}
#[derive(Default)]
struct Compiler {
    conversation_table: HashMap<String, Conversation>,
    variable_table: HashMap<String, String>,
}

impl Compiler {
    /*
    pub fn new() -> Self {
        Compiler {

        }
    }
    */

    pub fn parse_sequence(&self, rule: Pair<Rule>) {
        for line in rule.into_inner() {
            match line.as_rule() {
                Rule::conversation_block=> {
                    println!("conversation_block");
                    self.parse_conversation_block(line);
                },
                Rule::topic_block => {
                    println!("topic_block");
                    self.parse_topic_block(line);
                },
                Rule::choice_block => {
                    println!("choice_block");
                    self.parse_choice_block(line);
                },
                Rule::command_block => {
                    println!("command_block");
                    self.parse_command_block(line);
                },
                Rule::utterance_block => {
                    println!("utterance_block"); 
                    self.parse_utterance_block(line);
                },
                Rule::jump_block => {
                    println!("jump_block");
                    self.parse_jump_block(line);
                },
                _ => unreachable!()
            }
        }
    }

    fn parse_conversation_block(&self, rule: Pair<Rule>) {
        let mut it = rule.into_inner();
        let conversation_identifier = it.next().unwrap().as_str();
        println!("conversation identifier: {}", conversation_identifier);
    }

    fn parse_topic_block(&self, rule: Pair<Rule>) {
        let mut it = rule.into_inner();
        let header_depth = it.next().unwrap().as_str().len();
        let header_identifier: &str = it.next().unwrap().as_str();

        println!("header: depth {}, identifier: {}", header_depth, header_identifier);
    }

    fn parse_choice_block(&self, rule: Pair<Rule>) {
        for choice_line in rule.into_inner() {
            let mut it = choice_line.into_inner();
            let content = it.next().unwrap().as_str();
            println!("choice_line {}", content);
        }
    }

    fn parse_command_block(&self, rule: Pair<Rule>) {
        for pair in rule.into_inner() {
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::end_command_body => { println!("> end command"); },
                    Rule::code_command_body => { println!("> code command"); },
                    _ => unreachable!()
                }
            }
        }
    }

    fn parse_utterance_block(&self, rule: Pair<Rule>) {
        let text = rule.as_span().as_str().replace("\n", " ");
        println!("utterance {}", text);
    }

    fn parse_jump_block(&self, rule: Pair<Rule>) {
        let mut it = rule.into_inner();
        let jump_to = it.next().unwrap();
        match jump_to.as_rule() {
            Rule::conversation_identifier => {
                println!("jump block to conversation: @{}", jump_to.as_str());
            },
            Rule::topic_identifier => {
                println!("jump block to topic: {}", jump_to.as_str());
            }
            _ => unreachable!()
        }
    }
}
