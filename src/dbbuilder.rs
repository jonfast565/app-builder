use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Dialect {
    SqlServer,
    Postgres,
    Sqlite
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ColumnType {
    IntegerType,
    StringType,
    BinaryType,
    DateType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    pub primary_table_name: String,
    pub secondary_table_name: String,
    pub relationship_type: RelationshipType
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnTableAlias {
    pub column_name: String,
    pub table_name: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub column_names: Vec<String>,
    pub foreign_columns: Option<Vec<ColumnTableAlias>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub column_type: String,
    pub names: Vec<String>,
    pub nullable: bool,
    pub auto_increment: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub audit_fields: bool,
    pub constraints: Vec<Constraint>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    pub database_name: String,
    pub dialect: Dialect,
    pub tables: Vec<Table>,
    pub relationships: Vec<Relationship>,
    pub queries: Vec<Query>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DbSchema {
    pub database: Database
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignKeyDefinition {
    source_table: String,
    target_table: String,
    column_names: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexDefinition {
    column_names: Vec<String>
}

impl DbSchema {
	/*
    fn get_structure_from_db(conn_str: String) -> DbSchema {
        let connection_future = tiberius::SqlConnection::connect(conn_str.as_str());
        connection_future.and_then(| conn | conn);

        current_thread::block_on_all(connection_future).unwrap();
    }
    */
}