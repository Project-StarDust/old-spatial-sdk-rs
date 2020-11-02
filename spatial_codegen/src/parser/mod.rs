pub mod component;
pub mod package_name;
pub mod types;
pub mod utils;

use crate::Component;
use crate::SchemaAST;
use component::parse_component;
use nom::character::complete::multispace0;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;
use package_name::parse_package_name;
use std::path::Path;

named!(
    parse_components<Vec<Component>>,
    separated_list!(multispace0, parse_component)
);

named!(
    pub parse_schema<SchemaAST>,
    do_parse!(
        package_name: delimited!(multispace0, parse_package_name, multispace0)
            >> components: parse_components
            >> (SchemaAST {
                package_name,
                components
            })
    )
);

pub fn parse_file<P: AsRef<Path>>(
    file: P,
) -> Result<SchemaAST, Box<dyn std::error::Error + 'static>> {
    let data = std::fs::read(file)?;
    let (_, schema) = parse_schema(&data).map_err(|e| format!("Cannot parse file: {}", e))?;
    Ok(schema)
}

#[cfg(test)]
mod tests {
    use crate::ComponentName;
use super::*;
    use crate::ComponentMember;
    use crate::PackageComponent;
    use crate::PackageName;

    #[test]
    fn test_parse_file() {
        let schema = parse_file("../test/schema/base.schema").unwrap();
        assert_eq!(
            schema,
            SchemaAST {
                package_name: PackageName(vec![
                    PackageComponent("io".to_string()),
                    PackageComponent("nebulis".to_string()),
                ]),
                components: vec![Component {
                    name: ComponentName {
                        components: vec!["Player".to_string(), "Marker".to_string()]
                    },
                    body: vec![ComponentMember::ID(4000)]
                }]
            }
        );
    }
}
