extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate pathfinding;
use pathfinding::prelude::astar;
use std::u64;
use std::f64;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Node {
    id: u64,
    lat: f64,
    lon: f64,
}

#[derive(Serialize, Deserialize)]
struct Edge {
    from: u64,
    to: u64,
    cost: f64,
}

#[derive(Serialize, Deserialize)]
struct MapGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn load_raw_graph<P: AsRef<Path>>(path: P) -> Result<MapGraph, Box<Error>> {
    let file = File::open(path)?;
    let g = serde_json::from_reader(file)?;
    Ok(g)
}

fn transform_raw_graph(g: &MapGraph) -> (HashMap<u64, (f64, f64)>, HashMap<u64, HashMap<u64, f64>>) {
    let mut nodes: HashMap<u64, (f64, f64)> = HashMap::new();
    for node in g.nodes.iter() {
        nodes.insert(node.id, (node.lat, node.lon));
    }

    let mut edges: HashMap<u64, HashMap<u64, f64>> = HashMap::new();
    for edge in g.edges.iter() {
        if !edges.contains_key(&edge.from) {
            edges.insert(edge.from, HashMap::new());
        }
        edges.get_mut(&edge.from).unwrap().insert(edge.to, edge.cost);
    }

    (nodes, edges)
}

fn main() {
    let g: MapGraph = match load_raw_graph("map.json") {
        Err(e) => {
            eprintln!("Failed to load map: {:?}", e);
            return;
        },
        Ok(g) => g,
    };
    let (nodes, edges) = transform_raw_graph(&g);

    let target_node: u64 = 3;
    let result = astar(
        &42u64,
        |i| {
            match edges.get(i) {
                None => return Vec::new(),
                Some(e) => {
                    e.iter().map(|n| (*n.0, (n.1 * 100.0) as u64)).collect()
                },
            }
        },
        |i| {
            let coords_end = nodes.get(&target_node).unwrap();
            let coords_now = match nodes.get(i) {
                None => return u64::MAX,
                Some(c) => c,
            };
            let delta = ((coords_end.0 - coords_now.0).abs(), (coords_end.1 - coords_now.1).abs());
            let dist = (delta.0*delta.0 + delta.1*delta.1).sqrt();
            (dist * 100.0) as u64
        },
        |i| {
            *i == target_node
        }
    );

    println!("Result:");
    println!("{:?}", result);
}
