pub mod composer;
pub mod parser;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Bool,
    Uint32,
    Uint64,
    Int32,
    Int64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Float,
    Double,
    String,
    Bytes,
    EntityID,
    Entity,
    UserDefined(String),
}

impl Type {

    pub fn rust_type(&self) -> String {
        match self {
            Self::Bool => "bool",
            Self::Uint32 => "u32",
            Self::Uint64 => "u64",
            Self::Int32 => "i32",
            Self::Int64 => "i64",
            Self::Float => "f32",
            Self::Double => "f64",
            Self::String => "String",
            _ => "uninmplemented()!"
        }.to_string()
    }

}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ComponentName {
    pub components: Vec<String>,
}

impl ComponentName {
    pub fn camel_case(&self) -> String {
        self.components.join("")
    }

    pub fn snake_case(&self) -> String {
        self.components
            .iter()
            .map(|c| c.to_lowercase())
            .collect::<Vec<String>>()
            .join("_")
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageComponent(pub String);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Component {
    pub name: ComponentName,
    pub body: Vec<ComponentMember>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ComponentMember {
    ID(u32),
    Property(Type, String, u32),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PackageName(pub Vec<PackageComponent>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SchemaAST {
    pub package_name: PackageName,
    pub components: Vec<Component>,
}
