use super::{Instruction, Res};
use crate::parser::{
    operand::parse_operand,
    hexadecimal,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1, char},
    error::context,
    sequence::tuple,
};

fn immediate_value(input: &str) -> Res<&str, u16> {
    context("immediate raw value", 
        hexadecimal::i16
    )(input)
    .map(|(next_input, address)| (next_input, unsafe { std::mem::transmute(address)}))
}

fn opname(input: &str) -> Res<&str, &str> {
    context(
        "iii opcode name",
        alt((
            tag("ORI"),
            tag("ANDI"),
            tag("XORI"),
            tag("CI"),
            tag("ADDI"),
            tag("STRI"),
            tag("MULI"),
            tag("LI"),
            tag("LIMI"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context(
        "iii",
        tuple((
            opname,
            space1,
            parse_operand,
            space0,
            char(','),
            space0,
            immediate_value,
        )),
    )(input)
    .map(|(next_input, (opname, _, operand, _, _comma, _, immediate))| {
        (
            next_input,
            Instruction::Iii {
                opname: opname.to_owned(),
                immediate,
                tsd: operand,
            },
        )
    })
}
