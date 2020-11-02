use crate::PackageName;
use crate::PackageComponent;
use nom::character::is_alphabetic;
use nom::character::is_space;
use nom::do_parse;
use nom::map;
use nom::map_res;
use nom::named;
use nom::separated_list;
use nom::tag;
use nom::take_while;
use nom::take_while1;

use nom::bytes::complete::tag as tag_complete;
use nom::bytes::complete::take_while1 as take_while1;


impl From<String> for PackageComponent {
    fn from(data: String) -> Self {
        PackageComponent(data)
    }
}

named!(
    pub parse_package_components<Vec<PackageComponent>>,
    separated_list!(
        tag_complete("."),
        map!(
            map!(
                map_res!(
                    take_while1(is_alphabetic),
                    std::str::from_utf8
                ),
                String::from
            ),
            PackageComponent::from
        )
    )
);

named!(
    pub parse_package_name<PackageName>,
    do_parse!(
        tag!("package") >>
        take_while1!(is_space) >>
        package_components: parse_package_components >>
        take_while!(is_space) >>
        tag!(";") >>
        (PackageName(package_components))
    )
);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_package_components() {
        assert_eq!(
            parse_package_components(b"io.nebulis"),
            Ok((
                "".as_bytes(),
                vec![
                    PackageComponent("io".to_string()),
                    PackageComponent("nebulis".to_string())
                ]
            ))
        )
    }

    #[test]
    fn test_parse_package_name() {
        assert_eq!(
            parse_package_name(b"package io.nebulis;"),
            Ok((
                "".as_bytes(),
                PackageName(vec![
                    PackageComponent("io".to_string()),
                    PackageComponent("nebulis".to_string()),
                ])
            ))
        )
    }
}
