use log::info;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use serde::ser::Error;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::blocks::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

pub struct Dialog {
    bookmark_table: HashMap<String, Option<u32>>,
    blocks: Vec<Block>,
}

impl Dialog {
    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    // actual bookmark definition
    pub fn define_bookmark(&mut self, bookmark_name: &str) -> u32 {
        let line_number = self.blocks.len() as u32;
        if self
            .bookmark_table
            .insert(bookmark_name.to_owned(), Some(line_number))
            .is_some()
        {
            // TODO should error if already exist
        }
        info!("bookmark added at line {}", line_number);
        line_number
    }

    // reference a bookmark without defining it
    pub fn mention_bookmark(&mut self, bookmark_name: &str) {
        if self.bookmark_table.contains_key(bookmark_name) {
            return;
        }
        self.bookmark_table.insert(bookmark_name.to_owned(), None);
    }

    // check if all referenced bookmarks have been defined
    pub fn is_valid(&self) -> bool {
        self.bookmark_table
            .iter()
            .find(|(_, v)| v.is_none())
            .is_some()
    }
}

impl Debug for Dialog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversation\n{:?}", self.blocks)
    }
}

impl Dialog {
    pub fn new() -> Self {
        Dialog {
            bookmark_table: HashMap::new(),
            blocks: Vec::new(),
        }
    }
}

pub struct FlowdownParser {
    dialog_table: HashMap<String, Dialog>,
    variables: Vec<String>,
    _cur_conv: String,
}

impl FlowdownParser {
    pub fn new() -> Self {
        // TODO insert 'main' dialog to dialog_table
        FlowdownParser {
            dialog_table: HashMap::new(),
            variables: Vec::new(),
            _cur_conv: "main".into(),
        }
    }

    pub fn parse(&mut self, input: &str) {
        let parsed = Lexer::parse(Rule::diagram, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();
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

        if stmt.as_rule() == Rule::dialog_stmt {
            self.parse_dialog_stmt(stmt);
        } else if stmt.as_rule() == Rule::bookmark_stmt {
            self.parse_bookmark_stmt(stmt);
        } else {
            // these all evaluate to blocks
            let block = match stmt.as_rule() {
                Rule::command_stmt => self.parse_command_stmt(stmt),
                Rule::jump_stmt => self.parse_jump_stmt(stmt),
                Rule::utterance_stmt => self.parse_utterance_stmt(stmt),
                _ => unreachable!(),
            };
            self.cur_dialog_mut().blocks.push(block);
        }
    }

    fn parse_dialog_stmt(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::dialog_stmt);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        info!("dialog_stmt {}", id);

        self.new_dialog(id);
    }

    fn parse_bookmark_stmt(&mut self, pair: Pair<Rule>) {
        assert!(pair.as_rule() == Rule::bookmark_stmt);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        info!("bookmark {}", id);
        self.cur_dialog_mut().define_bookmark(id);
    }

    fn parse_command_stmt(&mut self, pair: Pair<Rule>) -> Block {
        assert!(pair.as_rule() == Rule::command_stmt);

        let mut it = pair.into_inner();
        let command_stmt = it.next().unwrap();
        match command_stmt.as_rule() {
            Rule::end_command_body => {
                info!("end command");
                Block::EndCommand
            }
            Rule::set_command_body => {
                info!("set command");
                let mut it = command_stmt.into_inner();
                let variable = it.next().unwrap().as_str().to_owned();
                let value = it.next().unwrap().as_str().to_owned();
                self.mention_variable(&variable);
                Block::SetCommand { variable, value }
            }
            Rule::capture_command_body => {
                info!("capture command");
                let mut it = command_stmt.into_inner();
                let variable = it.next().unwrap().as_str().to_owned();
                self.mention_variable(&variable);
                Block::CaptureCommand { variable }
            }
            _ => unreachable!(),
        }
    }

    fn parse_jump_stmt(&mut self, pair: Pair<Rule>) -> Block {
        assert!(pair.as_rule() == Rule::jump_stmt);

        let mut it = pair.into_inner();
        let target = it.next().unwrap().as_str().to_owned();
        info!("jump_stmt {}", target);
        self.mention_variable(&target);

        Block::Jump { target }
    }

    fn parse_utterance_stmt(&mut self, pair: Pair<Rule>) -> Block {
        assert!(pair.as_rule() == Rule::utterance_stmt);

        let mut it = pair.into_inner();
        let content = it.next().unwrap().as_str();
        let mut voice: Option<String> = None;

        if let Some(voice_option) = it.next() {
            voice = Some(voice_option.as_str().to_owned());
        }

        info!("utterance_stmt {}", content);

        Block::Utterance {
            content: content.into(),
            voice,
        }
    }

    fn new_dialog(&mut self, name: &str) {
        if self.dialog_table.contains_key(name) {
            // TODO should error
            return;
        }
        self.dialog_table.insert(name.into(), Dialog::new());
        self._cur_conv = name.into();
    }

    pub fn cur_dialog(&self) -> &Dialog {
        self.dialog_table.get(&self._cur_conv).unwrap()
    }

    fn cur_dialog_mut(&mut self) -> &mut Dialog {
        self.dialog_table.get_mut(&self._cur_conv).unwrap()
    }

    pub fn variables(&self) -> &Vec<String> {
        &self.variables
    }

    fn mention_variable(&mut self, variable_name: &str) {
        self.variables.push(variable_name.to_owned());
    }
}

impl Debug for FlowdownParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.dialog_table.iter()).finish()
    }
}
