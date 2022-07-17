use super::address::parse_raw_address_value;
use super::Res;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space0,
    error::context,
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub struct Label {
    pub address: u16,
    pub name: String,
}

impl Label {
    pub fn new(address: u16, name: String) -> Self {
        Label { address, name }
    }
}

pub fn parse_label_name(input: &str) -> Res<&str, String> {
    context("label name", take_while(|e: char| e.is_alphabetic()))(input)
        .map(|(next_input, label)| (next_input, label.to_owned()))
}

pub fn parse_label(input: &str) -> Res<&str, Label> {
    context(
        "@Address",
        tuple((
            space0,
            tag("("),
            parse_raw_address_value,
            tag(")"),
            space0,
            parse_label_name,
        )),
    )(input)
    .map(|(next_input, (_, _lpar, address, _rpar, _, tag))| (next_input, Label::new(address, tag)))
}
