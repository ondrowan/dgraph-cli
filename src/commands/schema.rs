use std::collections::HashSet;
use std::iter::FromIterator;

use clap::ArgMatches;
use dgraph;

use crate::error;

#[derive(Debug, serde::Deserialize)]
struct Schema<'a> {
    #[serde(borrow)]
    schema: Vec<SchemaItem<'a>>,
}

#[derive(Debug, serde::Deserialize)]
struct SchemaItem<'a> {
    predicate: &'a str,
    #[serde(alias = "type")]
    ty: Option<&'a str>,
    index: Option<bool>,
    reverse: Option<bool>,
    tokenizer: Option<Vec<&'a str>>,
    list: Option<bool>,
    count: Option<bool>,
    upsert: Option<bool>,
    lang: Option<bool>,
}

pub fn handler(
    schema_matches: &ArgMatches,
    dgraph_client: &dgraph::Dgraph,
) -> Result<(), error::Error> {
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
    let response = txn.query(query)?;
    let data = serde_json::from_slice::<Schema>(response.get_json())?;

    print_table(wanted_fields, &data.schema);

    Ok(())
}

#[allow(clippy::cognitive_complexity)]
fn print_table(wanted_fields: HashSet<&str>, schema: &[SchemaItem]) {
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

    let mut buffer = String::new();

    if wanted_fields.contains("predicate") {
        let mut longest_predicate_size: usize = predicate_column_width;

        for item in schema {
            let len = item.predicate.len() + gap_width;

            if len > longest_predicate_size {
                longest_predicate_size = len;
            }
        }

        predicate_column_width = longest_predicate_size;
        total_width += predicate_column_width;

        buffer.push_str(&format!(
            "{:width$}",
            "Predicate",
            width = &predicate_column_width
        ));
    }

    if wanted_fields.contains("type") {
        total_width += type_column_width;

        buffer.push_str(&format!("{:width$}", "Type", width = &type_column_width));
    }

    if wanted_fields.contains("index") {
        total_width += index_column_width;

        buffer.push_str(&format!("{:width$}", "Index", width = &index_column_width));
    }

    if wanted_fields.contains("reverse") {
        total_width += reverse_column_width;

        buffer.push_str(&format!(
            "{:width$}",
            "Reverse",
            width = &reverse_column_width
        ));
    }

    if wanted_fields.contains("tokenizer") {
        let mut longest_tokenizers_size: usize = tokenizers_column_width;

        for item in schema {
            let len = match &item.tokenizer {
                Some(tokenizers) => tokenizers.join(", ").len() + gap_width,
                None => 0,
            };

            if len > longest_tokenizers_size {
                longest_tokenizers_size = len;
            }
        }

        tokenizers_column_width = longest_tokenizers_size;
        total_width += tokenizers_column_width;

        buffer.push_str(&format!(
            "{:width$}",
            "Tokenizers",
            width = &tokenizers_column_width
        ));
    }

    if wanted_fields.contains("list") {
        total_width += list_column_width;

        buffer.push_str(&format!("{:width$}", "List", width = &list_column_width));
    }

    if wanted_fields.contains("count") {
        total_width += count_column_width;

        buffer.push_str(&format!("{:width$}", "Count", width = &count_column_width));
    }

    if wanted_fields.contains("upsert") {
        total_width += upsert_column_width;

        buffer.push_str(&format!(
            "{:width$}",
            "Upsert",
            width = &upsert_column_width
        ));
    }

    if wanted_fields.contains("lang") {
        total_width += lang_column_width;

        buffer.push_str(&format!("{:width$}", "Lang", width = &lang_column_width));
    }

    buffer.push_str("\n");
    buffer.push_str(&format!("{}\n", "-".repeat(total_width)));

    for item in schema {
        if wanted_fields.contains("predicate") {
            buffer.push_str(&format!(
                "{:width$}",
                item.predicate,
                width = &predicate_column_width
            ));
        }

        if wanted_fields.contains("type") {
            buffer.push_str(&format!(
                "{:width$}",
                item.ty.unwrap_or_default(),
                width = &type_column_width
            ));
        }

        if wanted_fields.contains("index") {
            buffer.push_str(&format!(
                "{:width$}",
                item.index.unwrap_or_default(),
                width = &index_column_width
            ));
        }

        if wanted_fields.contains("reverse") {
            buffer.push_str(&format!(
                "{:width$}",
                item.reverse.unwrap_or_default(),
                width = &reverse_column_width
            ));
        }

        if wanted_fields.contains("tokenizer") {
            let tokenizers = match &item.tokenizer {
                Some(tokenizers) => tokenizers.join(","),
                None => "".to_string(),
            };

            buffer.push_str(&format!(
                "{:width$}",
                tokenizers,
                width = &tokenizers_column_width
            ));
        }

        if wanted_fields.contains("list") {
            buffer.push_str(&format!(
                "{:width$}",
                item.list.unwrap_or_default(),
                width = &list_column_width
            ));
        }

        if wanted_fields.contains("count") {
            buffer.push_str(&format!(
                "{:width$}",
                item.count.unwrap_or_default(),
                width = &count_column_width
            ));
        }

        if wanted_fields.contains("upsert") {
            buffer.push_str(&format!(
                "{:width$}",
                item.upsert.unwrap_or_default(),
                width = &upsert_column_width
            ));
        }

        if wanted_fields.contains("lang") {
            buffer.push_str(&format!(
                "{:width$}",
                item.lang.unwrap_or_default(),
                width = &lang_column_width
            ));
        }

        buffer.push_str("\n");
    }

    print!("{}", buffer);
}
