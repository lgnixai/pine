use super::parse_expression;
use crate::{

    tags::{comma_tag, paren_close_tag, paren_open_tag},
};

use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair},
};
use crate::input::{Input, PineResult, Positioned,positioned};
use crate::lexer::ast::{Expression, FunctionCallExpression};
use crate::parsing::parse_code_block::parse_code_block;


pub fn parse_call_expression(
    input: Input,
    fn_handle: Positioned<Expression>,
) -> PineResult<Positioned<Expression>> {
    map(
        positioned(pair(
            delimited(
                paren_open_tag,
                separated_list0(comma_tag, parse_expression),
                paren_close_tag,
            ),
            opt(parse_code_block),
        )),
        |Positioned {
             value: (arguments, lambda),
             span,
         }| {
            span.wrap(Expression::FunctionCallExpression(Box::new(span.wrap(
                FunctionCallExpression {
                    function: Box::new(fn_handle.clone()),
                    arguments,
                    lambda,
                },
            ))))
        },
    )(input)
}
