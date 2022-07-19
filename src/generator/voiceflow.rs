// voiceflow diagram types

use std::collections::HashMap;
use serde::Serialize;
use serde_json::{json, Value};

use crate::blocks::*;

pub fn generate_id() -> String {
    String::new()
}

pub fn serialize_vf_file() -> Value {
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

pub fn serialize_block(block: Block) -> Value {
    match block {
        Block::Start => json!({
            "ports": [],
        }),
        Block::Utterance { content } => json!({
            "randomize": true,
            "canvasVisibility": "preview",
            "dialogs": [
                {
                    "voice": "Alexa",
                    "content": content
                }
            ],
            "ports": [],
        }),
        Block::EndCommand => json!({
        })
    }
}
