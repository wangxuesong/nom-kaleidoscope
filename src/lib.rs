#![feature(array_methods)]

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::peek;
use nom::error::{ErrorKind, ParseError};
use nom::{IResult, InputLength};
use nom_locate::LocatedSpan;
use serde_derive::{Deserialize, Serialize};

mod keywords;
mod parser;

pub type Span<'a> = LocatedSpan<&'a [u8]>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub offset: usize,
}

impl Position {
    pub fn new(line: u32, offset: usize) -> Position {
        Position { line, offset }
    }

    pub fn new_empty() -> Position {
        Position { line: 0, offset: 0 }
    }

    fn from_span(span: Span) -> Position {
        Position {
            line: span.location_line(),
            offset: span.get_column(),
        }
    }
}

impl<'a> From<Span<'a>> for Position {
    fn from(s: Span<'a>) -> Self {
        Position::from_span(s)
    }
}

pub(crate) fn eof<I: Copy + InputLength, E: ParseError<I>>(input: I) -> IResult<I, I, E> {
    if input.input_len() == 0 {
        Ok((input, input))
    } else {
        Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
    }
}

pub fn token_terminated(i: Span) -> IResult<Span, Span> {
    peek(alt((
        multispace0,
        tag(" "),
        tag("\n"),
        tag(";"),
        tag("("),
        tag(")"),
        tag("\t"),
        tag(","),
        tag("="),
        eof,
    )))(i)
}
