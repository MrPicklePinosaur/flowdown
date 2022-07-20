// voiceflow diagram types

use std::collections::HashMap;
use serde::Serialize;
use serde_json::{json, Value};

use crate::{blocks::*, parser::Conversation};

const START_NODE_ID: &'static str = "start00000000000000000000";

pub struct VFConfig {
    pub project_name: String
}

pub fn serialize_vf_file(config: &VFConfig, conv: &Conversation) -> Value {
    let version_id = generate_id();
    let main_diagram_id = generate_id();

    let mut diagrams = json!({});
    diagrams[&main_diagram_id] = serialize_conversation(&main_diagram_id, &version_id, conv);

    let mut vf_file = json!({
        "_version": "1.2",
        "project": {
            "_id": generate_id(),
            "name": config.project_name,
            "teamID": "",
            "devVersion": version_id,
            "platform": "chatbot",
            "platformData": {
                "invocationName": "template project general"
            },
            "members": [],
            "linkType": "STRAIGHT",
            "image": "",
            "_version": 1.2,
            "customThemes": [],
            "creatorID": 0,
            "createdAt": ""
        },
        "version": {
            "_id": version_id,
            "variables": [],
            "platformData": {
                 "slots": [],
                 "intents": [],
                 "settings": {
                     "restart": true,
                     "repeat": 100,
                     "locales": [
                         "en-US"
                     ],
                     "defaultVoice": "Alexa"
                 },
                 "publishing": {},
                 "platform": "general"
            },
            "name": "Initial Version",
            "projectID": generate_id(),
            "manualSave": false,
            "autoSaveFromRestore": false,
            "rootDiagramID": main_diagram_id,
            "creatorID": 0,
            "_version": 2.2,
            "components": [],
            "topics": [
                {
                    "sourceID": main_diagram_id,
                     "type": "DIAGRAM"
                }
            ],
            "prototype": {
                "data": {
                    "name": "Untitled",
                    "locales": [
                        "en-US"
                    ]
                },
                "model": {
                    "intents": [],
                    "slots": []
                },
                "context": {
                    "stack": [
                        {
                            "programID": main_diagram_id,
                            "storage": {},
                            "variables": {}
                        }
                    ],
                    "variables": {}
                },
                "settings": {},
                "platform": "chatbot"
            },
        },
        "diagrams": diagrams,
    });

    vf_file
}


fn serialize_conversation(diagram_id: &str, version_id: &str, conv: &Conversation) -> Value {
    json!({
        "_id": diagram_id,
        "offsetX": 0,
        "offsetY": 0,
        "zoom": 100,
        "variables": [],
        "name": "ROOT",
        "versionID": version_id,
        "creatorID": 0,
        "modified": 0,
        "nodes": serialize_nodes(conv.blocks()),
        "children": [],
        "type": "TOPIC"
    })
}

fn serialize_nodes(blocks: &Vec<Block>) -> Value {
    let start_node = start_block();
    let start_node_id = get_node_id(&start_node).unwrap();
    let mut nodes = json!({
        &start_node_id: start_node
    });

    let mut prev_node_id: String = start_node_id.to_owned();
    for block in blocks.iter() {
        let mut new_step = serialize_step(block);
        let step_id = get_node_id(&new_step).unwrap();

        // make previous node point to the new node
        if let Value::Array(ports) = &mut nodes[&prev_node_id]["data"]["ports"] {
            ports.push(serialize_port(&step_id));
        }

        nodes[&step_id] = new_step; // TODO pretty bad to clone this

        // create the block for the step
        let new_block = serialize_block(&step_id);
        let block_id = get_node_id(&new_block).unwrap();
        nodes[&block_id] = new_block;

        prev_node_id = step_id;
    }

    nodes
}

fn start_block() -> Value {
    json!({
        "nodeID": START_NODE_ID,
        "type": "start",
        "data": {
            "steps": [],
            "ports": []
        },
        "coords": [0, 0],
    })
}

fn serialize_block(step_id: &str) -> Value {
    json!({
        "nodeID": generate_id(),
        "type": "block",
        "data": {
            "name": "",
            "steps": [step_id]
        },
        "coords": [0, 0]
    })
}

fn serialize_step(block: &Block) -> Value {
    let mut node = match block {
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
        "id": generate_id(),
        "data": {
            "points": []
        }
    })
}

fn generate_id() -> String {
    use rand::{seq::SliceRandom, Rng, thread_rng};

    const ID_LEN: u32 = 24;
    let alphabet: Vec<char> = ('0'..='9').chain('a'..='f').collect();

    (0..ID_LEN).map(|_| alphabet.choose(&mut thread_rng()).unwrap()).collect::<String>()
}

fn get_node_id(value: &Value) -> Option<String> {
    if let Value::String(id) = &value["nodeID"] {
        Some(id.clone())
    } else {
        None
    }
}
