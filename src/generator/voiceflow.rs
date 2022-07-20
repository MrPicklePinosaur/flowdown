// voiceflow diagram types

use std::collections::HashMap;
use serde::Serialize;
use serde_json::{json, Value};

use crate::blocks::*;

pub fn generate_id() -> String {
    use rand::{seq::SliceRandom, Rng, thread_rng};

    const ID_LEN: u32 = 24;
    let alphabet: Vec<char> = ('0'..='9').chain('a'..='f').collect();

    (0..ID_LEN).map(|_| alphabet.choose(&mut thread_rng()).unwrap()).collect::<String>()
}

fn serialize_vf_file() -> Value {
    json!({
        "_version": "1.2",
        "project": {},
        "version": {
            "variables": [],
            "platformData": {
                "slots": [],
                "intents": [],
            }
        },
        "diagrams": [
        ]
    })
}

pub fn serialize_diagram() -> Value {
    json!({
        "offsetX": 0,
        "offsetY": 0,
        "zoom": 100,
        "variables": [],
        "nodes": []
    })
}

pub fn serialize_blocks(blocks: &Vec<Block>) -> Value {
    let mut nodes = json!({});

    let mut prev_node: Option<String> = None;
    for block in blocks.iter() {
        let mut new_block = serialize_block(block);
        let block_id = new_block["nodeID"].as_str().unwrap().to_owned();

        if let Some(node_id) = prev_node {
            if let Value::Array(ports) = &mut new_block["data"]["ports"] {
                ports.push(serialize_port(&node_id));
            }
        }

        nodes[&block_id] = new_block; // TODO pretty bad to clone this

        prev_node = Some(block_id);
    }

    nodes
}

pub fn serialize_block(block: &Block) -> Value {
    let mut node = match block {
        Block::Start => json!({
            "type": "start",
            "data": {
                "ports": [],
            }
        }),
        Block::Utterance { content } => json!({
            "type": "speak",
            "data": {
                "randomize": true,
                "canvasVisibility": "preview",
                "dialogs": [
                    {
                        "voice": "Alexa",
                        "content": content
                    }
                ],
                "ports": [],
            }
        }),
        Block::EndCommand => json!({
            "type": "end",
            "data": {
                "ports": [],
            }
        })
    };
    node["nodeID"] = Value::String(generate_id());

    node
}

fn serialize_port(target: &str) -> Value {
    json!({
        "type": "next",
        "target": target,
        "id": "",
        "data": {
            "points": []
        }
    })
}
