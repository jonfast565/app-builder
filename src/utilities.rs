// https://stackoverflow.com/questions/33759072/why-doesnt-vect-implement-the-display-trait
use std::fmt;
use std::path::Path;
use crate::dbbuilder::{Dialect};

pub struct SliceDisplay<'a, T: 'a>(pub &'a [T]);

impl<'a, T: fmt::Display + 'a> fmt::Display for SliceDisplay<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        for item in self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        Ok(())
    }
}

pub fn process_header(a : String, dialect: Dialect) -> String {
    if dialect == Dialect::Postgres || dialect == Dialect::Sqlite {
        a.replace(" ", "_")
    } else {
        a.replace(" ", "")
    }
}

pub fn get_file_name(path: String) -> String {
    Path::new(&path).file_name().unwrap().to_str().unwrap().to_string()
}

