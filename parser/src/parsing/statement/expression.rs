mod array;
mod call;
mod index;
mod primitives;
mod simple_binary;
mod code_block;
mod arrow_function;


use self::{
    array::parse_array,
    arrow_function::parse_arrow_function_expression,
    call::parse_call_expression,
    code_block::parse_code_block_expression,
    index::parse_index_expression,
    primitives::{parse_ident_expression, parse_literal_expression, parse_null, parse_this},
    simple_binary::parse_simple_binary_expression,
};


use nom::{
    branch::alt,
    bytes::complete::take,
    error::{Error, ErrorKind},
    Err,
};
use nom::error::{ParseError, VerboseError};
use crate::error::NomError;

use crate::input::{Input, PineResult, Positioned, spaced};
use crate::lexer::ast::{BinaryExpression, Expression, Infix, Precedence};
use crate::lexer::delimiters::lex_delimiter;
use crate::lexer::operators::lex_operator;
use crate::lexer::punctuation::lex_punctuation;
use crate::lexer::token::Token;
use crate::lexer::token::{Operator,Delimiter, Punctuation};

pub type PrecedencedOperation = (Precedence, Option<Positioned<Infix>>);

pub trait FromRef<T> {
    fn from_ref(from: &T) -> Self;
}

impl FromRef<Positioned<Token>> for PrecedencedOperation {
    fn from_ref(token: &Positioned<Token>) -> Self {
        match &token.value {
            Token::Operator(operator) => match operator {

                Operator::Eq => (Precedence::PEquals, Some(token.wrap(Infix::Assign))),
                Operator::EqEq => (Precedence::PEquals, Some(token.wrap(Infix::Equal))),
                Operator::Ne => (Precedence::PEquals, Some(token.wrap(Infix::NotEqual))),
                Operator::Le => (
                    Precedence::PLessGreater,
                    Some(token.wrap(Infix::LessThanEqual)),
                ),
                Operator::Ge => (
                    Precedence::PLessGreater,
                    Some(token.wrap(Infix::GreaterThanEqual)),
                ),
                Operator::Lt => (Precedence::PLessGreater, Some(token.wrap(Infix::LessThan))),
                Operator::Gt => (
                    Precedence::PLessGreater,
                    Some(token.wrap(Infix::GreaterThan)),
                ),
                Operator::Plus => (Precedence::PSum, Some(token.wrap(Infix::Plus))),
                Operator::Minus => (Precedence::PSum, Some(token.wrap(Infix::Minus))),
                Operator::Star => (Precedence::PProduct, Some(token.wrap(Infix::Multiply))),
                Operator::Slash => (Precedence::PProduct, Some(token.wrap(Infix::Divide))),
                Operator::PlusPlus => (Precedence::PProduct, Some(token.wrap(Infix::Increment))),
                Operator::MinusMinus => (Precedence::PProduct, Some(token.wrap(Infix::Decrement))),
                Operator::Not => (Precedence::PProduct, Some(token.wrap(Infix::Inverse))),
                _ => (Precedence::PLowest, None),
            },
            Token::Delimiter(Delimiter::ParenOpen) => (Precedence::PCall, None),
            Token::Delimiter(Delimiter::BracketOpen) => (Precedence::PIndex, None),
            Token::Punctuation(Punctuation::Dot) => (Precedence::PIndex, None),
            _ => (Precedence::PLowest, None),
        }
    }
}



