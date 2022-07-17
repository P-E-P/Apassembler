use super::Res;
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::{
    bytes::complete::{tag, take_while},
    error::context,
    sequence::tuple,
};

use crate::parser::hexadecimal::hex_8bits;
use super::Address;

fn parse_hard_positive_dec_relative(input: &str) -> Res<&str, i8> {
    context("Hard positive decimal address", tuple((tag("+"), digit1)))(input)
        .map(|(next_input, (_plus, value))| (next_input, value.parse::<i8>().unwrap()))
}

fn parse_soft_positive_dec_relative(input: &str) -> Res<&str, i8> {
    context("Soft positive decimal address", digit1)(input)
        .map(|(next_input, value)| (next_input, value.parse::<i8>().unwrap()))
}

fn parse_positive_dec_relative(input: &str) -> Res<&str, i8> {
    context(
        "Positive decimal address",
        alt((
            parse_hard_positive_dec_relative,
            parse_soft_positive_dec_relative,
        )),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}

fn parse_negative_dec_relative(input: &str) -> Res<&str, i8> {
    context("Hard positive decimal address", tuple((tag("+"), digit1)))(input)
        .map(|(next_input, (_plus, value))| (next_input, -value.parse::<i8>().unwrap()))
}

fn parse_hard_positive_hex_relative(input: &str) -> Res<&str, i8> {
    context(
        "Hard positive hexadecimal address",
        tuple((tag("+"), hex_8bits)),
    )(input)
    .map(|(next_input, (_plus, value))| (next_input, i8::try_from(value).unwrap()))
}

fn parse_soft_positive_hex_relative(input: &str) -> Res<&str, i8> {
    context("Soft positive hexadecimal address", hex_8bits)(input)
        .map(|(next_input, value)| (next_input, i8::try_from(value).unwrap()))
}

fn parse_positive_hex_relative(input: &str) -> Res<&str, i8> {
    context(
        "Positive decimal address",
        alt((
            parse_hard_positive_hex_relative,
            parse_soft_positive_hex_relative,
        )),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}

fn parse_negative_hex_relative(input: &str) -> Res<&str, i8> {
    context(
        "Hard positive hexadecimal address",
        tuple((tag("-"), hex_8bits)),
    )(input)
    .map(|(next_input, (_plus, value))| (next_input, -i8::try_from(value).unwrap()))
}

fn parse_hex_relative(input: &str) -> Res<&str, Address> {
    context(
        "Hexadecimal relative address",
        alt((parse_negative_hex_relative, parse_positive_hex_relative)),
    )(input)
    .map(|(next_input, address)| (next_input, Address::Relative(address)))
}

fn parse_dec_relative(input: &str) -> Res<&str, Address> {
    context(
        "Decimal relative address",
        alt((parse_negative_dec_relative, parse_positive_dec_relative)),
    )(input)
    .map(|(next_input, address)| (next_input, Address::Relative(address)))
}

fn parse_relative_symbolic(input: &str) -> Res<&str, Address> {
    context(
        "Relative symbolic Address",
        take_while(super::sym_address_char),
    )(input)
    .map(|(next_input, address)| (next_input, Address::RelativeSymbolic(address.to_owned())))
}

pub fn parse_relative(input: &str) -> Res<&str, Address> {
    context(
        "Relative address",
        alt((
            parse_dec_relative,
            parse_hex_relative,
            parse_relative_symbolic,
        )),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}
