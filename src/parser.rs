use pest::Parser;
use pest::error::Error;

use pest::iterators::{Pair};

use crate::models::{ExprNode, Property, ExpressionChoice, TypeDecl};

#[derive(Parser)]
#[grammar = "./grammars/grammar.pest"]
struct AppGenParser;

pub fn parse_file(input: &String) -> Result<Vec<ExprNode>, Error<Rule>> {
    let statements = AppGenParser::parse(Rule::statements, input)?.next().unwrap();
    let parsed = unroll(statements);
    Ok(parsed)
}

fn unroll(root: Pair<Rule>) -> Vec<ExprNode> {
    let mut results = Vec::new();
    match root.as_rule() {
        Rule::statements => {
            let statements = root.into_inner();
            for token in statements {
                if token.as_rule() == Rule::EOI {
                    break
                }
                results.push(parse_expr(token));
            }
        } 
        _ => panic!("failed to unpack statements")
    }
    results
}

fn filter_pairs_option<'a>(expr: &'a Pair<Rule>, rule: crate::parser::Rule) -> Option<Pair<'a, Rule>> {
    let expr_innards = expr.clone().into_inner();
    let filter : Vec<Pair<Rule>> = expr_innards
                .clone()
                .filter(|x| x.as_rule() == rule)
                .collect();

    let first_item = filter.first().to_owned();
    match first_item {
        Some(a) => Some(a.clone()),
        None => None
    }
}

fn filter_pairs_inner_list<'a>(expr: &'a Pair<Rule>, rule: crate::parser::Rule) -> Vec<Pair<'a, Rule>> {
    let expr_innards = expr.clone().into_inner();
    let filter : Vec<Pair<Rule>> = expr_innards
                .clone()
                .filter(|x| x.as_rule() == rule)
                .collect();
    filter
}

fn filter_pairs_inner<'a>(expr: &'a Pair<Rule>, rule: crate::parser::Rule) -> Pair<'a, Rule> {
    let filter = filter_pairs_inner_list(expr, rule);
    let first_item = filter.first().unwrap().to_owned();
    first_item
}

fn filter_pairs_inner_first<'a>(expr: &'a Pair<Rule>) -> Pair<'a, Rule> {
    let expr_innards = expr.clone().into_inner();
    let pairs_iter : Vec<Pair<Rule>> = expr_innards.into_iter().collect();
    let first_item = pairs_iter.first().unwrap().to_owned();
    first_item
}

fn parse_expr(expr: Pair<Rule>) -> ExprNode {
    match expr.as_rule() {
        Rule::expr => {
            let keyword = filter_pairs_inner(&expr, Rule::keyword);
            let param_list = filter_pairs_inner(&expr, Rule::param_list);

            let result = ExprNode {
                node_type: keyword.as_str().to_string(),
                prop_list: parse_param_list(param_list),
            };
            
            return result
        }
        _ => panic!("failed to unpack exprs")
    }
}

fn parse_array(array: Pair<Rule>) -> ExpressionChoice {
    match array.as_rule() {
        Rule::array => {
            let array_list_filter = filter_pairs_inner(&array, Rule::array_list);
            let array_list_parsed = parse_array_list(array_list_filter);
            ExpressionChoice::Array(array_list_parsed)
        }
        _ => panic!("failed to unpack exprs")
    }
}

fn parse_array_list(array_list: Pair<Rule>) -> Vec<ExpressionChoice> {
    match array_list.as_rule() {
        Rule::array_list => {
            let values = filter_pairs_inner_list(&array_list, Rule::value);
            let mapped : Vec<ExpressionChoice> = values
                .into_iter()
                .map(|x| parse_value(x))
                .collect();
            mapped
        }
        _ => panic!("failed to unpack exprs")
    }
}

fn parse_param_list(param_list: Pair<Rule>) -> Vec<Property> { 
    match param_list.as_rule() {
        Rule::param_list => {
            let mut properties = Vec::new();
            let params = filter_pairs_inner_list(&param_list, Rule::param);
            for token in params {
                properties.push(parse_param(token));
            }
            return properties
        }
        _ => panic!("failed to unpack exprs")
    }
}

fn parse_param(param: Pair<Rule>) -> Property {
    match param.as_rule() {
        Rule::param => {
            let key = filter_pairs_inner(&param, Rule::identifier);
            let value = filter_pairs_inner(&param, Rule::value);

            let parsed_value = parse_value(value);
            let key_str = key.as_span().as_str();
            Property {
                key: key_str.to_string(),
                value: parsed_value,
            }
        }
        _ => panic!("failed to unpack exprs")
    }
}

fn parse_type_declaration(type_declaration: Pair<Rule>) -> TypeDecl {
    let nullable = filter_pairs_option(&type_declaration, Rule::nullable);
    let identifier = filter_pairs_inner(&type_declaration, Rule::identifier);
    
    TypeDecl {
        nullable: match nullable {
            Some(_) => true,
            None => false
        },
        type_name: identifier.as_str().to_string(),
    }
}

fn parse_value(value: Pair<Rule>) -> ExpressionChoice {
    match value.as_rule() {
        Rule::value => {
            let first_value = filter_pairs_inner_first(&value);
            parse_value_choice(first_value)
        },
        _ => panic!("failed to parse value")
    }
}

fn parse_value_choice(value: Pair<Rule>) -> ExpressionChoice { 
    match value.as_rule() {
        Rule::identifier => {
            ExpressionChoice::Identifier(value.as_str().to_string())
        },
        Rule::type_declaration => {
            let type_decl_parsed = parse_type_declaration(value.clone());
            ExpressionChoice::TypeDecl(type_decl_parsed)
        },
        Rule::numeric_constant => {
            let numeric_constant_str = value.as_str().to_string();
            let parsed = &numeric_constant_str.parse::<f64>().unwrap();
            ExpressionChoice::NumericConst(*parsed)
        },
        Rule::string => { 
            ExpressionChoice::String(value.as_str().to_string()) 
        },
        Rule::array => { 
            let array = parse_array(value);
            array
        },
        Rule::expr => { 
            let expression_parsed = parse_expr(value);
            ExpressionChoice::Expression(expression_parsed)
        },
        _ => panic!("failed to unpack values")
    }
}