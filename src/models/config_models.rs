use clap::Parser;

pub struct DatabaseConfig {
    pub connection_string: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
   #[command(subcommand)]
   pub command: CliCommand,
}

#[derive(clap::Subcommand, Debug)]
pub enum CliCommand {
   BuildViewFromDatabase {
        #[clap(short, long)]
        connection_string: String
   },
   BuildViewSearchQueryFromJson,
}