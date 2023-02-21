use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRoot {
    pub queries: Vec<PagedViewQuery>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PagedViewQuery {
    pub name: String,
    pub result_columns: Vec<String>,
    pub view: String,
    pub search_columns: Vec<SqlColumn>,
    pub how_many_filters: u64,
    pub return_cursor: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LengthField {
    pub min: usize,
    pub max: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DecimalNumericField {
    pub precision: usize,
    pub scale: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "type_params")]
pub enum SqlColumnType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Numeric(DecimalNumericField),
    Decimal(DecimalNumericField),
    Money(DecimalNumericField),
    BoundedString(LengthField),
    UnboundedString,
    DateTime,
    Time,
    Date,
    TimeWithTimezone,
    DateTimeWithTimezone,
    Timestamp
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlQueryMetadata {
    pub columns: Vec<SqlColumn>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SqlEngine {
    Postgres
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlDatabase {
    pub database_name: String,
    pub tables: Vec<SqlTable>,
    pub relationships: Vec<SqlRelationship>,
    pub engine: SqlEngine,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlTable {
    pub table_name: String,
    pub columns: Vec<SqlColumn>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlColumn {
    pub column_name: String,
    pub column_type: SqlColumnType,
    pub is_primary_key: bool,
    pub is_auto_increment: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SqlRelationshipType {
    ManyToMany,
    OneToOne,
    OneToMany,
    ManyToOne,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SqlRelationship {
    pub relationship_name: String,
    pub source_table: String,
    pub destination_table: String,
    pub relationship_type: SqlRelationshipType,
}