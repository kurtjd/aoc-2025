use itertools::Itertools;
use std::collections::HashSet;

// Change this for real input (to 1000)
const MAX_CONNECTIONS: usize = 10;
const TOP_COUNT: usize = 3;

type Junction = (usize, usize, usize);

// Represents a node in the graph we are constructing
#[derive(Clone, Debug)]
struct Node {
    junction: Junction,
    neighbors: HashSet<usize>,
    visited: bool,
}

// 3D distance
fn distance(a: Junction, b: Junction) -> usize {
    let dx = (a.0 - b.0).pow(2);
    let dy = (a.1 - b.1).pow(2);
    let dz = (a.2 - b.2).pow(2);
    let d = dx + dy + dz;
    usize::isqrt(d)
}

// basically we visit each neighbor (vertex) from start point recursively
// we mark each node we visit to avoid loops
// we keep a sum of nodes visited to know how big this circuit is
fn flood(nodes: &mut Vec<Node>, start: usize) -> usize {
    if !nodes[start].visited {
        let mut sum = 1;
        let n = nodes[start].neighbors.clone();
        nodes[start].visited = true;

        for v in n.iter() {
            sum += flood(nodes, *v);
        }

        sum
    } else {
        0
    }
}

pub fn part1() {
    let input = include_str!("../test.txt");
    let mut nodes = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<Junction>()
                .unwrap()
        })
        .map(|j| Node {
            junction: j,
            neighbors: HashSet::new(),
            visited: false,
        })
        .collect::<Vec<Node>>();

    // Basically building up the graph here, finding neighbors of each node
    for _ in 0..MAX_CONNECTIONS {
        // Keep track of the minimum junction for this cycle
        let mut min_d = usize::MAX;
        let mut min_i = 0;
        let mut min_j = 0;

        // Find the shortest distance between any two nodes
        for j in 0..nodes.len() {
            for i in 0..nodes.len() {
                if i != j {
                    let a = &nodes[j];
                    let b = &nodes[i];

                    // Make sure one of these hasn't already marked the other as a neighbor
                    if !b.neighbors.contains(&j) && !a.neighbors.contains(&i) {
                        let d = distance(a.junction, b.junction);
                        if d < min_d {
                            min_d = d;
                            min_i = i;
                            min_j = j;
                        }
                    }
                }
            }
        }

        // Then mark eachother as neighbors
        nodes[min_j].neighbors.insert(min_i);
        nodes[min_i].neighbors.insert(min_j);
    }

    let mut circuits = Vec::<usize>::new();

    // Then flood the graph we built to determine how many individual circuits there are and their sizes
    for i in 0..nodes.len() {
        let res = flood(&mut nodes, i);
        circuits.push(res);
    }

    // Want the product of top 3 circuits in size
    circuits.sort();
    let prod = circuits.iter().rev().take(TOP_COUNT).product::<usize>();

    println!("Product: {prod}");
}
