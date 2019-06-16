use clap::{App, AppSettings, Arg, SubCommand};

pub fn make<'a, 'b>() -> App<'a, 'b> {
    App::new("Dgraph CLI")
        .version("0.1.0")
        .author("Ondrej Slinták <ondrowan@gmail.com>")
        .about("CLI utility for communication with Dgraph.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("url")
                .help("URL of Dgraph server.")
                .long("url")
                .short("u")
                .takes_value(true)
                .default_value("localhost:9080"),
        )
        .arg(
            Arg::with_name("root_ca")
                .help("Path to Root CA certificate.")
                .long("root_ca")
                .takes_value(true)
                .requires_all(&["cert", "private_key"]),
        )
        .arg(
            Arg::with_name("cert")
                .help("Path to certificate.")
                .long("cert")
                .takes_value(true)
                .requires_all(&["root_ca", "private_key"]),
        )
        .arg(
            Arg::with_name("private_key")
                .help("Path to private key.")
                .long("private_key")
                .takes_value(true)
                .requires_all(&["root_ca", "cert"]),
        )
        .subcommand(
            SubCommand::with_name("schema")
                .about("Outputs schema formatted in a table.")
                .arg(
                    Arg::with_name("fields")
                        .long("fields")
                        .takes_value(true)
                        .multiple(true)
                        .possible_values(&[
                            "predicate",
                            "type",
                            "index",
                            "reverse",
                            "tokenizer",
                            "list",
                            "count",
                            "upsert",
                            "lang",
                        ]),
                ),
        )
        .subcommand(
            SubCommand::with_name("alter")
                .about("Alters predicate in schema.")
                .arg(Arg::with_name("alter_value").index(1).required(true)),
        )
}
