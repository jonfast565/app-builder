use std::fs;

pub fn create_results_path(results_path: &str) {
    fs::create_dir_all(results_path).expect("Results directory could not be created");
}