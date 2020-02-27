//use crate::api;
use crate::services::Context;
use crate::models;
use crate::reports;
use reports::system;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Node {
    pub id: i32,
    pub label: String,
    color: Option<String>,
    mass: u32,
    #[serde(rename = "borderWidth")]    border_width: i32,
    #[serde(rename = "skip")]           neighbors: Vec<i32>,

}
impl Node {
    pub fn new<S: Into<String>>(id: i32, label: S) -> Self {
        Self {
            id: id,
            label: label.into(),
            color: Some(String::from("red")),
            mass: 1,
            border_width: 1,
            neighbors: Vec::new(),
        }
    }
}
impl From<models::system::SystemNamed> for Node {
    fn from(system: models::system::SystemNamed) -> Self {
        let id = system.system_id;
        let label = format!("{} ({})", system.get_name("system"), system.get_security_status());
        let color = reports::get_security_status_color(system.security_status);
        Self {
            id: id,
            label: label,
            color: Some(color),
            mass: 1,
            border_width: 1,
            neighbors: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct Edge {
    pub from: i32,
    pub to: i32
}
impl Edge {
    pub fn new(from: i32, to: i32) -> Self { Self{from, to} }
}

fn create_node(id: &i32, ctx: &Context) -> Option<Node> {
    if let Some(system) = system::System::load(id, ctx) {
        let mut node = Node::from(system);
        node.neighbors = system::System::get_neighbors(id, ctx).iter().map(|x| x.neighbor_id).collect();
        return Some(node);
    }
    return None;
}

fn make_system_network(id: &i32, ctx: &Context, nodes: &mut HashMap<i32, Node>, deep: u32) {
    if deep > 0 {
        if let Some(mut node) = create_node(id, ctx) {
            node.mass = deep;
            let neighbors = node.neighbors.clone();
            nodes.insert(*id, node);
            for id in &neighbors {
                make_system_network(id, ctx, nodes, deep - 1);
            }
        }
    }
}

pub fn get_system_network_nodes(id: &i32, deep: u32, ctx: &Context) -> Vec<Node> {
    let mut nodes:  HashMap<i32, Node> = HashMap::new();
    make_system_network(id, ctx, &mut nodes, deep);
    return nodes.values().into_iter().cloned().collect();
}
