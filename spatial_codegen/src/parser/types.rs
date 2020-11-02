use crate::Type;
use nom::alt;
use nom::bytes::complete::take_while;
use nom::character::is_alphabetic;
use nom::complete;
use nom::do_parse;
use nom::map_res;
use nom::named;
use nom::one_of;
use nom::tag;

named!(
    pub parse_primitives<Type>,
    alt!(
        complete!(tag!("bool"))     => { |_| Type::Bool }     |
        complete!(tag!("float"))    => { |_| Type::Float }    |
        complete!(tag!("bytes"))    => { |_| Type::Bytes }    |
        complete!(tag!("int32"))    => { |_| Type::Int32 }    |
        complete!(tag!("int64"))    => { |_| Type::Int64 }    |
        complete!(tag!("string"))   => { |_| Type::String }   |
        complete!(tag!("double"))   => { |_| Type::Double }   |
        complete!(tag!("uint32"))   => { |_| Type::Uint32 }   |
        complete!(tag!("uint64"))   => { |_| Type::Uint64 }   |
        complete!(tag!("sint32"))   => { |_| Type::SInt32 }   |
        complete!(tag!("sint64"))   => { |_| Type::SInt64 }   |
        complete!(tag!("fixed32"))  => { |_| Type::Fixed32 }  |
        complete!(tag!("fixed64"))  => { |_| Type::Fixed64 }  |
        complete!(tag!("sfixed32")) => { |_| Type::SFixed32 } |
        complete!(tag!("sfixed64")) => { |_| Type::SFixed64 } |
        complete!(tag!("EntityId")) => { |_| Type::EntityID } |
        complete!(tag!("Entity"))   => { |_| Type::Entity }
    )
);

named!(
    pub parse_custom_type<String>,
    do_parse!(
        first_letter: one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
            >> rest: map_res!(complete!(take_while(is_alphabetic)), |s| std::str::from_utf8(s))
            >> (first_letter.to_string() + rest)
    )
);

named!(
    pub parse_type<Type>,
    alt!(
        complete!(parse_primitives) |
        complete!(parse_custom_type) => { |s| Type::UserDefined(s) }
    )
);

#[cfg(test)]
mod tests {

    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn test_parse_primitives() {
        assert_eq!(parse_primitives(b"bool"), Ok(("".as_bytes(), Type::Bool)));
        assert_eq!(
            parse_primitives(b"uint32"),
            Ok(("".as_bytes(), Type::Uint32))
        );
        assert_eq!(
            parse_primitives(b"uint64"),
            Ok(("".as_bytes(), Type::Uint64))
        );
        assert_eq!(parse_primitives(b"int32"), Ok(("".as_bytes(), Type::Int32)));
        assert_eq!(parse_primitives(b"int64"), Ok(("".as_bytes(), Type::Int64)));
        assert_eq!(
            parse_primitives(b"sint32"),
            Ok(("".as_bytes(), Type::SInt32))
        );
        assert_eq!(
            parse_primitives(b"sint64"),
            Ok(("".as_bytes(), Type::SInt64))
        );
        assert_eq!(
            parse_primitives(b"fixed32"),
            Ok(("".as_bytes(), Type::Fixed32))
        );
        assert_eq!(
            parse_primitives(b"fixed64"),
            Ok(("".as_bytes(), Type::Fixed64))
        );
        assert_eq!(
            parse_primitives(b"sfixed32"),
            Ok(("".as_bytes(), Type::SFixed32))
        );
        assert_eq!(
            parse_primitives(b"sfixed64"),
            Ok(("".as_bytes(), Type::SFixed64))
        );
        assert_eq!(parse_primitives(b"float"), Ok(("".as_bytes(), Type::Float)));
        assert_eq!(
            parse_primitives(b"double"),
            Ok(("".as_bytes(), Type::Double))
        );
        assert_eq!(
            parse_primitives(b"string"),
            Ok(("".as_bytes(), Type::String))
        );
        assert_eq!(parse_primitives(b"bytes"), Ok(("".as_bytes(), Type::Bytes)));
        assert_eq!(
            parse_primitives(b"EntityId"),
            Ok(("".as_bytes(), Type::EntityID))
        );
        assert_eq!(
            parse_primitives(b"Entity"),
            Ok(("".as_bytes(), Type::Entity))
        );
        assert_eq!(
            parse_primitives(b"CustomComponent"),
            Err(Error(("CustomComponent".as_bytes(), ErrorKind::Alt)))
        );
    }

    #[test]
    fn test_parse_type() {
        assert_eq!(parse_type(b"bool"), Ok(("".as_bytes(), Type::Bool)));
        assert_eq!(parse_type(b"uint32"), Ok(("".as_bytes(), Type::Uint32)));
        assert_eq!(parse_type(b"uint64"), Ok(("".as_bytes(), Type::Uint64)));
        assert_eq!(parse_type(b"int32"), Ok(("".as_bytes(), Type::Int32)));
        assert_eq!(parse_type(b"int64"), Ok(("".as_bytes(), Type::Int64)));
        assert_eq!(parse_type(b"sint32"), Ok(("".as_bytes(), Type::SInt32)));
        assert_eq!(parse_type(b"sint64"), Ok(("".as_bytes(), Type::SInt64)));
        assert_eq!(parse_type(b"fixed32"), Ok(("".as_bytes(), Type::Fixed32)));
        assert_eq!(parse_type(b"fixed64"), Ok(("".as_bytes(), Type::Fixed64)));
        assert_eq!(parse_type(b"sfixed32"), Ok(("".as_bytes(), Type::SFixed32)));
        assert_eq!(parse_type(b"sfixed64"), Ok(("".as_bytes(), Type::SFixed64)));
        assert_eq!(parse_type(b"float"), Ok(("".as_bytes(), Type::Float)));
        assert_eq!(parse_type(b"double"), Ok(("".as_bytes(), Type::Double)));
        assert_eq!(parse_type(b"string"), Ok(("".as_bytes(), Type::String)));
        assert_eq!(parse_type(b"bytes"), Ok(("".as_bytes(), Type::Bytes)));
        assert_eq!(parse_type(b"EntityId"), Ok(("".as_bytes(), Type::EntityID)));
        assert_eq!(parse_type(b"Entity"), Ok(("".as_bytes(), Type::Entity)));
        assert_eq!(
            parse_type(b"CustomComponent"),
            Ok((
                "".as_bytes(),
                Type::UserDefined("CustomComponent".to_string())
            ))
        );
        assert_eq!(
            parse_type(b"customComponent"),
            Err(Error(("customComponent".as_bytes(), ErrorKind::Alt)))
        );
    }
}
