use std::cell::Cell;
use nom::{IResult, InputLength, Offset};
use nom_locate::{LocatedSpan};

use std::path::PathBuf;
use nom::bytes::complete::take;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::map;
use nom::error::{Error, ErrorKind, VerboseError};
use nom::sequence::tuple;
use nom_greedyerror::GreedyError;
use crate::inputctx::ParserContext;
use crate::lexer::ast::Literal;
use std::{cmp::Ordering, fmt::Display};
use std::fmt::Debug;


#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: usize,
}

impl Span {
    pub fn new(start: Input, end: Input) -> Span {
        let len = end.location_offset() - start.location_offset();

        // trim span
        let mut fragment = &start.fragment()[0..len];
        let trim_start = fragment.chars().take_while(|c| c.is_whitespace()).count();
        let trim_end = fragment
            .chars()
            .rev()
            .take_while(|c| c.is_whitespace())
            .count();

        fragment = &fragment[trim_start..(len - trim_end)];

        Span {
            start: start.location_line() as usize,
            end: end.location_line() as usize,
            line: start.location_line(),
            column: start.get_column(),
            // offset: start.get_utf8_column() + trim_start,
            // fragment,
            // source: start.extra.source,
        }
    }

    pub fn empty() -> Span {
        Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }
    pub fn between(&self, to: Span) -> Span {
        Span {
            start: self.start,
            end: to.end,
            line: self.line,
            column: self.column,
        }
    }

    pub fn wrap<A>(self, value: A) -> Positioned<A> {
        Positioned { value, span: self }
    }
}

impl From<BytesSpan<'_>> for Span {
    fn from(value: BytesSpan) -> Self {
        Span {
            start: value.location_offset(),
            end: value.location_offset() + value.input_len(),
            line: value.location_line(),
            column: value.naive_get_utf8_column(),
        }
    }
}


#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Positioned<T> {
    pub value: T,
    pub span: Span,

}

impl<T> Positioned<T> {
    pub fn new(value: T, span: Span) -> Positioned<T> {
        Positioned { value, span }
    }

    pub fn between<U>(&self, value: &Positioned<U>) -> Span {
        self.span.between(value.span)
    }

    pub fn wrap<U>(&self, value: U) -> Positioned<U> {
        self.span.wrap(value)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Positioned<U> {
        self.span.wrap(f(self.value))
    }

    pub fn unpack(self) -> (Span, T) {
        (self.span, self.value)
    }
}

impl<T: Debug> Debug for Positioned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.value)
    }
}

pub type BytesSpan<'a> = LocatedSpan<&'a [u8]>;
//pub type Input<'a> = LocatedSpan<&'a str>;
pub type Input<'a> = LocatedSpan<&'a str, ParserContext>;

impl From<Input<'_>> for Span {
    fn from(value: Input) -> Self {
        Span {
            start: value.location_offset(),
            end: value.location_offset() + value.input_len(),
            line: value.location_line(),
            column: value.naive_get_utf8_column(),
        }
    }
}


// pub fn new_input(input: &str) -> Input<'_> {
//     Input::new(input)
// }

pub type ByteResult<'a, T> = IResult<BytesSpan<'a>, T>;

pub trait Parser<'a, O>: nom::Parser<Input<'a>, O, GreedyError<Input<'a>, ErrorKind>> {}

impl<'a, O, P: nom::Parser<Input<'a>, O, GreedyError<Input<'a>, ErrorKind>>> Parser<'a, O> for P {}

pub type PineResult<'a, T> = IResult<Input<'a>, T,NomError<'a>>;
//type IResult<'a, T> = nom::IResult<Input<'a>, T, NomError<'a>>;


// pub fn parse_ast(input: &str) -> Result<AST<'_>, ParserError<'_>> {
//     match module::module(new_input(input)) {
//         Ok(module) => Ok(AST::from(module)),
//         Err(err) => Err(err),
//     }
// }
pub type NomError<'a> = VerboseError<Input<'a>>;

