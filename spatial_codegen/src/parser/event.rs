use crate::ast::Event;
use nom::character::complete::multispace1;
use nom::do_parse;
use nom::named;
use nom::tag;

use crate::parser::data_type::parse_type;
use crate::parser::utils::snake_case as parse_event_name;

named!(
    pub parse_event<Event>,
    do_parse!(
        tag!("event") >> multispace1 >> r_type: parse_type >> multispace1 >> name: parse_event_name >> (Event { r_type, name })
    )
);
