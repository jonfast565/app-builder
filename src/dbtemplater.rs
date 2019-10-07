use crate::dbbuilder::{DbSchema, Dialect};

pub struct Templater {
    tera: tera::Tera
}

impl Templater {
    pub fn init() -> Templater {
        let tera = compile_templates!("templates/**/*");
        Templater {
            tera: tera
        }
    }
    pub fn template(&self, schema: DbSchema) -> String {
        let dialect = &schema.database.dialect;
        let template_context: String;
        
        if dialect == &Dialect::SqlServer {
            template_context = "sqlserver.tera".to_string()
        } else {
            panic!("Dialect not valid.");
        }
        
        let serialized_db = schema.database;
        let template_context_str = template_context.as_str();
        let rendered = self.tera.render(template_context_str, &serialized_db);
        
        match rendered {
            Ok(val) => val,
            Err(err) => panic!(err.to_string()),
        }
    }
}
