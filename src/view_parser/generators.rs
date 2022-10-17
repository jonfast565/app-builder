use tera::{Context, Tera};

use std::fs;

pub fn generate_forms(
    file_path: &str,
    results_path: &str,
    template_name: &str,
    file_format: &str,
    tera: &Tera,
) -> Result<(), ()> {
    println!("Create results directory...");
    fs::create_dir_all(results_path).expect("Results directory could not be created");
    let exprs = super::extractors::parse_file(file_path);
    let forms = super::extractors::get_forms(exprs);
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