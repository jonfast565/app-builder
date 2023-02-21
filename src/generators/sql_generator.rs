use std::fs;
use tera::{Tera, Context};
use super::super::models::sql_models::*;

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
) -> Result<(), ()> {
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
) -> Result<(), ()> {
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

fn create_results_path(results_path: &str) {
    fs::create_dir_all(results_path).expect("Results directory could not be created");
}

fn render_template(tera: &Tera, template_name: &str, ctx: Context, path: String) {
    let output = tera.render(template_name, &ctx);
    match output {
        Ok(a) => fs::write(
            path,
            a,
        )
        .expect("file not written"),
        Err(e) => panic!("Parsing error(s): {}", e),
    }
}