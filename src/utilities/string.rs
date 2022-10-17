
pub fn pascal_to_kebab(s: String) -> String {
    let mut result : Vec<char>  = Vec::new();
    let mut counter : u64 = 0;
    for c in s.chars() {
        if (char::is_uppercase(c) || char::is_numeric(c)) && counter != 0 {
            result.push('-');
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
        counter += 1;
    }
    result.into_iter().collect::<String>()
}

pub fn pascal_to_spaced(s: String) -> String {
    let mut result : Vec<char>  = Vec::new();
    let mut counter : u64 = 0;
    for c in s.chars() {
        if (char::is_uppercase(c) || char::is_numeric(c)) && counter != 0 {
            result.push(' ');
            result.push(c);
        } else {
            result.push(c);
        }
        counter += 1;
    }
    result.into_iter().collect::<String>()    
}