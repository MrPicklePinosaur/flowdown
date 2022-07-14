use pest::{Parser, iterators::{Pair, Pairs}};

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

const _INPUT: &str = r#"
= main
== sub
== another sub
hello world
goodbye world

hello world
[code myfile.js]

how do you do
[end]
"#;

const INPUT: &str = r#"
choose something
* red
* green
* blue
"#;

fn main() {
    let parse = Lexer::parse(Rule::lines, INPUT).expect("unsuccessful parse").next().unwrap();

    for line in parse.into_inner() {
        match line.as_rule() {
            Rule::topic_block => {
                println!("topic_block");
                parse_topic_block(line);
            },
            Rule::choice_block => {
                println!("choice_block");
                parse_choice_block(line);
            },
            Rule::command_block => {
                println!("command_block");
                parse_command_block(line);
            },
            Rule::utterance_block => {
                println!("utterance_block"); 
                parse_utterance_block(line);
            },
            _ => {}
        }
    }
}

fn parse_topic_block(rule: Pair<Rule>) {
    let mut it = rule.into_inner();
    let header_depth = it.next().unwrap().as_str().len();
    let header_identifier: &str = it.next().unwrap().as_str();

    println!("header: depth {}, identifier: {}", header_depth, header_identifier);
}

fn parse_choice_block(rule: Pair<Rule>) {
    for choice_line in rule.into_inner() {
        let mut it = choice_line.into_inner();
        let content = it.next().unwrap().as_str();
        println!("choice_line {}", content);
    }
}

fn parse_command_block(rule: Pair<Rule>) {
    for pair in rule.into_inner() {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::end_command_body => { println!("> end command"); },
                Rule::code_command_body => { println!("> code command"); },
                _ => {}
            }
        }
    }
}

fn parse_utterance_block(rule: Pair<Rule>) {
    let text = rule.as_span().as_str().replace("\n", " ");
    println!("utterance {}", text);
}
