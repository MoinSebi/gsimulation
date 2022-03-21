use rand::{Rng};
use rand::distributions::{Uniform};
use rand::seq::IteratorRandom;
use crate::bed::{Bed, BedEntry};
use crate::fasta::FastaFile; // 0.6.5



/// Simulate SNPs in the new genome
/// No change of sequence
/// Return: Start and end position of SNPs
pub fn snps(fasta: &FastaFile, prob: f64, bed1: &mut Bed) -> FastaFile{
    let mut number_snps = 0;
    let mut rng = rand::thread_rng();



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
    let mut numb_snp: u32;
    let mut len_chr: u32;
    let mut new_fasta = fasta.clone();
    for x in 0..new_fasta.fasta_entry.len(){
        len_chr =  new_fasta.fasta_entry[x].seq.len() as u32;
        numb_snp = (prob*(len_chr as f64)) as u32;
        step = Uniform::new(0, len_chr);
        vals = (0..numb_snp+((0.1*(numb_snp as f64)) as u32) ).map(|_| rng.sample(&step)).collect();
        for y in vals.iter(){
            new_fasta.fasta_entry[x].seq[y.clone() as usize] = test();
            number_snps += 1;
            bed1.entries.push(BedEntry::new2(new_fasta.fasta_entry[x].header.as_str(), y.clone() as usize, (y+1) as usize, "daskdja"));
            bed1.entries.push(BedEntry::new());
        }
    }
    //let hs: HashSet<u32> = vals.iter().cloned().collect();
    //let vals2: Vec<u32> = step.sample_iter(&mut rng).take(12000000).collect();
    //eprintln!("{} {}", vals.len(), hs.len());
    //eprintln!("{:?}", vals);

    //eprintln!("{}", numberSNPs);


    new_fasta
}

pub fn test() -> char {
    let g = "ATCG";
    let mut rng = rand::thread_rng();
    let faces = g.chars().choose(&mut rng).unwrap();

    faces
}
