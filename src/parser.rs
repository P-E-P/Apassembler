use nom::error::VerboseError;

use nom::{error::context, IResult};

use instruction::{parse_instruction, Instruction};
use operand::Operand;

mod address;
pub mod instruction;
mod operand;
mod pointer;
mod register;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub fn parse_line(input: &str) -> Res<&str, Instruction> {
    context("line", parse_instruction)(input).map(|(next_input, res)| (next_input, res))
}
