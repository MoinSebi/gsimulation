use crate::fasta::FastaFile;
use rand::distributions::Uniform;
use crate::bed::Bed;

#[allow(dead_code)]
/// Add insertions to the new genome
pub fn simulate_insertion(_fasta: & mut FastaFile, minsize: usize, maxsize: usize, _amount: usize, _bed1: &mut Bed) {
    // How to change genomes and how much per genome
    let _step: Uniform<u32> = Uniform::new(minsize as u32, maxsize as u32);
    //vals = (0..numb_snp+((0.1*(10000 as f64)) as u32) ).map(|_| rng.sample(&step)).collect();




}