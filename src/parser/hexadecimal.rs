use super::Res;

use nom::{bytes::complete::take_while_m_n, error::context};

fn from_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

pub fn hex_8bits(input: &str) -> Res<&str, u16> {
    context("hex primary", take_while_m_n(1, 2, is_hex_digit))(input).map(|(next_input, hexa)| {
        (
            next_input,
            from_hex(hexa).expect("Unable to convert from hexadecimal"),
        )
    })
}

pub fn hex_16bits(input: &str) -> Res<&str, u16> {
    context("hex primary", take_while_m_n(1, 4, is_hex_digit))(input).map(|(next_input, hexa)| {
        (
            next_input,
            from_hex(hexa).expect("Unable to convert from hexadecimal"),
        )
    })
}
