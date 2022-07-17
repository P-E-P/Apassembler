use super::{Instruction, Res};
use crate::parser::address::parse_relative;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, error::context, sequence::tuple,
};

fn parse_vi_opname(input: &str) -> Res<&str, &str> {
    context(
        "vi opcode name",
        alt((
            tag("JMP"),
            tag("JEQ"),
            tag("JNE"),
            tag("JC"),
            tag("JNC"),
            tag("JGT"),
            tag("JLT"),
            tag("JN"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context("vi", tuple((parse_vi_opname, space1, parse_relative)))(input).map(
        |(next_input, (opname, _, address))| {
            (
                next_input,
                Instruction::VI {
                    opname: opname.to_owned(),
                    displacement: address,
                },
            )
        },
    )
}
