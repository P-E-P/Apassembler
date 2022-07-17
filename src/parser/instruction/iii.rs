use super::{Instruction, Res};
use crate::parser::operand::parse_operand;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, error::context, sequence::tuple,
};

fn parse_iii_opname(input: &str) -> Res<&str, &str> {
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
    context("iii", tuple((parse_iii_opname, space1, parse_operand)))(input).map(
        |(next_input, (opname, _, operand))| {
            (
                next_input,
                Instruction::Iii {
                    opname: opname.to_owned(),
                    tsd: operand,
                },
            )
        },
    )
}
