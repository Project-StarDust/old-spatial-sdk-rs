use crate::ast::DataType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Event {
    pub name: String,
    pub r_type: DataType,
}
