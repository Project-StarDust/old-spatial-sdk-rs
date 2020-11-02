use crate::ComponentName;
use crate::Type;
use crate::Component;
use crate::ComponentMember;
use super::types::parse_type;
use super::utils::camel_case_component;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::is_alphanumeric;
use nom::character::is_digit;
use nom::complete;
use nom::delimited;
use nom::do_parse;
use nom::many1;
use nom::map;
use nom::map_res;
use nom::named;
use nom::tag;
use nom::take_while1;
use nom::terminated;
use nom::alt;

named!(
    parse_member_name<String>,
    map!(
        map_res!(
            complete!(take_while1!(is_alphanumeric)),
            std::str::from_utf8
        ),
        |s| s.to_string()
    )
);

named!(
    parse_member_type_name<(Type, String)>,
    do_parse!(
        member_type: parse_type
            >> multispace1
            >> member_name: parse_member_name
            >> (member_type, member_name)
    )
);

named!(
    parse_u32<u32>,
    map_res!(
        map_res!(take_while1!(is_digit), std::str::from_utf8),
        |s: &str| s.parse::<u32>()
    )
);

named!(
    parse_id<ComponentMember>,
    do_parse!(
        tag!("id")
            >> delimited!(multispace0, tag!("="), multispace0)
            >> id: parse_u32
            >> (ComponentMember::ID(id))
    )
);

named!(
    parse_property<ComponentMember>,
    do_parse!(
        member_type_name: parse_member_type_name
            >> delimited!(multispace0, tag!("="), multispace0)
            >> id: parse_u32
            >> (ComponentMember::Property(member_type_name.0, member_type_name.1, id))
    )
);

named!(
    parse_member<ComponentMember>,
    alt!(parse_property | parse_id)
);

named!(
    parse_component_members<Vec<ComponentMember>>,
    many1!(terminated!(
        delimited!(multispace0, parse_member, multispace0),
        tag!(";")
    ))
);

named!(
    parse_component_body<Vec<ComponentMember>>,
    delimited!(
        char!('{'),
        delimited!(multispace0, parse_component_members, multispace0),
        char!('}')
    )
);

named!(
    pub parse_component_name<ComponentName>,
    do_parse!(
        components: many1!(camel_case_component) >> (ComponentName { components })
    )
);

named!(
    pub parse_component<Component>,
    do_parse!(
        complete!(tag!("component"))
            >> component_name: delimited!(multispace0, parse_component_name, multispace0)
            >> component_body: parse_component_body
            >> (Component {
                name: component_name,
                body: component_body
            })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_name() {
        let (input, component) =
            parse_component("component PlayerMarker {\tid = 4000;\n}".as_bytes())
                .expect("Parsing failed");
        assert_eq!(
            component,
            Component {
                name: ComponentName {
                    components: vec!["Player".to_string(), "Marker".to_string()]
                },
                body: vec![ComponentMember::ID(4000)]
            }
        );
        assert_eq!(input, []);
    }
}
