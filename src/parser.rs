
use std::collections::HashMap;
use std::fmt::Debug;

use pest::{Parser, iterators::{Pair, Pairs}};
use crate::blocks::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

#[derive(Default)]
struct Bookmark {

}

struct Conversation {
    bookmark_table: HashMap<String, Bookmark>,
    blocks: Vec<Block>,
}

impl Debug for Conversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversation\n{:?}", self.blocks)
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

pub struct FlowdownParser {
    conv_table: HashMap<String, Conversation>,
    _cur_conv: String,
}

impl FlowdownParser {

    pub fn new() -> Self {
        // TODO insert 'main' conversation to conversation_table
        FlowdownParser {
            conv_table: HashMap::new(),
            _cur_conv: "main".into(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        let parsed = Lexer::parse(Rule::diagram, input).expect("unsuccessful parse").next().unwrap();
        self.parse_diagram(parsed);
    }

    fn parse_diagram(&mut self, pair: Pair<Rule>) {
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

    fn parse_command_stmt(&mut self, pair: Pair<Rule>) -> Block {
        assert!(pair.as_rule() == Rule::command_stmt);

        let mut it = pair.into_inner();
        let command_stmt = it.next().unwrap();
        match command_stmt.as_rule() {
            Rule::end_command_body => {
                println!("end command");
                Block::EndCommand
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

impl Debug for FlowdownParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.conv_table.iter()).finish()
    }
}
