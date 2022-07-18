mod generator;
mod blocks;

use std::collections::HashMap;
use pest::{Parser, iterators::{Pair, Pairs}};

use crate::blocks::*;

extern crate pest;
#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

const INPUT: &str = r#"


@ conversation1
[end]
[end]
[end]


[end]

@ conversation2
[end]

"#;

fn main() {
    let parse = Lexer::parse(Rule::diagram, INPUT).expect("unsuccessful parse").next().unwrap();
    let mut compiler = Compiler::new();
    compiler.parse_diagram(parse);
}

#[derive(Default)]
struct Bookmark {

}

struct Conversation {
    bookmark_table: HashMap<String, Bookmark>,
    blocks: Vec<Box<dyn Block>>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            bookmark_table: HashMap::new(),
            blocks: Vec::new()
        }
    }
}

struct Compiler {
    conv_table: HashMap<String, Conversation>,
    _cur_conv: String,
}

impl Compiler {

    pub fn new() -> Self {
        // TODO insert 'main' conversation to conversation_table
        Compiler {
            conv_table: HashMap::new(),
            _cur_conv: "main".into(),
        }
    }

    pub fn parse_diagram(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::diagram);

        for line in pair.into_inner() {

            if line.as_rule() == Rule::block {
                self.parse_block(line);
            }
        }
    }

    fn parse_block(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::block);

        let mut it = pair.into_inner();
        let block = it.next().unwrap();
        match block.as_rule() {
            Rule::conversation_block => {
                self.parse_conversation_block(block);
            },
            Rule::command_block => {
                let cmd_block = self.parse_command_block(block);
                self.cur_conv_mut().blocks.push(cmd_block);
            },
            _ => {}
        }
    }

    fn parse_conversation_block(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::conversation_block);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        println!("conversation_block {}", id);

        self.new_conv(id);

    }

    fn parse_command_block(&mut self, pair: Pair<Rule>) -> Box<dyn Block> {
        assert!(pair.as_rule() == Rule::command_block);

        let mut it = pair.into_inner();
        let command_block = it.next().unwrap();
        match command_block.as_rule() {
            Rule::end_command_body => {
                println!("end command");
                Box::new(EndCommandBlock {})
            },
            _ => unreachable!()
        }
    }

    fn new_conv(&mut self, name: &str) {
        if self.conv_table.contains_key(name) {
            // TODO should error
            return;
        }
        self.conv_table.insert(name.into(), Conversation::new());
        self._cur_conv = name.into();
    }

    fn cur_conv(&self) -> &Conversation {
        self.conv_table.get(&self._cur_conv).unwrap()
    }

    fn cur_conv_mut(&mut self) -> &mut Conversation {
        self.conv_table.get_mut(&self._cur_conv).unwrap()
    }
}
