// voiceflow diagram types

use log::debug;
use serde::Serialize;
use serde_json::{json, Value};
use std::{collections::HashMap, hash::Hash};

use crate::{blocks::*, parser::*};

const START_NODE_ID: &'static str = "start00000000000000000000";
const ENTRY_DIALOG: &'static str = "main";

pub struct VFConfig {
    pub project_name: String,
    pub default_voice: String,
}

impl Default for VFConfig {
    fn default() -> Self {
        VFConfig {
            project_name: "Untitled".into(),
            default_voice: "Alexa".into(),
        }
    }
}

pub struct VFCompiler {
    config: VFConfig,
    // map from dialog name to diagram id
    dialog_symbols: HashMap<String, String>,
    // map from (dialog name, line number) to block id
    block_symbols: HashMap<(String, u32), String>,
    // store (dialog name, block id) to the jump target to be relocated
    pub jump_relocation_table: HashMap<(String, String), JumpTarget>,
}

// state information to pass down compilation handlers
struct State {
    dialog_name: String,
}

impl State {
    fn is_main(&self) -> bool {
        self.dialog_name.eq(ENTRY_DIALOG)
    }
}

// return struct used by serialize step function
struct SerializedStep {
    // root node that connects to the previous connecting nodes
    root_step_id: String,
    // newly created nodes
    new_steps: Vec<Value>,
    // new nodes that connect to the next node
    connecting_node_ids: Vec<(String, PortType)>,
}

impl SerializedStep {
    pub fn new(
        root_step_id: String,
        new_steps: Vec<Value>,
        connecting_node_ids: Vec<(String, PortType)>,
    ) -> Self {
        SerializedStep {
            root_step_id,
            new_steps,
            connecting_node_ids,
        }
    }
}

impl VFCompiler {
    pub fn new(config: VFConfig) -> Self {
        VFCompiler {
            config,
            dialog_symbols: HashMap::new(),
            block_symbols: HashMap::new(),
            jump_relocation_table: HashMap::new(),
        }
    }

    fn main_diagram_id(&self) -> &str {
        self.dialog_symbols.get(ENTRY_DIALOG).unwrap()
    }

    fn diagram_id(&self, dialog_name: &str) -> &str {
        // danger unwrapping?
        self.dialog_symbols.get(dialog_name).unwrap()
    }

    pub fn compile(&mut self, conv: &Conversation, variables: &Vec<String>) -> Value {
        let mut serialized = self.serialize_vf_file(conv, variables);

        self.relocate(&mut serialized, conv);

        serialized
    }

