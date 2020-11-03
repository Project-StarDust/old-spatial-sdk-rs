use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;

use nom::delimited;

use nom::named;
use nom::separated_list;
use nom::tag;

use nom::tuple;

use crate::parser::utils::snake_case;
use nom::bytes::complete::tag as tag_complete;

named!(
    pub parse_package_components<Vec<String>>,
    separated_list!(
        tag_complete("."),
        snake_case
    )
);

named!(
    pub parse_package_name<Vec<String>>,
    delimited!(
        tuple!(
            tag!("package"),
            multispace1
        ),
        parse_package_components,
        tuple!(
            multispace0,
            char!(';')
        )
    )
);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_package_components() {
        assert_eq!(
            parse_package_components(b"io.nebulis"),
            Ok(("".as_bytes(), vec!["io".to_string(), "nebulis".to_string()]))
        )
    }

    #[test]
    fn test_parse_package_name() {
        assert_eq!(
            parse_package_name(b"package io.nebulis;"),
            Ok(("".as_bytes(), vec!["io".to_string(), "nebulis".to_string()]))
        )
    }
}
