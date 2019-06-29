use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Opens file and reads its contents. Its path is expected to be already
/// validated.
pub fn open(path: &str) -> Vec<u8> {
    let file = File::open(path).expect(&format!("File {} to be loaded", path));
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect(&format!("Read contents of file {} into string.", path));

    contents.into_bytes()
}

pub fn file_exists_validator(path: String) -> Result<(), String> {
    if Path::new(&path).exists() {
        Ok(())
    } else {
        Err(String::from(format!("File {} does not exist.", path)))
    }
}
