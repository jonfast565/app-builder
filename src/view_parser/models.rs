use serde::{Serialize, Deserialize};

use super::view_models::{FieldDefinition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDecl {
    pub nullable: bool,
    pub type_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpressionChoice {
    NumericConst(f64),
    Expression(ExprNode),
    Identifier(String),
    String(String),
    TypeDecl(TypeDecl),
    Array(Vec<ExpressionChoice>)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub key: String,
    pub value: ExpressionChoice
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExprNode {
    pub node_type: String,
    pub prop_list: Vec<Property>
}

impl ExprNode {
    pub fn get_expression_by_key(&self, key: &str) -> ExpressionChoice {
        let key_array = self.prop_list
            .clone()
            .into_iter()
            .filter(|x| x.key == key)
            .collect::<Vec<Property>>();

        let key_array_elem = key_array
            .first()
            .unwrap()
            .clone()
            .value;
        
        key_array_elem
    }

    pub fn get_name_value_props(&self) -> FieldDefinition {
        let id_elem = self.get_expression_by_key("name");
        let id = match id_elem {
            ExpressionChoice::Identifier(value) => value.clone(),
            _ => String::new(),
        };

        let type_decl_elem = self.get_expression_by_key("type");
        let type_decl = match type_decl_elem {
            ExpressionChoice::TypeDecl(t) => t.clone(),
            _ => TypeDecl { nullable: false, type_name: "".to_string() }
        };

        FieldDefinition::new(id, &type_decl)
    }
}