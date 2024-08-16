use super::{parse_expression, primitives::parse_ident_expression};
use crate::{
    lexer::ast::{Expression, IndexExpression},
    tags::{bracket_close_tag, bracket_open_tag, dot_tag},
};

use nom::{
    branch::alt,
    combinator::map,
    sequence::{delimited, preceded},
};
use crate::input::{Input, PineResult, Positioned};


pub fn parse_index_expression(
    input: Input,
    target: Positioned<Expression>,
) -> PineResult<Positioned<Expression>> {
    map(
        alt((
            delimited(bracket_open_tag, parse_expression, bracket_close_tag),
            preceded(dot_tag, parse_ident_expression),
        )),
        |index| {
            let distance = target.between(&index);

            distance.wrap(Expression::IndexExpression(Box::new(distance.wrap(
                IndexExpression {
                    target: target.clone(),
                    index,
                },
            ))))
        },
    )(input)
}
