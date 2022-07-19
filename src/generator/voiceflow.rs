// voiceflow diagram types

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use serde::Serialize;

use crate::blocks::*;
use flowdown_derive::base_line;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    id: String,
    name: String,
    start_id: String,
    version_id: String,
    variables: Vec<()>,
    commands: Vec<()>,
    // lines: HashMap<String, Line>,
}

#[base_line]
pub struct StartLine;

type _Line = HashMap<String, String>;
pub struct Line(_Line);

impl Line {
    pub fn new() -> Self {
        Line(HashMap::new())
    } 
}

impl Deref for Line {
    type Target = _Line;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Block> for Line {
    fn from(block: Block) -> Self {
        let mut line = Line::new();

        // match block {
        //     Block::Start => 
        // }
    }
}

pub fn generate_id() -> String {
    String::new()
}
