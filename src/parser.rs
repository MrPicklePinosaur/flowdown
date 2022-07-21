use log::info;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use serde::ser::Error;
use std::fmt::Debug;
use std::{collections::HashMap, fs};

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
    fn define_bookmark(&mut self, bookmark_name: &str) -> Result<u32> {
        let line_number = self.blocks.len() as u32;

        // check if bookmark already defined
        if let Some(v) = self.bookmark_table.get(bookmark_name) {
            if v.is_some() {
                return Err(FlowdownError::BookmarkAlreadyDefined(
                    bookmark_name.to_owned(),
                ));
            }
        }
        self.bookmark_table
            .insert(bookmark_name.to_owned(), Some(line_number));
        info!("bookmark added at line {}", line_number);
        Ok(line_number)
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

    pub fn parse(&mut self, input: &str) -> Result<()> {
        let parsed = Lexer::parse(Rule::diagram, input)
            .expect("unsuccessful parse")
            .next()
            .unwrap();
        self.parse_diagram(parsed)?;
        Ok(())
    }

    fn parse_diagram(&mut self, pair: Pair<Rule>) -> Result<()> {
        assert!(pair.as_rule() == Rule::diagram);

        for line in pair.into_inner() {
            if line.as_rule() == Rule::stmt {
                self.parse_stmt(line)?;
            }
        }

        Ok(())
    }

    fn parse_stmt(&mut self, pair: Pair<Rule>) -> Result<()> {
        assert!(pair.as_rule() == Rule::stmt);

        let mut it = pair.into_inner();
        let stmt = it.next().unwrap();

        if stmt.as_rule() == Rule::dialog_stmt {
            self.parse_dialog_stmt(stmt)?;
        } else if stmt.as_rule() == Rule::bookmark_stmt {
            self.parse_bookmark_stmt(stmt)?;
        } else {
            // these all evaluate to blocks
            let block = match stmt.as_rule() {
                Rule::command_stmt => self.parse_command_stmt(stmt)?,
                Rule::jump_stmt => self.parse_jump_stmt(stmt),
                Rule::utterance_stmt => self.parse_utterance_stmt(stmt),
                _ => unreachable!(),
            };
            self.cur_dialog_mut().blocks.push(block);
        }
        Ok(())
    }

    fn parse_dialog_stmt(&mut self, pair: Pair<Rule>) -> Result<()> {
        assert!(pair.as_rule() == Rule::dialog_stmt);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        info!("dialog_stmt {}", id);

        self.new_dialog(id)?;
        Ok(())
    }

    fn parse_bookmark_stmt(&mut self, pair: Pair<Rule>) -> Result<()> {
        assert!(pair.as_rule() == Rule::bookmark_stmt);

        let mut it = pair.into_inner();
        let id = it.next().unwrap().as_str();
        info!("bookmark {}", id);
        self.cur_dialog_mut().define_bookmark(id)?;
        Ok(())
    }

    fn parse_command_stmt(&mut self, pair: Pair<Rule>) -> Result<Block> {
        assert!(pair.as_rule() == Rule::command_stmt);

        let mut it = pair.into_inner();
        let command_stmt = it.next().unwrap();
        match command_stmt.as_rule() {
            Rule::end_command_body => {
                info!("end command");
                Ok(Block::EndCommand)
            }
            Rule::set_command_body => {
                info!("set command");
                let mut it = command_stmt.into_inner();
                let variable = strip_variable(it.next().unwrap().as_str());
                let value = strip_string_literal(it.next().unwrap().as_str());
                self.mention_variable(&variable);
                Ok(Block::SetCommand {
                    variable: variable.to_owned(),
                    value: value.to_owned(),
                })
            }
            Rule::capture_command_body => {
                info!("capture command");
                let mut it = command_stmt.into_inner();
                let variable = strip_variable(it.next().unwrap().as_str());
                self.mention_variable(&variable);
                Ok(Block::CaptureCommand {
                    variable: variable.to_owned(),
                })
            }
            Rule::code_command_body => {
                use std::fs::read_to_string;

                // attempt to read contents of file
                let code_path = command_stmt
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .to_owned();
                info!("code command: {}", code_path);
                let body = read_to_string(&code_path)
                    .map_err(|_| FlowdownError::CannotReadCodeFile(code_path))?;

                Ok(Block::CodeCommand { body })
            }
            _ => unreachable!(),
        }
    }

    fn parse_jump_stmt(&mut self, pair: Pair<Rule>) -> Block {
        assert!(pair.as_rule() == Rule::jump_stmt);

        let mut it = pair.into_inner();
        let target = it.next().unwrap();
        match target.as_rule() {
            Rule::dialog_identifier => {
                let dialog_id = target.into_inner().next().unwrap().as_str().to_owned();
                // TODO send 'mention dialog' to catch undefined dialog errors
                info!("jmp_stmt dialog @{}", dialog_id);
                Block::Jump {
                    target: JumpTarget::Dialog(dialog_id),
                }
            }
            Rule::bookmark_identifier => {
                let bookmark_id = target.as_str().to_owned();
                self.cur_dialog_mut().mention_bookmark(&bookmark_id);
                info!("jump_stmt bookmark {}", &bookmark_id);
                Block::Jump {
                    target: JumpTarget::Bookmark(bookmark_id),
                }
            }
            _ => unreachable!(),
        }
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

    fn new_dialog(&mut self, name: &str) -> Result<()> {
        if self.dialog_table.contains_key(name) {
            return Err(FlowdownError::DialogAlreadyDefined(name.to_owned()));
        }
        self.dialog_table.insert(name.into(), DialogBuilder::new());
        self._cur_dialog = name.into();
        Ok(())
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

fn strip_variable(variable: &str) -> &str {
    assert!(variable.starts_with("$"));
    variable.strip_prefix("$").unwrap()
}

fn strip_string_literal(string_literal: &str) -> &str {
    assert!(string_literal.starts_with("\"") && string_literal.ends_with("\""));
    string_literal
        .strip_prefix("\"")
        .unwrap()
        .strip_suffix("\"")
        .unwrap()
}
