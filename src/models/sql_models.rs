use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SqlType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    String,
    StringWithSize(i32),
    Timestamp
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRoot {
    pub queries: Vec<PagedViewQuery>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: SqlType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagedViewQuery {
    pub name: String,
    pub result_columns: Vec<String>,
    pub view: String,
    pub search_columns: Vec<Column>,
    pub how_many_filters: u64,
    pub return_cursor: bool
}