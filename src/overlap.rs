use crate::Fasta;

fn check_ovelrap(seq_s: &str, seq_p: &str, k: usize) -> bool {
    // Check if two sequences overlap
    //
    // # Arguments
    // * `seq_s` - The subject sequence
    // * `seq_p` - The query sequence
    // * `k` - The length of the kmer
    //
    // # Returns
    // A boolean value indicating if the sequences overlap
    let seq_s_suffix = &seq_s[seq_s.len() - k..];
    let seq_p_prefix = &seq_p[..k];

    seq_s_suffix == seq_p_prefix
}

fn mk_overlap_graph(fastas: Vec<Fasta>, k: usize) -> Vec<(String, String)> {
    // Create an overlap graph from a vector of Fasta structs
    //
    // # Arguments
    // * `fastas` - A vector of Fasta structs
    // * `k` - The length of the kmer
    //
    // # Returns
    // A vector of tuples containing the headers of the sequences that overlap
    let mut graph = Vec::new();

    for i in fastas.iter() {
        for j in fastas.iter() {
            if i.header != j.header && check_ovelrap(&i.seq, &j.seq, k) {
                graph.push((i.header.clone(), j.header.clone()));
            }
        }
    }
    graph
}

fn generate_formated_output(graph: Vec<(String, String)>) -> String {
    // Generate a formated output from a vector of tuples
    //
    // # Arguments
    // * `graph` - A vector of tuples containing the headers of the sequences that overlap
    //
    // # Returns
    // A string containing the formated output
    let mut output = String::new();

    for (s, p) in graph {
        output.push_str(&format!("{} {}\n", s, p));
    }

    output.pop();

    output
}

pub fn run_overlap_graph(input: Vec<Fasta>, k: usize) -> Result<String, String> {
    // Run the overlap graph problem
    //
    // # Arguments
    // * `input` - A vector of Fasta structs
    // * `k` - The length of the kmer
    //
    // # Returns
    // Result containing a string with the formated output or an error message
    if input.is_empty() {
        return Err(String::from("No input sequences"));
    } else if input.is_empty() {
        return Err(String::from("Only one input sequence"));
    }

    let graph = mk_overlap_graph(input, k);

    Ok(generate_formated_output(graph))
}
