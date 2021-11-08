use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("opcode missing")]
    OpcodeMissing,
    #[error("unknown instruction `{0}`")]
    UnknownInstruction(String),
    #[error("invalid no. of args, expected `{0}`, found `{1}`")]
    InvalidNoOfArgs(usize, usize),
    #[error("unknown register `{0}`")]
    UnknownRegister(String),
    #[error("failed to parse number `{0}`")]
    InvalidNumber(String),
    #[error("invalid instruction format `{0}`")]
    InvalidInstruction(String),
    #[error("no address attached to label `{0}`")]
    FloatingLabel(String),
}

pub type Result<T> = StdResult<T, AssemblerError>;
