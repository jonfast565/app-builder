use serde::{Serialize, Deserialize};

pub struct BasicProperties {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ButtonProps {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableProps {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextInputProps {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct NumericInputProps {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct DateInputProps {

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "properties")]
pub enum ComponentType {
    DateInput(DateInputProps),
    NumericInput(NumericInputProps),
    TextInput(TextInputProps),
    Table(TableProps),
    Button(ButtonProps)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableLayoutProps {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxLayoutProps {

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Layout {
    BoxLayout(BoxLayoutProps),
    TableLayout(TableLayoutProps)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicFormField {
    pub data_type: String,
    pub nullable: bool,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicForm {
    pub name: String,
    pub fields: Vec<BasicFormField>,
}