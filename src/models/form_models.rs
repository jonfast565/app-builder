use serde::{Serialize, Deserialize};

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
pub enum Component {
    DateInput(DateInputProps),
    NumericInput(NumericInputProps),
    TextInput(TextInputProps),
    Table(TableProps),
    Button(ButtonProps)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableComponent {
    pub row: usize,
    pub column: usize,
    components: Vec<Component>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableLayoutProps {
    contents: Vec<TableComponent>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoxLayoutProps {
    components: Vec<Component>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Layout {
    BoxLayout(BoxLayoutProps),
    TableLayout(TableLayoutProps)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
    layout: Layout,
}