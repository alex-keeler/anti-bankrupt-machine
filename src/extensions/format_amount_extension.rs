use std::collections::HashMap;

use num_format::Locale;
use num_format::ToFormattedString;
use rocket_dyn_templates::tera::{from_value, Function, to_value, Value};
use rocket_dyn_templates::tera::Result;

pub fn build() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("amount") {
            Some(value) => match from_value::<i32>(value.clone()) {
                Ok(amount) => {
                    let negative = amount < 0;
                    let dollars = amount.abs() / 100;
                    let cents = amount.abs() % 100;
                    let formatted_amount = match negative {
                        true => format!("-${}.{:02}", dollars.to_formatted_string(&Locale::en), cents),
                        false => format!("${}.{:02}", dollars.to_formatted_string(&Locale::en), cents),
                    };
                    Ok(to_value(formatted_amount).unwrap())
                }
                Err(err) => Err(err.into())
            }
            None => Err("Amount argument not found".into()),
        }
    })
}