use super::{
    address::{parse_address, Address},
    pointer::{parse_incremented_pointer, parse_pointer},
    register::{parse_register, Register},
    Res,
};
use nom::{branch::alt, error::context};

#[derive(Debug, PartialEq)]
pub enum Operand {
    Direct(Register),
    NextWord(Address),
    Address(Register),
    AddressIncrement(Register),
}

impl From<&Operand> for u16 {
    fn from(op: &Operand) -> Self {
        match op {
            Operand::Direct(_) => 0b00,
            Operand::NextWord(_) => 0b01,
            Operand::Address(_) => 0b10,
            Operand::AddressIncrement(_) => 0b11,
        }
    }
}

impl Operand {
    pub fn get_register_value(&self) -> u16 {
        match self {
            Operand::Direct(r) => r.0.into(),
            Operand::NextWord(_) => 0,
            Operand::Address(r) => r.0.into(),
            Operand::AddressIncrement(r) => r.0.into(),
        }
    }
}

fn parse_register_operand(input: &str) -> Res<&str, Operand> {
    context("register operand", parse_register)(input)
        .map(|(next_input, operand)| (next_input, Operand::Direct(operand)))
}

fn parse_address_operand(input: &str) -> Res<&str, Operand> {
    context("address operand", parse_address)(input)
        .map(|(next_input, address)| (next_input, Operand::NextWord(address)))
}

pub fn parse_operand(input: &str) -> Res<&str, Operand> {
    context(
        "operand",
        alt((
            parse_incremented_pointer,
            parse_pointer,
            parse_register_operand,
            parse_address_operand,
        )),
    )(input)
    .map(|(next_input, operand)| (next_input, operand))
}
