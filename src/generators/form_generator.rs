use tera::{Tera, Context};
use super::super::utilities::directories::*;
use super::super::utilities::tera::render_template;
use super::super::accessors::json_accessor::read_json;
use crate::models::form_models::Form;

pub fn generate_form_from_json(
    file_path: &str,
    results_path: &str,
    template_name: &str,
    file_format: &str,
    tera: &Tera,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Create results directory...");
    create_results_path(results_path);

    let filename = format!("{}/form-{}.{}", results_path, "form", file_format);
    println!("Generating {} for {} from {}...", template_name, "form", file_path);
    let mut ctx = Context::new();

    let deserialized = read_json::<Form>(file_path)?;

    ctx.insert("form", &deserialized);

    render_template(tera, template_name, ctx, filename);
    Ok(())
}