use std::collections::{BTreeMap, HashMap};
use rocket_dyn_templates::tera::{from_value, Function, to_value, Value};
use rocket_dyn_templates::tera::Result;
use num_format::ToFormattedString;
use num_format::Locale;

pub fn build() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("amount") {
            Some(value) => match from_value::<i32>(value.clone()) {
                Ok(amount) => {
                    let negative = amount < 0;
                    let dollars = (amount / 100).abs();
                    let cents = (amount % 100).abs();
                    let formatted_amount = match negative {
                        true => format!("-${}.{:02}", dollars.to_formatted_string(&Locale::en), cents),
                        false => format!("${}.{:02}", dollars.to_formatted_string(&Locale::en), cents),
                    };
                    Ok(to_value(formatted_amount).unwrap())
                },
                Err(_) => Err("Amount argument not found".into())
            }
            None => Err("Amount argument not found".into()),
        }
    })
}