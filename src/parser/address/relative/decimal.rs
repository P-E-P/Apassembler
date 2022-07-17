use super::{Address, Res};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, error::context, sequence::tuple,
};

fn parse_hard_positive(input: &str) -> Res<&str, i8> {
    context("Hard positive decimal address", tuple((tag("+"), digit1)))(input)
        .map(|(next_input, (_plus, value))| (next_input, value.parse::<i8>().unwrap()))
}

fn parse_soft_positive(input: &str) -> Res<&str, i8> {
    context("Soft positive decimal address", digit1)(input)
        .map(|(next_input, value)| (next_input, value.parse::<i8>().unwrap()))
}

fn parse_positive(input: &str) -> Res<&str, i8> {
    context(
        "Positive decimal address",
        alt((parse_hard_positive, parse_soft_positive)),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}

fn parse_negative(input: &str) -> Res<&str, i8> {
    context("Negative decimal address", tuple((tag("-"), digit1)))(input)
        .map(|(next_input, (_plus, value))| (next_input, -value.parse::<i8>().unwrap()))
}

pub fn parse(input: &str) -> Res<&str, Address> {
    context(
        "Decimal relative address",
        alt((parse_negative, parse_positive)),
    )(input)
    .map(|(next_input, address)| (next_input, Address::Relative(address)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soft_positive() {
        assert_eq!(parse_soft_positive("8"), Ok(("", 8)))
    }

    #[test]
    fn hard_positive() {
        assert_eq!(parse_hard_positive("+8"), Ok(("", 8)))
    }

    #[test]
    fn negative() {
        assert_eq!(parse_negative("-8"), Ok(("", -8)))
    }
}