    fn serialize_vf_file(&mut self, conv: &Conversation, variables: &Vec<String>) -> Value {
        let version_id = generate_id();

        // compile each diagram
        let mut diagrams = json!({});
        let mut components: Vec<Value> = Vec::new();
        for (name, dialog) in conv.dialog_table.iter() {
            let id = generate_id();
            self.dialog_symbols.insert(name.to_owned(), id.to_owned());

            let new_state = State {
                dialog_name: name.to_owned(),
            };
            diagrams[&id] = self.serialize_dialog(&new_state, &version_id, dialog);
            components.push(json!({
                "sourceID": &id,
                "type": "DIAGRAM"
            }));
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
                         "defaultVoice": self.config.default_voice,
                     },
                     "publishing": {},
                     "platform": "general"
                },
                "name": "Initial Version",
                "projectID": generate_id(),
                "manualSave": false,
                "autoSaveFromRestore": false,
                "rootDiagramID": self.main_diagram_id(),
                "creatorID": 0,
                "_version": 2.6,
                "components": components,
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
            "nodes": self.serialize_nodes(state, &state.dialog_name, &dialog.blocks),
            "children": [],
            "type": if state.is_main() { "TOPIC" } else { "COMPONENT" }
        })
    }

    fn serialize_nodes(&mut self, state: &State, diagram_name: &str, blocks: &Vec<Block>) -> Value {
        // generate start node id (if in main diagram, it's id must be a specific value)
        let start_node_id = if diagram_name.eq(ENTRY_DIALOG) {
            START_NODE_ID.to_owned()
        } else {
            generate_id()
        };
        let start_node = self.start_block(&start_node_id);
        let mut nodes = json!({ &start_node_id: start_node });

        let mut prev_node_ids: Vec<(String, PortType)> =
            vec![(start_node_id.to_owned(), PortType::Next)];
        for (line_number, block) in blocks.iter().enumerate() {
            let SerializedStep {
                root_step_id,
                new_steps,
                mut connecting_node_ids,
            } = self.serialize_step(state, block);

            // make previous nodes point to the new node
            for (prev_node_id, prev_node_port_type) in prev_node_ids.iter() {
                nodes[&prev_node_id]["data"]["portsV2"]["builtIn"]
                    [prev_node_port_type.to_string()] =
                    serialize_port(prev_node_port_type, Some(&root_step_id));
            }

            for new_step in new_steps.iter() {
                let step_id = get_node_id(&new_step).unwrap();
                nodes[&step_id] = new_step.clone(); // TODO pretty bad to clone this
                self.block_symbols.insert(
                    (state.dialog_name.to_owned(), line_number as u32),
                    root_step_id.to_owned(),
                );

                // create the block for the step
                let new_block = self.serialize_block(&step_id);
                let block_id = get_node_id(&new_block).unwrap();
                nodes[&block_id] = new_block;
            }

            prev_node_ids.clear();
            prev_node_ids.append(&mut connecting_node_ids);
        }

        nodes
    }

    fn start_block(&mut self, id: &str) -> Value {
        json!({
            "nodeID": id,
            "type": "start",
            "data": {
                "steps": [],
                "portsV2": {
                    "byKey": {},
                    "builtIn": {},
                    "dynamic": [],
                },
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

    fn serialize_step(&mut self, state: &State, block: &Block) -> SerializedStep {
        match block {
            Block::Jump { target } => {
                let node_id = generate_id();
                self.jump_relocation_table
                    .insert((state.dialog_name.clone(), node_id.clone()), target.clone());
                match target {
                    JumpTarget::Bookmark(_) => {
                        let value = json!({
                            /* empty speak is a NOOP */
                            "nodeID": node_id,
                            "type": "speak",
                            "data": {
                                "randomize": true,
                                "canvasVisibility": "preview",
                                "dialogs": [
                                    {
                                        "voice": self.config.default_voice,
                                        "content": "",
                                    }
                                ],
                                "portsV2": {
                                    "byKey": {},
                                    "builtIn": {
                                        /* to be inserted by jump relocation table */
                                    },
                                    "dynamic": [],
                                },
                            }
                        });
                        SerializedStep::new(node_id.clone(), vec![value], vec![])
                    }
                    JumpTarget::Dialog(_) => {
                        let value = json!({
                            "nodeID": node_id,
                            "type": "component",
                            "data": {
                                // to be inserted by jump relocation table
                                "diagramID": "",
                                "variableMap": None as Option<String>,
                                "portsV2": {
                                    "byKey": {},
                                    "builtIn": {
                                        "next": serialize_port(&PortType::Next, None),
                                    },
                                    "dynamic": [],
                                },
                            }
                        });
                        SerializedStep::new(node_id.clone(), vec![value], vec![])
                    }
                }
            }
            Block::Utterance { content, voice } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "speak",
                    "data": {
                        "randomize": true,
                        "canvasVisibility": "preview",
                        "dialogs": [
                            {
                                "voice": if let Some(voice) = voice { voice } else { &self.config.default_voice },
                                "content": content,
                            }
                        ],
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    }
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::EndCommand => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "exit",
                    "data": {
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {},
                            "dynamic": [],
                        },
                    }
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::SetCommand {
                variable: id,
                value,
            } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "setV2",
                    "data": {
                        "sets": [
                            {
                                "type": "value",
                                "variable": id,
                                "expression": value
                            }
                        ],
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    }
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::CaptureCommand { variable } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
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
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    },
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::CodeCommand { body } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "code",
                    "data": {
                        "code": body,
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    },
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::AudioCommand { url } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "speak",
                    "data": {
                        "randomize": true,
                        "canvasVisibility": "preview",
                        "dialogs": [
                            {
                                "voice": "audio",
                                "content": url,
                            }
                        ],
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    }
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::ImageCommand { url } => {
                let node_id = generate_id();
                let value = json!({
                    "nodeID": node_id,
                    "type": "visual",
                    "data": {
                        "visualType": "image",
                        "canvasVisibility": "full",
                        "frameType": "AUTO",
                        "image": url,
                        "device": None as Option<String>,
                        "dimensions": {
                            "width": 460,
                            "height": 460
                        },
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {
                                "next": serialize_port(&PortType::Next, None),
                            },
                            "dynamic": [],
                        },
                    }
                });
                SerializedStep::new(
                    node_id.clone(),
                    vec![value],
                    vec![(node_id, PortType::Next)],
                )
            }
            Block::Choice {
                cond: Conditional { operator, op1, op2 },
                block,
            } => {
                let from_operand = |op: &Operand| match op {
                    Operand::Variable(value) => json!({
                        "type": "variable",
                        "value": value,
                    }),
                    Operand::Literal(value) => json!({
                        "type": "value",
                        "value": value,
                    }),
                };

                // build step into conditional
                let SerializedStep {
                    root_step_id: cond_step_id,
                    mut new_steps,
                    mut connecting_node_ids,
                } = self.serialize_step(state, block);

                // build if step
                let if_node_id = generate_id();
                let if_value = json!({
                    "nodeID": if_node_id,
                    "type": "ifV2",
                    "data": {
                        "noMatch": {
                          "type": "path",
                          "pathName": "No match"
                        },
                        "expressions": [
                            {
                                "type": None as Option<String>,
                                "name": "",
                                "value": [
                                    {
                                        "logicInterface": "variable",
                                        "type": operator.to_string(),
                                        "value": [
                                            from_operand(op1),
                                            from_operand(op2),
                                        ]
                                    }
                                ]
                            }
                        ],
                        "portsV2": {
                            "byKey": {},
                            "builtIn": {},
                            "dynamic": [
                                {
                                    "type": "",
                                    "target": cond_step_id,
                                    "id": generate_id(),
                                    "data": {
                                        "points": []
                                    }
                                }
                            ]
                        }
                    },
                });

                new_steps.push(if_value);
                connecting_node_ids.push((if_node_id.clone(), PortType::Else));
                SerializedStep::new(if_node_id.clone(), new_steps, connecting_node_ids)
            }
        }
    }

    fn relocate(&mut self, vf_file: &mut Value, conv: &Conversation) {
        for ((diagram_name, block_id), target) in self.jump_relocation_table.iter() {
            let diagram_id = self.dialog_symbols.get(diagram_name).unwrap();
            let block = &mut vf_file["diagrams"][diagram_id]["nodes"][block_id];
            match target {
                JumpTarget::Bookmark(target_name) => {
                    // this line is so cringe pls do something about it
                    let line_number = conv
                        .dialog_table
                        .get(diagram_name)
                        .unwrap()
                        .bookmark_table
                        .get(target_name)
                        .unwrap();
                    debug!("line number {}", line_number);
                    let block_id = self
                        .block_symbols
                        .get(&(diagram_name.to_owned(), *line_number))
                        .unwrap();

                    block["data"]["portsV2"]["builtIn"][PortType::Next.to_string()] =
                        serialize_port(&PortType::Next, Some(block_id));
                }
                JumpTarget::Dialog(target_name) => {
                    let jump_diagram_id = self.dialog_symbols.get(target_name).unwrap().to_owned();
                    debug!(
                        "relocating dialog {} to block {}",
                        target_name, jump_diagram_id
                    );
                    block["data"]["diagramID"] = Value::String(jump_diagram_id);
                }
            }
        }
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

enum PortType {
    Next,
    Else,
}

impl std::fmt::Display for PortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortType::Next => write!(f, "next"),
            PortType::Else => write!(f, "else"),
        }
    }
}

fn serialize_port(port_type: &PortType, target: Option<&str>) -> Value {
    json!({
        "type": port_type.to_string(),
        "target": target,
        "id": generate_id(),
        "data": {
            "points": []
        }
    })
}
