use std::collections::HashSet;
use std::iter::FromIterator;

use clap::ArgMatches;
use dgraph;

use crate::error;

pub fn handler(schema_matches: &ArgMatches, dgraph_client: &dgraph::Dgraph) {
    let wanted_fields: HashSet<&str> = if schema_matches.is_present("fields") {
        schema_matches
            .values_of("fields")
            .expect("Schema should contain fields value.")
            .collect()
    } else {
        [
            "predicate",
            "type",
            "index",
            "reverse",
            "tokenizer",
            "list",
            "count",
            "upsert",
            "lang",
        ]
        .iter()
        .cloned()
        .collect()
    };

    let query = format!(
        "schema {{ {} }}",
        Vec::<&str>::from_iter(wanted_fields.iter().cloned()).join(" ")
    );
    let mut txn = dgraph_client.new_txn();
    let response = txn.query(query);

    match response {
        Ok(response) => {
            print_table(wanted_fields, response.get_schema());
        }
        Err(failure_err) => error::parse(failure_err),
    }
}

fn print_table(wanted_fields: HashSet<&str>, schema: &[dgraph::SchemaNode]) {
    let gap_width: usize = 3;
    let mut predicate_column_width: usize = 9 + gap_width;
    let type_column_width: usize = 10;
    let index_column_width: usize = 5 + gap_width;
    let reverse_column_width: usize = 7 + gap_width;
    let mut tokenizers_column_width: usize = 10 + gap_width;
    let list_column_width: usize = 5 + gap_width;
    let count_column_width: usize = 5 + gap_width;
    let upsert_column_width: usize = 6 + gap_width;
    let lang_column_width: usize = 5 + gap_width;
    let mut total_width: usize = 0;

    if wanted_fields.contains("predicate") {
        let mut longest_predicate_size: usize = predicate_column_width;

        for item in schema.into_iter() {
            let len = item.predicate.len() + gap_width;

            if len > longest_predicate_size {
                longest_predicate_size = len;
            }
        }

        predicate_column_width = longest_predicate_size;
        total_width += predicate_column_width;

        print!("{:width$}", "Predicate", width = &predicate_column_width);
    }

    if wanted_fields.contains("type") {
        total_width += type_column_width;

        print!("{:width$}", "Type", width = &type_column_width);
    }

    if wanted_fields.contains("index") {
        total_width += index_column_width;

        print!("{:width$}", "Index", width = &index_column_width);
    }

    if wanted_fields.contains("reverse") {
        total_width += reverse_column_width;

        print!("{:width$}", "Reverse", width = &reverse_column_width);
    }

    if wanted_fields.contains("tokenizer") {
        let mut longest_tokenizers_size: usize = tokenizers_column_width;

        for item in schema.into_iter() {
            let len = item.tokenizer.join(", ").len() + gap_width;

            if len > longest_tokenizers_size {
                longest_tokenizers_size = len;
            }
        }

        tokenizers_column_width = longest_tokenizers_size;
        total_width += tokenizers_column_width;

        print!("{:width$}", "Tokenizers", width = &tokenizers_column_width);
    }

    if wanted_fields.contains("list") {
        total_width += list_column_width;

        print!("{:width$}", "List", width = &list_column_width);
    }

    if wanted_fields.contains("count") {
        total_width += count_column_width;

        print!("{:width$}", "Count", width = &count_column_width);
    }

    if wanted_fields.contains("upsert") {
        total_width += upsert_column_width;

        print!("{:width$}", "Upsert", width = &upsert_column_width);
    }

    if wanted_fields.contains("lang") {
        total_width += lang_column_width;

        print!("{:width$}", "Lang", width = &lang_column_width);
    }

    println!();

    println!("{}", "-".repeat(total_width));

    for item in schema.into_iter() {
        if wanted_fields.contains("predicate") {
            print!("{:width$}", item.predicate, width = &predicate_column_width);
        }

        if wanted_fields.contains("type") {
            print!("{:width$}", item.field_type, width = &type_column_width);
        }

        if wanted_fields.contains("index") {
            print!("{:width$}", item.index, width = &index_column_width);
        }

        if wanted_fields.contains("reverse") {
            print!("{:width$}", item.reverse, width = &reverse_column_width);
        }

        if wanted_fields.contains("tokenizer") {
            print!(
                "{:width$}",
                item.tokenizer.join(", "),
                width = &tokenizers_column_width
            );
        }

        if wanted_fields.contains("list") {
            print!("{:width$}", item.list, width = &list_column_width);
        }

        if wanted_fields.contains("count") {
            print!("{:width$}", item.count, width = &count_column_width);
        }

        if wanted_fields.contains("upsert") {
            print!("{:width$}", item.upsert, width = &upsert_column_width);
        }

        if wanted_fields.contains("lang") {
            print!("{:width$}", item.lang, width = &lang_column_width);
        }

        println!();
    }
}
