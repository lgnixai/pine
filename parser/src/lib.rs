use nom::combinator::{eof, map};
use nom::{IResult};
use nom::Parser as _;
use nom::multi::many0;
use nom::sequence::tuple;
use crate::input::{Input, PineResult, position, Positioned, positioned, Span};
use crate::lexer::ast::{Block, Statement};
use crate::parsing::statement::parse_program_statement;

pub mod input;
pub mod tags;
mod err;
mod inputctx;
mod ast;
pub mod lexer;

pub mod parsing;
pub mod error;

pub struct Parser;

impl Parser {


    pub fn parse_ast<'a>(input: Input<'a>) -> PineResult<Block>{



        map(
            tuple((position, many0(parse_program_statement), eof)),
            |(start, program, end)| start.between(Span::from(end)).wrap(program),
        ).parse(input)

    }
}

