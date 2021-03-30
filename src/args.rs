use crate::dbbuilder::Dialect;
use clap::{App, Arg, SubCommand};

#[derive(PartialEq)]
pub enum ProgramType {
    AppDatabase,
    CsvDatabase,
}

pub struct DatabaseBuilderArgs {
    pub config_file_path: String,
}

pub struct CsvBuilderArgs {
    pub database_name: String,
    pub dialect: Dialect,
    pub file_names: Vec<String>,
}

pub struct ProgramArgs {
    pub runtime: ProgramType,
    pub db_builder_args: Option<DatabaseBuilderArgs>,
    pub csv_builder_args: Option<CsvBuilderArgs>,
}

pub fn get_args() -> ProgramArgs {
    let matches = App::new("App Builder")
        .version("1.0")
        .author("Jon F. <jnfstdj656@gmail.com>")
        .about("Builds app components in a couple of keystrokes")
        .subcommand(
            SubCommand::with_name("csv-builder")
                .about(
                    "Build a database schema from a CSV file, using automatic data type inference",
                )
                .version("1.0")
                .author("Jon F. <jnfstdj656@gmail.com>")
                .arg(
                    Arg::with_name("database-name")
                        .short("n")
                        .long("dbname")
                        .value_name("NAME")
                        .help("Sets the database name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("dialect")
                        .short("d")
                        .long("dialct")
                        .value_name("SERVER TYPE")
                        .help("Sets the server dialect")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("file-names")
                        .short("fns")
                        .long("filenames")
                        .value_name("NAME")
                        .help("Sets the file names to pull from")
                        .takes_value(true)
                        .multiple(true)
                        .number_of_values(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("database-builder")
                .about("builds a database from a schema")
                .version("1.0")
                .author("Jon F. <jnfstdj656@gmail.com>")
                .arg(
                    Arg::with_name("config-file")
                        .short("c")
                        .long("config")
                        .value_name("PATH")
                        .help("Sets a path to a usable config file")
                        .takes_value(true),
                ),
        )
        .get_matches();
    
    if let Some(_matches) = matches.subcommand_matches("csv-builder") {
        return ProgramArgs {
            runtime: ProgramType::CsvDatabase,
            db_builder_args: None,
            csv_builder_args: Some(CsvBuilderArgs {
                database_name: matches
                    .value_of("database-name")
                    .unwrap_or("DefaultDb")
                    .to_string(),
                dialect: Dialect::Postgres,
                file_names: values_t!(matches.values_of("file-names"), String)
                    .unwrap_or_else(|e| e.exit()),
            }),
        };
    }

    if let Some(_matches) = matches.subcommand_matches("database-builder") {
        return ProgramArgs {
            runtime: ProgramType::AppDatabase,
            db_builder_args: Some(DatabaseBuilderArgs {
                config_file_path: matches
                    .value_of("config-file")
                    .unwrap_or("./config.json")
                    .to_string(),
            }),
            csv_builder_args: None,
        };
    }

    panic!("Invalid arguments");
}
