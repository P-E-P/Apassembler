use super::{Instruction, Res};
use crate::parser::operand::parse_operand;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    error::context,
    sequence::tuple,
};

fn parse_ii_opname(input: &str) -> Res<&str, &str> {
    context(
        "ii opcode name",
        alt((tag("SLL"), tag("SRL"), tag("SLA"), tag("SRA"), tag("ROT"))),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context(
        "ii",
        tuple((
            parse_ii_opname,
            space1,
            digit1,
            space0,
            tag(","),
            space0,
            parse_operand,
        )),
    )(input)
    .map(|(next_input, (opname, _, shift, _, _comma, _, operand))| {
        (
            next_input,
            Instruction::II {
                opname: opname.to_owned(),
                shift: shift.parse::<u8>().expect("cannot parse to u8"),
                tsd: operand,
            },
        )
    })
}
