use clap::{arg, command};
use parser::instruction::Instruction;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

mod parser;

fn parse_instruction(filepath: &Path) -> Vec<Instruction> {
    let file = File::open(filepath).expect("Cannot open file");

    let reader: BufReader<File> = BufReader::new(file);
    let mut result = vec![];

    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        if !line.is_empty() {
            match parser::parse_line(&line) {
                Ok((_, ins)) => result.push(ins),
                Err(why) => eprintln!("Failed to parse instruction: {}", why),
            }
        }
    }
    result
}

fn retrieve_symbols(filepath: &Path) -> HashMap<String, u16> {
    let file = File::open(filepath).expect("Cannot open file");

    let reader: BufReader<File> = BufReader::new(file);
    let mut result = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        if !line.is_empty() {
            match parser::parse_label(&line) {
                Ok((_, ins)) => {
                    result.insert(ins.name, ins.address);
                }
                Err(why) => eprintln!("Failed to parse symbol: {}", why),
            }
        }
    }

    result
}

fn main() {
    let matches = command!().arg(arg!([FILE])).get_matches();

    let filepath = Path::new(matches.value_of("FILE").expect("No file specified"));

    let symbols = retrieve_symbols(filepath);
    let instructions: Vec<Instruction> = parse_instruction(filepath);

    for instruction in instructions {
        let words = instruction.to_binary(&symbols);
        println!("{:?}", instruction);
        print!("Binary: ");
        for word in &words {
            print!("{:016b} ", word);
        }
        print!("\nHex: ");
        for word in words {
            print!("{:X} ", word);
        }
        println!();
    }
}
