use crate::ast::Value;
use crate::parser::utils::parse_usize;
use nom::character::complete::multispace0;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::tag;

use crate::parser::utils::upper_snake_case as parse_value_name;

named!(
    pub parse_value<Value>,
    do_parse!(
        name: parse_value_name
            >> delimited!(multispace0, tag!("="), multispace0)
            >> id: parse_usize
            >> (Value { name, id })
    )
);
