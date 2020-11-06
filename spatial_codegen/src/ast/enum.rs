use crate::ast::Value;
use std::convert::identity;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Enum {
    pub name: String,
    pub values: Vec<Value>,
}

impl Enum {
    pub fn generate_one(&self) -> String {
        format!(
            "{}\nenum {} {{{}}}\n",
            "#[spatial_enum]",
            self.name,
            Value::generate_multiple(&self.values)
        )
    }

    pub fn generate_multiple(data: &[Self]) -> String {
        if !data.is_empty() {
            data.iter()
                .map(Self::generate_one)
                .fold(String::new(), |acc, val| acc + "\n" + &val)
        } else {
            "".to_string()
        }
    }

    pub fn get_export(&self) -> Option<String> {
        Some(self.name.clone())
    }

    pub fn get_exports(data: &[Self]) -> Vec<String> {
        data.iter()
            .map(Self::get_export)
            .filter_map(identity)
            .collect()
    }
}
