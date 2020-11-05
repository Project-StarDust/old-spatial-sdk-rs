use crate::ast::Component;
use crate::ast::Enum;
use crate::ast::SchemaFile;
use crate::ast::Type;
use crate::parser::component::parse_component;
use crate::parser::package_name::parse_package_name;
use crate::parser::r#enum::parse_enum;
use crate::parser::r#type::parse_type;
use nom::alt;
use nom::character::complete::multispace0;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;

#[derive(Default)]
pub struct SchemaFileBuilder {
    pub package_name: Option<Vec<String>>,
    pub name: Option<String>,
    pub types: Vec<Type>,
    pub enums: Vec<Enum>,
    pub components: Vec<Component>,
}

#[derive(Debug)]
pub enum SchemaModel {
    Type(Type),
    Component(Component),
    Enum(Enum),
}

impl SchemaFileBuilder {
    pub fn with_model(mut self, model: SchemaModel) -> Self {
        match model {
            SchemaModel::Type(t) => self.types.push(t),
            SchemaModel::Component(c) => self.components.push(c),
            SchemaModel::Enum(e) => self.enums.push(e),
        };
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn with_package_name(mut self, package_name: Vec<String>) -> Self {
        self.package_name = Some(package_name);
        self
    }

    pub fn build(self) -> Result<SchemaFile, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        let package_name = self.package_name.ok_or("Package Name could not be found")?;
        Ok(SchemaFile {
            package_name,
            name,
            components: self.components,
            types: self.types,
            enums: self.enums,
        })
    }
}

named!(
    parse_model<SchemaModel>,
    alt!(
        parse_type => { |t| SchemaModel::Type(t) } |
        parse_component => { |c| SchemaModel::Component(c) } |
        parse_enum => { |e| SchemaModel::Enum(e) }
    )
);

named!(
    parse_models<Vec<SchemaModel>>,
    separated_list!(multispace0, parse_model)
);

named!(
    pub parse_schema<SchemaFileBuilder>,
    do_parse!(
        package_name_parts: parse_package_name
            >> models: delimited!(multispace0, parse_models, multispace0)
            >> (models
                .into_iter()
                .fold(SchemaFileBuilder::default(), |acc, val| acc.with_model(val))
                .with_package_name(package_name_parts))
    )
);
