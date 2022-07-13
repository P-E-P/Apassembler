use super::Res;
use nom::{bytes::complete::tag, character::complete::digit1, error::context, sequence::tuple};

#[derive(Debug, Eq, PartialEq)]
pub struct Register(pub u8);

pub fn parse_register(input: &str) -> Res<&str, Register> {
    context("register", tuple((tag("R"), digit1)))(input).map(|(next_input, (_r, digit))| {
        (
            next_input,
            Register(digit.parse::<u8>().expect("Cannot convert to int")),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_zero() {
        assert_eq!(parse_register("R0"), Ok(("", Register(0))))
    }

    #[test]
    fn register_one() {
        assert_eq!(parse_register("R1"), Ok(("", Register(1))))
    }

    #[test]
    fn register_fifteen() {
        assert_eq!(parse_register("R15"), Ok(("", Register(15))))
    }
}
