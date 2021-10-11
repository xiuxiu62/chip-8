use std::{error::Error, fmt::Display};

pub type CPUResult<T> = Result<T, CPUError>;

#[derive(Debug)]
pub enum CPUError {
    InvalidOpcode(u16),
    InvalidRegister(usize),
}

impl Error for CPUError {}

impl Display for CPUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CPUError::InvalidOpcode(val) => write!(f, "{} is not a valid opcode.", val),
            CPUError::InvalidRegister(val) => write!(f, "{} is not a valid register.", val),
        }
    }
}
