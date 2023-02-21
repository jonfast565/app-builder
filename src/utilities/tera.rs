use tera::{Result, Value, Tera, Context};
use std::fs;
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

pub fn snake_to_pascal(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::snake_to_pascal(s))),
        _ => Ok(value.clone())
    }
}

pub fn pascal_to_camel(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::pascal_to_camel(s))),
        _ => Ok(value.clone())
    }
}

pub fn kebab_to_pascal(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::kebab_to_pascal(s))),
        _ => Ok(value.clone())
    }
}

pub fn snake_to_camel(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::string::snake_to_camel(s))),
        _ => Ok(value.clone())
    }
}

pub fn data_type_to_csharp_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
    match value.clone() {
        Value::String(s) => Ok(Value::String(super::csharp::data_type_to_csharp_type(s.as_str()).to_string())),
        _ => Ok(value.clone())
    }
}

pub fn render_template(tera: &Tera, template_name: &str, ctx: Context, path: String) {
    let output = tera.render(template_name, &ctx);
    match output {
        Ok(a) => fs::write(
            path,
            a,
        )
        .expect("file not written"),
        Err(e) => panic!("Parsing error(s): {}", e),
    }
}