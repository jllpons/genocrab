pub fn run_assembly_quality(mut sequences: Vec<&str>) -> String {
    // Sort sequences by length (shortest first) and calculate total length
    // of all sequences. Then calculate N50 and N75 values.
    //
    // # Arguments
    // * `sequences` - A vector of DNA sequences
    //
    // # Returns
    // * A string containing the N50 and N75 values
    //
    // # Example
    // ```
    // let sequences = vec!["ATGCG", "GCATG", "CATGC", "AGGCA", "GGCAT"];
    // let result = run_assembly_quality(sequences);
    // assert_eq!(result, "5 5");

    sequences.sort_by_key(|s| s.len());

    let total_length = sequences.iter().map(|s| s.len()).sum::<usize>();

    let n50_value = total_length / 2;
    let n75_value = total_length * 3 / 4;
    let mut n50 = 0;
    let mut n75 = 0;
    let mut current_length = 0;

    for i in sequences.iter().rev() {
        current_length += i.len();
        if current_length >= n50_value && n50 == 0 {
            n50 = i.len();
        }
        if current_length >= n75_value && n75 == 0 {
            n75 = i.len();
        }
    }
    format!("{} {}", n50, n75)
}
