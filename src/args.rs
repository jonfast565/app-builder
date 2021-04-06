use crate::dbbuilder::Dialect;
use crate::utilities;
use clap::{App, AppSettings, Arg, SubCommand};
use std::str::FromStr;

#[derive(PartialEq)]
pub enum ProgramType {
    AppDatabase,
    CsvDatabase,
    ExcelDatabase,
}

pub struct DatabaseBuilderArgs {
    pub config_file_path: String,
}

pub struct CsvBuilderArgs {
    pub database_name: String,
    pub dialect: Dialect,
    pub file_names: Vec<String>,
    pub delimiter: u8,
    pub end_of_line_delimiter: u8,
}

pub struct ExcelBuilderArgs {
    pub database_name: String,
    pub dialect: Dialect,
    pub file_names: Vec<String>,
}

pub struct ProgramArgs {
    pub runtime: ProgramType,
    pub db_builder_args: Option<DatabaseBuilderArgs>,
    pub csv_builder_args: Option<CsvBuilderArgs>,
    pub excel_builder_args: Option<ExcelBuilderArgs>,
}

pub fn get_args() -> ProgramArgs {
    let app = App::new("App Builder")
        .version("1.0")
        .author("Jon F. <jnfstdj656@gmail.com>")
        .about("Builds app components in a couple of keystrokes")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("excel-builder")
                .about(
                    "Build a database schema from an Excel file, using automatic data type inference",
                )
                .version("1.0")
                .author("Jon F. <jnfstdj656@gmail.com>")
                .arg(
                    Arg::with_name("database-name")
                        .short("n")
                        .long("dbname")
                        .value_name("NAME")
                        .help("Sets the database name")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("dialect")
                        .short("d")
                        .long("dialect")
                        .value_name("SERVER TYPE")
                        .help("Sets the server dialect")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("file-dir")
                        .short("f")
                        .long("filedir")
                        .value_name("PATH")
                        .help("Sets the path to use to get files")
                        .takes_value(true)
                        .multiple(true)
                        .number_of_values(1)
                        .required(true),
                ),
        )
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
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("dialect")
                        .short("d")
                        .long("dialect")
                        .value_name("SERVER TYPE")
                        .help("Sets the server dialect")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("file-dir")
                        .short("f")
                        .long("filedir")
                        .value_name("PATH")
                        .help("Sets the path to use to get files")
                        .takes_value(true)
                        .multiple(true)
                        .number_of_values(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("delimiter")
                        .short("l")
                        .long("delimiter")
                        .value_name("DELIMITER CHAR")
                        .help("Sets the row delimiter to use when parsing CSV files")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("eol-delimiter")
                        .short("e")
                        .long("eoldelimiter")
                        .value_name("DELIMITER CHAR")
                        .help("Sets the row end delimiter to use when parsing CSV files")
                        .takes_value(true)
                        .required(false),
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
                        .takes_value(true)
                        .required(true),
                ),
        );
    let matches = &app.get_matches();

    if let Some(excel_matches) = matches.subcommand_matches("excel-builder") {
        let filenames = utilities::get_glob_matches(
            excel_matches.value_of("file-dir").unwrap().to_string(),
            "xlsx".to_string(),
        );

        let args = ProgramArgs {
            runtime: ProgramType::ExcelDatabase,
            db_builder_args: None,
            csv_builder_args: None,
            excel_builder_args: Some(ExcelBuilderArgs {
                database_name: excel_matches
                    .value_of("database-name")
                    .unwrap_or("DefaultDb")
                    .to_string(),
                dialect: Dialect::from_str(excel_matches.value_of("dialect").unwrap()).unwrap(),
                file_names: filenames,
            }),
        };

        return args;
    }

    if let Some(csv_matches) = matches.subcommand_matches("csv-builder") {
        let filenames = utilities::get_glob_matches(
            csv_matches.value_of("file-dir").unwrap().to_string(),
            "csv".to_string(),
        );

        let args = ProgramArgs {
            runtime: ProgramType::CsvDatabase,
            db_builder_args: None,
            excel_builder_args: None,
            csv_builder_args: Some(CsvBuilderArgs {
                database_name: csv_matches
                    .value_of("database-name")
                    .unwrap_or("DefaultDb")
                    .to_string(),
                dialect: Dialect::from_str(csv_matches.value_of("dialect").unwrap()).unwrap(),
                file_names: filenames,
                delimiter: csv_matches
                    .value_of("delimiter")
                    .unwrap_or("\t")
                    .chars()
                    .nth(0)
                    .unwrap() as u8,
                end_of_line_delimiter: csv_matches
                    .value_of("eol-delimiter")
                    .unwrap_or("\n")
                    .chars()
                    .nth(0)
                    .unwrap() as u8,
            }),
        };

        return args;
    }

    if let Some(db_matches) = matches.subcommand_matches("database-builder") {
        let args = ProgramArgs {
            runtime: ProgramType::AppDatabase,
            db_builder_args: Some(DatabaseBuilderArgs {
                config_file_path: db_matches
                    .value_of("config-file")
                    .unwrap_or("./config.json")
                    .to_string(),
            }),
            csv_builder_args: None,
            excel_builder_args: None,
        };

        return args;
    }

    panic!("Invalid arguments");
}
