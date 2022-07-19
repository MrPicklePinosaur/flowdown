// voiceflow diagram types

use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use serde::Serialize;

use crate::blocks::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Program {
    id: String,
    name: String,
    start_id: String,
    version_id: String,
    variables: Vec<()>,
    commands: Vec<()>,
    lines: HashMap<String, Line>,
}

type _Line = HashMap<String, Box<dyn Any>>;
pub struct Line(_Line);

impl Line {
    pub fn new() -> Self {
        let id: Box<dyn Any> = Box::new(generate_id());
        let next_id: Box<dyn Any> = Box::new(None as Option<String>);

        let new_line_map: _Line = HashMap::from([
            ("id".into(), id),
            ("next_id".into(), next_id)
        ]);
        Line(new_line_map)
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

        match block {
            Block::Start => 
        }
    }
}

pub fn generate_id() -> String {
    String::new()
}
