// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while1},
//     character::complete::{char, multispace0},
//     combinator::{opt, recognize},
//     sequence::{pair, preceded, tuple},
//     IResult,
// };
// use nom::bytes::complete::take_while;
// use nom::character::complete::not_line_ending;
// use nom::combinator::map;
// use nom::sequence::terminated;
// use crate::ast::node::{DeclarationMode, Stmt, Type, Variable};
// use crate::ast::node::Expr::Na;
// use crate::input::{Input, PineResult};
// use crate::parsing::parse_declaration_mode::parse_declaration_mode;
// use crate::parsing::parse_express::parse_expr;
// use crate::parsing::parse_identifier::parse_identifier;
// use crate::parsing::parse_type::parse_type;
// use crate::parsing::parse_variable::parse_variable_declaration;
//
// // pub fn parse_assign(input: Parser) -> Res<Parser, Assign> {
// //     map(
// //         tuple((
// //             opt(terminated(tag("let"), space1)),
// //             terminated(parse_assign_left_side, space0),
// //             terminated(tag("="), space0),
// //             terminated(parse_expression, space0),
// //         )),
// //         |(opt_let, var, _, expr)| Assign::new(var, expr, opt_let.is_some()),
// //     )(input)
// // }
//
// pub fn parse_assignment(input: Input) -> PineResult<Variable> {
//     map(
//         tuple((
//             parse_variable_declaration,
//
//             preceded(multispace0, char('=')),
//             preceded(multispace0, not_line_ending),
//         )),
//         |(variable, _, expr)| Variable {
//             declaration_mode: variable.declaration_mode,
//             identifier: variable.identifier,
//             var_type: variable.var_type,
//             //identifier: identifier.to_string(),
//
//             value: parse_expr(expr).unwrap().1, // 使用 parse_expr 解析表达式
//         })(input)
//
//     //
//     //  let (input,  variable) = parse_variable_declaration(input)?;
//     //  let (input, _) = preceded(multispace0, char('='))(input)?;  // 解析等号
//     //
//     //  //let (input, expr) = preceded(multispace0, take_while1(|c| c != '\n'))(input)?;  // 解析表达式
//     //  let (input, expr) = preceded(multispace0, not_line_ending)(input)?;  // 解析表达式
//     //
//     //  // Ok((input, (mode, var_type, identifier, expr)))
//     // // variable.value=parse_expr(expr).unwrap().1;
//     //  let variable1 = Stmt::VariableDeclaration  {
//     //      declaration_mode: variable.declaration_mode,
//     //      //identifier:variable.identifier,
//     //      var_type:variable.var_type,
//     //      //identifier: identifier.to_string(),
//     //      value: parse_expr(expr).unwrap().1 // 使用 parse_expr 解析表达式
//     //  };
//     //  Ok((input, variable1))
// }
//
