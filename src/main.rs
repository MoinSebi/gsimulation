mod fasta;
mod snp;
mod bed;
mod insertion;

use std::io::stdin;
use fastq::Parser;
use crate::fasta::{read_fasta, read_fasta2, fasta_file};
use crate::snp::snps;

fn main() {
    println!("Hello, world!");
    eprintln!("Read fasta");
    let mut fasta: fasta_file = fasta_file::new();
    fasta.from_file("/home/svorbrugg_local/Rust/data/TAIR10.fasta");
    //let fasta = read_fasta("/home/svorbrugg_local/Rust/data/TAIR10.fasta");
    eprintln!("SNPs");
    let mut bed1 = bed::bed::new();
    let g = snps(&fasta, 0.001, & mut bed1);
    bed1.to_file("test.csv");
}





/*
Input fasta and parameterfile or command line

Strings are not replaceable
1. Always rerun and write new file
2. Store everything in char vector then change this

snps easy =
indels = remove or add sequence

translocation = extract vector
add vector



vector os string

Add command line
Add SNP
Think about reference structure for populaton struture (azbe later)
add insertions and deletions
add translocations
add inversions
tandem duplication
distal duplication

remove genomes (how to find distal and tandem duplications and then revert functionality)


check difference tranlocation and translatation


output
fasta
with rename if wanted
bed file with output
maybe vcf


 */