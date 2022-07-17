use super::{Instruction, Res};
use crate::parser::{
    operand::{parse_operand, Operand},
    register::Register,
};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, error::context, sequence::tuple,
};

fn parse_iv_opname_generic(input: &str) -> Res<&str, &str> {
    context(
        "iv opcode name",
        alt((
            tag("NOT"),
            tag("INC"),
            tag("DEC"),
            tag("CLR"),
            tag("PUSH"),
            tag("PULL"),
            tag("TST"),
            tag("SET"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

fn parse_iv_generic(input: &str) -> Res<&str, Instruction> {
    context(
        "iv generic",
        tuple((parse_iv_opname_generic, space1, parse_operand)),
    )(input)
    .map(|(next_input, (opname, _, operand))| {
        (
            next_input,
            Instruction::IV {
                opname: opname.to_owned(),
                tsd: operand,
            },
        )
    })
}

fn parse_iv_opname_roi(input: &str) -> Res<&str, &str> {
    context("iv roi opcode name", tag("ROI"))(input).map(|(next_input, res)| (next_input, res))
}

fn parse_iv_roi(input: &str) -> Res<&str, Instruction> {
    context("iv roi", parse_iv_opname_roi)(input).map(|(next_input, opname)| {
        (
            next_input,
            Instruction::IV {
                opname: opname.to_owned(),
                tsd: Operand::Direct(Register(0)),
            },
        )
    })
}

pub fn parse(input: &str) -> Res<&str, Instruction> {
    context("iv", alt((parse_iv_generic, parse_iv_roi)))(input)
        .map(|(next_input, instruction)| (next_input, instruction))
}
