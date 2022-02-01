
/// Dummy bed file struct
pub struct bed{
    entries: Vec<bed_entry>,
    total_len: usize,
    accessions: Vec<String>,
}


pub struct bed_entry{
    acc: String,
    start: usize,
    end: usize,
    name: String,
}