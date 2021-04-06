use calamine::{open_workbook_auto, Reader, DataType};
use crate::models::RowDocument;

pub fn get_excel(file_path: String, filename: String, first_row_only: bool) -> RowDocument {
    let mut excel = open_workbook_auto(file_path.clone()).expect("Cannot open file");
    
    // default to the first worksheet for reading.
    if let Some(Ok(r)) = excel.worksheet_range_at(0) {
        let mut counter = 0;
        let mut header = Vec::<String>::new();
        let mut rows = Vec::<Vec<String>>::new();

        for row in r.rows() {
            let mut column_vec = Vec::<String>::new();
            for (_, c) in row.iter().enumerate() {
                let value : String = match *c {
                    DataType::Empty => String::from(""),
                    DataType::String(ref s) => format!("{}", s),
                    DataType::Float(ref f) | DataType::DateTime(ref f) => format!("{}", f),
                    DataType::Int(ref i) => format!("{}", i),
                    DataType::Error(ref e) => format!("{:?}", e),
                    DataType::Bool(ref b) => format!("{}", b),
                };

                if counter == 0 {
                    header.push(value.to_string());
                } else {
                    column_vec.push(value.to_string());
                }
            }

            if counter != 0 {
                rows.push(column_vec);
            }
    
            if first_row_only && counter == 1 {
                break;
            }

            counter += 1;
        }

        let result = RowDocument {
            name: filename,
            header: header,
            first_row: rows[0].clone(),
            document: rows
        };

        return result;
    }

    panic!("Default sheet not found, document unable to be parsed");
}
