use super::{Address, Res};

use nom::{branch::alt, bytes::complete::take_while, error::context};

use decimal::parse as dec_parse;
use hexadecimal::parse as hex_parse;

mod decimal;
mod hexadecimal;

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
        alt((hex_parse, dec_parse, parse_relative_symbolic)),
    )(input)
    .map(|(next_input, address)| (next_input, address))
}
