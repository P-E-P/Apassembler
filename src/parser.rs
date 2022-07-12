use nom::error::VerboseError;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    error::context,
    sequence::tuple,
    IResult,
};

use register::Register;

mod register;

pub enum Ts {
    Direct(Register),
    NextWordRaw(u16),
    NextWord(u16),
    Address(Register),
    AddressIncrement(Register),
}

pub enum Instruction {
    I { ts: Ts, tsd: Ts },
    II { decrement: u8, tsd: Ts },
    III { tsd: Ts },
    IV { tsd: Ts },
    V { tsd: Ts },
    VI { displacement: u8 },
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_instruction(input: &str) -> Res<&str, Instruction> {
    Ok(("", Instruction::VI { displacement: 0 }))
}
