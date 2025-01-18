use std::collections::{HashMap, HashSet};

type Node = (i32, i32);

struct NodeMap {
    width: i32,
    height: i32,
    // map: Vec<Vec<String>>,
    nodes: HashMap<String, Vec<Node>>,
}

fn parse_input_file(file: &str) -> NodeMap {
    let contents = std::fs::read_to_string(file).unwrap_or_else(|_| {
        println!("ERROR: Input file not found.");
        std::process::exit(1);
    });

    let map: Vec<Vec<String>> = contents
        .lines()
        .map(|line| line.chars().map(|char| char.to_string()).collect())
        .collect();

    let height = map.len() as i32;
    let width = map[0].len() as i32;
    let mut nodes: HashMap<String, Vec<Node>> = HashMap::new();

    for row in 0..height {
        for col in 0..width {
            let cell = &map[row as usize][col as usize].chars().next().unwrap();
            // If not "."
            if *cell as u32 != 46 {
                let node_collection = nodes.entry(cell.to_string()).or_insert(Vec::new());
                node_collection.push((row as i32, col as i32))
            }
        }
    }

    return NodeMap {
        width,
        height,
        nodes,
    };
}

fn is_inside_map(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < height as i32 && pos.1 < width as i32
}

fn compute_anti_nodes(node_map: &NodeMap, resonant: bool) -> HashSet<(i32, i32)> {
    let nodes = &node_map.nodes;
    let mut anti_nodes: HashSet<(i32, i32)> = HashSet::new();
    for id in nodes.keys() {
        let nodes_for_id = nodes.get(id).unwrap();
        for node in nodes_for_id {
            for other_node in nodes_for_id {
                if node != other_node {
                    for direction in [-1, 1] {
                        let distance = (other_node.0 - node.0, other_node.1 - node.1);
                        let mut anti_node = if direction == -1 { *node } else { *other_node };
                        loop {
                            anti_node = (
                                anti_node.0 + (distance.0 * direction),
                                anti_node.1 + (distance.1 * direction),
                            );
                            if is_inside_map(anti_node, node_map.width, node_map.height) {
                                anti_nodes.insert(anti_node);
                            } else {
                                break;
                            }
                            if !resonant {
                                break;
                            }
                        }
                    }
                }
            }
            if resonant {
                anti_nodes.insert(*node);
            }
        }
    }
    anti_nodes
}

fn main() {
    let test_data = parse_input_file("08.test.dat");
    let anti_nodes = compute_anti_nodes(&test_data, false);
    assert!(anti_nodes.len() == 14);
    let resonant_anti_nodes = compute_anti_nodes(&test_data, true);
    assert!(resonant_anti_nodes.len() == 34);

    let data = parse_input_file("08.dat");
    let anti_nodes = compute_anti_nodes(&data, false);
    println!("First answer: {}", anti_nodes.len());
    let resonant_anti_nodes = compute_anti_nodes(&data, true);
    println!("Second answer: {}", resonant_anti_nodes.len());
}
