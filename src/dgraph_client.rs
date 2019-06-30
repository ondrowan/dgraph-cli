use dgraph::{make_dgraph, new_dgraph_client, new_secure_dgraph_client};

pub struct Certificates {
    pub root_ca: Vec<u8>,
    pub cert: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub fn make(url: &str, certs: Option<Certificates>) -> dgraph::Dgraph {
    if certs.is_some() {
        let certs = certs.expect("Should contain certs structure.");

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
