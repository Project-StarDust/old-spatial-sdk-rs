use crate::ast::Command;
use crate::ast::DataType;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::separated_list;
use nom::tag;

use crate::parser::data_type::parse_type;
use crate::parser::utils::snake_case as parse_command_name;

named!(
    parse_args<Vec<DataType>>,
    delimited!(
        char!('('),
        delimited!(
            multispace0,
            separated_list!(delimited!(multispace0, char!(','), multispace0), parse_type),
            multispace0
        ),
        char!(')')
    )
);

named!(
    pub parse_command<Command>,
    do_parse!(
        tag!("command") >> multispace1 >> r_type: parse_type >> multispace1 >> name: parse_command_name >> multispace0 >> args: parse_args >> (Command { r_type, name, args })
    )
);
