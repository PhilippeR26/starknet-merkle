use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct Merkle {
    name: String,
    age: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct MerkleTree {
    name: String,
    age: Vec<u8>,
}

fn main() {
    let file_write = File::create("./storage/merkle2.json").expect("Can't create file.");
    let my_merkle = MerkleTree {
        name: String::from("Zorg"),
        age: vec![56, 57],
    };
    serde_json::to_writer(file_write, &my_merkle).expect("Pb during writing.");

    let file_read = File::open("./storage/merkle.json").expect("file should open read only");
    let reader = BufReader::new(file_read); // read/only
    let merk2: MerkleTree = serde_json::from_reader(reader).expect("JSON was not well-formatted");
    let file_read2 = File::open("./storage/merkle.json").expect("file should open read only");
    let json2: serde_json::Value =
        serde_json::from_reader(file_read2).expect("JSON was not well-formatted");
    let first_name = json2.get("name").expect("file should have name key");
    let a = first_name.as_str().unwrap();

    println!("{}", merk2.name);
    println!("{}", a);
}
