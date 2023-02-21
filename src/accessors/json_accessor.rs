use serde::Deserialize;


pub fn read_json<T: for<'a> Deserialize<'a>>(path_string: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file_read: String = std::fs::read_to_string(path_string)?.clone();
    let deserialized: T = serde_json::from_str::<T>(&file_read)?;
    Ok(deserialized)
}