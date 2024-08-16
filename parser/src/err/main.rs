mod error;
mod ast;

use nom::{self, character::complete::{alphanumeric1, line_ending, tab}, combinator::{all_consuming, eof, map}, IResult, multi::{many0, many_till}, sequence::{pair, terminated}};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{newline, one_of};
use nom::error::VerboseError;
use nom::multi::many1;
use nom_locate::LocatedSpan;

pub type Text<'a> = LocatedSpan<&'a str, ParseContext<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct ParseContext<'a> {
    pub file: &'a str,
    pub expecting: crate::error::Expecting,
}

pub const PLACEHOLDER_CONTEXT: ParseContext = ParseContext {
    file: "placeholder.txt",
    expecting: crate::error::Expecting::PyreFile,
};

fn expecting(input: Text, expecting: crate::error::Expecting) -> Text {
    input.map_extra(|mut ctxt| {
        ctxt.expecting = expecting;
        ctxt
    })
}

type ParseResult<'a, Output> = IResult<Text<'a>, Output, VerboseError<Text<'a>>>;

fn parse_lines(input: Text) -> ParseResult<ast::Definition> {
    // Parse any whitespace (spaces, tabs, or newlines)
    let (input, whitespaces) = many1(one_of(" \t\n"))(input)?;

    // Count the newlines
    let count = whitespaces.iter().filter(|&&c| c == '\n').count();

    Ok((input, ast::Definition::Lines { count }))
}

fn parse_comment(input: Text) -> ParseResult<ast::Definition> {
    let (input, _) = tag("//")(input)?;
    let (input, text) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    Ok((
        input,
        ast::Definition::Comment {
            text: text.to_string(),
        },
    ))
}

pub fn run<'a>(

    input_string: &'a str,
) -> Result<(), nom::Err<VerboseError<Text<'a>>>> {
    let input = Text::new_extra(
        input_string,
        ParseContext {
            file: "path",
            expecting: crate::error::Expecting::PyreFile,
        },
    );

    match parse_comment(input) {
        Ok((remaining)) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub fn render_error(input: &str, err: nom::Err<VerboseError<Text>>) -> String {
    match err {
        nom::Err::Incomplete(_) => {
            return "Incomplete".to_string();
        }
        nom::Err::Error(error) => {
            // println!("PARSER ERROR {:#?}", &error);
            let err_text: String = convert_error(input, error);

            // println!("PARSER ERROR, formatted {:#?}", &err_text);
            // return err_text;
            return err_text;
        }
        nom::Err::Failure(error) => {
            // println!("PARSER ERROR {:#?}", &error);
            let err_text: String = convert_error(input, error);

            // println!("PARSER ERROR, formatted {:#?}", &err_text);
            return err_text;
        }
    }
}

fn convert_error(input: &str, err: VerboseError<Text>) -> String {
    if let Some((text, error_kind)) = err.errors.get(0) {
        let error = crate::error::Error {
            filepath: text.extra.file.to_string(),
            error_type: crate::error::ErrorType::ParsingError(crate::error::ParsingErrorDetails {
                expecting: text.extra.expecting.clone(),
            }),
            locations: vec![crate::error::Location {
                contexts: vec![],
                primary: vec![crate::error::Range {
                    start: to_location(text),
                    end: to_location(text),
                }],
            }],
        };

        crate::error::format_error(input, &error)
    } else {
        "No errors".to_string()
    }
}
fn to_location(pos: &Text) -> ast::Location {
    ast::Location {
        offset: pos.location_offset(),
        line: pos.location_line(),
        column: pos.get_column(),
    }
}
fn main() {
    let source = "\
    //asdfsad\
    adfasdf";


    match run(source ) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("34234{}",  render_error(source, err));
        }
    }
}
