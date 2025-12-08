use itertools::Itertools;
use std::collections::HashSet;

type Junction = (usize, usize, usize);

// Not really a node any more, less graphy
#[derive(Clone, Debug)]
struct Node {
    junction: Junction,
}

// 3D distance
fn distance(a: Junction, b: Junction) -> usize {
    let dx = (a.0 - b.0).pow(2);
    let dy = (a.1 - b.1).pow(2);
    let dz = (a.2 - b.2).pow(2);
    let d = dx + dy + dz;
    usize::isqrt(d)
}

pub fn part2() {
    let input = include_str!("../test.txt");
    let nodes = input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<Junction>()
                .unwrap()
        })
        .map(|j| Node { junction: j })
        .collect::<Vec<Node>>();

    // Keep list of found connections and track last 2 connections we formed
    let mut last2 = [0; 2];
    let mut conn: HashSet<Junction> = HashSet::new();

    // Keep repeating until we have one circuit
    loop {
        let mut found = false;
        let mut min_d = usize::MAX;
        let mut min_i = 0;
        let mut min_j = 0;

        for j in 0..nodes.len() {
            for i in 0..nodes.len() {
                if i != j {
                    let a = &nodes[j];
                    let b = &nodes[i];

                    // Make sure this connection hasn't been formed
                    if !conn.contains(&a.junction) || !conn.contains(&b.junction) {
                        let d = distance(a.junction, b.junction);
                        if d < min_d {
                            min_d = d;
                            min_i = i;
                            min_j = j;
                            found = true;
                        }
                    }
                }
            }
        }

        // There are still new connections, so add them to list and repeat
        if found {
            conn.insert(nodes[min_j].junction);
            conn.insert(nodes[min_i].junction);
            last2 = [nodes[min_j].junction.0, nodes[min_i].junction.0];
        } else {
            break;
        }
    }

    println!("Product: {}", last2[0] * last2[1]);
}
