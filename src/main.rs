mod app;
mod commands;
mod dgraph_client;
mod error;
mod file;

fn main() {
    let cli_app = app::make();
    let matches = cli_app.get_matches();

    if let Some(config_matches) = matches.subcommand_matches("config") {
        let output_path = config_matches
            .value_of("output_path")
            .expect("Config should contain output_path.");

        commands::config::handler(output_path);
    } else {
        let dgraph_url = matches
            .value_of("url")
            .expect("url value should be present.");
        let dgraph_certs = if matches.is_present("root_ca") {
            Some(dgraph_client::Certificates {
                root_ca: file::open(
                    matches
                        .value_of("root_ca")
                        .expect("root_ca value should be present."),
                ),
                cert: file::open(
                    matches
                        .value_of("cert")
                        .expect("cert value should be present."),
                ),
                private_key: file::open(
                    matches
                        .value_of("private_key")
                        .expect("private_key value should be present."),
                ),
            })
        } else {
            None
        };

        let dgraph_client = dgraph_client::make(dgraph_url, dgraph_certs);

        println!("Using Dgraph at URL: {}\n", dgraph_url);

        if let Some(schema_matches) = matches.subcommand_matches("schema") {
            if let Err(err) = commands::schema::handler(schema_matches, &dgraph_client) {
                eprintln!("{}", err);
            }
        }

        if let Some(alter_matches) = matches.subcommand_matches("alter") {
            let op = ::dgraph::Operation {
                schema: alter_matches
                    .value_of("alter_value")
                    .expect("Alter should contain alter_value.")
                    .to_string(),
                ..Default::default()
            };

            let result = dgraph_client.alter(&op);

            if let Err(err) = result {
                eprintln!("{}", error::Error::DgraphError(err));
            }
        }

        if let Some(query_matches) = matches.subcommand_matches("query") {
            if let Err(err) = commands::query::handler(query_matches, &dgraph_client) {
                eprintln!("{}", err);
            }
        }
    }
}
