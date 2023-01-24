/*
            Type::BOOL => SqlType::Bool,
            Type::CHAR => SqlType::Int8,
            Type::INT2 => SqlType::Int16,
            Type::INT4 => SqlType::Int32,
            Type::INT8 => SqlType::Int64,
            Type::VARCHAR => SqlType::String,
            Type::TIMESTAMP => SqlType::Timestamp,
            Type::JSONB => SqlType::String,
*/

pub fn data_type_to_csharp_type(s: &str) -> &str {
    match s {
        "Bool" => "bool",
        "Int8" => "byte",
        "Int16" => "short",
        "Int32" => "int",
        "Int64" => "long",
        "String"=> "string",
        "Timestamp" => "DateTime",
        "Date" => "DateTime",
        "Text" => "string",    
        _ => todo!("{} is not a valid type", s)
    }
}