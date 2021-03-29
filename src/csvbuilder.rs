use csv::ReaderBuilder;
use std::io;

pub struct CsvDocument {
    pub header: Vec<String>,
    pub first_row: Vec<String>,
    pub document: Vec<Vec<String>>,
}

pub fn get_csv<R: io::Read>(delimiter: u8, rdr: R, first_row_only: bool) -> CsvDocument {
    let mut rdr = ReaderBuilder::new().delimiter(delimiter).from_reader(rdr);

    let mut counter = 0;
    let mut header = Vec::<String>::new();
    let mut columns = Vec::<Vec<String>>::new();

    for result in rdr.records() {
        let unwrapped = result.unwrap();
        let mut column_vec = Vec::<String>::new();

        for column in unwrapped.into_iter() {
            if counter == 0 {
                header.push(column.to_string());
            } else {
                column_vec.push(column.to_string());
            }
            print!("{:?}", column);
        }
        print!("\n");
        if first_row_only {
            break;
        }

        if counter != 0 {
            columns.push(column_vec);
        }

        counter += 1;
    }

    CsvDocument {
        header: header,
        first_row: columns[0].clone(),
        document: columns,
    }
}
