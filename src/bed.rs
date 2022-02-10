
/// Dummy bed file struct
pub struct bed{
    pub entries: Vec<bed_entry>,
    pub total_len: usize,
    pub accessions: Vec<String>,
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
    pub acc: String,
    pub start: usize,
    pub end: usize,
    pub name: String,

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