use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::{delimited, terminated};

use crate::input::{Input, PineResult, position};
use crate::lexer::ast::Block;
use crate::parsing::statement::parse_statement;
use crate::tags::{brace_close_tag, brace_open_tag, semi_tag};

pub fn parse_code_block(input: Input) -> PineResult<Block> {
    let (input, start) = position(input)?;
    let (input, value) = delimited(
        brace_open_tag,
        many0(terminated(parse_statement, opt(semi_tag))),
        brace_close_tag,
    )(input)?;
    let (input, end) = position(input)?;

    Ok((input, start.between(end).wrap(value)))
}
