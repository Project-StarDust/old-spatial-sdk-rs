use std::convert::identity;
use crate::ast::Member;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Type {
    pub name: String,
    pub members: Vec<Member>,
}

impl Type {
    pub fn generate_one(&self) -> String {
        format!(
            "{}\nstruct {} {{{}}}",
            "#[spatial_type]",
            self.name,
            Member::generate_multiple(&self.members)
        )
    }

    pub fn generate_multiple(data: &Vec<Self>) -> String {
        data.iter()
            .map(Type::generate_one)
            .fold(String::new(), |acc, val| acc + "\n\n" + &val)
    }
    
    pub fn get_export(&self) -> Option<String> {
        Some(self.name.clone())
    }

    pub fn get_exports(data: &Vec<Self>) -> Vec<String> {
        data.iter()
            .map(Self::get_export)
            .filter_map(identity)
            .collect()
    }
}
