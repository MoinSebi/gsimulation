mod fasta;
mod snp;
mod bed;
mod insertion;

use chrono::Local;
use clap::{App, Arg};
use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use crate::fasta::{FastaFile};
use crate::snp::snps;
use std::io::Write;


fn main() {

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .target(Target::Stderr)
        .init();

    let _matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("packing")
        .arg(Arg::new("fasta")
            .short('f')
                 .long("fasta")
                 .about("hello")
                 .takes_value(true))
        .get_matches();


    // Collect the name
    info!("Genome simulation");


    let mut fasta: FastaFile = FastaFile::new();
    fasta.from_file("/home/svorbrugg_local/Rust/data/TAIR10.fasta");
    //let fasta = read_fasta("/home/svorbrugg_local/Rust/data/TAIR10.fasta");

    info!("Add SNPs");
    let mut bed1 = bed::Bed::new();
    let _g = snps(&fasta, 0.001, & mut bed1);
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