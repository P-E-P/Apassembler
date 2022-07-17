use std::collections::HashMap;

use super::Res;
use nom::branch::alt;
use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    error::context,
    sequence::tuple,
};
pub use relative::parse_relative;

mod relative;

#[derive(Debug, PartialEq)]
pub enum Address {
    Raw(u16),
    Symbolic(String),
    RelativeSymbolic(String),
    Relative(i8),
}

impl Address {
    pub fn resolve(&self, symbols: &HashMap<String, u16>) -> Option<u16> {
        match self {
            Address::Raw(value) => Some(*value),
            Address::Relative(_) => None,
            Address::Symbolic(value) => symbols.get(value).copied(),
            Address::RelativeSymbolic(_) => None,
        }
    }

    pub fn resolve_relative(&self) -> Option<i8> {
        match self {
            Address::Raw(_) => None,
            Address::Relative(value) => Some(*value),
            Address::Symbolic(_) => None,
            Address::RelativeSymbolic(_) => todo!("Cannot resolve relative symbols yet"),
        }
    }
}

fn sym_address_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

pub fn parse_address(input: &str) -> Res<&str, Address> {
    context("Address", alt((parse_raw_address, parse_symbolic_address)))(input)
}

fn parse_symbolic_address(input: &str) -> Res<&str, Address> {
    context("@Address", tuple((tag("@"), take_while(sym_address_char))))(input)
        .map(|(next_input, (_a, address))| (next_input, Address::Symbolic(address.to_owned())))
}

fn from_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> Res<&str, u16> {
    context("hex primary", take_while_m_n(1, 8, is_hex_digit))(input).map(|(next_input, hexa)| {
        (
            next_input,
            from_hex(hexa).expect("Unable to convert from hexadecimal"),
        )
    })
}

pub fn parse_raw_address_value(input: &str) -> Res<&str, u16> {
    context("Raw address", tuple((tag("0x"), hex_primary)))(input)
        .map(|(next_input, (_prefix, address))| (next_input, address))
}

fn parse_raw_address(input: &str) -> Res<&str, Address> {
    context("Raw address", parse_raw_address_value)(input)
        .map(|(next_input, address)| (next_input, Address::Raw(address)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_address() {
        assert_eq!(parse_raw_address("0x1102"), Ok(("", Address::Raw(0x1102))))
    }

    #[test]
    fn address() {
        assert_eq!(
            parse_symbolic_address("@Address"),
            Ok(("", Address::Symbolic("Address".to_owned())))
        )
    }
}
