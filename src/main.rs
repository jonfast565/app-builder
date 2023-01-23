extern crate pest;
#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

mod accessors;
mod generators;
mod models;
mod utilities;

use core::panic;

use clap::Parser;
use generators::sql_generator::generate_sql_view_from_json;
use tera::Tera;

use crate::{
    accessors::sql_accessor::get_columns,
    generators::sql_generator::generate_sql_view_from_database,
    models::config_models::{CliArgs, CliCommand, DatabaseConfig},
};

fn print_header() {
    println!("{}", "--- AppGen Engine ----");
    println!("{}", "Author: Jonathan Fast");
    println!("{}\n", "Description: Generate application code easily!");
}

fn main() -> Result<(), ()> {
    print_header();
    println!("Initializing templating engine...");
    let tera = get_tera_instance();
    println!("Templating engine initialized.");

    let command_line_args = CliArgs::parse();

    match &command_line_args.command {
        CliCommand::BuildViewFromDatabase {
            view_name,
            connection_string,
            query,
            paging,
            materialized,
        } => {
            let db_config = DatabaseConfig {
                connection_string: connection_string.clone(),
            };
            let columns = get_columns(query.to_string(), &db_config);
            match columns {
                Ok(columns) => {
                    generate_sql_view_from_database(
                        &query,
                        &columns,
                        view_name,
                        paging,
                        materialized,
                        "./results",
                        "sql_view_workflow/postgres_view.tera",
                        "sql",
                        &tera,
                    )?;
                    generate_sql_view_from_database(
                        &query,
                        &columns,
                        view_name,
                        paging,
                        materialized,
                        "./results",
                        "sql_view_workflow/firehose_midtier.tera",
                        "cs",
                        &tera,
                    )?;
                    generate_sql_view_from_database(
                        &query,
                        &columns,
                        view_name,
                        paging,
                        materialized,
                        "./results",
                        "sql_view_workflow/dashboard_frontend.tera",
                        "razor",
                        &tera,
                    )?;
                }
                Err(err) => panic!("{:?}", err),
            }
        }
        CliCommand::BuildViewSearchQueryFromDatabase {
            connection_string,
            query,
            view_name,
            paging,
        } => todo!(),
        CliCommand::BuildViewSearchQueryFromJson => {
            generate_sql_view_from_json(
                "./data/sql.json",
                "./results",
                "postgres_paged_view_query.tera",
                "sql",
                &tera,
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
    tera.register_filter("snake_to_pascal", utilities::tera_filters::snake_to_pascal);
    tera.register_filter("pascal_to_camel", utilities::tera_filters::pascal_to_camel);
    tera.register_filter("kebab_to_pascal", utilities::tera_filters::kebab_to_pascal);
    tera.register_filter("snake_to_camel", utilities::tera_filters::snake_to_camel);
    tera.register_filter("data_type_to_csharp_type", utilities::tera_filters::data_type_to_csharp_type);
    tera
}
