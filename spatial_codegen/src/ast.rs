#[derive(Debug, Eq, PartialEq, Default)]
pub struct AST {
    pub root: Option<ASTNode>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ASTNode {
    PackageNode(PackageNode),
    SchemaNode(SchemaFile),
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl PackageNode {
    pub fn add_node(self, node: ASTNode) -> Self {
        let mut inner = self.inner;
        inner.push(Box::new(node));
        Self {
            name: self.name,
            inner,
        }
    }

    pub fn has_path<S: AsRef<str>>(&self, path: S) -> bool {
        self.inner
            .iter()
            .map(|node| match &**node {
                ASTNode::SchemaNode(_) => false,
                ASTNode::PackageNode(pn) => pn.name == path.as_ref().to_string(),
            })
            .fold(false, |acc, val| acc | val)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SchemaFile {
    pub name: String,
    pub types: Vec<Type>,
    pub components: Vec<Component>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub members: Vec<Member>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Component {
    pub name: String,
    pub id: usize,
    pub members: Vec<Member>,
    pub events: Vec<Event>,
    pub commands: Vec<Command>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Member {
    pub name: String,
    pub m_type: DataType,
    pub id: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Event {
    pub name: String,
    pub r_type: DataType,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub r_type: DataType,
    pub args: Vec<DataType>,
}
