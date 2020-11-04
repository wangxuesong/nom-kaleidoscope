use crate::keywords::keywords;
use crate::{token_terminated, Position, Span};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::digit1;
use nom::character::is_alphanumeric;
use nom::combinator::{map, not, opt, peek, recognize};
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::IResult;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

pub fn identifier(i: Span) -> IResult<Span, Span> {
    preceded(not(peek(keywords)), take_while1(is_alphanumeric))(i)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Number { pos: Position, value: f64 },
    Variable { pos: Position, name: String },
}

fn number_expr(i: Span) -> IResult<Span, Expression> {
    map(
        terminated(
            recognize(pair(digit1, opt(tuple((tag("."), digit1))))),
            token_terminated,
        ),
        |s: Span| Expression::Number {
            pos: Position::from_span(s),
            value: f64::from_str(std::str::from_utf8(s.fragment()).unwrap()).unwrap(),
        },
    )(i)
}

fn variable_expr(i: Span) -> IResult<Span, Expression> {
    map(terminated(identifier, token_terminated), |s| {
        Expression::Variable {
            pos: Position::from_span(s),
            name: std::str::from_utf8(s.fragment()).unwrap().to_string(),
        }
    })(i)
}

fn expression(i: Span) -> IResult<Span, Expression> {
    alt((number_expr, variable_expr))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers() {
        let id1 = Span::new(b"foo");
        let id2 = Span::new(b"def");
        // let id3 = Span::new(b"1abc");

        assert!(identifier(id1).is_ok());
        assert!(identifier(id2).is_err());
        // assert!(identifier(id3).is_err());
    }

    #[test]
    fn parse_number() {
        let integer = Span::new(b"123");
        let float = Span::new(b"123.123");

        let p_integer = number_expr(integer).unwrap().1;
        let p_float = number_expr(float).unwrap().1;

        let expect_integer = Expression::Number {
            pos: Position::new(1, 1),
            value: 123 as f64,
        };
        let expect_float = Expression::Number {
            pos: Position::new(1, 1),
            value: 123.123 as f64,
        };

        assert_eq!(expect_integer, p_integer);
        assert_eq!(expect_float, p_float);
    }

    #[test]
    fn parse_variable() {
        let var = Span::new(b"a");

        let p_var = variable_expr(var).unwrap().1;

        let expect_var = Expression::Variable {
            pos: Position::new(1, 1),
            name: "a".to_string(),
        };

        assert_eq!(expect_var, p_var);
    }

    #[test]
    fn parse_expression() {
        let var = Span::new(b"a");
        let integer = Span::new(b"123");
        let float = Span::new(b"123.123");

        let p_var = expression(var).unwrap().1;
        let p_integer = expression(integer).unwrap().1;
        let p_float = expression(float).unwrap().1;

        let expect_integer = Expression::Number {
            pos: Position::new(1, 1),
            value: 123 as f64,
        };
        let expect_float = Expression::Number {
            pos: Position::new(1, 1),
            value: 123.123 as f64,
        };
        let expect_var = Expression::Variable {
            pos: Position::new(1, 1),
            name: "a".to_string(),
        };

        assert_eq!(expect_integer, p_integer);
        assert_eq!(expect_float, p_float);
        assert_eq!(expect_var, p_var);
    }
}
