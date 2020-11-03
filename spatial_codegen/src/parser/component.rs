use crate::ast::Command;
use crate::ast::Component;
use crate::ast::Event;
use crate::ast::Member;
use nom::alt;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::complete;
use nom::delimited;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::separated_list;
use nom::tag;

use nom::terminated;
use nom::tuple;

use crate::parser::command::parse_command;
use crate::parser::event::parse_event;
use crate::parser::member::parse_member;
use crate::parser::utils::camel_case as parse_component_name;
use crate::parser::utils::parse_usize;

enum ComponentProperty {
    ID(usize),
    Member(Member),
    Command(Command),
    Event(Event),
}

#[derive(Default)]
struct ComponentBuilder {
    pub name: Option<String>,
    pub id: Option<usize>,
    pub members: Vec<Member>,
    pub commands: Vec<Command>,
    pub events: Vec<Event>,
}

impl ComponentBuilder {
    pub fn with_property(mut self, property: ComponentProperty) -> Self {
        match property {
            ComponentProperty::ID(id) => self.id = Some(id),
            ComponentProperty::Member(m) => self.members.push(m),
            ComponentProperty::Event(e) => self.events.push(e),
            ComponentProperty::Command(c) => self.commands.push(c),
        };
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn build(self) -> Result<Component, &'static str> {
        let name = self.name.ok_or("Name could not be found")?;
        let id = self.id.ok_or("ID could not be found")?;
        Ok(Component {
            name,
            id,
            members: self.members,
            commands: self.commands,
            events: self.events,
        })
    }
}

named!(
    parse_id<usize>,
    do_parse!(
        tag!("id") >> delimited!(multispace0, tag!("="), multispace0) >> id: parse_usize >> (id)
    )
);

named!(
    parse_property<ComponentProperty>,
    alt!(
        parse_id => { |i| ComponentProperty::ID(i) }  |
        parse_member => { |m| ComponentProperty::Member(m) } |
        parse_command => { |c| ComponentProperty::Command(c) } |
        parse_event => { |e| ComponentProperty::Event(e) }
    )
);

named!(
    parse_properties<Vec<ComponentProperty>>,
    separated_list!(
        multispace0,
        terminated!(parse_property, tuple!(multispace0, char!(';')))
    )
);

named!(
    parse_component_body<Vec<ComponentProperty>>,
    delimited!(
        char!('{'),
        delimited!(multispace0, parse_properties, multispace0),
        char!('}')
    )
);

named!(
    pub parse_component<Component>,
    map_res!(do_parse!(
        complete!(tag!("component"))
            >> name: delimited!(multispace1, parse_component_name, multispace1)
            >> properties: parse_component_body
            >> (properties.into_iter().fold(ComponentBuilder::default(), |acc, val| acc.with_property(val)).with_name(name))
    ), |builder: ComponentBuilder| builder.build())
);
