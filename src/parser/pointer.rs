use super::register::parse_register;
use super::{Operand, Res};
use nom::{bytes::complete::tag, error::context, sequence::tuple};

pub fn parse_pointer(input: &str) -> Res<&str, Operand> {
    context("pointer register", tuple((tag("*"), parse_register)))(input)
        .map(|(next_input, (_star, register))| (next_input, Operand::Address(register)))
}

pub fn parse_incremented_pointer(input: &str) -> Res<&str, Operand> {
    context(
        "incremented pointer register",
        tuple((tag("*"), parse_register, tag("+"))),
    )(input)
    .map(|(next_input, (_star, register, _plus))| (next_input, Operand::AddressIncrement(register)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::register::Register;

    #[test]
    fn incremented() {
        assert_eq!(
            parse_incremented_pointer("*R15+"),
            Ok(("", Operand::AddressIncrement(Register(15))))
        )
    }

    #[test]
    fn pointer() {
        assert_eq!(
            parse_pointer("*R15"),
            Ok(("", Operand::Address(Register(15))))
        )
    }
}
