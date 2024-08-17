use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, char, digit1, multispace0, multispace1, none_of, one_of,
    },
    combinator::{
        all_consuming, cut, into, map, not, opt, peek, recognize, success, value, verify,
    },
    error::context,
    multi::{count, many0, many0_count, many1, separated_list1},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Parser,
};
use crate::error::NomError;
use crate::input::{Input, PineResult, spaced};
use crate::lexer::binary_operator::BinaryOperator;

const KEYWORDS: &[&str] = &[
    "as", "else", "export", "for", "foreign", "if", "in", "import", "type",
];
const OPERATOR_CHARACTERS: &str = "+-*/=<>&|!?";
const OPERATOR_MODIFIERS: &str = "=";



pub enum BracketType {
    Round,
    Square,
    Curly,
}

impl BracketType {
    pub fn open(&self) -> &'static str {
        match self {
            BracketType::Round => "(",
            BracketType::Square => "[",
            BracketType::Curly => "{",
        }
    }

    pub fn close(&self) -> &'static str {
        match self {
            BracketType::Round => ")",
            BracketType::Square => "]",
            BracketType::Curly => "}",
        }
    }
}

pub fn surround_brackets<'a, F, O>(
    brackets: BracketType,
    parser: F,
) -> impl FnMut(Input<'a>) -> PineResult< O>
    where
        F: FnMut(Input<'a>) -> PineResult< O>,
{
    delimited(
        spaced(tag(brackets.open())),
        parser,
        preceded(multispace0, tag(brackets.close())),
    )
}
pub fn sign(sign: &'static str) -> impl Fn(Input) -> PineResult<()> + Clone {
    move |input| {
        let parser = context("sign", token(tag(sign)));

        if sign
            .chars()
            .any(|character| OPERATOR_CHARACTERS.contains(character))
        {
            value((), tuple((parser, peek(not(one_of(OPERATOR_MODIFIERS))))))(input)
        } else {
            value((), parser)(input)
        }
    }
}

pub fn token<'a, O>(
    mut parser: impl Parser<Input<'a>, O, NomError<'a>>,
) -> impl FnMut(Input<'a>) -> PineResult<'a, O> {
    move |input| {
        let (input, _) = blank(input)?;

        parser.parse(input)
    }
}

pub fn blank(input: Input) -> PineResult<()> {
    value(
        (),
        many0_count(alt((value((), multispace1), skipped_comment))),
    )(input)
}

// fn comment(input: Input) -> PineResult<Comment> {
//     context(
//         "comment",
//         map(
//             tuple((comment_position, tag("#"), many0(none_of("\n\r")))),
//             |(position, _, characters)| Comment::new(String::from_iter(characters), position),
//         ),
//     )(input)
// }

// Optimize comment parsing by skipping contents.
fn skipped_comment(input: Input) -> PineResult<()> {
    value((), pair(tag("#"), many0_count(none_of("\n\r"))))(input)
}




fn binary_operator(input: Input) -> PineResult<BinaryOperator> {
    context(
        "binary operator",
        alt((
            value(BinaryOperator::Add, sign("+")),
            value(BinaryOperator::Subtract, sign("-")),
            value(BinaryOperator::Multiply, sign("*")),
            value(BinaryOperator::Divide, sign("/")),
            value(BinaryOperator::Equal, sign("==")),
            value(BinaryOperator::NotEqual, sign("!=")),
            value(BinaryOperator::LessThanOrEqual, sign("<=")),
            value(BinaryOperator::LessThan, sign("<")),
            value(BinaryOperator::GreaterThanOrEqual, sign(">=")),
            value(BinaryOperator::GreaterThan, sign(">")),
            value(BinaryOperator::And, sign("&")),
            value(BinaryOperator::Or, sign("|")),
        )),
    )(input)
}