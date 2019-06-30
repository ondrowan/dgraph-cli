use clap::ArgMatches;
use dgraph;

use crate::error;

pub fn handler(query_matches: &ArgMatches, dgraph_client: &dgraph::Dgraph) {
    let mut txn = dgraph_client.new_readonly_txn();
    let result = txn.query(query_matches.value_of("query_value").expect("Query should contain query_value.").to_string());

    match result {
        Ok(response) => {
            if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response.json) {
                if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                    println!("{}", pretty_json);
                }

                let latency = response.get_latency();

                println!(
                    "\nLatency:\n\nProcessing: {}\nParsing: {}\nEncoding: {}",
                    convert_and_format_ns(latency.processing_ns),
                    convert_and_format_ns(latency.parsing_ns),
                    convert_and_format_ns(latency.encoding_ns)
                );
            }
        }
        Err(failure_err) => error::parse(failure_err),
    }
}

fn convert_and_format_ns(time: u64) -> std::string::String {
    let ms = time as f64 / 1_000_000.0;

    if ms > 1000 as f64 {
        let s = ms / 1000 as f64;

        return format!("{}s", s);
    }

    format!("{}ms", ms)
}
