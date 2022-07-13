use super::Res;
use nom::branch::alt;
use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{digit1, space0},
    error::context,
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub enum Address {
    Raw(u16),
    Symbolic(String),
}

fn sym_address_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

pub fn parse_address(input: &str) -> Res<&str, Address> {
    context("Address", alt((parse_raw_address, parse_symbolic_address)))(input)
}

fn parse_symbolic_address(input: &str) -> Res<&str, Address> {
    context(
        "@Address",
        tuple((space0, tag("@"), take_while(sym_address_char), space0)),
    )(input)
    .map(|(next_input, (_, _a, address, _))| (next_input, Address::Symbolic(address.to_owned())))
}

fn from_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> Res<&str, u16> {
    context("hex primary", take_while_m_n(1, 8, is_hex_digit))(input).map(|(next_input, (hexa))| {
        (
            next_input,
            from_hex(hexa).expect("Unable to convert from hexadecimal"),
        )
    })
}

fn parse_raw_address(input: &str) -> Res<&str, Address> {
    context("Raw address", tuple((tag("0x"), hex_primary)))(input)
        .map(|(next_input, (_prefix, address))| (next_input, Address::Raw(address)))
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
