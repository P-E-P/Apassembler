use std::fmt;
use nom::character::complete::digit1;

use nom::branch::alt;
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    error::context,
    sequence::tuple,
};

use super::address::{parse_address, Address};
use super::operand::{parse_operand, Operand};
use super::Res;

pub enum Instruction {
    I {
        opname: String,
        ts: Operand,
        tsd: Operand,
    },
    II {
        opname: String,
        shift: u8,
        tsd: Operand,
    },
    Iii {
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

impl Instruction {
    pub fn to_binary(&self, symtable: Vec<(String, String)>) -> Vec<u16> {

        let mut result : Vec<u16> = vec![];

        fn get_word(operand: &Operand) -> Option<u16> {
            match operand {
                Operand::NextWord(address) => match address {
                    Address::Raw(value) => Some(*value),
                    Address::Symbolic(_) => todo!(),
                },
                _ => None
            }
        }

        result.push(match self {
            Instruction::I { opname, ts, tsd } => 0,
            Instruction::II { opname, shift, tsd } => todo!(),
            Instruction::Iii { opname, tsd } => todo!(),
            Instruction::IV { opname, tsd } => todo!(),
            Instruction::V { opname, tsd } => todo!(),
            Instruction::VI { opname, displacement } => todo!(),
        });

        if let Some(value) = match &self {
            Instruction::I { opname: _, ts, tsd: _ } => get_word(ts),
            _ => None,
        } {
            result.push(value);
        }

        if let Some(value) = match &self {
            Instruction::I { opname: _, ts: _, tsd } => get_word(tsd),
            Instruction::II { opname: _, shift: _, tsd } => get_word(tsd),
            Instruction::Iii { opname: _, tsd } => get_word(tsd),
            Instruction::IV { opname: _, tsd } => get_word(tsd),
            Instruction::V { opname: _, tsd } => get_word(tsd),
            _ => None
        } {
            result.push(value);
        }

        result
    }
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
    context("iv", tuple((parse_iv_opname, space1, parse_operand)))(input).map(
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

fn parse_ii_opname(input: &str) -> Res<&str, &str> {
    context(
        "ii opcode name",
        alt((tag("SLL"), tag("SRL"), tag("SLA"), tag("SRA"), tag("ROT"))),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

fn parse_ii(input: &str) -> Res<&str, Instruction> {
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

fn parse_i_opname(input: &str) -> Res<&str, &str> {
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

fn parse_i(input: &str) -> Res<&str, Instruction> {
    context(
        "i",
        tuple((
            parse_i_opname,
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

pub fn parse_instruction(input: &str) -> Res<&str, Instruction> {
    context(
        "instruction",
        alt((parse_i, parse_ii, parse_iii, parse_iv, parse_v, parse_vi)),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}
