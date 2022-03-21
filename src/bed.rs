use std::fs::File;
use std::io::{Write, BufWriter};

/// Dummy bed file struct
pub struct Bed {
    pub entries: Vec<BedEntry>,
    pub total_len: usize,
    pub accessions: Vec<String>,
}

impl Bed {
    pub fn new() -> Self{
        Self{
            entries: Vec::new(),
            total_len: 0,
            accessions: Vec::new(),
        }
    }


    pub fn to_file(self: &Self, filename: &str){
        let file =  File::create(filename).expect("Unable to create file");
        let mut file = BufWriter::new(file);

        for x in 0..self.entries.len(){
            write!(file, "{}\t{}\t{}\t{}\n", self.entries[x].acc, self.entries[x].start, self.entries[x].end, self.entries[x].name).expect("Not working");
        }
    }


}


pub struct BedEntry {
    pub acc: String,
    pub start: usize,
    pub end: usize,
    pub name: String,

}

impl BedEntry {
    pub fn new() -> Self{
        Self{
            acc: "".to_string(),
            start: 0,
            end: 0,
            name: "".to_string(),
        }

    }
    pub fn new2(s1: &str, start: usize, end: usize, name: &str) -> Self{
        Self{
            acc: s1.to_string(),
            start: start,
            end: end,
            name: name.to_string()
        }
    }
}