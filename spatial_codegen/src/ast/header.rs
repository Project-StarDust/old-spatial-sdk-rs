pub struct Header {}

impl Header {
    pub fn generate() -> String {
        format!(
            "{}\n{}\n{}\n{}\n",
            "#[allow(unused_imports)]\nuse spatial_macro::spatial_enum;",
            "#[allow(unused_imports)]\nuse spatial_macro::spatial_type;",
            "#[allow(unused_imports)]\nuse spatial_macro::spatial_component;",
            "#[allow(unused_imports)]\nuse std::collections::HashMap;"
        )
    }
}
