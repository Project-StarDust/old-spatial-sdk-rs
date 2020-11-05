use crate::ast::DataType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub r_type: DataType,
    pub args: Vec<DataType>,
}
