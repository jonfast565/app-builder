extern crate pest;
extern crate pest_derive;
extern crate clap;

mod accessors;
mod generators;
mod models;
mod utilities;

use core::panic;

use color_eyre::eyre::Result;

use clap::Parser;
use generators::sql_generator::generate_sql_view_from_json;
use generators::form_generator::generate_form_from_json;
use tera::Tera;

use crate::{
    accessors::sql_accessor::get_columns,
    generators::sql_generator::{generate_sql_view_from_database, generate_sql_view_from_database_2},
    models::config_models::{CliArgs, CliCommand, DatabaseConfig},
};

fn print_header() {
    println!("{}", "--- AppGen Engine ----");
    println!("{}", "Author: Jonathan Fast");
    println!("{}\n", "Description: Generate application code easily!");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_eyre::install()?;

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
                        &columns.columns,
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
                        &columns.columns,
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
                        &columns.columns,
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
            connection_string: _,
            query,
            view_name,
            paging,
        } => {
            generate_sql_view_from_database_2(
                &query,
                &Vec::new(),
                view_name,
                paging,
                &false,
                "./results",
                "sql_view_workflow/view_search_query.tera",
                "razor",
                &tera,
            )?;
        },
        CliCommand::BuildViewSearchQueryFromJson {
            path 
        } => {
            generate_sql_view_from_json(
                path,
                "./results",
                "postgres_paged_view_query.tera",
                "sql",
                &tera,
            )?;
        },
        CliCommand::BuildFormFromJson {
            path 
        } => {
            generate_form_from_json(
                path,
                "./results",
                "blazor.tera",
                "blazor",
                &tera,
            )?;
            generate_form_from_json(
                path,
                "./results",
                "term-gui.tera",
                "cs",
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

    tera.register_filter("pascal_to_kebab", utilities::tera::pascal_to_kebab);
    tera.register_filter("pascal_to_spaced", utilities::tera::pascal_to_spaced);
    tera.register_filter("snake_to_pascal", utilities::tera::snake_to_pascal);
    tera.register_filter("pascal_to_camel", utilities::tera::pascal_to_camel);
    tera.register_filter("kebab_to_pascal", utilities::tera::kebab_to_pascal);
    tera.register_filter("snake_to_camel", utilities::tera::snake_to_camel);
    tera.register_filter("data_type_to_csharp_type", utilities::tera::data_type_to_csharp_type);
    tera
}
