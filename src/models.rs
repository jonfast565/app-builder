#[derive(Debug)]
pub struct RowDocument {
    pub name: String,
    pub header: Vec<String>,
    pub first_row: Vec<String>,
    pub document: Vec<Vec<String>>,
}