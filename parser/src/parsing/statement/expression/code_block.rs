use crate::{ parsing::parse_code_block};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, terminated};
use nom_locate::position;

use crate::input::{Input, PineResult, Positioned};
use crate::lexer::ast::{Block, Expression};
use crate::parsing::parse_code_block::parse_code_block;
use crate::parsing::statement::parse_statement;
use crate::tags::{brace_close_tag, brace_open_tag, semi_tag};



pub fn parse_code_block_expression(input: Input) -> PineResult<Positioned<Expression>> {
    // map(parse_code_block, |block| {
    //     block.span.wrap(Expression::Block(block))
    // })(input)

    map(
        parse_code_block,|block|{
            block.span.wrap(Expression::Block(block))
        }
    )(input)


}
