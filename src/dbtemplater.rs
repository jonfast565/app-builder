use crate::dbbuilder::{DbSchema, Dialect};
use tera::{Tera, Context};

pub fn template(schema: DbSchema) -> String {
    let mut tera = match Tera::new("./templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing error(s): {}", e);
        }
    };

    let dialect = &schema.database.dialect;
    let template_context: String;
    
    if dialect == &Dialect::SqlServer {
        template_context = "sqlserver.tera".to_string()
    } else {
        panic!("Dialect not valid.");
    }
    
    let serialized_db = schema.database;
    let template_context_str = template_context.as_str();
    let rendered = tera.render(template_context_str, &Context::from_serialize(&serialized_db).unwrap());
    
    match rendered {
        Ok(val) => val,
        Err(err) => panic!(err.to_string()),
    }
}
