extern crate pest;
#[macro_use]
extern crate pest_derive;

mod extractors;
mod filters;
mod models;
mod parser;
mod utility;

use models::{ExprNode, ExpressionChoice, FormDefinition};
use std::fs;
use tera::{Context, Tera};

fn print_header() {
    println!("{}", "--- AppGen Engine ----");
    println!("{}", "Author: Jonathan Fast");
    println!("{}\n", "Description: Generate application code easily!");
}

fn main() -> Result<(), ()> {
    print_header();
    println!("Initializing templating engine...");
    let tera = get_tera_instance();
    generate_forms(
        "./views/idash_view.view",
        "./target/results",
        "blazor.tera",
        "blazor",
        &tera,
    )?;
    generate_forms(
        "./views/idash_view.view",
        "./target/results",
        "term-gui.tera",
        "cs",
        &tera,
    )?;
    println!("{}", "Done!");
    Ok(())
}

fn generate_forms(
    file_path: &str,
    results_path: &str,
    template_name: &str,
    file_format: &str,
    tera: &Tera,
) -> Result<(), ()> {
    println!("Create results directory...");
    fs::create_dir_all(results_path).expect("Results directory could not be created");
    let exprs = parse_file(file_path);
    let forms = get_forms(exprs);
    for form in forms {
        let filename = format!("{}/form-{}.{}", results_path, form.name, file_format);
        println!("Generating {} from {}...", filename, file_path);
        let mut ctx = Context::new();
        ctx.insert("name", &form.name);
        ctx.insert("fields", &form.fields);
        let output = tera.render(template_name, &ctx);
        match output {
            Ok(a) => fs::write(
                format!("{}/form-{}.{}", results_path, form.name, file_format),
                a,
            )
            .expect("file not written"),
            Err(e) => panic!("Parsing error(s): {}", e),
        }
    }
    Ok(())
}

fn get_tera_instance() -> Tera {
    let mut tera = match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.register_filter("pascal_to_kebab", filters::pascal_to_kebab);
    tera.register_filter("pascal_to_spaced", filters::pascal_to_spaced);
    tera
}

fn get_forms(exprs: Vec<ExprNode>) -> Vec<FormDefinition> {
    let mut forms = Vec::new();

    let exprs_forms: Vec<ExprNode> = exprs
        .clone()
        .into_iter()
        .filter(|x| x.node_type == "form")
        .collect();

    for form in &exprs_forms {
        let mut fields = Vec::new();

        let name_array_elem = form.get_expression_by_key("name");
        let name = match name_array_elem {
            ExpressionChoice::Identifier(value) => value.to_string(),
            _ => String::new(),
        };

        let field_array_elem = form.get_expression_by_key("fields");

        match field_array_elem {
            ExpressionChoice::Array(value) => {
                for v in value {
                    match v {
                        ExpressionChoice::Expression(v) => fields.push(v.get_name_value_props()),
                        _ => (),
                    }
                }
            }
            _ => (),
        };

        forms.push(FormDefinition {
            name: name,
            fields: fields,
        })
    }

    forms
}

fn parse_file(file: &str) -> Vec<ExprNode> {
    let file = fs::read_to_string(file).expect("File not found");
    let results = parser::parse_file(&file);
    let unwrapped = match results {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };
    unwrapped
}
