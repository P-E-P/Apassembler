use nom::character::complete::space0;
use nom::error::VerboseError;

use nom::{error::context, sequence::tuple, IResult};

use instruction::{parse_instruction, Instruction};
use label::Label;
use operand::Operand;

mod address;
pub mod instruction;
pub mod label;
mod operand;
mod pointer;
mod register;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_line(input: &str) -> Res<&str, Instruction> {
    context("line", tuple((space0, parse_instruction, space0)))(input)
        .map(|(next_input, (_, res, _))| (next_input, res))
}

pub fn parse_label(input: &str) -> Res<&str, Label> {
    context("label parser", label::parse_label)(input)
}
