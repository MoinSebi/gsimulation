use rand::{Rng, thread_rng};
use std::collections::HashSet;
use rand::distributions::{Distribution, Uniform};
use rand::seq::IteratorRandom; // 0.6.5



/// Simulate SNPs in the new genome
/// No change of sequence
/// Return: Start and end position of SNPs
pub fn snps(fasta: & Vec<(String, Vec<char>)>, prob: f64){
    let step = Uniform::new(0.0, 1.0);
    let mut numberSNPs = 0;



    let mut new_fasta = fasta.clone();
    let mut rng = rand::thread_rng();
    for x in 0..new_fasta.len(){
        for y in 0..new_fasta[x].1.len(){
            let choice = step.sample(&mut rng);
            if choice < prob{
                new_fasta[x].1[y] = test();
                numberSNPs += 1;
            }
        }
    }

    eprintln!("{}", numberSNPs);



}

pub fn test() -> char {
    let G = "ATCG";
    let mut rng = rand::thread_rng();
    let faces = G.chars().choose(&mut rng).unwrap();

    faces
}
