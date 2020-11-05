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
    Map(Box<DataType>, Box<DataType>),
    List(Box<DataType>),
    Option(Box<DataType>),
    UserDefined(String),
}

impl DataType {
    pub fn rust_type(&self) -> String {
        match self {
            Self::Bool => "bool".to_string(),
            Self::Uint32 => "u32".to_string(),
            Self::Uint64 => "u64".to_string(),
            Self::Int32 => "i32".to_string(),
            Self::Int64 => "i64".to_string(),
            Self::Float => "f32".to_string(),
            Self::Double => "f64".to_string(),
            Self::String => "String".to_string(),
            Self::Bytes => "Vec<u8>".to_string(),
            Self::Map(fst, snd) => format!(
                "std::collections::HashMap<{}, {}>",
                (*fst).rust_type(),
                (*snd).rust_type()
            ),
            Self::List(fst) => format!("Vec<{}>", (*fst).rust_type()),
            Self::Option(fst) => format!("Option<{}>", (*fst).rust_type()),
            Self::UserDefined(fst) => fst.to_string(),
            _ => "uninmplemented()!".to_string(),
        }
    }
}
