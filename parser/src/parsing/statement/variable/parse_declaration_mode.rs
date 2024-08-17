use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{opt, recognize},
    sequence::{pair, preceded, tuple},
    IResult,
};
use nom::bytes::complete::take_while;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::sequence::terminated;
use crate::ast::types::types::DeclarationMode;
use crate::input::{Input, PineResult, Positioned, positioned};
use crate::parsing::parse_identifier::parse_identifier;

pub fn parse_declaration_mode(input: Input) -> PineResult<Positioned< DeclarationMode>> {
   positioned( alt((
        map(tag("varip"), |_| DeclarationMode::Varip),
        map(tag("const"), |_| DeclarationMode::Const),

        map(tag("var"), |_| DeclarationMode::Var),
    )))(input)
}