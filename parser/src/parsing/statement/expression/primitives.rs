use crate::{

    tags::{null_tag, this_tag},
};

use nom::combinator::map;

use crate::input::{Input, PineResult, Positioned, positioned};
use crate::lexer::ast::Expression;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_literal::parse_literal;

pub fn parse_literal_expression(input: Input) -> PineResult<Positioned<Expression>> {
    positioned(map(parse_literal, Expression::Literal))(input)
}

pub fn parse_ident_expression(input: Input) -> PineResult<Positioned<Expression>> {
    positioned(map(parse_identifier, Expression::Identifier))(input)
}

pub fn parse_this(input: Input) -> PineResult<Positioned<Expression>> {
    map(this_tag, |tag| tag.wrap(Expression::This))(input)
}

pub fn parse_null(input: Input) -> PineResult<Positioned<Expression>> {
    map(null_tag, |tag| tag.wrap(Expression::Null))(input)
}
