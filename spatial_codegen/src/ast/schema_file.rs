use crate::ast::header::Header;
use crate::ast::Component;
use crate::ast::Enum;
use crate::ast::Type;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SchemaFile {
    pub name: String,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
    pub components: Vec<Component>,
}

impl SchemaFile {
    pub fn generate(&self) -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            Header::generate(),
            Enum::generate_multiple(&self.enums),
            Type::generate_multiple(&self.types),
            Component::generate_multiple(&self.components)
        )
    }

    pub fn get_exports(&self) -> Vec<String> {
        let mut exports = vec![];
        exports.extend(Enum::get_exports(&self.enums));
        exports.extend(Type::get_exports(&self.types));
        exports.extend(Component::get_exports(&self.components));
        exports
    }
}
