use crate::input::{ Input, Span};
use nom::error::{VerboseError, VerboseErrorKind};
use std::{error::Error, fmt, fmt::Display};
use std::str;
use position::Position;

pub type NomError<'a> = VerboseError<Input<'a>>;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    message: String,
    position: Position,
}
pub fn posd(input: &Input) -> Position {
    Position::new(
        &input.extra.source,
        input.location_line() as usize,
        input.get_column(),
        str::from_utf8(input.get_line_beginning()).unwrap(),
    )
}
impl ParseError {
    pub fn new(source: &str, path: &str, error: nom::Err<NomError<'_>>) -> Self {
        match error {
            nom::Err::Incomplete(_) => Self::unexpected_end(source, path),
            nom::Err::Error(error) | nom::Err::Failure(error) => {
                let context = error
                    .errors
                    .iter()
                    .find_map(|(_, kind)| {
                        if let VerboseErrorKind::Context(context) = kind {
                            Some(context)
                        } else {
                            None
                        }
                    })
                    .copied();

                if let Some(&(ref input, _)) = error.errors.first() {
                    Self {
                        message: if let Some(character) =
                            error.errors.iter().find_map(|(_, kind)| {
                                if let VerboseErrorKind::Char(character) = kind {
                                    Some(character)
                                } else {
                                    None
                                }
                            }) {
                            [format!("'{character}' expected")]
                                .into_iter()
                                .chain(context.map(|context| format!("in {context}")))
                                .collect::<Vec<_>>()
                                .join(" ")
                        } else {
                            ["failed to parse"]
                                .into_iter()
                                .chain(context)
                                .collect::<Vec<_>>()
                                .join(" ")
                        },
                        position: posd(input),
                    }
                } else {
                    Self::unexpected_end(source, path)
                }
            }
        }
    }

    fn unexpected_end(source: &str, path: &str) -> Self {
        let lines = source.split('\n').collect::<Vec<_>>();
        let line = lines
            .iter()
            .rev()
            .find(|string| !string.is_empty())
            .map(|string| string.to_string())
            .unwrap_or_default();

        Self {
            message: "unexpected end of source".into(),
            position: Position::new(path, lines.len(), line.len(), line),


        }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            formatter,
            "{}",
            [self.message.as_str(), &self.position.to_string()].join("\n"),
        )
    }
}
