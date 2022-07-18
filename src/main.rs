mod generator;
mod blocks;

use std::collections::HashMap;
use std::fmt::Debug;
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

    println!("{:?}", compiler);
}

#[derive(Default)]
struct Bookmark {

}

struct Conversation {
    bookmark_table: HashMap<String, Bookmark>,
    blocks: Vec<Box<dyn Block>>,
}

impl Debug for Conversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversation with {} blocks", self.blocks.len())
    }
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

            if line.as_rule() == Rule::stmt {
                self.parse_stmt(line);
            }
        }
    }

    fn parse_stmt(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::stmt);

        let mut it = pair.into_inner();
        let stmt = it.next().unwrap();

        if stmt.as_rule() == Rule::conversation_stmt {
            self.parse_conversation_stmt(stmt);
        } else {

            // these all evaluate to blocks
            let block = match stmt.as_rule() {
                Rule::command_stmt => {
                    self.parse_command_stmt(stmt)
                },
                _ => unreachable!()
            };
            self.cur_conv_mut().blocks.push(block);
        }
    }

    fn parse_conversation_stmt(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::conversation_stmt);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        println!("conversation_block {}", id);

        self.new_conv(id);

    }

    fn parse_command_stmt(&mut self, pair: Pair<Rule>) -> Box<dyn Block> {
        assert!(pair.as_rule() == Rule::command_stmt);

        let mut it = pair.into_inner();
        let command_stmt = it.next().unwrap();
        match command_stmt.as_rule() {
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

impl Debug for Compiler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.conv_table.iter()).finish()
    }
}
