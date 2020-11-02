use nom::many0;
use nom::map;
use nom::named;
use nom::one_of;
use nom::pair;

named!(
    pub uppercase<char>,
    one_of!("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
);

named!(
    pub lowercase<char>,
    one_of!("abcdefghijklmnopqrstuvwxyz")
);

named!(
    pub camel_case_component<String>,
    map!(pair!(uppercase, many0!(lowercase)), |(c, s)| c.to_string() + &s.iter().collect::<String>())
);
