use std::collections::HashMap;

pub struct Signature {
    typ: String,
    team: usize,
    alliance: usize,
    event: String,
    matchn: usize,
}

impl Signature {
    pub fn from(input: String) -> Result<Signature, String> {
        todo!()
    }
    pub fn drain(input: &String, location: usize) -> Result<(usize, Signature), String> {
        todo!()
    }
}

pub struct Entry {
    signature: Signature,
    data: HashMap<String, Data>,
}

impl Entry {
    pub fn from(input: String) -> Result<Entry, String> {
        todo!()
    }
}

pub enum Data {
    Integer(i32),
    Float(f64),
    Boolean(bool),
    Text(String),
    Note(usize, String),
    Position(usize, usize),
}