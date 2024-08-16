use super::{expression::parse_expression};
use crate::{
    tags::{brace_close_tag, brace_open_tag, comma_tag, enum_tag, eq_tag},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, terminated, tuple},
};
use nom::bytes::complete::tag;


use crate::input::{Input, PineResult, Positioned,positioned};
use crate::lexer::ast::{EnumDeclaration, EnumMember};
use crate::parsing::parse_identifier::parse_identifier;


// pub fn parse_enum_declaration<'a>(&'a self, input: Input<'a>) -> PineResult<Positioned<EnumDeclaration>> {
//     positioned(map(
//         tuple((
//             positioned( tag("enum")),
//             parse_identifier,
//         )),
//         |(_, name)| EnumDeclaration { name },
//     ))(input)
// }
/*
enum Direction {
    Up = "UP",
    Down = "DOWN",
    Left = "LEFT",
    Right ,
}

//@enum           An enumeration of named values representing buy, sell, and neutral signal states.
//@field buy      Represents a "Buy signal" state.
//@field sell     Represents a "Sell signal" state.
//@field neutral  Represents a "neutral" state.
enum Signal
    buy     = "Buy signal"
    sell    = "Sell signal"
    neutral


*/
pub fn parse_enum_declaration(input: Input) -> PineResult<Positioned<EnumDeclaration>> {
    positioned(map(
        tuple((
            enum_tag,
            parse_identifier,
            delimited(
                brace_open_tag,
                terminated(
                    separated_list0(
                        comma_tag,
                        positioned(alt((
                            map(
                                tuple((parse_identifier, eq_tag, parse_expression)),
                                |(name, _, init)| EnumMember {
                                    name,
                                    initializer: Some(init),
                                },
                            ),
                            map(parse_identifier, |name| EnumMember {
                                name,
                                initializer: None,
                            }),
                        ))),
                    ),
                    opt(comma_tag),
                ),
                brace_close_tag,
            ),
        )),
        |(_, name, members)| EnumDeclaration { name, members },
    ))(input)
}
