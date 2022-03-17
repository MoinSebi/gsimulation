use crate::fasta::fasta_file;
use rand::distributions::Uniform;
use crate::bed::bed;


/// Add insertions to the new genome
pub fn simulate_insertion(fasta: & mut fasta_file, minsize: usize, maxsize: usize, amount: usize, bed1: &mut bed) {
    // How to change genomes and how much per genome
    let mut step: Uniform<u32>;
    step = Uniform::new(minsize as u32, maxsize as u32);
    //vals = (0..numb_snp+((0.1*(10000 as f64)) as u32) ).map(|_| rng.sample(&step)).collect();




}