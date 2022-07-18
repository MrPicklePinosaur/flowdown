use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagram {
    pub name: String,
    pub offset_x: f32,
    pub offset_y: f32,
    pub zoom: f32,
    // pub variables: Vec<>
    pub nodes: HashMap<String, Node>
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: String,
    #[serde(rename = "type")]
    pub node_type: String,
    pub coords: (f32, f32),
}

pub fn generate_id() -> String {
    String::new()
}

