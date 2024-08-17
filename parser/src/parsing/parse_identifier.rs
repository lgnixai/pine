use std::path::PathBuf;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::take_while;
use nom::combinator::{map, value, verify};
use nom::error::context;
use nom::multi::many0_count;
use nom::sequence::tuple;

use crate::input::{Input, PineResult, Positioned, positioned};
use crate::inputctx::ParserContext;
use crate::lexer::identifier::Identifier;
use crate::parsing::util::token;

use std::{collections::HashSet, str};

const KEYWORDS: &[&str] = &[
    "as", "else", "export", "for", "foreign", "if", "in", "import", "type",
];
fn is_valid_start_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_valid_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

// 识别标识符的解析器
pub fn _parse_identifier_(input:Input) -> PineResult<Positioned<Identifier>> {
    // recognize(pair(
    //     take_while(is_valid_start_char),
    //     take_while(is_valid_char)
    // ))(input)

    positioned(map(recognize(pair(
        take_while(is_valid_start_char),
        take_while(is_valid_char),
    )), |s:Input| Identifier::new(s.to_string(),0)))(input)

}


pub(crate) fn parse_identifier(input: Input) -> PineResult<Positioned<Identifier>> {
    context("identifier", token(raw_identifier))(input)
}

fn raw_identifier(input: Input) -> PineResult<Positioned<Identifier>> {
    verify(unchecked_identifier, |identifier| {
        !KEYWORDS.contains(&&*identifier.value.name)
    })(input)
}

fn unchecked_identifier(input: Input) -> PineResult<Positioned<Identifier>> {
    positioned(map(
        recognize(tuple((
            alt((value((), alpha1::<Input, _>), value((), char('_')))),
            many0_count(alt((value((), alphanumeric1), value((), char('_'))))),
        ))),
        |(span)| {
            let s: String = str::from_utf8(span.as_bytes()).unwrap().to_string();
            Identifier::new(s, 0)
        }

    ))(input)
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
        match parse_identifier(Input::new_extra(input,ctx)) {
            Ok((remaining, parsed)) => {
                println!("Parsed identifier: {:?}, Remaining: {}", parsed, remaining);
            }
            Err(err) => {
                println!("Failed to parse '{}': {:?}", input, err);
            }
        }
    }
}
