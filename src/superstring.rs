use std::cmp::min;
use std::collections::HashMap;

use crate::Fasta;

fn obtain_sequence_list(fastas: Vec<Fasta>) -> Vec<String> {
    // Obtain a list of sequences from a list of FASTA objects
    //
    // # Arguments
    // * `fastas` - A vector of FASTA objects
    //
    // # Returns
    // A vector of sequences
    let mut sequences: Vec<String> = Vec::new();

    for fasta in fastas {
        sequences.push(fasta.seq);
    }

    sequences
}

fn get_min_suffix(seq1: &str, seq2: &str) -> String {
    // Get the minimum suffix of seq2 that doesn't overlap with seq1
    //
    // # Arguments
    // * `seq1` - The first sequence
    // * `seq2` - The second sequence
    //
    // # Returns
    // The minimum suffix of seq2 that doesn't overlap with seq1.
    // If there is no such suffix, seq2 is returned
    let n = min(seq1.len(), seq2.len());
    // Check each possible suffix from the longest to the shortest
    for i in (1..n + 1).rev() {
        if seq1[seq1.len() - i..] == seq2[..i] {
            return seq2[i..].to_string(); // non-overlapping suffix found
        }
    }
    seq2.to_string() // no non-overlapping suffix found
}

// TODO: Handle situation where there are multiple valid next nodes
fn find_next_node(
    overlaps: &HashMap<usize, HashMap<usize, i32>>,
    node: usize,
) -> Result<usize, &'static str> {
    // Find the next node in the path
    //
    // # Arguments
    // * `overlaps` - A hashmap of hashmaps containing the overlaps between sequences
    // * `node` - The current Node
    //
    // # Returns
    // The next node in the path
    //
    // # Errors
    // If the node is not found in the overlaps hashmap, an error is returned.

    // Attempt to get the sub-hashmap for the node. If it doesn't exist, return an error.
    let sub_map = overlaps
        .get(&node)
        .ok_or("Error: Node not found in overlaps")?;

    // Proceed to find the minimum, ensuring there's at least one valid entry.
    match sub_map
        .iter()
        .filter(|(&_j, &value)| value != -1) // Filter out values of '-1'.
        .min_by_key(|&(_, &value)| value)
    {
        // Find the minimum by value.
        Some((&j, _)) => Ok(j), // If found, return the index as Ok.
        None => Err("Error: No valid next node found"), // If not found, return an error.
    }
}

pub fn shortest_superstring(sequences: Vec<String>) -> Result<String, &'static str> {
    // Find the shortest superstring of a list of sequences
    //
    // # Arguments
    // * `sequences` - A vector of sequences
    //
    // # Returns
    // The shortest superstring of the sequences
    let n = sequences.len();
    let mut overlaps_len = HashMap::new();
    let mut overlaps = HashMap::new();

    for i in 0..n {
        overlaps_len.insert(i, HashMap::new());
        overlaps.insert(i, HashMap::new());
        for j in 0..n {
            if i != j {
                overlaps_len
                    .get_mut(&i)
                    .unwrap()
                    .insert(j, get_min_suffix(&sequences[i], &sequences[j]).len() as i32);
                overlaps
                    .get_mut(&i)
                    .unwrap()
                    .insert(j, get_min_suffix(&sequences[i], &sequences[j]));
            } else {
                overlaps_len.get_mut(&i).unwrap().insert(j, -1);
            }
        }
    }

    let mut candidates = Vec::new();

    for i in 0..n {
        let mut visited = vec![false; n];
        let mut path = vec![i];
        visited[i] = true;

        let mut next_node = find_next_node(&overlaps_len, (i as i32).try_into().unwrap())?;

        while !visited[next_node as usize] {
            path.push(next_node as usize);
            visited[next_node as usize] = true;
            next_node = find_next_node(&overlaps_len, next_node)?;
        }

        if path.len() == n {
            candidates.push(path);
        }
    }

    let mut superstrings = Vec::new();

    for candidate in candidates {
        let mut sstring = sequences[candidate[0]].clone();

        for i in 1..candidate.len() {
            sstring += &overlaps
                .get(&candidate[i - 1])
                .unwrap()
                .get(&candidate[i])
                .unwrap();
        }
        superstrings.push(sstring);
    }

    let best_superstring = superstrings.iter().min();
    match best_superstring {
        Some(x) => Ok(x.to_string()),
        None => Err("Error: empty list when finding best superstring"),
    }
}

pub fn run_superstring(fastas: Vec<Fasta>) -> Result<String, &'static str> {
    // Run the shortest superstring algorithm
    //
    // # Arguments
    // * `fastas` - A vector of FASTA structs
    //
    // # Returns
    // The shortest superstring of the sequences
    let sequences = obtain_sequence_list(fastas);

    let result = shortest_superstring(sequences);
    match result {
        Ok(x) => Ok(x),
        Err(e) => Err(e),
    }
}
