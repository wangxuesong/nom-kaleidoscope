// NOTE: Each keyword_$start_letter_to_$end_letter function uses `alt`,
// which is implemented for tuples sizes up to 21. Because of this constraint
// on maximum tuple sizes, keywords are aggregated into groups of 20

use crate::Span;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::sequence::terminated;
use nom::IResult;

pub fn keywords(i: Span) -> IResult<Span, Span> {
    alt((
        terminated(tag_no_case("def"), crate::token_terminated),
        terminated(tag_no_case("extern"), crate::token_terminated),
    ))(i)
}
