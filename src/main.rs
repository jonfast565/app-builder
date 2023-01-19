extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

mod accessors;
mod utilities;
mod models;
mod generators;

use core::panic;

use clap::Parser;
use tera::{Tera};
use generators::sql_generator::{generate_sql_view_from_json};

use crate::{models::config_models::{CliArgs, CliCommand, DatabaseConfig}, accessors::sql_accessor::get_columns};

fn print_header() {
    println!("{}", "--- AppGen Engine ----");
    println!("{}", "Author: Jonathan Fast");
    println!("{}\n", "Description: Generate application code easily!");
}

fn main() -> Result<(), ()> {
    print_header();
    println!("Initializing templating engine...");
    let tera = get_tera_instance();

    let command_line_args = CliArgs::parse();

    match &command_line_args.command {
        CliCommand::BuildViewFromDatabase { connection_string, query, paging, materialized } => {
            let db_config = DatabaseConfig {
                connection_string: connection_string.clone()
            };
            let columns = get_columns(query.to_string(), &db_config);
            match columns {
                Ok(columns) => {},
                Err(err) => panic!("{:?}", err),
            }
        },
        CliCommand::BuildViewSearchQueryFromJson => {
            generate_sql_view_from_json(
                "./data/sql.json",
                "./results",
                "postgres_paged_view_query.tera",
                "sql",
                &tera
            )?;
        }
    }

    println!("{}", "Done!");
    Ok(())
}

fn get_tera_instance() -> Tera {
    let mut tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.register_filter("pascal_to_kebab", utilities::tera_filters::pascal_to_kebab);
    tera.register_filter("pascal_to_spaced", utilities::tera_filters::pascal_to_spaced);
    tera
}

