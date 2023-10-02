use std::collections::HashMap;

use rocket_dyn_templates::tera::{from_value, Function, to_value, Value};
use rocket_dyn_templates::tera::Result;

pub fn build() -> impl Function {
    Box::new(move |args: &HashMap<String, Value>| -> Result<Value> {
        match args.get("l") {
            Some(value) => match from_value::<f64>(value.clone()) {
                Ok(left) => {
                    match args.get("r") {
                        Some(value) => match from_value::<f64>(value.clone()) {
                            Ok(right) => {
                                Ok(to_value(left.max(right)).unwrap())
                            }
                            Err(err) => Err(err.into()),
                        }
                        None => Err("Argument `r` not found".into()),
                    }
                }
                Err(err) => Err(err.into()),
            }
            None => Err("Argument `l` not found".into()),
        }
    })
}