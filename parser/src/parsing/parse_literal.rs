use std::str::Utf8Error;
use nom::{
    branch::alt,
    bytes::complete::take_while1,
    character::complete::{alpha1, alphanumeric1, char},
    combinator::{recognize},
    sequence::{pair, preceded},
    IResult,
};
use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::character::complete::{digit1, i64};
use nom::combinator::{map, map_res, opt};
use nom::number::complete::{double, float};
use nom::sequence::{delimited, tuple};



use crate::input::{Input, PineResult, Positioned, position,positioned, Span};
use crate::lexer::ast::Literal;
use crate::lexer::token::Token;

// fn parse_float(input: &str) -> IResult<&str, &str> {
//     let re = Regex::new(r"^[+-]?(\d+(\.\d*)?|\.\d+)([eE][+-]?\d+)?$").unwrap();
//     if re.is_match(input) {
//         let pos = input.find(|c: char| !c.is_digit(10) && c != '.' && c != '+' && c != '-' && c != 'e' && c != 'E');
//         if let Some(pos) = pos {
//             Ok((&input[pos..], &input[..pos]))
//         } else {
//             Ok(("", input))
//         }
//     } else {
//         Err(nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Float)))
//     }
// }
// 判断字符是否是数字
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

// 解析整数
fn parse_int(input: Input) -> PineResult<Positioned<Literal>> {
    positioned(map_res(
        digit1,
        |s: Input| s.parse::<i32>().map(|i| Literal::Int(Positioned::new(i, Span::from(s)))),
    ))(input)
}

// 解析浮点数
fn parse_float(input: Input) -> PineResult<Positioned<Literal>> {
    positioned(map_res(
        take_while(|c: char| is_digit(c) || c == '.'),
        |s: Input| s.parse::<f64>().map(|i| Literal::Float(Positioned::new(i, Span::from(s)))),
    ))(input)
}


// fn parse_string(input: Input) -> Positioned<Positioned<Literal>> {
//     let (input, start) = position(input)?;
//
//     let (input, _) = char('"')(input)?;
//     let (input, content) = take_while(|c| c != '"')(input)?;
//
//     let (input, _) = char('"')(input)?;
//
//     let (input, end) = position(input)?;
//
//     let start: Span = start.into();
//     let end: Span = end.into();
//     let s=content.to_string();
//     let ret=Literal::String( Positioned::new(s,Span::from(input.clone())));
//     Ok((
//         input,
//         start
//             .between(end)
//             .wrap(Token::Literal(ret)),
//     ))
//
//     // Positioned::new(Literal::String( Positioned::new(s,Span::from(input))),Span::from(input)
//     //
// }

// 解析布尔值
fn parse_bool(input: Input) -> PineResult<Positioned<Literal>> {
    positioned(alt((
        map(tag("true"), |input| Literal::Boolean(Positioned::new(false, Span::from(input)))),
        map(tag("false"), |input| Literal::Boolean(Positioned::new(false, Span::from(input)))),
    )))(input)
}

//  fn parse_double(input: Input) -> PineResult<Positioned<Literal>> {
//     positioned(map(double,
//        (|i| Literal::Double(Positioned::new(i, Span::from(input)))),
//     ))
// }

// fn parse_integer(input: Input) -> PineResult<Positioned<Literal>> {
//     positioned(map(i64,
//        (|i| Literal::Double(Positioned::new(i, Span::from(input)))),
//     ))(input)
// }


// fn parse_integer(input: Input) -> PineResult<Literal> {
//     map(i64, Literal::Integer)(input)
// }

// fn parse_variable(input: Input) -> PineResult<Literal> {
//     map(alpha1, |var: Input| Literal::Variable(var.to_string()))(input)
// }

pub fn parse_literal(input: Input) -> PineResult<Positioned<Literal>> {
    alt((
        parse_bool,
        parse_float,
        parse_int,
        //parse_variable,


        //map_res(parse_float, |s: &str| s.parse::<f64>().map(Literal::Float)),

        // map_res(digit1, |s: &str| s.parse::<i32>().map(Literal::Int)),
        //
        // map(delimited(char('"'), take_until("\""), char('"')), |s: &str| {
        //     Literal::String(s.to_string())
        // }),
        //parse_string
    ))(input)
}