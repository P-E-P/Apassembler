use super::{Instruction, Res};
use crate::parser::operand::parse_operand;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, error::context, sequence::tuple,
};

fn parse_v_opname(input: &str) -> Res<&str, &str> {
    context(
        "v opcode name",
        alt((
            tag("B"),
            tag("BEQ"),
            tag("BNE"),
            tag("BC"),
            tag("BNC"),
            tag("BGT"),
            tag("BLT"),
            tag("BN"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context("v", tuple((parse_v_opname, space1, parse_operand)))(input).map(
        |(next_input, (opname, _, operand))| {
            (
                next_input,
                Instruction::V {
                    opname: opname.to_owned(),
                    tsd: operand,
                },
            )
        },
    )
}
