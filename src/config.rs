use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    url: String,
    root_ca_path: Option<String>,
    cert_path: Option<String>,
    private_key_path: Option<String>,
}

pub fn handler(output_path: &str) {
    let dgraph_url = prompt("Dgraph URL", Some("localhost:9080"));
    let is_secure = prompt("Is secure connection? [y/n]", Some("y"));

    let mut root_ca_path = None;
    let mut cert_path = None;
    let mut private_key_path = None;

    if is_secure == "y" {
        let root_ca_path_string = prompt("Root CA", None);
        check_path(&root_ca_path_string);
        root_ca_path = Some(root_ca_path_string);

        let cert_path_string = prompt("Certificate", None);
        check_path(&cert_path_string);
        cert_path = Some(cert_path_string);

        let private_key_path_string = prompt("Private key", None);
        check_path(&private_key_path_string);
        private_key_path = Some(private_key_path_string);
    }

    let config = Config {
        url: dgraph_url,
        root_ca_path: root_ca_path,
        cert_path: cert_path,
        private_key_path: private_key_path,
    };

    let config_string = toml::to_string(&config).expect("Convert config to TOML.");

    let mut file = File::create(output_path).unwrap();
    file.write_all(&config_string.as_bytes()).unwrap();
}

fn prompt(text: &str, default_value: Option<&str>) -> String {
    if default_value.is_some() {
        print!(
            "{} ({}): ",
            text,
            default_value.expect("default_value to be present.")
        );
    } else {
        print!("{}: ", text);
    }

    stdout().flush().unwrap();

    let mut value = String::new();

    match stdin().read_line(&mut value) {
        Ok(_) => {
            if value.len() == 1 {
                value = if default_value.is_some() {
                    default_value
                        .expect("default_value to be present.")
                        .to_owned()
                } else {
                    prompt(text, default_value)
                }
            } else {
                value.pop();
            }
        }
        Err(error) => eprintln!("{}", error),
    }

    value
}

fn check_path(path: &str) {
    if !Path::new(path).exists() {
        println!("Note that path {} doesn't exist.", path);
    }
}