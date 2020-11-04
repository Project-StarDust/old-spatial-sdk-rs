use nom::character::is_digit;
use nom::fold_many1;
use nom::many0;
use nom::map;
use nom::map_opt;
use nom::map_res;
use nom::named;
use nom::one_of;
use nom::pair;
use nom::separated_list;
use nom::tag;
use nom::take_while1;

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

named!(
    pub camel_case<String>,
    fold_many1!(camel_case_component, String::new(), |acc, val| acc + &val)
);

named!(
    pub snake_case_component<String>,
    fold_many1!(lowercase, String::new(), |acc, val| acc + &val.to_string())
);

named!(
    pub upper_snake_case_component<String>,
    fold_many1!(uppercase, String::new(), |acc, val| acc + &val.to_string())
);

named!(
    pub snake_case<String>,
    map_opt!(map!(separated_list!(tag!("_"), snake_case_component), |v| {
        let mut it = v.into_iter();
        it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
    }), |o| o)
);

named!(
    pub upper_snake_case<String>,
    map_opt!(map!(separated_list!(tag!("_"), upper_snake_case_component), |v| {
        let mut it = v.into_iter();
        it.next().map(|e| it.fold(e, |acc, val| acc + "_" + &val))
    }), |o| o)
);

named!(
    pub parse_usize<usize>,
    map_res!(
        map_res!(take_while1!(is_digit), std::str::from_utf8),
        |s: &str| s.parse::<usize>()
    )
);
