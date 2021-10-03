use std::{error::Error, fmt::Display};

pub type ProcessorResult<T> = Result<T, ProcessorError>;

#[derive(Debug)]
pub enum ProcessorError {
    InvalidOpcode(u16),
    InvalidRegister(usize),
}

impl Error for ProcessorError {}

impl Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProcessorError::InvalidOpcode(val) => write!(f, "{} is not a valid opcode.", val),
            ProcessorError::InvalidRegister(val) => write!(f, "{} is not a valid register.", val),
        }
    }
}
