// voiceflow diagram types

use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashMap, hash::Hash};

use crate::{blocks::*, parser::*};

const START_NODE_ID: &'static str = "start00000000000000000000";
const DEFAULT_VOICE: &'static str = "Alexa";
const ENTRY_DIALOG: &'static str = "main";

pub struct VFConfig {
    pub project_name: String,
}

// some state data passed during compilation
/*
struct State {
    diagram_name: String,
}

impl State {
    fn is_main(&self) -> bool {

    }
}
*/

pub struct VFCompiler {
    config: VFConfig,
    // map from dialog name to diagram id
    dialog_symbols: HashMap<String, String>,
    // map from (dialog name, line number) to block id
    block_symbols: HashMap<(String, u32), String>,
}

struct State {
    dialog_name: String,
}

impl State {
    fn is_main(&self) -> bool {
        self.dialog_name.eq(ENTRY_DIALOG)
    }
}

impl VFCompiler {
    pub fn new(config: VFConfig) -> Self {
        VFCompiler {
            config,
            dialog_symbols: HashMap::new(),
            block_symbols: HashMap::new(),
        }
    }

    fn main_diagram_id(&self) -> &str {
        self.dialog_symbols.get(ENTRY_DIALOG).unwrap()
    }

    fn diagram_id(&self, dialog_name: &str) -> &str {
        // danger unwrapping?
        self.dialog_symbols.get(dialog_name).unwrap()
    }

    pub fn serialize_vf_file(&mut self, conv: &Conversation, variables: &Vec<String>) -> Value {
        let version_id = generate_id();

        // compile each diagram
        let mut diagrams = json!({});
        for (name, dialog) in conv.dialog_table.iter() {
            let id = generate_id();
            self.dialog_symbols.insert(name.to_owned(), id.to_owned());

            let new_state = State {
                dialog_name: name.to_owned(),
            };
            diagrams[&id] = self.serialize_dialog(&new_state, &version_id, dialog);
        }

        let mut vf_file = json!({
            "_version": "1.2",
            "project": {
                "_id": generate_id(),
                "name": self.config.project_name,
                "teamID": "",
                "devVersion": version_id,
                "platform": "general",
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
                "variables": variables,
                "platformData": {
                     "slots": [],
                     "intents": [],
                     "settings": {
                         "restart": true,
                         "repeat": 100,
                         "locales": [
                             "en-US"
                         ],
                         "defaultVoice": DEFAULT_VOICE,
                     },
                     "publishing": {},
                     "platform": "voice_default"
                },
                "name": "Initial Version",
                "projectID": generate_id(),
                "manualSave": false,
                "autoSaveFromRestore": false,
                "rootDiagramID": self.main_diagram_id(),
                "creatorID": 0,
                "_version": 2.2,
                "components": [],
                "topics": [
                    {
                        "sourceID": self.main_diagram_id(),
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
                                "programID": self.main_diagram_id(),
                                "storage": {},
                                "variables": {}
                            }
                        ],
                        "variables": {}
                    },
                    "settings": {},
                    "platform": "general"
                },
            },
            "diagrams": diagrams,
        });

        vf_file
    }

    fn serialize_dialog(&mut self, state: &State, version_id: &str, dialog: &Dialog) -> Value {
        json!({
            "_id": self.diagram_id(&state.dialog_name),
            "offsetX": 0,
            "offsetY": 0,
            "zoom": 100,
            "variables": [],
            "name": if state.is_main() { "ROOT" } else { &state.dialog_name },
            "versionID": version_id,
            "creatorID": 0,
            "modified": 0,
            "nodes": self.serialize_nodes(&state.dialog_name, &dialog.blocks),
            "children": [],
            "type": if state.is_main() { "TOPIC" } else { "COMPONENT" }
        })
    }

    fn serialize_nodes(&mut self, diagram_name: &str, blocks: &Vec<Block>) -> Value {
        // generate start node id (if in main diagram, it's id must be a specific value)
        let start_node_id = if diagram_name.eq(ENTRY_DIALOG) {
            START_NODE_ID.to_owned()
        } else {
            generate_id()
        };
        let start_node = self.start_block(&start_node_id);
        let mut nodes = json!({ &start_node_id: start_node });

        let mut prev_node_id: String = start_node_id.to_owned();
        for block in blocks.iter() {
            let mut new_step = self.serialize_step(block);
            let step_id = get_node_id(&new_step).unwrap();

            // make previous node point to the new node
            if let Value::Array(ports) = &mut nodes[&prev_node_id]["data"]["ports"] {
                ports.push(self.serialize_port(&step_id));
            }

            nodes[&step_id] = new_step; // TODO pretty bad to clone this

            // create the block for the step
            let new_block = self.serialize_block(&step_id);
            let block_id = get_node_id(&new_block).unwrap();
            nodes[&block_id] = new_block;

            prev_node_id = step_id;
        }

        nodes
    }

    fn start_block(&mut self, id: &str) -> Value {
        json!({
            "nodeID": id,
            "type": "start",
            "data": {
                "steps": [],
                "ports": []
            },
            "coords": [0, 0],
        })
    }

    fn serialize_block(&mut self, step_id: &str) -> Value {
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

    fn serialize_step(&mut self, block: &Block) -> Value {
        let mut node = match block {
            Block::Jump { target } => json!({
                /* empty speak is a NOOP */
                "type": "speak",
                "data": {
                    "randomize": true,
                    "canvasVisibility": "preview",
                    "dialogs": [],
                    "ports": [],
                }
            }),
            Block::Utterance { content, voice } => json!({
                "type": "speak",
                "data": {
                    "randomize": true,
                    "canvasVisibility": "preview",
                    "dialogs": [
                        {
                            "voice": if let Some(voice) = voice { voice } else { DEFAULT_VOICE },
                            "content": content,
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
            }),
            Block::SetCommand {
                variable: id,
                value,
            } => json!({
                "type": "setV2",
                "data": {
                    "sets": [
                        {
                            "type": "value",
                            "variable": id,
                            "expression": value
                        }
                    ],
                    "ports": []
                }
            }),
            Block::CaptureCommand { variable } => json!({
                "type": "captureV2",
                "data": {
                    "intentScope": "GLOBAL",
                    "capture": {
                        "type": "query",
                        "variable": variable,
                    },
                    "noReply": null,
                    "noMatch": null,
                    "chips": null,
                    "ports": []
                },
            }),
        };
        node["nodeID"] = Value::String(generate_id());

        node
    }

    fn serialize_port(&mut self, target: &str) -> Value {
        json!({
            "type": "next",
            "target": target,
            "id": generate_id(),
            "data": {
                "points": []
            }
        })
    }
}

// couple of helpers

fn generate_id() -> String {
    use rand::{seq::SliceRandom, thread_rng, Rng};

    const ID_LEN: u32 = 24;
    let alphabet: Vec<char> = ('0'..='9').chain('a'..='f').collect();

    (0..ID_LEN)
        .map(|_| alphabet.choose(&mut thread_rng()).unwrap())
        .collect::<String>()
}

fn get_node_id(value: &Value) -> Option<String> {
    if let Value::String(id) = &value["nodeID"] {
        Some(id.clone())
    } else {
        None
    }
}
