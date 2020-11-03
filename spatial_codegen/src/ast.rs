pub struct AST {
    pub root: ASTNode,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ASTNode {
    PackageNode,
    SchemaNode,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DataType {
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

impl DataType {
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
            _ => "uninmplemented()!",
        }
        .to_string()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct PackageNode {
    pub name: String,
    pub inner: Vec<Box<ASTNode>>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SchemaFile {
    pub name: String,
    pub types: Vec<Type>,
    pub components: Vec<Component>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Type {
    pub name: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Component {
    pub name: String,
    pub id: usize,
    pub members: Vec<Member>,
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Member {
    pub name: String,
    pub m_type: DataType,
    pub id: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Event {
    pub name: String,
    pub r_type: DataType,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    pub name: String,
    pub r_type: DataType,
    pub args: Vec<DataType>,
}
