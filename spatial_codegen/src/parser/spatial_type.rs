use crate::ast::Member;
use crate::ast::Type;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::complete;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;
use nom::tag;

use crate::parser::member::parse_member;
use crate::parser::utils::camel_case as parse_type_name;

named!(
    parse_members<Vec<Member>>,
    separated_list!(
        delimited!(multispace0, char!(';'), multispace0),
        parse_member
    )
);

named!(
    parse_type_body<Vec<Member>>,
    delimited!(
        char!('{'),
        delimited!(multispace0, parse_members, multispace0),
        char!('}')
    )
);

named!(
    pub parse_type<Type>,
    do_parse!(
        complete!(tag!("type"))
            >> name: delimited!(multispace1, parse_type_name, multispace1)
            >> members: parse_type_body
            >> (Type { name, members })
    )
);
