use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path as file_path;
use std::collections::HashSet;


#[derive(Debug, Clone)]
pub struct fasta_file{
    pub fasta_entry: Vec<fasta>,
    pub number_entry: usize,
    pub is_valid: bool,
    pub total_len: usize,

}

impl fasta_file{
    pub fn new() -> Self {
        Self{
            fasta_entry: Vec::new(),
            number_entry: 0,
            is_valid: true,
            total_len: 0,
        }
    }

    pub fn count(self: & mut Self){
        self.number_entry = self.fasta_entry.len();
    }

    pub fn check_valid(self: & mut Self){
        let mut names = HashSet::new();
        for x in self.fasta_entry.iter(){
            names.insert(x.header.clone());
        }
        if names.len() != self.fasta_entry.len(){
            self.is_valid = false;
        }
    }

    pub fn set_entry_len(self: & mut Self){
        for x in 0..self.fasta_entry.len(){
            self.fasta_entry[x].len = self.fasta_entry[x].seq.len();
        }
    }

    pub fn check_len(self: & mut Self){
        let mut lens = 0;
        for x in self.fasta_entry.iter(){
            lens += x.len;
        }
        self.total_len = lens
    }

    pub fn from_file(self: & mut Self, file: &str){

        let mut fasta = fasta::new();
        if file_path::new(file).exists() {
            let file = File::open(file).expect("ERROR: CAN'T OPEN FILE");
            let reader = BufReader::new(file);


            let mut fasta = fasta::new();

            for line in reader.lines(){
                let l = line.unwrap();
                if l.starts_with(">"){

                    self.fasta_entry.push(fasta);

                    fasta = fasta::new();
                    fasta.header = l;
                    fasta.seq = Vec::new();
                } else {
                    for x in l.chars(){
                        fasta.seq.push(x);
                    }
                }


            }




        } else {
            eprintln!("No file")
        }
        eprintln!("ss {}", self.fasta_entry.len());
        self.fasta_entry.remove(0);
        self.check_len();
        eprintln!("{}", self.fasta_entry[0].seq.len());
        eprintln!("ss {}", self.fasta_entry.len());

    }

}

#[derive(Debug, Clone)]
pub struct fasta{
    pub header: String,
    pub seq: Vec<char>,
    pub len: usize,

}

impl fasta{
    pub fn new() -> Self{
        Self{
            header: "".to_string(),
            seq: Vec::new(),
            len: 0,
        }
    }

    pub fn from_file(file: &str){
        let mut fasta: Vec<(String, Vec<char>)> = Vec::new();


        if file_path::new(file).exists() {
            let file = File::open(file).expect("ERROR: CAN'T OPEN FILE");
            let reader = BufReader::new(file);


            let mut seq: Vec<char> = Vec::new();
            let mut header: String = "".to_string();

            for line in reader.lines(){
                let l = line.unwrap();
                if l.starts_with(">"){
                    fasta.push((header, seq));
                    header = l;
                    seq = Vec::new()
                } else {
                    for x in l.chars(){
                        seq.push(x);
                    }
                }


            }
            fasta.push((header.clone(), seq.clone()));




        } else {
            eprintln!("No file")
        }
        fasta.remove(0);
        eprintln!("{:?}", fasta.len());

    }

}


pub fn read_fasta(file: &str) -> Vec<(String, String)>{
    let mut fasta: Vec<(String, String)> = Vec::new();
    if file_path::new(file).exists() {
        let file = File::open(file).expect("ERROR: CAN'T OPEN FILE");
        let reader = BufReader::new(file);


        let mut seq: String = "".to_string();
        let mut header: String = "".to_string();

        for line in reader.lines(){
            let l = line.unwrap();
            if l.starts_with(">"){
                fasta.push((header, seq));
                header = l.clone();
                seq = "".to_string();
            } else {
                seq.push_str(&l);
            }


        }
        fasta.push((header.clone(), seq.clone()));




    } else {
        eprintln!("No file")
    }
    fasta.remove(fasta.len()-1);
    eprintln!("{:?}", fasta.len());
    fasta
}

pub fn read_fasta2(file: &str) -> Vec<(String, Vec<char>)>{
    let mut fasta: Vec<(String, Vec<char>)> = Vec::new();


    if file_path::new(file).exists() {
        let file = File::open(file).expect("ERROR: CAN'T OPEN FILE");
        let reader = BufReader::new(file);


        let mut seq: Vec<char> = Vec::new();
        let mut header: String = "".to_string();

        for line in reader.lines(){
            let l = line.unwrap();
            if l.starts_with(">"){
                fasta.push((header, seq));
                header = l;
                seq = Vec::new()
            } else {
                for x in l.chars(){
                    seq.push(x);
                }
            }


        }
        fasta.push((header.clone(), seq.clone()));




    } else {
        eprintln!("No file")
    }
    fasta.remove(fasta.len()-1);
    eprintln!("{:?}", fasta.len());
    fasta
}
