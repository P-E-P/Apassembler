use nom::error::VerboseError;

use nom::branch::alt;
use nom::{
    bytes::complete::tag, character::complete::space1, error::context, sequence::tuple, IResult,
};

use address::{parse_address, Address};
use operand::{parse_operand, Operand};

mod address;
mod operand;
mod pointer;
mod register;

pub enum Instruction {
    I {
        opname: String,
        ts: Operand,
        tsd: Operand,
    },
    II {
        opname: String,
        decrement: u8,
        tsd: Operand,
    },
    III {
        opname: String,
        tsd: Operand,
    },
    IV {
        opname: String,
        tsd: Operand,
    },
    V {
        opname: String,
        tsd: Operand,
    },
    VI {
        opname: String,
        displacement: Address,
    },
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn valid_op_char(chr: char) -> bool {
    chr.is_ascii_uppercase()
}

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

fn parse_vi(input: &str) -> Res<&str, Instruction> {
    context("vi", tuple((parse_vi_opname, space1, parse_address)))(input).map(
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

fn parse_v(input: &str) -> Res<&str, Instruction> {
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

fn parse_iv_opname(input: &str) -> Res<&str, &str> {
    context(
        "iv opcode name",
        alt((
            tag("NOT"),
            tag("INC"),
            tag("DEC"),
            tag("CLR"),
            tag("PUSH"),
            tag("PULL"),
            tag("ROI"),
            tag("TST"),
            tag("SET"),
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

fn parse_iv(input: &str) -> Res<&str, Instruction> {
    context("v", tuple((parse_iv_opname, space1, parse_operand)))(input).map(
        |(next_input, (opname, _, operand))| {
            (
                next_input,
                Instruction::IV {
                    opname: opname.to_owned(),
                    tsd: operand,
                },
            )
        },
    )
}

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

fn parse_iii(input: &str) -> Res<&str, Instruction> {
    context("v", tuple((parse_iii_opname, space1, parse_operand)))(input).map(
        |(next_input, (opname, _, operand))| {
            (
                next_input,
                Instruction::III {
                    opname: opname.to_owned(),
                    tsd: operand,
                },
            )
        },
    )
}

pub fn parse_instruction(input: &str) -> Res<&str, Instruction> {
    context(
        "instruction",
        alt((
            parse_iii,
            parse_iv,
            parse_v,
            parse_vi,
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}
