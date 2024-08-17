use std::path::PathBuf;
use nom::branch::alt;
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::combinator::{map, recognize, value, verify};
use nom::error::context;

use nom::multi::many0_count;
use nom::sequence::tuple;
use crate::error::NomError;
use crate::input::Input;
use crate::inputctx::ParserContext;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::util::token;
use std::{collections::HashSet, str};

const KEYWORDS: &[&str] = &[
    "as", "else", "export", "for", "foreign", "if", "in", "import", "type",
];
type IResult<'a, T> = nom::IResult<Input<'a>, T, NomError<'a>>;

fn identifier(input: Input) -> IResult<String> {
    context("identifier", token(raw_identifier))(input)
}

fn raw_identifier(input: Input) -> IResult<String> {
    verify(unchecked_identifier, |identifier: &str| {
        !KEYWORDS.contains(&identifier)
    })(input)
}

fn unchecked_identifier(input: Input) -> IResult<String> {
    map(
        recognize(tuple((
            alt((value((), alpha1::<Input, _>), value((), char('_')))),
            many0_count(alt((value((), alphanumeric1), value((), char('_'))))),
        ))),
        |span| str::from_utf8(span.as_bytes()).unwrap().into(),
    )(input)
}

#[test]
// 测试解析器
fn main() {
    let inputs = vec![
        " myVar",
        "_myVar",
        " my123Var",
        "functionName",
        "MAX_LEN",
        "max_len",
        "maxLen",
        "3barsDown",
        "InvalidIdentifier!",
    ];

    for input in inputs {
        let mut path = PathBuf::new();
        let ctx=ParserContext::new(path);
        match identifier(Input::new_extra(input,ctx)) {
            Ok((remaining, parsed)) => {
                println!("Parsed identifier: {:?}, Remaining: {}", parsed, remaining);
            }
            Err(err) => {
                println!("Failed to parse '{}': {:?}", input, err);
            }
        }
    }
}
