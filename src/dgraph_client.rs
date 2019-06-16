use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use dgraph::{make_dgraph, new_dgraph_client, new_secure_dgraph_client};

pub struct Certificates {
    pub root_ca: Vec<u8>,
    pub cert: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub fn make(url: &str, certs: Option<Certificates>) -> dgraph::Dgraph {
    if certs.is_some() {
        let certs = certs.unwrap();

        make_dgraph!(new_secure_dgraph_client(
            url,
            certs.root_ca,
            certs.cert,
            certs.private_key,
        ))
    } else {
        make_dgraph!(new_dgraph_client(url))
    }
}

pub fn open_cert_file(path: &str) -> Vec<u8> {
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
