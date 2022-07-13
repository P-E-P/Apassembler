use super::register::parse_register;
use super::{Res, Ts};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    error::context,
    sequence::tuple,
};

pub fn parse_pointer(input: &str) -> Res<&str, Ts> {
    context("pointer register", tuple((tag("*"), parse_register)))(input)
        .map(|(next_input, (_star, register))| (next_input, Ts::Address(register)))
}

pub fn parse_incremented_pointer(input: &str) -> Res<&str, Ts> {
    context(
        "incremented pointer register",
        tuple((tag("*"), parse_register, tag("+"))),
    )(input)
    .map(|(next_input, (_star, register, _plus))| (next_input, Ts::AddressIncrement(register)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::register::Register;

    #[test]
    fn incremented() {
        assert_eq!(
            parse_incremented_pointer("*R15+"),
            Ok(("", Ts::AddressIncrement(Register(15))))
        )
    }

    #[test]
    fn pointer() {
        assert_eq!(parse_pointer("*R15"), Ok(("", Ts::Address(Register(15)))))
    }
}
