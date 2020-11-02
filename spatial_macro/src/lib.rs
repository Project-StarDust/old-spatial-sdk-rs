extern crate proc_macro;

use nom::alt;
use nom::bytes::complete::take_while1;
use nom::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::is_alphanumeric;
use nom::character::is_digit;
use nom::delimited;
use nom::do_parse;
use nom::map;
use nom::map_res;
use nom::named;
use nom::separated_list;
use nom::tag;
use nom::tuple;
use proc_macro::TokenStream;

#[derive(Debug)]
struct StructAST {
    name: String,
    body: Vec<StructMember>,
}

#[derive(Debug)]
struct StructMember {
    name: String,
    member_type: String,
}

const VALIDATE_INDEX_ERROR: &'static str = "Unless you are using custom component replication code, this is most likely caused by a code generation bug. Please contact nebulis support if you encounter this issue.";

named!(
    parse_id<u32>,
    map_res!(
        map_res!(take_while1(is_digit), std::str::from_utf8),
        |s: &str| { s.parse::<u32>() }
    )
);

named!(
    parse_name<String>,
    map!(
        map_res!(take_while1(is_alphanumeric), std::str::from_utf8),
        String::from
    )
);

named!(parse_type<String>, do_parse!(mtype: parse_name >> (mtype)));

named!(
    parse_member<StructMember>,
    do_parse!(
        name: parse_name
            >> multispace0
            >> tag!(":")
            >> multispace0
            >> member_type: parse_type
            >> (StructMember { name, member_type })
    )
);

named!(
    parse_body<Vec<StructMember>>,
    delimited!(
        multispace0,
        separated_list!(
            tag!(","),
            delimited!(multispace0, parse_member, multispace0)
        ),
        multispace0
    )
);

named!(
    parse_struct<StructAST>,
    do_parse!(
        alt!(
            map!(tuple!(tag!("pub"), multispace1, tag!("struct")), |_| ())
                | map!(tag!("struct"), |_| ())
        ) >> multispace1
            >> name: parse_name
            >> multispace1
            >> body: delimited!(
                char!('{'),
                alt!(
                    do_parse!(
                        body: parse_body >> multispace0 >> char!(',') >> multispace0 >> (body)
                    ) | parse_body
                ),
                char!('}')
            )
            >> (StructAST { name, body })
    )
);

fn get_dirty_bits_count(ast: &StructAST) -> usize {
    ast.body.len()
}

fn generate_const_id(id: u32) -> String {
    format!("const ID: u32 = {};", id)
}

fn generate_getter(member: &StructMember) -> String {
    format!(
        "pub fn get_{}(&self) -> {} {{\nself.{}\n}}",
        member.name, member.member_type, member.name
    )
}

fn generate_setter(member: &StructMember, index: usize) -> String {
    format!("pub fn set_{}(&mut self, d: {}) -> Result<(), &'static str> {{\nself.{} = d;\nself.mark_data_dirty({})}}", member.name, member.member_type, member.name, index)
}

fn generate_constructor(ast: &StructAST) -> String {
    let arguments = ast
        .body
        .iter()
        .map(|m| format!("{}: {}", m.name, m.member_type))
        .collect::<Vec<String>>()
        .join(",");
    let constructor_vars = ast
        .body
        .iter()
        .map(|m| format!("{}", m.name))
        .collect::<Vec<String>>()
        .join(",");
    format!(
        "pub fn new({}) -> Self {{ Self {{ {} {} dirty_bits: [{}] }} }}",
        arguments,
        constructor_vars,
        if constructor_vars.len() > 0 { ',' } else { ' ' },
        (0..get_dirty_bits_count(&ast))
            .map(|_| "0".to_string())
            .collect::<Vec<String>>()
            .join(",")
    )
}

fn generate_getter_setter(ast: &StructAST) -> String {
    ast.body
        .iter()
        .enumerate()
        .map(|(index, m)| generate_getter(m) + &generate_setter(m, index))
        .fold(String::new(), |acc, val| acc + &val)
}

