pub mod command;
pub mod component;
pub mod data_type;
pub mod r#enum;
pub mod event;
pub mod header;
pub mod member;
pub mod package_node;
pub mod schema_file;
pub mod r#type;
pub mod value;

pub use command::Command;
pub use component::Component;
pub use data_type::DataType;
pub use event::Event;
pub use header::Header;
pub use member::Member;
pub use package_node::PackageNode;
pub use r#enum::Enum;
pub use r#type::Type;
pub use schema_file::SchemaFile;
pub use value::Value;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct AST {
    pub inner: Vec<ASTNode>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ASTNode {
    PackageNode(PackageNode),
    SchemaNode(SchemaFile),
}

impl ASTNode {
    pub fn get_export(&self) -> (String, Vec<String>) {
        match self {
            Self::PackageNode(pn) => (pn.name.clone(), Vec::new()),
            Self::SchemaNode(schema) => (schema.name.clone(), schema.get_exports()),
        }
    }

    pub fn get_exports(data: &Vec<Self>) -> Vec<(String, Vec<String>)> {
        data.iter().map(Self::get_export).collect()
    }
}
