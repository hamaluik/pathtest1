extern crate pathfinding;
use pathfinding::prelude::astar;    
use std::u64;

fn main() {
    let nodes: Vec<(f64, f64)> = vec![
        (0.0, 0.0),
        (0.1, 0.2),
        (0.2, -0.1),
        (0.5, 0.75)
    ];

    let edges: Vec<Vec<Option<u64>>> = vec![
        vec![None, Some(50), Some(25), None,],
        vec![Some(50), None, None, Some(25),],
        vec![Some(25), None, None, Some(100),],
        vec![None, Some(25), Some(100), None,],
    ];

    let result = astar(
        &0usize,
        |i| {
            edges[*i].iter().enumerate().filter(|n| *n.1 != None).map(|n| (n.0, n.1.unwrap()))
        },
        |i| {
            let coords_end = nodes[3];
            let coords_now = nodes[*i];
            let delta = ((coords_end.0 - coords_now.0).abs(), (coords_end.1 - coords_now.1).abs());
            let dist = (delta.0*delta.0 + delta.1*delta.1).sqrt();
            (dist * 100.0) as u64
        },
        |i| {
            *i == 3
        }
    );

    println!("result: {:?}", result);
}
