
/// Dummy bed file struct
pub struct bed{
    entries: Vec<bed_entry>,
    total_len: usize,
    accessions: Vec<String>,
}

impl bed{
    pub fn new() -> Self{
        Self{
            entries: Vec::new(),
            total_len: 0,
            accessions: Vec::new(),
        }
    }


}


pub struct bed_entry{
    acc: String,
    start: usize,
    end: usize,
    name: String,

}

impl bed_entry{
    pub fn new() -> Self{
        Self{
            acc: "".to_string(),
            start: 0,
            end: 0,
            name: "".to_string(),
        }

    }
}