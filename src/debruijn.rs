use std::collections::HashSet;

pub fn reverse_complement(seq: &str) -> Result<String, String> {
    // Given a sequence, return the reverse complement
    //
    // # Arguments
    // * `seq` - A string containing a sequence
    //
    // # Returns
    // A string containing the reverse complement of the input sequence
    let mut result = String::new();
    for c in seq.chars().rev() {
        match c {
            'A' => result.push('T'),
            'T' => result.push('A'),
            'C' => result.push('G'),
            'G' => result.push('C'),
            'N' => result.push('N'),
            _ => Err(format!("Invalid character in sequence: {}", c))?,
        }
    }
    Ok(result)
}

pub fn build_nodes(sequences: Vec<&str>, rc: bool) -> Result<HashSet<(String, String)>, String> {
    // Given a vector of sequences, return a set of nodes
    //
    // # Arguments
    // * `sequences` - A vector of strings containing sequences
    // * `rc` - A boolean indicating whether to include the reverse complement of the sequences
    //
    // # Returns
    // A set of nodes
    let mut nodes = HashSet::<(String, String)>::new();

    if sequences.is_empty() {
        return Err("No sequences provided".to_string());
    }
    for seq in sequences {
        if seq.len() < 2 {
            return Err(format!("Sequence too short: {}", seq));
        }

        let seq_nodes = (seq[0..(&seq.len() - 1)].to_string(), seq[1..].to_string());

        if !nodes.contains(&seq_nodes) {
            nodes.insert(seq_nodes);
        }

        if rc {
            let rc_seq = reverse_complement(seq);
            match rc_seq {
                Ok(rc_seq) => {
                    let rc_seq_nodes = (
                        rc_seq[0..(&rc_seq.len() - 1)].to_string(),
                        rc_seq[1..].to_string(),
                    );
                    if !nodes.contains(&rc_seq_nodes) {
                        nodes.insert(rc_seq_nodes);
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    Ok(nodes)
}

pub fn run_debruijn_graph(sequences: Vec<&str>, rc: bool) -> Result<String, String> {
    // Run the debuijn graph algorithm on a vector of sequences and return the nodes
    // as a string
    //
    // # Arguments
    // * `sequences` - A vector of strings containing sequences
    // * `rc` - A boolean indicating whether to include the reverse complement of the sequences
    //
    // # Returns
    // A string containing the nodes of the graph, sorted alphabetically and separated by newlines
    let nodes = build_nodes(sequences, rc);
    match nodes {
        Ok(nodes) => {
            let mut nodes_sorted: Vec<(String, String)> = nodes.into_iter().collect();
            nodes_sorted.sort();
            let mut nodes_in_str = String::new();
            for (k, v) in nodes_sorted {
                nodes_in_str.push_str(&format!("({}, {})\n", k, v));
            }

            Ok(nodes_in_str)
        }
        Err(e) => Err(e),
    }
}
