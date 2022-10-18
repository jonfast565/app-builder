use super::{models::{ExprNode, ExpressionChoice}, view_models::{FormDefinition, FormLayout, FormFieldSet}};

use std::fs;
// use conv::*;

pub fn parse_file(file: &str) -> Vec<ExprNode> {
    let file = fs::read_to_string(file).expect("File not found");
    let results = super::parser::parse_file(&file);
    let unwrapped = match results {
        Ok(p) => p,
        Err(e) => panic!("{}", e),
    };
    unwrapped
}

pub fn get_views() {
    
}

pub fn get_form_fieldset(form_field_set_element: &ExprNode) -> FormFieldSet {
    let name = match form_field_set_element.get_expression_by_key("name") {
        ExpressionChoice::String(name) => name.as_str().to_string(),
        _ => panic!("Unknown something"),
    };

    let fields = match form_field_set_element.get_expression_by_key("fields") {
        ExpressionChoice::Array(farray) => {
            let mut result = Vec::new();
            for f in farray {
                match f {
                    ExpressionChoice::Identifier(x) => result.push(x.as_str().to_string()),
                    _ => panic!("Unknown identifier"),
                }
            }
            result
        },
        _ => panic!("Unknown field"),
    };

    FormFieldSet {
        name,
        fields
    }
}

pub fn get_form_fieldset_list(form: &ExprNode) -> Vec<FormFieldSet> {
    let fieldsets = match form.get_expression_by_key("fieldsets") {
        ExpressionChoice::Array(x) => {
            let mut result = Vec::new();
            for fieldset in x {
                match fieldset {
                    ExpressionChoice::Expression(f) => {
                        result.push(get_form_fieldset(&f));
                    }
                    _ => panic!("{}", "Must be a fieldset here"),
                }
            }
            result
        }
        _ => panic!("{}", ""),
    };
    fieldsets
}

pub fn get_form_layout(form: &ExprNode) -> FormLayout {
    let layout = match form.get_expression_by_key("layout") {
        ExpressionChoice::Expression(e) => {
            let width = match e.get_expression_by_key("width") {
                ExpressionChoice::NumericConst(x) => x as u64,
                _ => panic!("failed to unpack width in layout elem"),
            };

            let height = match e.get_expression_by_key("height") {
                ExpressionChoice::NumericConst(x) => x as u64,
                _ => panic!("failed to unpack height in layout elem"),
            };

            let field_sets = get_form_fieldsets(form);

            FormLayout {
                width,
                height,
                field_sets,
            }
        },
        _ => panic!("Layout expression not found, could not parse"),
    };

    layout
}

fn get_form_fieldsets(form: &ExprNode) -> Vec<String> {
    let field_sets = match form.get_expression_by_key("fieldsets") {
        ExpressionChoice::Array(x) => {
            let mut fieldsets = Vec::new();
            for field_set_name in x {
                match field_set_name {
                    ExpressionChoice::Expression(x) => {
                            match x.get_expression_by_key("fields") {
                                ExpressionChoice::Array(x) => { 
                                    for y in x {
                                        match y {
                                            ExpressionChoice::Identifier(x) => fieldsets.push(x.as_str().to_string()),
                                            _ => panic!("failed to unpack field set in layout elem"),
                                        }
                                    }
                                },
                                _ => panic!("failed to unpack field set in layout elem"),
                            }
                        },
                    _ => panic!("failed to unpack field sets in field set elem"),
                }
            }
            fieldsets
        }
        _ => panic!("failed to unpack fieldsets in layout elem"),
    };
    field_sets
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

        let name = match form.get_expression_by_key("name") {
            ExpressionChoice::Identifier(value) => value.to_string(),
            _ => panic!("{}", "name not parsed"),
        };

        match form.get_expression_by_key("fields") {
            ExpressionChoice::Array(value) => {
                for v in value {
                    match v {
                        ExpressionChoice::Expression(v) => fields.push(v.get_name_value_props()),
                        _ => panic!("Expression not found, could not parse key value pairs."),
                    }
                }
            }
            _ => panic!("Fields array not found, could not parse."),
        };

        let layout = get_form_layout(&form);
        let field_sets = get_form_fieldset_list(&form);

        forms.push(FormDefinition {
            name,
            fields,
            layout,
            field_sets
        })
    }
    forms
}
