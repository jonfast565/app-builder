use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RowDocument {
    pub name: String,
    pub header: Vec<String>,
    pub first_row: Vec<String>,
    pub document: Vec<Vec<String>>,
}