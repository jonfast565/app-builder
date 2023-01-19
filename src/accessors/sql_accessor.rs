use std::error;

use postgres::{Client, NoTls};
use postgres::types::Type;
use crate::models::config_models::DatabaseConfig;
use crate::models::sql_models::{Column, SqlType};

pub fn get_columns(query: String, db: &DatabaseConfig) -> Result<Vec<Column>, Box<dyn std::error::Error>> {
    println!("attempt connection with string:{}", &db.connection_string);
    let mut client = Client::connect(&db.connection_string, NoTls)?;
    
    println!("successful connection");
    let query_result = client.query(&query, &[])?;
    
    println!("query successful");
    let first_row = query_result.first().unwrap();
    let row_columns = first_row.columns();
    let mut results = Vec::new();
    
    for column in row_columns.iter() {
        let column_name = column.name().clone();
        let column_type = column.type_().clone();
        let normalized_column_type = match column_type {
            Type::BOOL => SqlType::Bool,
            Type::CHAR => SqlType::Int8,
            Type::INT2 => SqlType::Int16,
            Type::INT4 => SqlType::Int32,
            Type::INT8 => SqlType::Int64,
            Type::VARCHAR => SqlType::String,
            _ => todo!()
        };

        let col = Column {
            name: column_name.to_string(),
            data_type: normalized_column_type,
        };

        results.push(col);
    }

    Ok(results)
} 