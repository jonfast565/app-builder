use std::io;

pub struct ExcelDocument {
    pub name: String,
    pub header: Vec<String>,
    pub first_row: Vec<String>,
    pub document: Vec<Vec<String>>,
}

/*
pub fn get_excel(    
    delimiter: u8,
    rdr: R,
    filename: String,
    first_row_only: bool) -> ExcelDocument {
    ExcelDocument {

    }
}
*/