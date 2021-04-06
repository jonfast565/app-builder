use crate::dbbuilder::{DbSchema, Dialect};
use handlebars::Handlebars;
use serde_json::json;

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{Write};

pub struct AppBuilder {
    schema: DbSchema
}

impl AppBuilder {
    pub fn init(config_string: String) -> AppBuilder {
        println!("Getting database schema...");
        let mut schema : DbSchema = serde_json::from_str(&config_string).unwrap();
        if schema.database.dialect == Dialect::Postgres {
            schema.lowercase_ids();
        }
        AppBuilder {
            schema: schema
        }
    }

    pub fn init_from_schema(schema: DbSchema) -> AppBuilder {
        AppBuilder {
            schema: schema
        }
    }

    pub fn template(&self) {
        self.template_database();
    }

    fn template_database(&self) {
        let mut reg = Handlebars::new();
        reg.set_strict_mode(true);

        let dialect = &self.schema.database.dialect;
        let template_context: String;
        if dialect == &Dialect::SqlServer {
            template_context = "./templates/database/sqlserver.hbs".to_string()
        } else if dialect == &Dialect::Postgres {
            template_context = "./templates/database/postgres.hbs".to_string()
        } else if dialect == &Dialect::Sqlite {
            template_context = "./templates/database/sqlite.hbs".to_string()
        } else {
            panic!("Dialect not valid.");
        }
        let serialized_db = &self.schema.database;
        let template_context_str = template_context.as_str();
        println!("Reading template {}", &template_context_str);
        let template_text = fs::read_to_string(template_context_str)
            .expect("Something went wrong reading the file");
        
        println!("Rendering template...");
        let serialized_context = json!(serialized_db);
        let rendered = reg.render_template(&template_text, &serialized_context).unwrap();

        println!("Writing file...");
        if !Path::new("./results").exists() {
            fs::create_dir("./results").unwrap();
        }

        let mut output = File::create("./results/sql-result.sql").unwrap();
        write!(output, "{}", rendered).unwrap();
    }
}