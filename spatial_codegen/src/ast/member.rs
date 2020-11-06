use crate::ast::DataType;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Member {
    pub name: String,
    pub m_type: DataType,
    pub id: usize,
}

impl Member {
    pub fn generate_one(&self) -> String {
        format!("    {}: {}", self.name, self.m_type.rust_type())
    }

    pub fn generate_multiple(data: &[Self]) -> String {
        if !data.is_empty() {
            let members = data
                .iter()
                .map(Member::generate_one)
                .fold(String::new(), |acc, val| {
                    if !acc.is_empty() {
                        acc + ",\n" + &val
                    } else {
                        val
                    }
                });
            "\n".to_string() + &members + "\n"
        } else {
            "".to_string()
        }
    }
}
