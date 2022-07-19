// voiceflow diagram types

use std::collections::HashMap;
use serde::Serialize;

use crate::blocks::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VFData {
    _version: String,
    project: Project
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    _id: String,
    _version: String,
    name: String,
    team_id: String,
    dev_version: String,
    platform: String,
    // platformData:
    members: Vec<String>,
    link_type: String,
    image: String,
    creator_id: u32,
    privacy: String,
    created_at: String
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    _id: String,
    variables: Vec<String>,
    // platform_data: Vec,

}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Diagram {
    pub _id: String,
    pub version_id: String,
    pub creator_id: u32,

    pub name: String,
    // #[serde(rename = "type")]
    // pub diagram_type: String
    pub zoom: f32,
    pub nodes: HashMap<String, Node>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub modified: f32,
    pub children: Vec<String>,
    // pub variables: Vec<>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub coords: (f32, f32),
    // pub data: HashMap<String, D>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeData {
    pub name: String,
    // pub color: 
    // steps: 
    // ports:
    
}

impl From<Block> for Node {
    fn from(block: Block) -> Self {
        match block {
            Block::Utterance { content } => {
                Node {
                    node_id: generate_id(),
                    node_type: "speak".into(),
                    coords: (0., 0.),
                }
            },
            Block::EndCommand => {
                Node {
                    node_id: generate_id(),
                    node_type: "end".into(),
                    coords: (0., 0.),
                }
            },
            _ => unreachable!()
        }
    }
}

pub fn generate_id() -> String {
    String::new()
}
