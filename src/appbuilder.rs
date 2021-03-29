use crate::dbbuilder::{DbSchema, Dialect};
use tera::{Context, Tera};

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
        let tera = match Tera::new("templates/database/**/*") {
            Ok(t) => t,
            Err(e) => {
                panic!("Parsing error(s): {}", e);
            }
        };
    
        let dialect = &self.schema.database.dialect;
        let template_context: String;
        if dialect == &Dialect::SqlServer {
            template_context = "sqlserver.tera".to_string()
        } else if dialect == &Dialect::Postgres {
            template_context = "postgres.tera".to_string()
        } else if dialect == &Dialect::Sqlite {
            template_context = "sqlite.tera".to_string()
        } else {
            panic!("Dialect not valid.");
        }
        let serialized_db = &self.schema.database;
        let template_context_str = template_context.as_str();

        
        println!("Rendering template...");
        let rendered = tera.render(
            template_context_str,
            &Context::from_serialize(&serialized_db).unwrap(),
        );
        
        let get_rendered = match rendered {
            Ok(val) => val,
            Err(err) => panic!(err.to_string()),
        };

        println!("Writing file...");
        if !Path::new("./results").exists() {
            fs::create_dir("./results").unwrap();
        }
        let mut output = File::create("./results/sql-result.sql").unwrap();
        write!(output, "{}", get_rendered).unwrap();
    }
}