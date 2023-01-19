use postgres::{Client, NoTls};
use crate::models::config_models::DatabaseConfig;
use crate::models::sql_models::Column;

pub fn get_columns(query: String, db: &DatabaseConfig) -> Result<Vec<Column>, ()> {
    let mut client = Client::connect(&db.connection_string, NoTls)?;
    let query_result = client.query(&query, &[])?;
    for row in query_result {
        
    }
} 