fn generate_validate_index(ast: &StructAST) -> String {
    format!("fn validate_index(&self, index: u32) -> Result<u32, &'static str> {{ if index < 0 || index >= {} {{ Err(\"{} {}\") }} else {{ Ok(index) }} }}", get_dirty_bits_count(&ast), format!("\\\"index\\\" argument out of range. Valid range is [0, {}]." , get_dirty_bits_count(&ast) as isize - 1), VALIDATE_INDEX_ERROR)
}

fn generate_mark_dirty() -> String {
    format!("fn mark_data_dirty(&mut self, index: u32) -> Result<(), &'static str> {{\nlet index = self.validate_index(index)?; let dirty_bits_byte_index = index >> 5; self.dirty_bits[dirty_bits_byte_index as usize] |= (0x1 << (index & 31)); Ok(())\n}}")
}

fn generate_is_data_dirty(ast: &StructAST) -> String {
    format!(
        "fn is_data_dirty(&self) -> bool {{ let mut data_dirty = false;\n{}\ndata_dirty}}",
        ast.body
            .iter()
            .enumerate()
            .map(|(i, _)| format!("data_dirty |= (self.dirty_bits[{}] != 0x0);", i))
            .fold(String::new(), |acc, val| acc + &val)
    )
}

fn generate_implementation(id: u32, ast: &StructAST) -> String {
    let name = ast.name.clone();
    format!(
        "impl {} {{\n{}\n{}\n{}\n{}\n{}\n{}\n}}",
        name,
        generate_const_id(id),
        generate_constructor(&ast),
        generate_getter_setter(&ast),
        generate_validate_index(&ast),
        generate_is_data_dirty(&ast),
        generate_mark_dirty()
    )
}

fn generate_display_impl(ast: &StructAST) -> String {
    let name = ast.name.clone();
    let ref_params = ast
        .body
        .iter()
        .enumerate()
        .map(|(i, m)| format!("{}: ref __self_0_{}", m.name, i))
        .collect::<Vec<String>>()
        .join(",");

    let write_macros = ast
        .body
        .iter()
        .enumerate()
        .map(|(i, m)| format!("write!(f, \"{}: {{}}\", __self_0_{})?;", m.name, i))
        .collect::<Vec<String>>()
        .join("write!(f, \", \")?;");

    format!(
        "impl std::fmt::Display for {} {{\nfn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{
            match *self {{
                {} {{
                    {}{}
                    dirty_bits: ref __self_1_0
                }} => {{
                    write!(f, \"{}\")?;
                    write!(f, \" {{{{ \")?;
                    {}
                    write!(f, \" }}}}\")
                }}
            }}
        }} }}", name, name, ref_params, if ref_params.len() > 0 { ',' } else { ' ' }, name, write_macros
    )
}

fn generate_struct(ast: &StructAST) -> String {
    let generate_fields = |ast: &StructAST| {
        ast.body
            .iter()
            .map(|val| format!("{}: {},", val.name, val.member_type))
            .fold(String::new(), |acc, val| acc + &val)
    };
    let generate_dirty_bits =
        |ast: &StructAST| format!("dirty_bits: [u32; {}]", get_dirty_bits_count(&ast));

    format!(
        "#[derive(Debug)]\npub struct {} {{\n{}\n{}\n}}",
        ast.name,
        generate_fields(ast),
        generate_dirty_bits(ast)
    )
}

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let id = parse_id(attr.to_string().as_bytes())
        .expect("ID is needed")
        .1;
    let ast = parse_struct(item.to_string().as_bytes())
        .expect("Can't determine AST")
        .1;
    let implementation = generate_implementation(id, &ast);
    let component = generate_struct(&ast);
    let display_impl = generate_display_impl(&ast);
    let t = format!("{}\n{}\n{}", component, implementation, display_impl);
    t.parse().unwrap()
}
