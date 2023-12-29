use std::io::Read;

use atty::Stream;
use clap::Parser;

mod cli;
mod debruijn;
mod kmer;
mod overlap;
mod perfect_assembly;
mod superstring;

#[derive(Debug)]
pub struct Fasta {
    // Struct to hold a fasta sequence
    header: String,
    seq: String,
}
impl Fasta {
    fn new(header: String, seq: String) -> Self {
        // Create a new Fasta struct
        //
        // # Arguments
        //
        // * `header` - The header of the sequence
        // * `seq` - The sequence itself
        //
        // # Returns
        // A new Fasta struct
        Self { header, seq }
    }
    fn from_string(fasta: String) -> Self {
        // Create a new Fasta struct from a string
        //
        // # Arguments
        // * `fasta` - A string containing a fasta sequence
        //
        // # Returns
        // A new Fasta struct
        let mut lines = fasta.split('\n');

        let header = lines.next().unwrap().to_string();
        let seq = lines.collect::<Vec<&str>>().join("");

        Self::new(header, seq)
    }
}

fn read_mulitfasta(fasta: String) -> Vec<Fasta> {
    // Create a vector of Fasta structs from a multi-fasta string
    //
    // # Arguments
    // * `fasta` - A string containing a multi-fasta sequence
    //
    // # Returns
    // A vector of Fasta structs
    let mut fastas = Vec::new();

    let lines = fasta.split('\n');

    let mut header = String::new();
    let mut seq = String::new();

    for line in lines {
        if line.starts_with('>') {
            if !seq.is_empty() {
                fastas.push(Fasta::new(header, seq));
            }
            header = line.replace('>', "").trim().to_string();
            seq = String::new();
        } else {
            seq.push_str(line);
        }
    }
    if !seq.is_empty() {
        fastas.push(Fasta::new(header, seq));
    }
    fastas
}

fn main() {
    // Get the command line arguments
    let args = cli::Cli::parse();

    // Run the appropriate command
    match args.command {
        //
        // Run the kmer operation
        //
        cli::Commands::Kmer { input, k } => {
            let input = match input.to_str() {
                // TODO: Make this a function
                // TODO: Handle errors in this operation
                Some("-") => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();
                    buffer
                }
                Some(path) => std::fs::read_to_string(path).unwrap(),
                None => {
                    eprintln!("Invalid input path");
                    std::process::exit(1);
                }
            };
            let fasta = Fasta::from_string(input);

            // Run the kmer function and match the result
            match kmer::run_kmers(fasta.seq, k) {
                Ok(result) => {
                    println!("{}", result);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::Overlap { input, k } => {
            let input = match input.to_str() {
                Some("-") => {
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();
                    buffer
                }
                Some(path) => std::fs::read_to_string(path).unwrap(),
                None => {
                    eprintln!("Invalid input path");
                    std::process::exit(1);
                }
            };

            let fastas = read_mulitfasta(input);

            let result = overlap::run_overlap_graph(fastas, k);
            match result {
                Ok(result) => {
                    println!("{}", result);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::Superstring { input } => {
            let input = match input {
                Some(path) => std::fs::read_to_string(path).unwrap(),
                None => {
                    if atty::is(Stream::Stdin) {
                        eprintln!("Error: No input provided");
                        std::process::exit(1);
                    }
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();

                    buffer
                }
            };

            let fastas = read_mulitfasta(input);

            let result = superstring::run_superstring(fastas);
            match result {
                Ok(result) => {
                    println!("{}", result);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::Debruijn {
            input,
            rc
        } => {
            let input = match input {
                Some(path) => std::fs::read_to_string(path).unwrap(),
                None => {
                    if atty::is(Stream::Stdin) {
                        eprintln!("Error: No input provided");
                        std::process::exit(1);
                    }
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();

                    buffer
                }
            };

            let mut sequences = input.split('\n').collect::<Vec<&str>>();
            for seq in &mut sequences {
                *seq = seq.trim();
            }
            sequences = sequences.into_iter().filter(|s| !s.is_empty()).collect();

            let result = debruijn::run_debruijn_graph(sequences, rc);
            match result {
                Ok(result) => {
                    print!("{}", result);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::PerfectAssembly { input } => {
            let input = match input {
                Some(path) => std::fs::read_to_string(path).unwrap(),
                None => {
                    if atty::is(Stream::Stdin) {
                        eprintln!("Error: No input provided");
                        std::process::exit(1);
                    }
                    let mut buffer = String::new();
                    std::io::stdin().read_to_string(&mut buffer).unwrap();

                    buffer
                }
            };

            let mut sequences = input.split('\n').collect::<Vec<&str>>();
            for seq in &mut sequences {
                *seq = seq.trim();
            }
            sequences = sequences.into_iter().filter(|s| !s.is_empty()).collect();

            let result = perfect_assembly::run_perfect_assembly(sequences);
            match result {
                Ok(result) => {
                    println!("{}", result);
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::AssemblyQuality => {
            eprintln!("AssemblyQuality not implemented yet");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmer() {
        let seq = String::from(
            "CTTCGAAAGTTTGGGCCGAGTCTTACAGTCGGTCTTGAAGCAAAGTAACGAACTCCACGGCCCTGACTACCGAACCAGTTGTGAGTACTCAACTGGGTGAGAGTGCAGTCCCTATTGAGTTTCCGAGACTCACCGGGATTTTCGATCCAGCCTCAGTCCAGTCTTGTGGCCAACTCACCAAATGACGTTGGAATATCCCTGTCTAGCTCACGCAGTACTTAGTAAGAGGTCGCTGCAGCGGGGCAAGGAGATCGGAAAATGTGCTCTATATGCGACTAAAGCTCCTAACTTACACGTAGACTTGCCCGTGTTAAAAACTCGGCTCACATGCTGTCTGCGGCTGGCTGTATACAGTATCTACCTAATACCCTTCAGTTCGCCGCACAAAAGCTGGGAGTTACCGCGGAAATCACAG",
        );
        let k = 4;
        let expected = String::from(
            "4 1 4 3 0 1 1 5 1 3 1 2 2 1 2 0 1 1 3 1 \
                                    2 1 3 1 1 1 1 2 2 5 1 3 0 2 2 1 1 1 1 3 1 \
                                    0 0 1 5 5 1 5 0 2 0 2 1 2 1 1 1 2 0 1 0 0 \
                                    1 1 3 2 1 0 3 2 3 0 0 2 0 8 0 0 1 0 2 1 3 \
                                    0 0 0 1 4 3 2 1 1 3 1 2 1 3 1 2 1 2 1 1 1 \
                                    2 3 2 1 1 0 1 1 3 2 1 2 6 2 1 1 1 2 3 3 3 \
                                    2 3 0 3 2 1 1 0 0 1 4 3 0 1 5 0 2 0 1 2 1 \
                                    3 0 1 2 2 1 1 0 3 0 0 4 5 0 3 0 2 1 1 3 0 \
                                    3 2 2 1 1 0 2 1 0 2 2 1 2 0 2 2 5 2 2 1 1 \
                                    2 1 2 2 2 2 1 1 3 4 0 2 1 1 0 1 2 2 1 1 1 \
                                    5 2 0 3 2 1 1 2 2 3 0 3 0 1 3 1 2 3 0 2 1 \
                                    2 2 1 2 3 0 1 2 3 1 1 3 1 0 1 1 3 0 2 1 2 \
                                    2 0 2 1 1",
        );

        assert_eq!(kmer::run_kmers(seq, k).unwrap(), expected);
    }
}
