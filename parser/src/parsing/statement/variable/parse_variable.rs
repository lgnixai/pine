// use nom::branch::alt;
// use nom::bytes::complete::{tag, take_until};
// use nom::character::complete::{char, multispace0, not_line_ending};
// use nom::combinator::{map, opt};
// use nom::IResult;
// use nom::sequence::{preceded, tuple};
//
// use crate::input::{Input, PineResult};
// use crate::lexer::ast::Expression::Na;
//
// use crate::parsing::parse_identifier::parse_identifier;
// use crate::parsing::statement::variable::parse_declaration_mode::parse_declaration_mode;
// use crate::parsing::statement::variable::parse_type::parse_type;
//
//
// pub fn parse_variable_declaration(input: Input) -> PineResult<Variable> {
//     map(
//         tuple((
//             opt(preceded(
//                 multispace0,
//                 parse_declaration_mode,
//                 //alt((parse_declaration_mode, map(tag(""), |_| None))),
//             )),
//             opt(preceded(multispace0, parse_type)),
//             preceded(multispace0, parse_identifier),
//         )),
//         |(declaration_mode, var_type, identifier)| Variable {
//             declaration_mode,
//             var_type,
//             identifier,
//             value: Na,
//         })(input)
// }
//
//
// // 解析赋值表达式
