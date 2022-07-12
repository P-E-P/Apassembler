use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0, character1},
    error::context,
    sequence::tuple,
    IResult,
};

pub fn parse_address(input: &str) -> Res<&str, Ts> {
    context(
        "@Address",
        tuple((space0, tag("@"), character1, space0)),
    )(input)
    .map(|(next_input, (_, _a, address, _))| {
        (
            next_input,
            Ts::NextWord(address)
        )
    })
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
  map_res(
    take_while_m_n(1, 8, is_hex_digit),
    from_hex
  )(input)
}

pub fn parse_raw_address(input: &str) -> Res<&str, Ts> {
    context(
        "Raw address",
        tuple((space0, tag("0x"), hex_primary, space0)),
    )(input)
    .map(|(next_input, (_, _a, address, _))| {
        (
            next_input,
            Ts::NextWordRaw(address.parse::<u16>().expect("Cannot parse address"))
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_address() {
        assert_eq!(parse_register("0x1102"), Ok(("", Ts::NextWordRaw(0x1102))))
    }

    #[test]
    fn address() {
        assert_eq!(parse_register("@Address"), Ok(("", Ts::NextWord("Address"))))
    }

}