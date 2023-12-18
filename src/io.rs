use std::fs;
use std::fs::File;
use std::io::Read;


pub fn read_bytes_from_file(filename: &String) -> Vec<u8> {
    let mut f = File::open(filename).expect("Could not read file");
    let metadata = fs::metadata(filename).expect("Could not read metadata");
    let size = metadata.len() as usize;
    let mut buffer = vec![0; size];
    f.read_exact(&mut buffer).expect("Buffer overflow");

    buffer
}