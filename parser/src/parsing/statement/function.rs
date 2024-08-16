

use nom::{
    combinator::{map, opt, value},
    sequence::{preceded, tuple},
};



use crate::input::{Input, PineResult, Positioned, positioned};
use crate::lexer::ast::FunctionDeclaration;
use crate::lexer::token::Modifier;
use crate::parsing::parse_code_block::parse_code_block;
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::signatures::parse_call_signature;
use crate::tags::{async_tag, function_tag};

pub fn parse_function_declaration(input: Input) ->PineResult<Positioned<FunctionDeclaration>> {
    positioned(map(
        tuple((
            opt(positioned(value(Modifier::Async, async_tag))),
            preceded(function_tag, parse_identifier),
            parse_call_signature,
            opt(parse_code_block),
        )),
        |(async_modifier, name, signature, body)| FunctionDeclaration {
            name,
            type_parameters: signature.value.0,
            parameters: signature.value.1,
            ty: signature.value.2,
            body,
            modifiers: async_modifier
                .map(|modifier| vec![modifier])
                .unwrap_or_default(),
        },
    ))(input)
}
