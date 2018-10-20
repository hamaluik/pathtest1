extern crate pathfinding;
use pathfinding::prelude::astar;
use std::u64;
use std::f64;
use std::collections::HashMap;

fn main() {
    let mut nodes: HashMap<u64, (f64, f64)> = HashMap::new();
    nodes.insert(42, (0.0, 0.0));
    nodes.insert(1, (0.5, -0.25));
    nodes.insert(22, (0.5, 0.0));
    nodes.insert(3, (1.0, 0.0));

    let mut edges: HashMap<u64, HashMap<u64, f64>> = HashMap::new();

    let mut edge: HashMap<u64, f64> = HashMap::new();
    edge.insert(1, 0.5);
    edge.insert(22, 1.5);
    edges.insert(42, edge);

    let mut edge: HashMap<u64, f64> = HashMap::new();
    edge.insert(3, 0.5);
    edges.insert(1, edge);

    let mut edge: HashMap<u64, f64> = HashMap::new();
    edge.insert(3, 0.5);
    edges.insert(22, edge);

    println!("Nodes:");
    for i in vec![42, 1, 22, 3] {
        let derp = match nodes.get(&i) {
            None => (f64::NAN, f64::NAN),
            Some(v) => *v,
        };
        println!("{} => {:?}", i, derp);
    }

    println!("Edges:");
    for i in vec![42, 1, 22, 3] {
        let derp: Vec<(u64, u64)> = match edges.get(&i) {
            None => Vec::new(),
            Some(e) => {
                e.iter().map(|n| (*n.0, (n.1 * 100.0) as u64)).collect()
            },
        };
        println!("{} => {:?}", i, derp);
    }

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
