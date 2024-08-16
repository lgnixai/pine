use super::{

    statement::{parse_parameter, parse_property_name, parse_type_parameter},
    types::parse_type,
};

use crate::{
    lexer::ast::{CallSignature, ConstructSignature, IndexSignature, MethodSignature, PropertySignature},
    tags::{
        bracket_close_tag, bracket_open_tag, colon_tag, comma_tag, gt_tag, lt_tag, new_tag,
        paren_close_tag, paren_open_tag, question_tag,
    },
};

use nom::{
    combinator::{map, opt},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, tuple},
};
use crate::input::{Input, PineResult, Positioned, positioned};
use crate::parsing::parse_identifier::parse_identifier;


pub fn parse_call_signature(input: Input) -> PineResult<Positioned<CallSignature>> {
    positioned(map(
        tuple((
            opt(delimited(
                lt_tag,
                separated_list1(comma_tag, parse_type_parameter),
                gt_tag,
            )),
            delimited(
                paren_open_tag,
                separated_list0(comma_tag, parse_parameter),
                paren_close_tag,
            ),
            opt (preceded(colon_tag, parse_type)),
        )),
        |(type_parameters, parameters, ty)| {

            // let ty = ty.map(|ty| {
            //     // Perform the necessary transformation on ty here
            //     // For example, if you need to wrap it in a specific type or modify it
            //     // Replace the following line with the actual transformation logic
            //     ty
            // });
            CallSignature(type_parameters.unwrap_or_default(), parameters, ty)
        },
    ))(input)
}

pub fn parse_construct_signature(input: Input) -> PineResult<Positioned<ConstructSignature>> {
    positioned(map(preceded(new_tag, parse_call_signature), |signature| {
        ConstructSignature(signature.value.0, signature.value.1, signature.value.2)
    }))(input)
}

pub fn parse_index_signature(input: Input) -> PineResult<Positioned<IndexSignature>> {
    positioned(map(
        tuple((
            delimited(
                bracket_open_tag,
                pair(parse_identifier, preceded(colon_tag, parse_type)),
                bracket_close_tag,
            ),
            preceded(colon_tag, parse_type),
        )),
        |((name, index_type), ty)| IndexSignature(name, index_type, ty),
    ))(input)
}

pub fn parse_method_signature(input: Input) -> PineResult<Positioned<MethodSignature>> {
    positioned(map(
        tuple((
            parse_property_name,
            positioned(opt(question_tag)),
            parse_call_signature,
        )),
        |(name, optional, signature)| {
            MethodSignature(
                name,
                optional.wrap(optional.value.is_some()),
                Box::new(signature),
            )
        },
    ))(input)
}

pub fn parse_property_signature(input: Input) -> PineResult<Positioned<PropertySignature>> {
    positioned(map(
        tuple((
            parse_identifier,
            positioned(opt(question_tag)),
            colon_tag,
            parse_type,
        )),
        |(name, nullable, _, ty)| PropertySignature {
            modifiers: Default::default(),
            name,
            nullable: nullable.wrap(nullable.value.is_some()),
            ty,
        },
    ))(input)
}
