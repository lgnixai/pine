use super::{expression::parse_expression};
use crate::{
    lexer::ast::{VariableDeclaration, VariableStatement},
    tags::{colon_tag, comma_tag, const_tag, eq_tag, let_tag, question_tag, semi_tag},
};

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
};


use crate::input::{Input, PineResult, Positioned,positioned};
use crate::lexer::token::Token;
use crate::lexer::token::Token::ReservedWord;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::types::parse_type;

pub fn parse_variable_statement(input: Input) -> PineResult<Positioned<VariableStatement>> {
    positioned(map(
        tuple((
            alt((let_tag, const_tag)),
            terminated(
                separated_list1(
                    comma_tag,
                    positioned(map(
                        tuple((
                            parse_identifier,
                            positioned(opt(question_tag)),
                            opt(preceded(colon_tag, parse_type)),
                            opt(preceded(eq_tag, parse_expression)),
                        )),
                        |(name, nullable, ty, init)| VariableDeclaration {
                            name,
                            ty,
                            nullable: nullable.wrap(nullable.value.is_some()),
                            initializer: init,
                        },
                    )),
                ),
                semi_tag,
            ),
        )),
        |(kind, declarations)| VariableStatement {
            mutable: kind.wrap(kind.value.fragment() == &"let"),
            declarations,
        },
    ))(input)
}
