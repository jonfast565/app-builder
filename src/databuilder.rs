use crate::dbbuilder::DbSchema;
use crate::models::RowDocument;
use handlebars::Handlebars;
use serde_json::json;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct DataBuilder {
    schema: DbSchema,
}

impl DataBuilder {
    pub fn init(schema: DbSchema) -> DataBuilder {
        println!("Getting data...");
        DataBuilder {
            schema: schema,
        }
    }

    pub fn template_insert_statements(&self) {
        let mut reg = Handlebars::new();
        reg.set_strict_mode(true);

        let template_context = "./templates/database_inserts/insert_csv.hbs";
        println!("Reading template {}", &template_context);
        let template_text =
            fs::read_to_string(template_context).expect("Something went wrong reading the file");
        println!("Rendering template...");

        let mut rendered = String::new();
        let serialized_context = json!(&self.schema.database);
        let template_result = reg
            .render_template(&template_text, &serialized_context)
            .unwrap();
        rendered += &template_result;

        println!("Writing file...");
        if !Path::new("./results").exists() {
            fs::create_dir("./results").unwrap();
        }

        let mut output = File::create("./results/sql-insert-result.sql").unwrap();
        write!(output, "{}", rendered).unwrap();
    }
}
