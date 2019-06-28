mod app;
mod commands;
mod dgraph_client;
mod error;
mod file;

fn main() {
    let cli_app = app::make();
    let matches = cli_app.get_matches();

    if let Some(config_matches) = matches.subcommand_matches("config") {
        let output_path = config_matches.value_of("output_path").unwrap();

        commands::config::handler(output_path);
    } else {
        let dgraph_url = matches.value_of("url").unwrap();
        let dgraph_certs = if matches.is_present("root_ca") {
            Some(dgraph_client::Certificates {
                root_ca: file::open(matches.value_of("root_ca").unwrap()),
                cert: file::open(matches.value_of("cert").unwrap()),
                private_key: file::open(matches.value_of("private_key").unwrap()),
            })
        } else {
            None
        };

        let dgraph_client = dgraph_client::make(dgraph_url, dgraph_certs);

        println!("Using Dgraph at URL: {}\n", dgraph_url);

        if let Some(schema_matches) = matches.subcommand_matches("schema") {
            commands::schema::handler(schema_matches, &dgraph_client);
        }

        if let Some(alter_matches) = matches.subcommand_matches("alter") {
            let op = ::dgraph::Operation {
                schema: alter_matches.value_of("alter_value").unwrap().to_string(),
                ..Default::default()
            };

            let result = dgraph_client.alter(&op);

            if let Err(failure_err) = result {
                error::parse(failure_err);
            }
        }

        if let Some(query_matches) = matches.subcommand_matches("query") {
            commands::query::handler(query_matches, &dgraph_client);
        }
    }
}
