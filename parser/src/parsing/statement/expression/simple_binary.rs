use super::parse_expression;
use crate::{
    lexer::ast::{BinaryExpression, Expression},
    tags::{not_tag},
};

use nom::{combinator::map, sequence::pair};

use crate::input::{Input, PineResult, Positioned, positioned};
use crate::lexer::token::Operator;

pub fn parse_simple_binary_expression(input: Input) -> PineResult<Positioned<Expression>> {
    positioned(map(pair(not_tag, parse_expression), |(operator, left)| {
        Expression::BinaryExpression(Box::new(operator.span.between(left.span).wrap(
            BinaryExpression {
                operator: operator.span.wrap(Operator::Not),
                right: left.span.wrap(Expression::Null),
                left,
            },
        )))
    }))(input)
}
