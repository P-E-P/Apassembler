use super::Res;

use nom::{
    branch::alt,
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

fn raw_8bits(input: &str) -> Res<&str, u16> {
    context("hex 8bits", take_while_m_n(1, 2, is_hex_digit))(input)
        .map(|(next_input, hexa)| (next_input, from_hex(hexa).unwrap()))
}

pub fn prefixed_hex_8bits(input: &str) -> Res<&str, u16> {
    context("prefixed hex 8bits", tuple((tag("0x"), raw_8bits)))(input)
        .map(|(next_input, (_prefix, value))| (next_input, value))
}

fn soft_positive_16bits(input: &str) -> Res<&str, i16> {
    context("prefixed hex 16bits", tuple((tag("0x"), raw_16bits)))(input)
        .map(|(next_input, (_prefix, value))| (next_input, value.try_into().unwrap()))
}

fn hard_positive_16bits(input: &str) -> Res<&str, i16> {
    context("hard positive hex 16bits", tuple((tag("+"), soft_positive_16bits)))(input)
        .map(|(next_input, (_prefix, value))| (next_input, value))
}

fn negative_16bits(input: &str) -> Res<&str, i16> {
    context("hard positive hex 16bits", tuple((tag("-"), soft_positive_16bits)))(input)
        .map(|(next_input, (_prefix, value))| (next_input, value))
}

pub fn i16(input: &str) -> Res<&str, i16> {
    context("i16 hexadecimal",
        alt((negative_16bits, hard_positive_16bits, soft_positive_16bits))
    )(input)
}

pub fn raw_16bits(input: &str) -> Res<&str, u16> {
    context("hex 16bits", take_while_m_n(1, 4, is_hex_digit))(input)
        .map(|(next_input, hexa)| (next_input, from_hex(hexa).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_8bits() {
        assert_eq!(raw_8bits("ff"), Ok(("", 255)))
    }

    #[test]
    fn uncomplete_8bits() {
        assert_eq!(raw_8bits("1"), Ok(("", 1)))
    }

    #[test]
    fn complete_16bits() {
        assert_eq!(raw_16bits("ffff"), Ok(("", 65535)))
    }

    #[test]
    fn uncomplete_16bits() {
        assert_eq!(raw_16bits("1"), Ok(("", 1)))
    }
}
