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
    color: String,
    mass: u32,
    group: Option<String>,
    title: Option<String>,
    shape: String,
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
            color: String::from("#D2E5FF"),
            mass: 1,
            group: None,
            title: None,
            shape: String::from("ellipse"),
            border_width: 1,
            neighbors: Vec::new(),
        }
    }

    fn create(model: models::system::SystemNamed, mass: u32, ctx: &Context) -> Self {
        let mut output = String::new();
        let id = model.system_id;
        let system = model.get_name("system");
        let status = model.get_security_status();
        let label = format!("{} ({})", system, status);
        let color = reports::get_security_status_color(model.security_status);
        let constellation = model.get_name("constellation");
        let region = model.get_name("region");
        let shape = String::from(if model.observatory.is_none() {"ellipse"} else {"box"});
        let sys_row = reports::span("", format!("color:{}; display: inline-block; width=100%", color), system);
        reports::div(&mut output, sys_row);
        reports::div(&mut output, format!("Constellation: {}", &constellation));
        reports::div(&mut output, format!("Region: {}", &region));
        reports::div(&mut output, format!("{}", model.observatory.map(|_| String::from("Jovian Observatory")).unwrap_or_default()));
        reports::lazy(&mut output, format!("api/system_brief/{}", id), &ctx);
        let title = format!("{}", output);
        Self {
            id: id,
            label: label,
            color: color,
            mass: mass,
            group: Some(constellation),
            title: Some(title),
            shape: shape,
            border_width: 1,
            neighbors: system::System::get_neighbors(&id, ctx).iter().map(|x| x.neighbor_id).collect(),
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

fn make_system_network(id: &i32, ctx: &Context, nodes: &mut HashMap<i32, Node>, deep: u32) {
    if deep > 0 && !nodes.contains_key(id) {
        if let Some(system) = system::System::load(id, ctx) {
            let node = Node::create(system, deep, ctx);
            let neighbors = node.neighbors.clone();
            nodes.insert(*id, node);
            for id in &neighbors {
                make_system_network(id, ctx, nodes, deep - 1);
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