#[derive(Debug)]
pub enum ParserError<'a> {
    NomErr(VerboseError<Input<'a>>),
    EarlyTermination(),
}

impl<'a> From<VerboseError<Input<'a>>> for ParserError<'a> {
    fn from(value: VerboseError<Input<'a>>) -> Self {
        ParserError::NomErr(value)
    }
}

impl<'a> Display for ParserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::NomErr(err) => std::fmt::Display::fmt(err, f),
            ParserError::EarlyTermination() => write!(f, "early termination"),
        }
    }
}


pub fn ws(i: Input<'_>) -> PineResult<Input<'_>> {
    return multispace1(i);
}

pub fn new_input(input: &str) -> Input<'_> {
    let mut path = PathBuf::new();
    let ctx = ParserContext::new(path);
    Input::new_extra(input, ctx)
}

pub fn spaced<'a, F, O>(mut parser: F) -> impl FnMut(Input<'a>) -> PineResult<O>
    where
        F: FnMut(Input<'a>) -> PineResult<O>,
// I: InputTakeAtPosition,
// <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    return move |i: Input<'a>| {
        let (i, _) = multispace0(i)?;
        let (i, res) = parser(i)?;
        let (i, _) = multispace0(i)?;

        return Ok((i, res));
    };
}


pub fn span<'a, F, O>(mut parser: F) -> impl FnMut(Input<'a>) -> PineResult<Span>
    where
        F: FnMut(Input<'a>) -> PineResult<O>,
{
    return move |i: Input<'a>| {
        let (start_i, _) = nom_locate::position(i)?;
        let (parsed_i, out) = parser(start_i.clone())?;
        let (i, end) = nom_locate::position(parsed_i.clone())?;

        Ok((i, Span::new(start_i, end)))
    };
}

// pub fn position<'a, F, O>(mut parser: F) -> impl FnMut(Input<'a>) -> PineResult<(Span, O)>
//     where
//         F: FnMut(Input<'a>) -> PineResult<O>,
// {
//     return move |i: Input<'a>| {
//         let (start_i, _) = nom_locate::position(i)?;
//         let (parsed_i, out) = parser(start_i.clone())?;
//         let (i, end) = nom_locate::position(parsed_i.clone())?;
//
//         Ok((i, (Span::new(start_i, end), out)))
//     };
// }


// pub fn position(input: Input) -> PineResult<Span> {
//     let (_, pos) = take(1usize)(input.clone())?;
//     let a = input.clone();
//
//     Ok((
//         a,
//         Span::from(pos)
//     ))
// }
pub fn position(input: Input) -> PineResult<Span> {
    // 克隆 input 以保留原始值
    let input_clone = input.clone();

    let (start_i, _) = nom_locate::position(input_clone)?;  // 使用克隆的 input

    let new_span = Span {
        start: 0,
        end: start_i.location_offset(),
        line: start_i.location_line(),
        column: start_i.get_column(),
    };

    // 返回原始的 input
    Ok((
        input,  // 返回原始 input 而非克隆的
        new_span,
    ))
}



pub fn positioned<'a, F, O1>(parser: F) -> impl FnMut(Input<'a>) -> PineResult<'a, Positioned<O1>>
    where
        F: nom::Parser<Input<'a>, O1, VerboseError<Input<'a>>>,
{
    map(
        tuple((position, parser, position)),
        |(start, result, end)| start.between(end).wrap(result),
    )
}


// pub fn spaned<'a, F, O1>(parser: F) -> impl FnMut(Input<'a>) -> PineResult<'a, Positioned<O1>>
//     where
//         F: nom::Parser<Input<'a>, O1, Error<Input<'a>>>,
// {
//     map(
//         tuple((span, parser, span)),
//         |(start, result, end)| start.between(end).wrap(result),
//     )
// }
