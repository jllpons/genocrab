use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "genocrab",
    version = "0.0.0",
    author = "Joan Lluis Pons <joanlluis@gmail.com>",
    about = "The tool for solving the Rosalind problems \
             from the command line that none asked for.",
    long_about = None,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Return the number of times each kmer appears in the input sequence
    Kmer {
        /// The input sequence (optional, reads from stdin if not present)
        #[clap(default_value = "-")]
        input: PathBuf,
        /// The length of the kmers
        #[arg(short)]
        k: usize,
    },
    /// Return an adjacency list of the overlap graph of the input sequences
    Overlap {
        /// The input sequencs (optional, reads from stdin if not present)
        #[clap(default_value = "-")]
        input: PathBuf,
        /// The length of overlap between sequences
        #[arg(short)]
        k: usize,
    },
    /// Return the shortest possible superstring containing all the input sequences
    Superstring {
        /// The input sequencs (optional, reads from stdin if not present)
        #[clap()]
        input: Option<PathBuf>,
    },
    // Constructs the De Bruijn graph from a set of DNA sequences and returns the adjacency list.
    Debruijn {
        /// The input sequence (optional, reads from stdin if not present)
        #[clap()]
        input: Option<PathBuf>,
        /// Include reverse complements of the sequences in the graph construction
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        reverse_complement: bool,
    },
    /// WIP
    PerfectAssembly,
    /// WIP
    AssemblyQuality,
}
