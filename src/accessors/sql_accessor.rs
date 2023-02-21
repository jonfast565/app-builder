use postgres::{Client, NoTls};
use postgres::types::Type;
use crate::models::config_models::DatabaseConfig;
use crate::models::sql_models::{SqlColumn, SqlColumnType, SqlQueryMetadata};

pub fn get_columns(query: String, db: &DatabaseConfig) -> Result<SqlQueryMetadata, Box<dyn std::error::Error>> {
    println!("Attempt connection to: {}.", &db.connection_string);
    let mut client = Client::connect(&db.connection_string, NoTls)?;
    
    println!("Successful connection.");
    let use_query = query.trim_end_matches(';').to_string();
    let query_result = client.query(&use_query, &[])?;
    
    println!("Query successful.");
    let first_row = match query_result.first() {
        Some(x) => x,
        None => panic!("Couldn't get first row of this query :`{}`, consider adding a row to the table to obtain it's data.", &use_query)
    };

    let row_columns = first_row.columns();
    let mut results = Vec::new();

    for column in row_columns.iter() {
        let column_name = column.name().clone();
        let column_type = column.type_().clone();
        let normalized_column_type = match column_type {
            Type::BOOL => SqlColumnType::Bool,
            Type::CHAR => SqlColumnType::Int8,
            Type::INT2 => SqlColumnType::Int16,
            Type::INT4 => SqlColumnType::Int32,
            Type::INT8 => SqlColumnType::Int64,
            Type::VARCHAR => SqlColumnType::UnboundedString,
            Type::TIMESTAMP => SqlColumnType::Timestamp,
            Type::DATE => SqlColumnType::Date,
            Type::JSONB => SqlColumnType::UnboundedString,
            Type::TEXT => SqlColumnType::UnboundedString,
            Type::TIMESTAMPTZ => SqlColumnType::Timestamp,
            _ => SqlColumnType::UnboundedString
            // todo!("{}", format!("{} - type {:?} not implemented", column_name, column_type))
        };

        let col = SqlColumn {
            column_name: column_name.to_string(),
            column_type: normalized_column_type,
            nullable: true,
            // TODO: Fill this in with correct information
            is_auto_increment: false,
            is_primary_key: false,
            has_unique_key: false,
        };

        results.push(col);
    }

    let package = SqlQueryMetadata { columns: results };
    Ok(package)
} 