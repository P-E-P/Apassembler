use super::Res;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    error::context,
    sequence::tuple,
};

fn from_hex(input: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_8bits(input: &str) -> Res<&str, u16> {
    context("hex 8bits", take_while_m_n(1, 2, is_hex_digit))(input)
        .map(|(next_input, hexa)| (next_input, from_hex(hexa).unwrap()))
}

pub fn prefixed_hex_8bits(input: &str) -> Res<&str, u16> {
    context("prefixed hex 8bits", tuple((tag("0x"), hex_8bits)))(input)
        .map(|(next_input, (_prefix, value))| (next_input, value))
}

pub fn hex_16bits(input: &str) -> Res<&str, u16> {
    context("hex 16bits", take_while_m_n(1, 4, is_hex_digit))(input)
        .map(|(next_input, hexa)| (next_input, from_hex(hexa).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_8bits() {
        assert_eq!(hex_8bits("ff"), Ok(("", 255)))
    }

    #[test]
    fn uncomplete_8bits() {
        assert_eq!(hex_8bits("1"), Ok(("", 1)))
    }

    #[test]
    fn complete_16bits() {
        assert_eq!(hex_16bits("ffff"), Ok(("", 65535)))
    }

    #[test]
    fn uncomplete_16bits() {
        assert_eq!(hex_16bits("1"), Ok(("", 1)))
    }
}
