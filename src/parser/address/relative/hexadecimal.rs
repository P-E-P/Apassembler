use super::{Address, Res};
use crate::parser::hexadecimal::prefixed_hex_8bits;

use nom::{branch::alt, bytes::complete::tag, error::context, sequence::tuple};

fn parse_hard_positive(input: &str) -> Res<&str, i8> {
    context(
        "Hard positive hexadecimal address",
        tuple((tag("+"), prefixed_hex_8bits)),
    )(input)
    .map(|(next_input, (_plus, value))| (next_input, i8::try_from(value).unwrap()))
}

fn parse_soft_positive(input: &str) -> Res<&str, i8> {
    context("Soft positive hexadecimal address", prefixed_hex_8bits)(input)
        .map(|(next_input, value)| (next_input, i8::try_from(value).unwrap()))
}

fn parse_positive(input: &str) -> Res<&str, i8> {
    context(
        "Positive decimal address",
        alt((parse_hard_positive, parse_soft_positive)),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}

fn parse_negative(input: &str) -> Res<&str, i8> {
    context(
        "Hard positive hexadecimal address",
        tuple((tag("-"), prefixed_hex_8bits)),
    )(input)
    .map(|(next_input, (_plus, value))| (next_input, -i8::try_from(value).unwrap()))
}

pub fn parse(input: &str) -> Res<&str, Address> {
    context(
        "Hexadecimal relative address",
        alt((parse_negative, parse_positive)),
    )(input)
    .map(|(next_input, address)| (next_input, Address::Relative(address)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_positive() {
        assert_eq!(parse_soft_positive("0x8"), Ok(("", 8)))
    }

    #[test]
    fn hard_positive() {
        assert_eq!(parse_hard_positive("+0x8"), Ok(("", 8)))
    }

    #[test]
    fn negative() {
        assert_eq!(parse_negative("-0x8"), Ok(("", -8)))
    }
}
