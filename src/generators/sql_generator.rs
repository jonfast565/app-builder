use std::fs;
use tera::{Tera, Context};
use super::super::models::sql_models::*;
use super::super::utilities::directories::*;
use super::super::utilities::tera::render_template;

pub fn generate_sql_view_from_database(
    query: &str,
    columns: &Vec<SqlColumn>,
    view_name: &str,
    paged: &bool,
    materialized: &bool,
    results_path: &str,
    template_name: &str,
    file_format: &str,
    tera: &Tera,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Create results directory...");
    create_results_path(results_path);

    let filename = format!("{}/view-{}.{}", results_path, view_name, file_format);
    println!("Generating {} for {} with {} columns...", template_name, view_name, columns.len());
    let mut ctx = Context::new();

    ctx.insert("view_name", view_name);
    ctx.insert("columns", columns);
    ctx.insert("paged", paged);
    ctx.insert("materialized", materialized);
    ctx.insert("query", query);

    render_template(tera, template_name, ctx, filename);
    Ok(())
}

pub fn generate_sql_view_from_json(
    file_path: &str,
    results_path: &str,
    template_name: &str,
    file_format: &str,
    tera: &Tera,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Create results directory...");
    create_results_path(results_path);
    let serialized = fs::read_to_string("./data/sql.json").expect("Failed to read sql.json");
    let queries: QueryRoot = serde_json::from_str(&serialized).unwrap();
    for query in queries.queries {
        let filename = format!("{}/query-{}.{}", results_path, query.name, file_format);
        println!("Generating {} from {}...", filename, file_path);
        let mut ctx = Context::new();
        ctx.insert("name", &query.name);
        ctx.insert("result_columns", &query.result_columns);
        ctx.insert("view", &query.view);
        ctx.insert("search_columns", &query.search_columns);
        ctx.insert("return_cursor", &query.return_cursor);
        ctx.insert("how_many_filters", &query.how_many_filters);
        render_template(tera, template_name, ctx, filename);
    }
    Ok(())
}

pub fn generate_sql_view_from_database_2(
    _query: &str,
    _columns: &Vec<SqlColumn>,
    _view_name: &str,
    _paged: &bool,
    _materialized: &bool,
    _results_path: &str,
    _template_name: &str,
    _file_format: &str,
    _tera: &Tera,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}