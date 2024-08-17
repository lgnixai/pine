use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{opt, recognize},
    sequence::{pair, preceded, tuple},
    IResult,
};

use nom::combinator::map;
use nom::sequence::terminated;
use crate::ast::types::types::Type;
use crate::input::{Input, PineResult, Positioned, positioned};


pub fn parse_type(input: Input) -> PineResult< Positioned<Type>> {
   positioned( alt((

        map(tag("int"), |_| Type::Int),
        map(tag("float"), |_| Type::Float),
        map(tag("bool"), |_| Type::Bool),
        map(tag("color"), |_| Type::Color),
        map(tag("string"), |_| Type::String),
        map(tag("line"), |_| Type::Line),
        map(tag("linefill"), |_| Type::LineFill),
        map(tag("label"), |_| Type::Label),
        map(tag("box"), |_| Type::Box),
        map(tag("table"), |_| Type::Table),
        map(tag("UDF"), |_| Type::UDF),
        // 添加对 array<int> 等复杂类型的支持
        map(preceded(tag("array<"), terminated(parse_type, tag(">"))), |t| {
            Type::Array(Box::new(t.value))
        }),
        map(preceded(tag("matrix<"), terminated(parse_type, tag(">"))), |t| {
            Type::Matrix(Box::new(t.value))
        }),

    )))(input)
}