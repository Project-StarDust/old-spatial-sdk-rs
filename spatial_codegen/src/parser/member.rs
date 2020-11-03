use crate::ast::DataType;
use crate::ast::Member;
use crate::parser::utils::parse_usize;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::delimited;
use nom::do_parse;
use nom::named;
use nom::tag;

use crate::parser::data_type::parse_type;
use crate::parser::utils::snake_case as parse_member_name;

named!(
    parse_member_type_name<(DataType, String)>,
    do_parse!(
        member_type: parse_type
            >> multispace1
            >> member_name: parse_member_name
            >> (member_type, member_name)
    )
);

named!(
    pub parse_member<Member>,
    do_parse!(
        type_name: parse_member_type_name
            >> delimited!(multispace0, tag!("="), multispace0)
            >> id: parse_usize
            >> (Member { m_type: type_name.0, name: type_name.1, id })
    )
);
