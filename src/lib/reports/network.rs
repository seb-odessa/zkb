//use crate::api;
use crate::services::Context;
use crate::models;
use crate::reports;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
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
    fn new(model: models::system::SystemNamed, mass: u32, ctx: &Context) -> Self {
        let mut output = String::new();
        let id = model.system_id;
        let system = model.get_name("system");
        let status = model.get_security_status();
        let color = reports::get_security_status_color(model.security_status);
        let constellation = model.get_name("constellation");
        let region = model.get_name("region");
        let shape = String::from(if model.observatory.is_none() {"ellipse"} else {"box"});
        let style = format!("background-color: {}; display: inline-block; width=100%", color);
        let label = format!("{} ({})", system, status);
        let colored = reports::span("", style, &label);
        reports::div(&mut output, format!("System: {}", colored));
        reports::div(&mut output, format!("Constellation: {}", &constellation));
        reports::div(&mut output, format!("Region: {}", &region));
        reports::div(&mut output, format!("Kills last 10 minutes: {}", reports::History::system_count(&id, &10, ctx)));
        reports::div(&mut output, format!("Kills last 60 minutes: {}", reports::History::system_count(&id, &60, ctx)));
        reports::div(&mut output, format!("{}", model.observatory.map(|_| String::from("Jovian Observatory")).unwrap_or_default()));

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
            neighbors: reports::system::System::get_neighbors(&id, ctx).iter().map(|x| x.neighbor_id).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Eq)]
pub struct Edge {
    pub from: i32,
    pub to: i32,
    pub color: String,
}
impl Edge {
    pub fn new(from: i32, to: i32) -> Self { Self { from, to, color: String::from("Black") } }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from && self.to == other.to) || (self.from == other.to && self.to == other.from)
    }
}

fn make_system_network(ids: &Vec<i32>, ctx: &Context, nodes: &mut HashMap<i32, Node>, deep: u32) {
    let mut neighbors = Vec::new();
    for id in ids {
        if !nodes.contains_key(id) {
            if let Some(system) = reports::system::System::load(id, ctx) {
                let mut node = Node::new(system, 1, ctx);
                if 0 == deep {
                    node.shape = String::from("hexagon");    
                }    
                neighbors.append(&mut node.neighbors.clone());
                nodes.insert(*id, node);
            }
        }
    }
    if deep > 0 {
        make_system_network(&neighbors, ctx, nodes, deep - 1);
    }
}

pub fn get_system_nodes(id: &i32, deep: u32, ctx: &Context) -> HashMap<i32, Node> {
    let mut nodes:  HashMap<i32, Node> = HashMap::new();
    if deep > 0 {
        if let Some(system) = reports::system::System::load(id, ctx) {
            let node = Node::new(system, 3, ctx);
            let neighbors = node.neighbors.clone();
            nodes.insert(node.id, node);
            make_system_network(&neighbors, ctx, &mut nodes, deep-1);
        }
    }
    return nodes;
}

pub fn build_edges(nodes: &HashMap<i32, Node>) -> Vec<Edge> {
    let mut edges = Vec::new();
    for (from, node) in nodes {
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

pub fn get_system_edges(id: &i32, deep: u32, ctx: &Context) -> Vec<Edge> {
    return build_edges(&get_system_nodes(id, deep, ctx));
}

pub fn get_constellation_nodes(id: &i32, ctx: &Context) -> HashMap<i32, Node> {
    let mut nodes:  HashMap<i32, Node> = HashMap::new();
    let mut neighbors = Vec::new();
    if let Some(systems) = reports::constellation::Constellation::get_systems(id, ctx) {
        for system in systems.into_iter() {
            let node = Node::new(system, 3, ctx);
            neighbors.append(&mut node.neighbors.clone());
            nodes.insert(node.id, node);    
        }
        for id in &neighbors {
            if !nodes.contains_key(id) {
                if let Some(system) = reports::system::System::load(id, ctx) {
                    let mut node = Node::new(system, 1, ctx);
                    node.shape = String::from("hexagon");
                    nodes.insert(node.id, node);
                }
            }
        }
    }
    return nodes;
}

pub fn get_constellation_edges(id: &i32, ctx: &Context) -> Vec<Edge> {
    return build_edges(&get_constellation_nodes(id, ctx));
}

pub fn get_region_nodes(id: &i32, ctx: &Context) -> HashMap<i32, Node> {
    let mut nodes:  HashMap<i32, Node> = HashMap::new();    
    if let Some(constellations) = reports::region::Region::get_constellation(id, ctx) {
        let mut neighbors = HashSet::new();
        for constellation in constellations.into_iter() {
            let id = constellation.constellation_id;
            if let Some(systems) = reports::constellation::Constellation::get_systems(&id, ctx) {
                for system in systems.into_iter() {
                    let node = Node::new(system, 1, ctx);
                    let other: HashSet<i32> = node.neighbors.clone().into_iter().collect();
                    neighbors = neighbors.union(&other).cloned().collect::<HashSet<_>>();
                    nodes.insert(node.id, node);
                }
            }
        }
        for id in &neighbors {
            if !nodes.contains_key(id) {
                if let Some(system) = reports::system::System::load(id, ctx) {
                    let mut node = Node::new(system, 1, ctx);
                    node.shape = String::from("hexagon");
                    nodes.insert(node.id, node);
                }
            }
        }
    }
    return nodes;
}

pub fn get_region_edges(id: &i32, ctx: &Context) -> Vec<Edge> {
    return build_edges(&get_region_nodes(id, ctx));
}
