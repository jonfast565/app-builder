use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;

pub fn get_csv_schema(path_string: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}