pub fn parse_binary_operation_expression(
    input: Input,
    left: Positioned<Expression>,
) -> PineResult<Positioned<Expression>> {

    match spaced(lex_operator)(input.clone()) {
        Ok(tokens) => {
           // print!("b1111");

            let (input, token) = (tokens.0,tokens.1);

            //print!("b1111{:?},{:?},{:?}",token,left,input);
            let (precedence, maybe_op) = PrecedencedOperation::from_ref(&token);
            //let error = NomError::from_error_kind(input, ErrorKind::Tag);
            //print!("b1111{:?},{:?},{:?},{:?}",token,left,input,maybe_op);

            // 返回一个自定义错误
            //Err(nom::Err::Error(error))
            match maybe_op {
                None => Err(Err::Error(NomError::from_error_kind(input, ErrorKind::Tag))),
                Some(operation) => {
                    let operator = operation.wrap(match &operation.value {
                        Infix::Increment => Operator::PlusPlus,
                        Infix::Decrement => Operator::MinusMinus,
                        Infix::Plus => Operator::Plus,
                        Infix::Minus => Operator::Minus,
                        Infix::Divide => Operator::Slash,
                        Infix::Multiply => Operator::Star,
                        Infix::Assign => Operator::Eq,
                        Infix::Inverse => Operator::Not,
                        Infix::Equal => Operator::EqEq,
                        Infix::NotEqual => Operator::Ne,
                        Infix::GreaterThanEqual => Operator::Gt,
                        Infix::LessThanEqual => Operator::Lt,
                        Infix::GreaterThan => Operator::Ge,
                        Infix::LessThan => Operator::Le,
                    });
                    //println!("A1111{:?},{:?}",operation,operation.value);
                    if matches!(
                    operation.value,
                    Infix::Increment | Infix::Decrement | Infix::Inverse
                ) {

                        println!("A2222");
                        let distance = if matches!(operation.value, Infix::Increment | Infix::Decrement)
                        {
                            left.between(&operation)
                        } else {
                            operation.between(&left)
                        };
                        println!("A3333");
                        Ok((
                            input,
                            distance.wrap(Expression::BinaryExpression(Box::new(distance.wrap(
                                BinaryExpression {
                                    operator,
                                    right: left.wrap(Expression::Null),
                                    left,
                                },
                            )))),
                        ))
                    } else {
                        //println!("A4444{:?}",input);
                        let (input, right) = parse_pratt_expr(input, precedence)?;
                        let distance = left.between(&right);

                        Ok((
                            input,
                            distance.wrap(Expression::BinaryExpression(Box::new(distance.wrap(
                                BinaryExpression {
                                    operator,
                                    left,
                                    right,
                                },
                            )))),
                        ))
                    }
                }
            }
        }
        Err(_) => {

            print!("b00000");
            //Err(Err::Error(Error::new(input, ErrorKind::Tag)))
            Err(Err::Error(NomError::from_error_kind(input.clone(), ErrorKind::Tag)))
        }
    }

}

pub fn parse_pratt_expr(
    input: Input,
    precedence: Precedence,
) -> PineResult<Positioned<Expression>> {
    let (input, left) = parse_atom(input)?;
    //println!("token22,{:?}",left);
    go_parse_pratt_expr(input, precedence, left)
}

pub fn go_parse_pratt_expr(
    input: Input,
    precedence: Precedence,
    left: Positioned<Expression>,
) -> PineResult<Positioned<Expression>> {


    match spaced(lex_operator)(input.clone()) {
        Ok(tokens) => {
           // println!("4444");
            let (_, token) = spaced(lex_operator)(input.clone())?;

            //println!("token33,{:?},,,,{:?}",input,token);
            let p = PrecedencedOperation::from_ref(&token);
            //println!("token44,{:?}",p);

            match p {
                (Precedence::PCall, _) if precedence < Precedence::PCall => {
                    //print!("7777");
                    let (input, left) = parse_call_expression(input, left)?;

                    go_parse_pratt_expr(input, precedence, left)
                }
                (Precedence::PIndex, _) if precedence < Precedence::PIndex => {
                    //print!("8888");
                    let (input, left) = parse_index_expression(input, left)?;

                    go_parse_pratt_expr(input, precedence, left)
                }
                (ref peek_precedence, _) if precedence < *peek_precedence => {
                    //print!("999");
                    ///println!("999,input{:?},left{:?}",input,left);

                    let (input, left) = parse_binary_operation_expression(input, left)?;


                    go_parse_pratt_expr(input, precedence, left)
                }
                _ => {
                    ///print!("66666");
                    Ok((input, left))
                },
            }
        }
        Err(_) => {
            //println!("55555");
            Ok((input.clone(), left))
        }
    }
    // let token2 = lex_token(input.clone());
    //
    // println!("token11,{:?}",token2);
    // let (second_input, token) = lex_token(input.clone())?;
    // //let (second_input, token) = take(1usize)(input)?;
    //
    // // if token.is_empty() {
    // //     Ok((second_input, left))
    // // } else {
    //

}

pub fn parse_expression(input: Input) -> PineResult<Positioned<Expression>> {
    parse_pratt_expr(input, Precedence::PLowest)
}

pub fn parse_atom(input: Input) -> PineResult<Positioned<Expression>> {
    alt((
        //parse_new_expression,
        // parse_switch_expression,
        //parse_arrow_function_expression,
        parse_array,
        //parse_code_block_expression,
        parse_literal_expression,
        parse_this,
        parse_null,
        parse_simple_binary_expression,
        parse_ident_expression,
    ))(input)
}
