# GenoCrab 🦀

**GenoCrab**: the Rust-based toolkit for solving your
[Rosalind](<https://rosalind.info/problems/list-view/>) problems 🧬

## About

GenoCrab brings blazingly fast computing performance to your 
genomics problems. Born out of an MSc Bioinformatics assignment where no one asked for a CLI tool.

## Features

1. **k-Mer Composition**: Display the counts of all posible k-mers of k length
   found in a given sequence ordered lexicographically.
2. **Ovelap Graph**: Given multiple sequences, display the adjency list of ovelaps between the sequencnes.
3. **Shortest Superstring**: Return the shortest possible superstring containing
   all of the input sequences.
4. **De Bruijn Graph**: Constructs the De Bruijn graph from a set of DNA sequences and returns the adjacency list.
5. **Perfect Assembly**: Constructs the shortest possible cyclic superstring from a collection of DNA sequences
6. **Assembly Quality**: Given a collection of reads, output the N50 and N75
   metrics.

## Getting Started

To get started with GenoCrab, simply clone this repository, make sure you have Rust installed, and run the following command:

```shell
cargo run --release
```

## Contributing

Fancy adding a bit of your own magic to GenoCrab? Fork it, tweak it, and send a pull request! All contributions are welcomed.
