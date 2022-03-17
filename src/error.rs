use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MailError {
    /// InvalidInstruction
    #[error("Invalid Instruction!")]
    InvalidInstruction,
    /// NotWritable
    #[error("The account is not writable!")]
    NotWritable,
}

impl From<MailError> for ProgramError {
    fn from(err: MailError) -> Self {
        ProgramError::Custom(err as u32)
    }
}
