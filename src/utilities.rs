use crate::dbbuilder::Dialect;
use glob::glob;
use std::fmt;
use std::path::{Path, PathBuf};

// https://stackoverflow.com/questions/33759072/why-doesnt-vect-implement-the-display-trait
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

pub fn process_header(a: String, dialect: Dialect) -> String {
    let id_replacements = vec!(s(" "), s(")"), s("("), s("\t"));
    let replacer = s("");
    if dialect != Dialect::SqlServer {
        replace_all_with(a, id_replacements, replacer).to_lowercase()
    } else {
        replace_all_with(a, id_replacements, replacer)
    }
}

pub fn get_file_name(path: String) -> String {
    Path::new(&path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn pathbuf_to_string(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn get_glob_matches(directory: String, extension: String) -> Vec<String> {
    let mut filenames = Vec::<String>::new();
    let extension_glob = format!(r"**\*.{}", extension.as_str());
    let file_glob = directory + &extension_glob;
    for entry in glob(file_glob.as_str()).expect("Failed to read glob pattern") {
        filenames.push(pathbuf_to_string(&entry.unwrap()));
    }
    filenames
}

pub fn replace_all_with(target: String, strings: Vec<String>, replacement: String) -> String {
    let mut new_target = target;
    for s in strings {
        new_target = new_target.replace(s.as_str(), &replacement);
    }
    new_target
}

pub fn s(t: &str) -> String {
    t.to_string()
}