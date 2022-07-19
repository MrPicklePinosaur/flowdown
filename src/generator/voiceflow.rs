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
    lines: HashMap<String, Box<dyn BaseLine>>,
}

pub trait BaseLine {}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartLine {
    id: String,
    line_type: String,
    next_id: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeakLine {
    id: String,
    line_type: String,
    next_id: Option<String>,
    random_speak: Vec<String>
}

// impl From<Block> for Line {
//     fn from(block: Block) -> Self {
//         let mut line = Line::new();

//         match block {
//             Block::Start => 
//         }
//     }
// }

pub fn generate_id() -> String {
    String::new()
}
