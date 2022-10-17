use tera::{Result, Value};
use std::collections::HashMap;

pub fn pascal_to_kebab(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::pascal_to_kebab(s))),
        _ => Ok(value.clone())
    }
}

pub fn pascal_to_spaced(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::pascal_to_spaced(s))),
        _ => Ok(value.clone())
    }
}