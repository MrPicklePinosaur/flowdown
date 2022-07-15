use serde::Serialize;

#[derive(Serialize)]
pub struct Program {
    id: String,
    name: String,
    start_id: String,
    version_id: String,
    variables: Vec<()>,
    commands: Vec<()>,
    lines: Vec<Block>,
}

#[derive(Serialize)]
pub enum BlockType {
    Start,
    Speak,
    Interaction,
    Exit,
}

#[derive(Serialize)]
pub struct Block {
    id: String,
    next_id: Option<String>,
    block_type: BlockType,
}

pub fn generate_id() -> String {
    String::new()
}

pub fn start() -> Block {
    Block {
        id: generate_id(),
        next_id: None,
        block_type: BlockType::Start
    }
}

// pub fn speak() -> Block {
// }
