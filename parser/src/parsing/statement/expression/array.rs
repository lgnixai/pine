use super::parse_expression;
use nom::{
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{delimited, pair},
};
use crate::input::{Input, PineResult, Positioned,positioned};
use crate::lexer::ast::Expression;
use crate::tags::{bracket_close_tag, bracket_open_tag, comma_tag, ellipsis_tag};

pub fn parse_array(input: Input) -> PineResult<Positioned<Expression>> {
    positioned(map(
        delimited(
            bracket_open_tag,
            pair(
                separated_list0(comma_tag, parse_expression),
                positioned(opt(ellipsis_tag)),
            ),
            bracket_close_tag,
        ),
        |(elements, dynamic)| Expression::Array {
            elements,
            is_dynamic: dynamic.span.wrap(dynamic.value.is_some()),
        },
    ))(input)
}
