mod parse_type;
mod parse_variable;
mod parse_declaration_mode;

use std::arch::aarch64::vld1_f64_x4;
use std::path::PathBuf;
use super::{expression::parse_expression};
use crate::{

    tags::{colon_tag, comma_tag, const_tag, eq_tag, let_tag, question_tag, semi_tag},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, not_line_ending};
use crate::ast::node::variable::{VariableDeclaration, VariableStatement};
use crate::ast::types::types::DeclarationMode;


use crate::input::{Input, new_input, PineResult, Positioned, positioned, spaced};
use crate::inputctx::ParserContext;
use crate::lexer::token::Token;
use crate::lexer::token::Token::ReservedWord;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::statement::variable::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::types::parse_type;
use crate::tags::{var_tag, varip_tag};

/**
[<declaration_mode>] [<type>] <identifier> = <expression> | <structure>
 */

pub fn parse_variable_statement(input: Input) -> PineResult<Positioned<VariableStatement>> {
    positioned(map(
        tuple((
            opt(preceded(
                multispace0,
                parse_declaration_mode,
                 //alt(( const_tag,var_tag,varip_tag)),
                //alt((parse_declaration_mode, map(tag(""), |_| None))),
            )),
            opt(preceded(multispace0, parse_type)),
            preceded(multispace0, parse_identifier),
            opt(preceded(spaced(tag("=")), parse_expression)),


            // preceded(multispace0, char('=')),
            // preceded(multispace0, not_line_ending),
        )),
        |(declaration_mode, var_type, identifier, init)| {
            let dm=declaration_mode.clone();
            let declarations = VariableDeclaration {
                declaration_mode: declaration_mode,
                name: identifier,
                ty: var_type,
                //identifier: identifier.to_string(),
                initializer: init, // 使用 parse_expr 解析表达式
            };

            VariableStatement {
                mutable: match dm {
                    None => {
                        Positioned::new(true, Default::default())
                    }
                    Some(_) => {
                        let dm=dm.unwrap();
                        dm.wrap(dm.value == DeclarationMode::Const)

                    }
                }
               ,
                declarations:vec![Positioned::new(declarations, Default::default())],
            }
        }))(input)
}

// pub fn _parse_variable_statement(input: Input) -> PineResult<Positioned<VariableStatement>> {
//     positioned(map(
//         tuple((
//             alt((let_tag, const_tag)),
//             terminated(
//                 separated_list1(
//                     comma_tag,
//                     positioned(map(
//                         tuple((
//                             parse_identifier,
//                             positioned(opt(question_tag)),
//                             opt(preceded(colon_tag, parse_type)),
//                             //opt(preceded(eq_tag, parse_expression)),
//                         )),
//                         |(name, nullable, ty)| VariableDeclaration {
//                             name,
//                             ty,
//                             nullable: nullable.wrap(nullable.value.is_some()),
//                             initializer: None,
//                         },
//                     )),
//                 ),
//                 semi_tag,
//             ),
//         )),
//         |(kind, declarations)| VariableStatement {
//             mutable: kind.wrap(kind.value.fragment() == &"let"),
//             declarations,
//         },
//     ))(input)
// }

#[test]
fn main() {
    let script = "var int a= 3 + 4";

    // let  input = new_input(&script);
    //
    // let ret=parse_variable_statement(input);
    // println!("{:?}",ret);


    let mut path = PathBuf::new();
    let ctx = ParserContext::new(path);
    match parse_variable_statement(Input::new_extra(script, ctx)) {
        Ok((remaining, parsed)) => {
            println!("Parsed identifier: {:?}, Remaining: {}", parsed, remaining);
        }
        Err(err) => {
            println!("Failed to parse '{}': {:?}", script, err);
        }
    }
}
