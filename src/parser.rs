use nom::bytes::complete::take_while;
use nom::error::VerboseError;

use nom::branch::alt;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    error::context,
    sequence::tuple,
    IResult,
};

use address::{parse_address, Address};
use register::Register;

mod address;
mod pointer;
mod register;

#[derive(Debug, PartialEq)]
pub enum Ts {
    Direct(Register),
    NextWord(Address),
    Address(Register),
    AddressIncrement(Register),
}

pub enum Instruction {
    I { opname: String, ts: Ts, tsd: Ts },
    II { opname: String, decrement: u8, tsd: Ts },
    III { opname: String, tsd: Ts },
    IV { opname: String, tsd: Ts },
    V { opname: String, tsd: Ts },
    VI { opname: String, displacement: Address },
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn valid_op_char(chr: char) -> bool {
    chr.is_ascii_uppercase()
}

fn parse_vi_opname(input: &str) -> Res<&str, &str> {
    context(
        "opcode name",
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

pub fn parse_instruction(input: &str) -> Res<&str, Instruction> {
    Ok((
        "",
        Instruction::VI {
            opname: "".to_owned(),
            displacement: Address::Raw(0),
        },
    ))
}
