use clap::{arg, command};
use parser::Instruction;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod parser;

fn parse_instruction(file: File) -> Vec<Instruction> {
    let reader: BufReader<File> = BufReader::new(file);
    let mut result = vec![];

    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        if !line.is_empty() {
            match parser::parse_instruction(&line) {
                Ok((_, ins)) => result.push(ins),
                Err(why) => eprintln!("Error while parsing line \n{}\n Error is {:?}", line, why),
            }
        }
    }

    result
}

fn main() {
    let matches = command!().arg(arg!([FILE])).get_matches();

    let filepath = Path::new(matches.value_of("FILE").expect("No file specified"));

    let file = File::open(filepath).expect("Cannot open file");
    let instructions: Vec<Instruction> = parse_instruction(file);
}
