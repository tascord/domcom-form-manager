use core::panic;
use std::collections::HashMap;

use futures_signals::signal::Mutable;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

/*

form! {
    .field(
        id: "first_name"
        label: "First Name",
        pattern: ".{3,}",
        required: true
    )
}

*/

const FIELD_PAT: &str = r#"\.field\(((.|\n)+?)\)"#;
const FIELD_ARGS_PAT: &str = r#"([A-z]+? *: *(?:".+?"|.+? *(?:,|$)))"#;

#[proc_macro]
pub fn form(item: TokenStream) -> TokenStream {
    let string = item.to_string();
    extract_field(string).into()
}

fn extract_field(string: String) -> proc_macro2::TokenStream {
    let fields = Regex::new(FIELD_PAT).unwrap();
    let fields = fields.captures_iter(&string);

    for field in fields.into_iter() {
        let field = field[1].trim().replace("\n", "");

        let args = Regex::new(FIELD_ARGS_PAT).unwrap();
        let args = args.captures_iter(&field);

        let mut map = HashMap::<String, String>::new();
        for arg in args.into_iter() {
            let arg = arg[0].trim();
            let arg = arg.split(":").collect::<Vec<&str>>();

            let key = arg[0].trim();
            let mut value = arg[1].trim();

            if value.ends_with(',') {
                value = value[..value.len() - 1].trim();
            }

            if value.starts_with('"') && value.ends_with('"') {
                value = &value[1..value.len() - 1];
            }

            map.insert(key.to_string(), value.to_string());
        }

        let d = String::new();

        let label = map.get("label").unwrap();
        let required = map
            .get("required")
            .map(|x| x.parse::<bool>().unwrap())
            .unwrap_or(false);

        let pattern = map.get("pattern");
        let pattern = pattern.unwrap_or(&d);

        let field_type = map.get("type").unwrap().as_str();
        let field_type = format_ident!("{}", field_type);

        return quote! {
            Field::<#field_type> {
                label: #label,
                value: futures_signals::signal::Mutable::new(),
                error: futures_signals::signal::Mutable::new(),
                regex: #pattern,
                required: #required,
            }
        };
    }

    panic!("No field found");
}
