use clap::{App, Arg, SubCommand};

#[derive(PartialEq)]
pub enum ProgramType {
    AppDatabase,
    CsvDatabase,
}

pub struct ProgramArgs {
    pub runtime: ProgramType,
}

pub fn get_args() -> ProgramArgs {
    let matches = App::new("App Builder")
        .version("1.0")
        .author("Jon F. <jnfstdj656@gmail.com>")
        .about("Builds app components in a couple of keystrokes")
        .subcommand(
            SubCommand::with_name("csv-database")
                .about(
                    "Build a database schema from a CSV file, using automatic data type inference",
                )
                .version("1.0")
                .author("Jon F. <jnfstdj656@gmail.com>"),
        )
        .subcommand(
            SubCommand::with_name("app-database")
                .about("builds a database from a schema")
                .version("1.0")
                .author("Jon F. <jnfstdj656@gmail.com>"),
        )
        .get_matches();
        if let Some(_matches) = matches.subcommand_matches("csv-database") {
            return ProgramArgs {
                runtime: ProgramType::CsvDatabase
            }
        }

        if let Some(_matches) = matches.subcommand_matches("app-database") {
            return ProgramArgs {
                runtime: ProgramType::AppDatabase
            }
        }

    panic!("Invalid arguments");
}
