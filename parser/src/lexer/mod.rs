use nom::{
    branch::alt,
    bytes::complete::{is_not, take},
    character::complete::multispace0,
    combinator::map,
    multi::many0,
    sequence::{delimited, pair},
};
use crate::input::{Input, PineResult, Positioned};
use crate::lexer::token::Token;

pub mod ast;
pub mod identifier;
pub mod binary_operator;
pub mod token;
pub mod module;
pub mod delimiters;
pub mod operators;
pub mod punctuation;
pub mod util;



#[macro_export]
macro_rules! syntax {
    ($func_name:ident: $tag_string:literal => $output_token:expr) => {
        pub fn $func_name(input: Input) -> PineResult<Positioned<Token>> {
            let (input, position) = tag($tag_string)(input)?;

            Ok((input, Positioned::new($output_token, position.into())))
        }
    };

    ($($func_name:ident: $tag_string:literal => $output_token:expr);*;) => {
        use nom::bytes::complete::tag;

        $(
            pub fn $func_name(input: Input) -> PineResult<Positioned<Token>> {
                let (input, position) = tag($tag_string)(input)?;

                Ok((input, Positioned::new($output_token, position.into())))
            }
        )*
    };
}