# GenoCrab 🦀

**GenoCrab**: the Rust-based toolkit for solving your
[Rosalind](<https://rosalind.info/problems/list-view/>) problems 🧬

## About

GenoCrab brings blazingly fast computing performance to your 
genomics problems. Born out of an MSc Bioinformatics assignment where no one asked for a CLI tool.

## Features

1. [**k-Mer Composition**](#kmer): Display the counts of all posible k-mers of k length
   found in a given sequence ordered lexicographically.
2. [**Ovelap Graph**](#overlap): Given multiple sequences, display the adjency list of ovelaps between the sequencnes.
3. [**Shortest Superstring**](#superstring): Return the shortest possible superstring containing
   all of the input sequences.
4. [**De Bruijn Graph**](#debruijn): Constructs the De Bruijn graph from a set of DNA sequences and returns the adjacency list.
5. [**Perfect Assembly**](#perfect-assembly): Constructs the shortest possible cyclic superstring from a collection of DNA sequences
6. [**Assembly Quality**](#assembly-quality): Given a collection of reads, output the N50 and N75
   metrics.

## Getting Started

To get started with GenoCrab, simply clone this repository, make sure you have Rust installed, and run the following command:

```shell
cargo run --release
```

Then, you can execute the binary with: 

```shell
./target/release/genocrab
```

Which should output the following help message:

```text
The tool for solving the Rosalind problems from the command line that none asked for.

Usage: genocrab <COMMAND>

Commands:
  kmer              Return the number of times each kmer appears in the input sequence
  overlap           Return an adjacency list of the overlap graph of the input sequences
  superstring       Return the shortest possible superstring containing all the input sequences
  debruijn          Constructs the De Bruijn graph from a set of DNA sequences and returns the adjacency list
  perfect-assembly  Constructs the shortest possible cyclic superstring from a collection of DNA sequences
  assembly-quality  Given a collection of reads, ouput the N50 and N75 assembly quality metrics
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

### kmer

```
./target/release/genocrab kmer data/kmer_example.fasta -k 4
4 1 4 3 0 1 1 5 1 3 1 2 2 1 2 0 1 1 3 1 2 1 3 1 1 1 1 2 2 5 1 3 0 2 2 1 1 1 1 3 1 0 0 1 5 5 1 5 0 2 0 2 1 2 1 1 1 2 0 1 0 0 1 1 3 2 1 0 3 2 3 0 0 2 0 8 0 0 1 0 2 1 3 0 0 0 1 4 3 2 1 1 3 1 2 1 3 1 2 1 2 1 1 1 2 3 2 1 1 0 1 1 3 2 1 2 6 2 1 1 1 2 3 3 3 2 3 0 3 2 1 1 0 0 1 4 3 0 1 5 0 2 0 1 2 1 3 0 1 2 2 1 1 0 3 0 0 4 5 0 3 0 2 1 1 3 0 3 2 2 1 1 0 2 1 0 2 2 1 2 0 2 2 5 2 2 1 1 2 1 2 2 2 2 1 1 3 4 0 2 1 1 0 1 2 2 1 1 1 5 2 0 3 2 1 1 2 2 3 0 3 0 1 3 1 2 3 0 2 1 2 2 1 2 3 0 1 2 3 1 1 3 1 0 1 1 3 0 2 1 2 2 0 2 1 1
```
### overlap

```
./target/release/genocrab overlap data/graph_example.fasta -k 3
Rosalind_0498 Rosalind_2391
Rosalind_0498 Rosalind_0442
Rosalind_2391 Rosalind_2323
```

### superstring

```
./target/release/genocrab superstring data/superstring_example.fasta
ATTAGACCTGCCGGAATAC
```

###  debruijn

```
> ./target/release/genocrab debruijn -r data/debruijn_example.txt
(ATC, TCA)
(ATG, TGA)
(ATG, TGC)
(CAT, ATC)
(CAT, ATG)
(GAT, ATG)
(GCA, CAT)
(TCA, CAT)
(TGA, GAT)
```

### perfect-assembly

```
./target/release/genocrab perfect-assembly data/perfect_coverage_example.txt
ACAGATT
```

### assembly-quality

```
./target/release/genocrab assembly-quality data/quality_example.txt
7 6
```




