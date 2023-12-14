use std::collections::HashMap;

fn populate_kmers(depth: usize, base: String, kmers: &mut Vec<String>, k: usize) {
    // Use recursion to generate all possible DNA kmers of length k
    // and store them in a vector. The vector is passed and modified
    // by reference
    //
    // # Arguments
    //
    // * `depth` - the current depth of the recursion
    // * `base` - the current base string
    // * `kmers` - the vector that will store the kmers
    // * `k` - the length of the kmers
    //
    // # Modifies
    // * `kmers` - the vector that will store the kmers
    if depth == k {
        kmers.push(base);
    } else {
        let bases = vec!["A", "C", "G", "T"];
        for &b in &bases {
            let mut new_base = base.clone();
            new_base.push_str(b);
            populate_kmers(depth + 1, new_base, kmers, k);
        }
    }
}

fn create_kmer_map(kmers: Vec<String>) -> HashMap<String, i32> {
    // Create a HashMap with the kmers as keys and 0 as the value
    //
    // # Arguments
    //
    // * `kmers` - the vector of kmers
    //
    // # Returns
    //
    // * `HashMap<String, i32>` - the HashMap with the kmers as keys and 0 as the value
    let mut kmer_map: HashMap<String, i32> = HashMap::new();
    for kmer in kmers {
        kmer_map.insert(kmer, 0);
    }
    kmer_map
}

// TODO: Handle errors in or before this operation
fn count_kmer_occurrences(
    sequence: String,
    kmer_map: &mut HashMap<String, i32>,
    k: usize,
) -> Result<(), String> {
    // Count the number of occurrences of each kmer in the sequence and
    // update the value in the HashMap
    // If an uncanonical kmer is found, print an error message and exit
    //
    // # Arguments
    //
    // * `sequence` - the DNA sequence
    // * `kmer_map` - the HashMap with the kmers as keys and 0 as the value
    //
    // # Returns
    //
    // * `Result<(), String>` - an empty Ok() if no error is found
    //
    // # Modifies
    // * `kmer_map` - the HashMap with the kmers as keys and
    //                the number of occurrences as the value
    for i in 0..(sequence.len() - k + 1) {
        let kmer = &sequence[i..(i + k)];
        match kmer_map.get_mut(kmer) {
            Some(count) => *count += 1,
            None => return Err(format!("Uncanonical kmer found: {}", kmer)),
        }
    }
    Ok(())
}

fn generate_ordered_counts(kmer_map: HashMap<String, i32>) -> Vec<(String, i32)> {
    // Generate a vector of tuples with the kmers and their counts
    // and sort the vector by the kmers in alphabetical order
    //
    // # Arguments
    // * `kmer_map` - the HashMap with the kmers as keys
    //                and their counts as the values
    //
    // # Returns
    // * `Vec<(String, i32)>` - the vector of tuples with the kmers and their counts
    let mut ordered_counts: Vec<(String, i32)> = kmer_map.into_iter().collect();
    ordered_counts.sort();
    ordered_counts
}

fn generate_result_string(ordered_counts: Vec<(String, i32)>) -> String {
    // Generate a string with the counts of the kmers in the order
    // that they appear in the sequence. The string is space delimited
    // and the last space is removed
    //
    // # Arguments
    //
    // * `ordered_counts` - the vector of tuples with the kmers and their counts
    //                      sorted by the kmers in alphabetical order
    //
    // # Returns
    //
    // * `String` - the string with the counts of the kmers in the order
    let mut result_string = String::new();
    for (_kmer, count) in ordered_counts {
        result_string.push_str(&count.to_string());
        result_string.push(' ');
    }
    result_string.pop();
    result_string
}

pub fn run_kmers(sequence: String, k: usize) -> Result<String, String> {
    let mut kmers: Vec<String> = vec![];
    populate_kmers(0, "".to_string(), &mut kmers, k);

    let mut kmer_map = create_kmer_map(kmers);

    // If `count_kmer_occurrences` returns an error, it will be propagated upwards
    count_kmer_occurrences(sequence.clone(), &mut kmer_map, k)?;

    let ordered_counts = generate_ordered_counts(kmer_map);
    let kmer_counts_string = generate_result_string(ordered_counts);

    Ok(kmer_counts_string)
}
