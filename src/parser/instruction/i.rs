use super::{Instruction, Res};
use crate::parser::operand::parse_operand;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    error::context,
    sequence::tuple,
};

fn opname(input: &str) -> Res<&str, &str> {
    context(
        "i opcode name",
        alt((
            tag("OR"),
            tag("AND"),
            tag("XOR"),
            tag("CMP"),
            tag("ADD"),
            tag("STR"),
            tag("MUL"),
            tag("MOV"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context(
        "i",
        tuple((
            opname,
            space1,
            parse_operand,
            space0,
            tag(","),
            space0,
            parse_operand,
        )),
    )(input)
    .map(
        |(next_input, (opname, _, operand_src, _, _comma, _, operand_dest))| {
            (
                next_input,
                Instruction::I {
                    opname: opname.to_owned(),
                    ts: operand_src,
                    tsd: operand_dest,
                },
            )
        },
    )
}
