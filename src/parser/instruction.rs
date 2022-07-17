use std::collections::HashMap;

use bit_field::BitField;
use phf::phf_map;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0, space1},
    error::context,
    sequence::tuple,
};

use super::operand::{parse_operand, Operand};
use super::Res;
use super::{
    address::{parse_relative, Address},
    register::Register,
};

static OPCODES: phf::Map<&'static str, u16> = phf_map! {
    // I
    "OR" => 0b000,
    "AND" => 0b001,
    "XOR" => 0b010,
    "CMP" => 0b011,
    "ADD" => 0b110,
    "STR" => 0b101,
    "MUL" => 0b110,
    "MOV" => 0b111,
    // II
    "SLL" => 0b000,
    "SRL" => 0b001,
    "SLA" => 0b010,
    "SRA" => 0b011,
    "ROT" => 0b100,
    // III
    "ORI" => 0b0000,
    "ANDI" => 0b0001,
    "XORI" => 0b0010,
    "CI" => 0b0011,
    "ADDI" => 0b0100,
    "STRI" => 0b0101,
    "MULI" => 0b0110,
    "LI" => 0b1000,
    "LIMI" => 0b1001,
    // IV
    "NOT" => 0b000,
    "INC" => 0b001,
    "DEC" => 0b010,
    "CLR" => 0b011,
    "PUSH" => 0b100,
    "PULL" => 0b101,
    "ROI" => 0b110,
    "TEST" => 0b111,
    "SET" => 0b111,
    // V
    "B" => 0b000,
    "BEQ" => 0b001,
    "BNE" => 0b010,
    "BC" => 0b011,
    "BNC" => 0b100,
    "BGT" => 0b101,
    "BLT" => 0b110,
    "BN" => 0b111,
    // VI
    "JMP" => 0b000,
    "JEQ" => 0b001,
    "JNE" => 0b010,
    "JC" => 0b011,
    "JNC" => 0b100,
    "JGT" => 0b101,
    "JLT" => 0b110,
    "JN" => 0b111,
};

trait Padd<T> {
    fn padd(self) -> T;
}

impl Padd<u16> for i8 {
    fn padd(self) -> u16 {
        let bytes = [0, self.to_be_bytes()[0]];
        u16::from_be_bytes(bytes)
    }
}

#[derive(Debug)]
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
    pub fn to_binary(&self, symtable: &HashMap<String, u16>) -> Vec<u16> {
        let mut result: Vec<u16> = vec![];

        fn get_word(operand: &Operand, symtable: &HashMap<String, u16>) -> Option<u16> {
            match operand {
                Operand::NextWord(address) => address.resolve(symtable),
                _ => None,
            }
        }

        result.push(match self {
            Instruction::I { opname, ts, tsd } => *0u16
                .set_bit(15, false)
                .set_bits(12..=14, *OPCODES.get(opname).unwrap())
                .set_bits(10..=11, ts.into())
                .set_bits(6..=9, ts.get_register_value())
                .set_bits(4..=5, tsd.into())
                .set_bits(..=3, tsd.get_register_value()),
            Instruction::II { opname, shift, tsd } => *0u16
                .set_bits(14..=15, 0b10)
                .set_bits(11..=13, *OPCODES.get(opname).unwrap())
                .set_bit(10, false)
                .set_bits(6..=9, (*shift).into())
                .set_bits(4..=5, tsd.into())
                .set_bits(0..=3, tsd.get_register_value()),
            Instruction::Iii { opname, tsd } => *0u16
                .set_bits(13..=15, 0b110)
                .set_bits(9..=12, *OPCODES.get(opname).unwrap())
                .set_bits(6..=8, 0b000)
                .set_bits(4..=5, tsd.into())
                .set_bits(..=3, tsd.get_register_value()),
            Instruction::IV { opname, tsd } => *0u16
                .set_bits(11..=15, 0b11100)
                .set_bits(8..=10, *OPCODES.get(opname).unwrap())
                .set_bits(6..=7, 0b00)
                .set_bits(4..=5, tsd.into())
                .set_bits(..=3, tsd.get_register_value()),
            Instruction::V { opname, tsd } => *0u16
                .set_bits(11..=15, 0b11101)
                .set_bits(8..=10, *OPCODES.get(opname).unwrap())
                .set_bits(6..=7, 0b00)
                .set_bits(4..=5, tsd.into())
                .set_bits(..=3, tsd.get_register_value()),
            Instruction::VI {
                opname,
                displacement,
            } => *0u16
                .set_bits(11..=15, 0b11110)
                .set_bits(8..=10, *OPCODES.get(opname).unwrap())
                .set_bits(..=7, displacement.resolve_relative().unwrap_or(0x0).padd()),
        });

        if let Some(value) = match &self {
            Instruction::I {
                opname: _,
                ts,
                tsd: _,
            } => get_word(ts, symtable),
            _ => None,
        } {
            result.push(value);
        }

        if let Some(value) = match &self {
            Instruction::I {
                opname: _,
                ts: _,
                tsd,
            } => get_word(tsd, symtable),
            Instruction::II {
                opname: _,
                shift: _,
                tsd,
            } => get_word(tsd, symtable),
            Instruction::Iii { opname: _, tsd } => get_word(tsd, symtable),
            Instruction::IV { opname: _, tsd } => get_word(tsd, symtable),
            Instruction::V { opname: _, tsd } => get_word(tsd, symtable),
            _ => None,
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

fn parse_iv(input: &str) -> Res<&str, Instruction> {
    context("iv", alt((parse_iv_generic, parse_iv_roi)))(input)
        .map(|(next_input, instruction)| (next_input, instruction))
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
