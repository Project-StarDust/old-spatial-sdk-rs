use crate::ast::Component;
use crate::ast::SchemaFile;
use crate::ast::Type;
use crate::parser::component::parse_component;
use crate::parser::package_name::parse_package_name;
use crate::parser::spatial_type::parse_type;
use nom::alt;
use nom::character::complete::multispace0;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Default)]
struct SchemaFileBuilder {
    pub name: Option<String>,
    pub types: Vec<Type>,
    pub components: Vec<Component>,
}

enum SchemaModel {
    Type(Type),
    Component(Component),
}

impl SchemaFileBuilder {
    pub fn with_model(mut self, model: SchemaModel) -> Self {
        match model {
            SchemaModel::Type(t) => self.types.push(t),
            SchemaModel::Component(c) => self.components.push(c),
        };
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<SchemaFile, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        Ok(SchemaFile {
            name,
            components: self.components,
            types: self.types,
        })
    }
}

named!(
    parse_model<SchemaModel>,
    alt!(
        parse_type => { |t| SchemaModel::Type(t) } |
        parse_component => { |c| SchemaModel::Component(c) }
    )
);

named!(
    parse_models<Vec<SchemaModel>>,
    separated_list!(multispace0, parse_model)
);

named!(
    parse_schema<(Vec<String>, SchemaFileBuilder)>,
    do_parse!(
        package_name_parts: parse_package_name
            >> models: delimited!(multispace0, parse_models, multispace0)
            >> (
                package_name_parts,
                models
                    .into_iter()
                    .fold(SchemaFileBuilder::default(), |acc, val| acc.with_model(val))
            )
    )
);

fn parse_schema_content(file_content: String) -> Result<(Vec<String>, SchemaFileBuilder), String> {
    parse_schema(file_content.as_bytes())
        .map(|r| r.1)
        .map_err(|e| format!("Unable to parse data: {}", e))
}

pub fn parse_schema_file<P: AsRef<Path>>(path: P) -> Result<(Vec<String>, SchemaFile), String> {
    let filename = path
        .as_ref()
        .file_stem()
        .ok_or("Unable to get file stem")
        .map(|s| s.to_str())?
        .ok_or("Can't convert file stem to UTF-8")
        .map(|s| s.to_string())?;
    let mut file = File::open(path).map_err(|e| format!("Unable to open file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| format!("Unable to read file: {}", e))?;
    parse_schema_content(contents)
        .map(|(pn, sb)| sb.with_name(filename).build().map(|s| (pn, s)))?
        .map_err(|e| format!("Cannot conver SchemaFile: {}", e))
}
