#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Value {
    pub name: String,
    pub id: usize,
}

impl Value {
    pub fn generate_one(&self) -> String {
        format!("    {}", self.name)
    }

    pub fn generate_multiple(data: &Vec<Self>) -> String {
        if data.len() > 0 {
            let values = data
                .iter()
                .map(Self::generate_one)
                .fold(String::new(), |acc, val| {
                    if acc.len() > 0 {
                        acc + ",\n" + &val
                    } else {
                        val
                    }
                });
            "\n".to_string() + &values + "\n"
        } else {
            "".to_string()
        }
    }
}
