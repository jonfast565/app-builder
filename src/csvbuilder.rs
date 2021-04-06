use csv::ReaderBuilder;
use std::io;
use crate::models::RowDocument;

pub fn get_csv<R: io::Read>(
    delimiter: u8,
    rdr: R,
    filename: String,
    first_row_only: bool,
) -> RowDocument {
    let mut rdr = ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_reader(rdr);

    let mut counter = 0;
    let mut header = Vec::<String>::new();
    let mut rows = Vec::<Vec<String>>::new();

    for result in rdr.records() {
        let unwrapped = result.unwrap();
        let mut column_vec = Vec::<String>::new();

        for column in unwrapped.into_iter() {
            if counter == 0 {
                header.push(column.to_string());
            } else {
                column_vec.push(column.to_string());
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
        document: rows,
    };

    result
}
