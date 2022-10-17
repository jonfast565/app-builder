use super::{models::{ExprNode, ExpressionChoice}, view_models::FormDefinition};

use std::fs;

pub fn parse_file(file: &str) -> Vec<ExprNode> {
    let file = fs::read_to_string(file).expect("File not found");
    let results = super::parser::parse_file(&file);
    let unwrapped = match results {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };
    unwrapped
}

pub fn get_forms(exprs: Vec<ExprNode>) -> Vec<FormDefinition> {
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
            name,
            fields,
        })
    }

    forms
}