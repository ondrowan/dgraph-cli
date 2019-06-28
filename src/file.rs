use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn open(path: &str) -> Vec<u8> {
    if let Result::Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader
            .read_to_string(&mut contents)
            .unwrap_or_else(|_| panic!("Read contents of file {} into string.", path));

        contents.into_bytes()
    } else {
        panic!("File {} not found!", path);
    }
}
