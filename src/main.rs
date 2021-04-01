#[macro_use] extern crate lazy_static;
// #[macro_use] extern crate clap;

use std::fs;
use std::fs::{File};
use std::io::{Error};
use crate::appbuilder::AppBuilder;
use crate::args::{ProgramType, ProgramArgs};
use crate::dbbuilder::{DbSchema};

mod models;
mod dbbuilder;
mod utilities;
mod appbuilder;
mod csvbuilder;
mod excelbuilder;
mod args;

fn main() -> Result<(), Error> {
    println!("--- App Builder ---");
    println!("Working Directory: {}", std::env::current_dir().unwrap().display());
    do_program()?;
    println!("Done!");
    Ok(())
}

fn do_program() -> Result<(), Error> {
    let options = args::get_args();
    if options.runtime == ProgramType::AppDatabase {
        template_config(&options)?;
    } else if options.runtime == ProgramType::CsvDatabase {
        template_csvs(&options);
    } else if options.runtime == ProgramType::ExcelDatabase {
        template_excel(&options);
    }
    Ok(())
}

fn template_config(options: &ProgramArgs) -> Result<(), Error> {
    println!("Reading config...");
    let contents = fs::read_to_string(options.db_builder_args.as_ref().unwrap().config_file_path.to_string())?;
    let app_builder = AppBuilder::init(contents);
    app_builder.template();
    Ok(())
}

fn template_csvs(options: &ProgramArgs) {
    println!("Reading CSV files and determining structure.");
    let csv_options = options.csv_builder_args.as_ref().unwrap();
    let mut csv_doc_vec = Vec::<models::RowDocument>::new();
    for file in &csv_options.file_names {
        println!("Reading file {}", &file);
        let rdr = File::open(&file).unwrap();
        let csv_result = csvbuilder::get_csv(csv_options.delimiter, rdr, utilities::get_file_name(file.to_string()), false);
        csv_doc_vec.push(csv_result);
    }
    let result = DbSchema::from_documents(csv_doc_vec, csv_options.database_name.to_string(), csv_options.dialect.clone());
    let app_builder = AppBuilder::init_from_schema(result);
    app_builder.template();
}

fn template_excel(options: &ProgramArgs) {
    println!("Reading Excel files and determining structure");
    let excel_options = options.excel_builder_args.as_ref().unwrap();
    let mut excel_doc_vec = Vec::<models::RowDocument>::new();
    for file in &excel_options.file_names {
        println!("Reading file {}", &file);
        let excel_result = excelbuilder::get_excel(file.to_string(), utilities::get_file_name(file.to_string()), true);
        excel_doc_vec.push(excel_result);
    }
    let result = DbSchema::from_documents(excel_doc_vec, excel_options.database_name.to_string(), excel_options.dialect.clone());
    let app_builder = AppBuilder::init_from_schema(result);
    app_builder.template();
}
