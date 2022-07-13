use super::address::{parse_address, Address};
use super::pointer::{parse_incremented_pointer, parse_pointer};
use super::register::{parse_register, Register};
use super::Res;
use nom::branch::alt;
use nom::error::context;

#[derive(Debug, PartialEq)]
pub enum Operand {
    Direct(Register),
    NextWord(Address),
    Address(Register),
    AddressIncrement(Register),
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
