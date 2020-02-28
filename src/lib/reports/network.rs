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
    group: Option<String>,
    title: Option<String>,
    #[serde(rename = "borderWidth")]
    border_width: i32,
    #[serde(skip)]
    neighbors: Vec<i32>,

}
impl Node {
    pub fn new<S: Into<String>>(id: i32, label: S) -> Self {
        Self {
            id: id,
            label: label.into(),
            color: Some(String::from("red")),
            mass: 1,
            group: None,
            title: None,
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
        let constellation = system.get_name("constellation");
        let region = system.get_name("region");
        let title = format!("Constellation: {}<br/>Region: {}<br/>{}",
                            &constellation,
                            &region,
                            system.observatory.map(|_| String::from("Jovian Observatory exist")).unwrap_or_default()
                            );
        Self {
            id: id,
            label: label,
            color: Some(color),
            mass: 1,
            group: Some(constellation),
            title: Some(title),
            border_width: 1,
            neighbors: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq)]
pub struct Edge {
    pub from: i32,
    pub to: i32
}
impl Edge {
    pub fn new(from: i32, to: i32) -> Self { Self{from, to} }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to) || (self.from == other.to && self.to == other.from)
    }
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
    if !nodes.contains_key(id) {
        if let Some(mut node) = create_node(id, ctx) {
            if deep > 0 {
                node.mass = deep;
                let neighbors = node.neighbors.clone();
                nodes.insert(*id, node);
                for id in &neighbors {
                    make_system_network(id, ctx, nodes, deep - 1);
                }
            }
        }
    }
}

pub fn get_system_network_nodes(id: &i32, deep: u32, ctx: &Context) -> HashMap<i32, Node> {
    let mut nodes:  HashMap<i32, Node> = HashMap::new();
    make_system_network(id, ctx, &mut nodes, deep);
    return nodes;
}

pub fn get_system_network_edges(id: &i32, deep: u32, ctx: &Context) -> Vec<Edge> {
    let nodes = get_system_network_nodes(id, deep, ctx);
    let mut edges = Vec::new();
    for (from, node) in &nodes {
        for to in &node.neighbors {
            let edge = Edge::new(*from, *to);
            let unknown = edges.iter().find(|&e| *e == edge).is_none();
            if unknown && nodes.contains_key(from) && nodes.contains_key(to) {
                edges.push(edge);
            }
        }
    }
    return edges;
}

