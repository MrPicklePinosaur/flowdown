use log::info;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use serde::ser::Error;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::{blocks::*, error::*};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Lexer;

#[derive(Debug)]
pub struct Dialog {
    pub bookmark_table: HashMap<String, u32>,
    pub blocks: Vec<Block>,
}

struct DialogBuilder {
    bookmark_table: HashMap<String, Option<u32>>,
    blocks: Vec<Block>,
}

impl DialogBuilder {
    // actual bookmark definition
    fn define_bookmark(&mut self, bookmark_name: &str) -> u32 {
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
    fn mention_bookmark(&mut self, bookmark_name: &str) {
        if self.bookmark_table.contains_key(bookmark_name) {
            return;
        }
        self.bookmark_table.insert(bookmark_name.to_owned(), None);
    }

    fn build(mut self) -> Result<Dialog> {
        let undefined = self.undefined_bookmarks();
        if !undefined.is_empty() {
            return Err(FlowdownError::UndefinedBookmark(undefined));
        }

        let bookmark_table =
            HashMap::from_iter(self.bookmark_table.drain().map(|(k, v)| (k, v.unwrap())));
        Ok(Dialog {
            bookmark_table,
            blocks: self.blocks,
        })
    }

    // check if all referenced bookmarks have been defined
    fn undefined_bookmarks(&self) -> Vec<String> {
        self.bookmark_table
            .iter()
            .filter(|(_, v)| v.is_none())
            .map(|(k, _)| k.to_owned())
            .collect()
    }
}

impl Debug for DialogBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conversation\n{:?}", self.blocks)
    }
}

impl DialogBuilder {
    pub fn new() -> Self {
        DialogBuilder {
            bookmark_table: HashMap::new(),
            blocks: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Conversation {
    pub dialog_table: HashMap<String, Dialog>,
    pub variables: Vec<String>,
}

pub struct ConversationBuilder {
    dialog_table: HashMap<String, DialogBuilder>,
    variables: Vec<String>,
    _cur_dialog: String,
}

impl ConversationBuilder {
    pub fn new() -> Self {
        // create the main dialog by default
        let mut dialog_table = HashMap::new();
        dialog_table.insert("main".into(), DialogBuilder::new());

        ConversationBuilder {
            dialog_table,
            variables: Vec::new(),
            _cur_dialog: "main".into(),
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
        self.cur_dialog_mut().mention_bookmark(&target);

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
        self.dialog_table.insert(name.into(), DialogBuilder::new());
        self._cur_dialog = name.into();
    }

    fn cur_dialog(&self) -> &DialogBuilder {
        self.dialog_table.get(&self._cur_dialog).unwrap()
    }

    fn cur_dialog_mut(&mut self) -> &mut DialogBuilder {
        self.dialog_table.get_mut(&self._cur_dialog).unwrap()
    }

    fn mention_variable(&mut self, variable_name: &str) {
        self.variables.push(variable_name.to_owned());
    }

    pub fn build(self) -> Result<Conversation> {
        // build all dialogs
        let mut dialog_table = HashMap::<String, Dialog>::new();
        for (id, dialog_builder) in self.dialog_table.into_iter() {
            let dialog = dialog_builder.build()?;
            dialog_table.insert(id, dialog);
        }

        Ok(Conversation {
            dialog_table,
            variables: self.variables,
        })
    }
}

impl Debug for ConversationBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.dialog_table.iter()).finish()
    }
}
