use crate::models::RowDocument;
use crate::utilities;
use crate::utilities::SliceDisplay;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub trait SemanticCheck {
    fn is_valid(&self);
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Dialect {
    SqlServer,
    Postgres,
    Sqlite,
}

impl FromStr for Dialect {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SqlServer" => Ok(Dialect::SqlServer),
            "Postgres" => Ok(Dialect::Postgres),
            "Sqlite" => Ok(Dialect::Sqlite),
            _ => Err("No match"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ColumnType {
    Integer,
    String,
    Decimal,
    Binary,
    Date,
    Boolean,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relationship {
    pub table_names: Vec<String>,
    pub relationship_type: RelationshipType,
}

impl SemanticCheck for Relationship {
    fn is_valid(&self) {
        if self.relationship_type == RelationshipType::OneToOne && self.table_names.len() == 2
            || self.relationship_type == RelationshipType::OneToMany && self.table_names.len() == 2
            || self.relationship_type == RelationshipType::ManyToMany
        {
            return;
        }
        panic!(
            "Invalid relationship between tables {}, see config file for more detail.",
            SliceDisplay(&self.table_names)
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Query {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColumnTableAlias {
    pub column_name: String,
    pub table_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub column_names: Vec<String>,
    pub foreign_columns: Option<Vec<ColumnTableAlias>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Column {
    pub column_type: ColumnType,
    pub name: String,
    pub nullable: bool,
    pub auto_increment: Option<bool>,

    // type bools for handlebars
    pub is_integer: bool,
    pub is_string: bool,
    pub is_decimal: bool,
    pub is_binary: bool,
    pub is_date: bool,
    pub is_boolean: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub table_name: String,
    pub columns: Vec<Column>,
    pub audit_fields: bool,
    pub constraints: Vec<Constraint>,
    pub document: RowDocument
}

impl Table {
    pub fn lowercase_name(&mut self) {
        self.table_name = self.table_name.to_lowercase();
    }
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
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ForeignKeyDefinition {
    source_table: String,
    target_table: String,
    column_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexDefinition {
    column_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueBox {
    value: String,
}

impl DbSchema {
    /*
    fn get_structure_from_db(conn_str: String) -> DbSchema {
        let connection_future = tiberius::SqlConnection::connect(conn_str.as_str());
        connection_future.and_then(| conn | conn);

        current_thread::block_on_all(connection_future).unwrap();
    }
    */
    pub fn lowercase_ids(&mut self) {
        for table in &mut self.database.tables {
            table.lowercase_name();
        }
    }

    pub fn from_documents(
        docs: Vec<RowDocument>,
        database_name: String,
        dialect: Dialect,
    ) -> DbSchema {
        let mut tables = Vec::<Table>::new();

        for doc in docs {
            let column_names = doc
                .header
                .clone()
                .into_iter()
                .map(|x| utilities::process_column_name(x, dialect.clone()))
                .collect::<Vec<String>>();
            let data_types = doc
                .first_row
                .clone()
                .into_iter()
                .map(|x| {
                    lazy_static! {
                        static ref INTEGER_RE: Regex = Regex::new(r"[0-9]+").unwrap();
                        static ref STRING_RE: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
                        static ref DECIMAL_RE: Regex = Regex::new(r"[0-9]+\.[0-9]+").unwrap();
                        static ref BINARY_RE: Regex = Regex::new(r"0x[a-fA-F0-9]*").unwrap();
                        static ref DATE_RE: Regex =
                            Regex::new(r"\d{1,2}/\d{1,2}/\d{2, 4}").unwrap();
                        static ref BOOLEAN_RE: Regex =
                            Regex::new(r"true|false|True|False").unwrap();
                    }

                    let matcher = &x.to_string();
                    if DECIMAL_RE.is_match(matcher) {
                        ColumnType::Decimal
                    } else if BINARY_RE.is_match(matcher) {
                        ColumnType::Binary
                    } else if DATE_RE.is_match(matcher) {
                        ColumnType::Date
                    } else if BOOLEAN_RE.is_match(matcher) {
                        ColumnType::Boolean
                    } else if INTEGER_RE.is_match(matcher) {
                        ColumnType::Integer
                    } else if STRING_RE.is_match(matcher) {
                        ColumnType::String
                    } else {
                        ColumnType::String
                    }
                })
                .collect::<Vec<ColumnType>>();

            let zipped = column_names.iter().zip(data_types.iter());

            let columns = zipped
                .into_iter()
                .map(|x| {
                    let column_type = x.1.clone();
                    let result = Column {
                        name: x.0.to_string(),
                        column_type: column_type.clone(),
                        nullable: true,
                        auto_increment: None,
                        is_integer: column_type == ColumnType::Integer,
                        is_string: column_type == ColumnType::String,
                        is_decimal: column_type == ColumnType::Decimal,
                        is_binary: column_type == ColumnType::Binary,
                        is_date: column_type == ColumnType::Date,
                        is_boolean: column_type == ColumnType::Boolean,
                    };
                    result
                })
                .collect::<Vec<Column>>();

            let result_document = doc.clone();
            let table = Table {
                columns: columns,
                audit_fields: false,
                constraints: vec![],
                table_name: doc.name,
                document: result_document
            };

            tables.push(table);
        }

        DbSchema {
            database: Database {
                database_name: database_name,
                dialect: dialect,
                tables: tables,
                queries: vec![],
                relationships: vec![],
            },
        }
    }
}
