use std::{fmt, num::ParseIntError};

#[test]
fn hex_to_num() {
    let s = "00 00 00 18 66 74 79 70 33 67 70 35";
    let list: Vec<u8> = hex(s).unwrap();
    println!("{:?}, len: {}", list, list.len());
}

fn hex(s: &str) -> Result<Vec<u8>,DecodeHexError> {
    (0..s.len())
        .step_by(3)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeHexError {
    OddLength,
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for DecodeHexError {
    fn from(e: ParseIntError) -> Self {
        DecodeHexError::ParseInt(e)
    }
}

impl fmt::Display for DecodeHexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeHexError::OddLength => "input string has an odd number of bytes".fmt(f),
            DecodeHexError::ParseInt(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for DecodeHexError {}

