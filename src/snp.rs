use rand::{Rng, thread_rng};
use std::collections::HashSet;
use rand::distributions::{Distribution, Uniform};
use rand::seq::IteratorRandom;
use crate::bed::bed;
use crate::fasta::fasta_file; // 0.6.5



/// Simulate SNPs in the new genome
/// No change of sequence
/// Return: Start and end position of SNPs
pub fn snps(fasta: & fasta_file, prob: f64){
    let step = Uniform::new(0, 120000000);
    let mut numberSNPs = 0;
    let mut rng = rand::thread_rng();
    let mut bed = bed::new();



    // let mut new_fasta = fasta.clone();
    // let mut rng = rand::thread_rng();
    // for x in 0..new_fasta.len(){
    //     for y in 0..new_fasta[x].1.len(){
    //         let choice = step.sample(&mut rng);
    //         if choice < prob{
    //             new_fasta[x].1[y] = test();
    //             numberSNPs += 1;
    //         }
    //     }
    // }

    let mut vals: Vec<u32> ;
    let mut step: Uniform<u32>;
    let mut new_fasta = fasta.clone();
    for x in 0..new_fasta.fasta_entry.len(){
        eprintln!("{}", new_fasta.fasta_entry[x].seq.len() as u32);
        step = Uniform::new(0, new_fasta.fasta_entry[x].seq.len() as u32);
        vals = (0..1000).map(|_| rng.sample(&step)).collect();
        for y in vals.iter(){
            new_fasta.fasta_entry[x].seq[y.clone() as usize] = test();
            numberSNPs += 1;
        }
    }
    //let hs: HashSet<u32> = vals.iter().cloned().collect();
    //let vals2: Vec<u32> = step.sample_iter(&mut rng).take(12000000).collect();
    //eprintln!("{} {}", vals.len(), hs.len());
    //eprintln!("{:?}", vals);

    //eprintln!("{}", numberSNPs);



}

pub fn test() -> char {
    let G = "ATCG";
    let mut rng = rand::thread_rng();
    let faces = G.chars().choose(&mut rng).unwrap();

    faces
}
