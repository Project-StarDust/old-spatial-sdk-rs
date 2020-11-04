use crate::ast::Enum;
use crate::ast::Value;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::complete;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;
use nom::tag;

use nom::terminated;
use nom::tuple;

use crate::parser::utils::camel_case as parse_enum_name;
use crate::parser::value::parse_value;

named!(
    parse_values<Vec<Value>>,
    separated_list!(
        multispace0,
        terminated!(parse_value, tuple!(multispace0, char!(';')))
    )
);

named!(
    parse_enum_body<Vec<Value>>,
    delimited!(
        char!('{'),
        delimited!(multispace0, parse_values, multispace0),
        char!('}')
    )
);

named!(
    pub parse_enum<Enum>,
    do_parse!(
        complete!(tag!("enum"))
            >> name: delimited!(multispace1, parse_enum_name, multispace1)
            >> values: parse_enum_body
            >> (Enum { name, values })
    )
);
