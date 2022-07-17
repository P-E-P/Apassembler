use std::collections::HashMap;

use super::{hexadecimal::hex_16bits, Res};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
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
    context("Address", alt((parse_raw, parse_symbolic)))(input)
}

fn parse_symbolic(input: &str) -> Res<&str, Address> {
    context("@Address", tuple((tag("@"), take_while(sym_address_char))))(input)
        .map(|(next_input, (_a, address))| (next_input, Address::Symbolic(address.to_owned())))
}

pub fn parse_raw_value(input: &str) -> Res<&str, u16> {
    context("Raw address", tuple((tag("0x"), hex_16bits)))(input)
        .map(|(next_input, (_prefix, address))| (next_input, address))
}

fn parse_raw(input: &str) -> Res<&str, Address> {
    context("Raw address", parse_raw_value)(input)
        .map(|(next_input, address)| (next_input, Address::Raw(address)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_address() {
        assert_eq!(parse_raw("0x1102"), Ok(("", Address::Raw(0x1102))))
    }

    #[test]
    fn address() {
        assert_eq!(
            parse_symbolic("@Address"),
            Ok(("", Address::Symbolic("Address".to_owned())))
        )
    }
}
