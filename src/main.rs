use std::io::Read;

use clap::Parser;

mod cli;
mod kmer;

struct Fasta {
    name: String,
    seq: String,
}
impl Fasta {
    fn new(name: String, seq: String) -> Self {
        Self { name, seq }
    }
    fn from_string(fasta: String) -> Self {
        let mut lines = fasta.split('\n');
        let name = lines.next().unwrap().to_string();
        let seq = lines.collect::<Vec<&str>>().join("");
        Self::new(name, seq)
    }
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
        cli::Commands::Overlap => {
            eprintln!("Overlap not implemented yet");
            std::process::exit(1);
        }
        cli::Commands::Superstring => {
            eprintln!("Superstring not implemented yet");
            std::process::exit(1);
        }
        cli::Commands::Debruijn => {
            eprintln!("Debruijn not implemented yet");
            std::process::exit(1);
        }
        cli::Commands::PerfectAssembly => {
            eprintln!("PerfectAssembly not implemented yet");
            std::process::exit(1);
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
