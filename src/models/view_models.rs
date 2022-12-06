use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FormField {
    pub data_type: String,
    pub nullable: bool,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    pub name: String,
    pub fields: Vec<FormField>,
}