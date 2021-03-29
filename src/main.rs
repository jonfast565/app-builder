#[macro_use] extern crate lazy_static;

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
        let filename = "./config.json";
        println!("Reading config...");
        let contents = fs::read_to_string(filename)?;
        let app_builder = AppBuilder::init(contents);
        app_builder.template();
    } else if options.runtime == ProgramType::CsvDatabase {
        let rdr = File::open("C:\\Users\\jfast\\OneDrive - American College of Cardiology\\Desktop\\
        Oasis Bulk Export Files 03292021\\AllCreditRequests.txt").unwrap();
        let csv_result = csvbuilder::get_csv(b'\t', rdr, true);
        let result = DbSchema::from_csv_document(&csv_result, "CSVDatabase".to_string(), "CSVTable".to_string(), Dialect::Postgres);
        let app_builder = AppBuilder::init_from_schema(result);
        app_builder.template();
    }

    println!("Done!");
    Ok(())
}
