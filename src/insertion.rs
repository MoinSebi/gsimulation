pub fn simulate_insertion(fasta: & mut fasta_file, minsize: usize, maxsize: usize, amount: usize, bed1: &mut bed) {
    let mut step: Uniform<u32>;
    step = Uniform::new(0, 10000);
    vals = (0..numb_snp+((0.1*(10000 as f64)) as u32) ).map(|_| rng.sample(&step)).collect();




}