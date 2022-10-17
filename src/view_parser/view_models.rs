use serde::{Serialize, Deserialize};

use super::models::TypeDecl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub layout: FormLayout,
    pub field_sets: Vec<FormFieldSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub type_decl: TypeDecl,
    pub is_datetime: bool,
    pub is_string: bool,
    pub is_int: bool,
    pub is_float: bool,
}

impl FieldDefinition {
    pub fn new(name: String, type_decl: &TypeDecl) -> FieldDefinition {
        FieldDefinition { 
            name: name, 
            type_decl: type_decl.clone(), 
            is_datetime: type_decl.type_name == "datetime", 
            is_string: type_decl.type_name == "string", 
            is_int: type_decl.type_name == "int", 
            is_float: type_decl.type_name == "float", 
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormLayout {
    pub width: u64,
    pub height: u64,
    pub field_sets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFieldSet {
    pub name: String,
    pub fields: Vec<String>,
}