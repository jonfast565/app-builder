#[macro_use] extern crate lazy_static;
#[macro_use] extern crate clap;

use std::fs;
use std::fs::{File};
use std::io::{Error};
use crate::appbuilder::AppBuilder;
use crate::args::{ProgramType};
use crate::dbbuilder::{DbSchema, Dialect};

mod dbbuilder;
mod utilities;
mod appbuilder;
mod csvbuilder;
mod args;

fn main() -> Result<(), Error> {
    println!("--- App Builder ---");
    let options = args::get_args();
    if options.runtime == ProgramType::AppDatabase {
        println!("Reading config...");
        let contents = fs::read_to_string(options.db_builder_args.unwrap().config_file_path)?;
        let app_builder = AppBuilder::init(contents);
        app_builder.template();
    } else if options.runtime == ProgramType::CsvDatabase {
        let csv_options = options.csv_builder_args.unwrap();
        let mut csv_doc_vec = Vec::<csvbuilder::CsvDocument>::new();
        for file in &csv_options.file_names {
            let rdr = File::open(&file).unwrap();
            let csv_result = csvbuilder::get_csv(b'\t', rdr, utilities::get_file_name(file.to_string()), true);
            csv_doc_vec.push(csv_result);
        }
        let result = DbSchema::from_csv_document(csv_doc_vec, csv_options.database_name, Dialect::SqlServer);
        let app_builder = AppBuilder::init_from_schema(result);
        app_builder.template();
    }

    println!("Done!");
    Ok(())
}
