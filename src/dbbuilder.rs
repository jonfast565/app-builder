use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
enum Dialect {
    SqlServer,
    Postgres,
    Sqlite
}

#[derive(Serialize, Deserialize, Debug)]
enum ConstraintType {
    PrimaryKey,
    ForeignKey,
}

#[derive(Serialize, Deserialize, Debug)]
enum ColumnType {
    IntegerType,
    StringType,
    BinaryType,
}

#[derive(Serialize, Deserialize, Debug)]
struct Relationship {
    primary_table_name: String,
    secondary_table_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Query {

}

#[derive(Serialize, Deserialize, Debug)]
struct ColumnTableAlias {
    column_name: String,
    table_name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Constraint {
    constraint_type: ConstraintType,
    column_names: Vec<String>,
    foreign_columns: Option<Vec<ColumnTableAlias>>
}

#[derive(Serialize, Deserialize, Debug)]
struct Column {
    column_type: String,
    names: Vec<String>,
    nullable: bool,
    auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Table {
    table_name: String,
    columns: Vec<Column>,
    audit_fields: bool,
    constraints: Vec<Constraint>
}

#[derive(Serialize, Deserialize, Debug)]
struct Database {
    dialect: Dialect,
    tables: Vec<Table>,
    relationships: Vec<Relationship>,
    queries: Vec<Query>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbSchema {
    database: Database
}