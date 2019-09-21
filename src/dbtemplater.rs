use crate::dbbuilder::{DbSchema, Dialect};
pub struct Templater {}

impl Templater {
    pub fn template(&self, schema: DbSchema) -> String {
        let dialect = &schema.database.dialect;
        let template_context: String;
        
        if dialect == &Dialect::SqlServer {
            template_context = "sqlserver.tera".to_string()
        } else {
            panic!("Dialect not valid.");
        }
        
        let tera = compile_templates!("templates/**/*");
        let serialized_db = &schema.database;
        let rendered = tera.render(template_context.as_str(), serialized_db);
        match rendered {
            Ok(val) => val,
            Err(err) => panic!(err),
        }
    }
}
