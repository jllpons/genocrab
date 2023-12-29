use std::collections::HashMap;
use std::collections::HashSet;

use crate::debruijn::build_nodes;

fn create_string_from_nodes(nodes: HashSet<(String, String)>) -> Result<String, &'static str> {
    // Creates a string from a set of nodes trying to follow the eulerian path
    //
    // # Arguments
    // * `nodes` - A set of nodes
    //
    // # Returns
    // * `Result<String, &'static str>` - A string constructed from the nodes

    let nodes_hm: HashMap<String, String> = nodes.into_iter().collect();

    let mut constructed_string = String::new();
    let mut next_node = String::new();
    let mut visited_nodes = HashSet::new();

    for key in nodes_hm.keys() {
        constructed_string = key.clone();
        visited_nodes.insert(key.clone());
        next_node = nodes_hm.get(key).unwrap().clone();

        while !visited_nodes.contains(&next_node) {
            visited_nodes.insert(next_node.clone());
            constructed_string += &next_node.chars().last().unwrap().to_string();
            next_node = nodes_hm.get(&next_node).unwrap().clone();
        }

        if visited_nodes.len() == nodes_hm.len() {
            return Ok(constructed_string);
        }
    }
    Err("Not all nodes were visited and the string could not be constructed")
}

fn find_shortest_circular_string(linear_string: &str) -> Result<String, &'static str> {
    // Finds the shortest circular string from a linear string
    //
    // # Arguments
    // * `linear_string` - A linear stringify!
    //
    // # Returns
    // * `Result<String, &'static str>` - The shortest circular string

    for i in 1..linear_string.len() {
        if linear_string.starts_with(&linear_string[i..]) {
            return Ok(linear_string[..i].to_string());
        }
    }
    Err("No circular string found")
}

pub fn run_perfect_assembly(sequences: Vec<&str>) -> Result<String, String> {
    // Runs the perfect assembly algorithm
    //
    // # Arguments
    // * `sequences` - A vector of sequences
    //
    // # Returns
    // * `Result<String, String>` - The shortest circular string
    let nodes = build_nodes(sequences, false);
    match nodes {
        Ok(nodes) => {
            let str_from_nodes = create_string_from_nodes(nodes);
            match str_from_nodes {
                Ok(str_from_nodes) => {
                    let shortest_circular_string = find_shortest_circular_string(&str_from_nodes);
                    match shortest_circular_string {
                        Ok(shortest_circular_string) => {
                            return Ok(shortest_circular_string);
                        }
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}


