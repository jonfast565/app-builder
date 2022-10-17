extern crate pest;
#[macro_use]
extern crate pest_derive;

mod view_parser;
mod utilities;

use tera::{Tera};

fn print_header() {
    println!("{}", "--- AppGen Engine ----");
    println!("{}", "Author: Jonathan Fast");
    println!("{}\n", "Description: Generate application code easily!");
}

fn main() -> Result<(), ()> {
    print_header();
    println!("Initializing templating engine...");
    let tera = get_tera_instance();
    view_parser::generators::generate_forms(
        "./views/idash_view.view",
        "./target/results",
        "blazor.tera",
        "blazor",
        &tera,
    )?;
    view_parser::generators::generate_forms(
        "./views/idash_view.view",
        "./target/results",
        "term-gui.tera",
        "cs",
        &tera,
    )?;
    println!("{}", "Done!");
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
    tera.register_filter("pascal_to_kebab", utilities::tera_filters::pascal_to_kebab);
    tera.register_filter("pascal_to_spaced", utilities::tera_filters::pascal_to_spaced);
    tera
}

