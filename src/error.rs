use solana_program::program_error::ProgramError;

pub enum DexSwapError {
    InvalidInstructionData = 6000,
}

impl From<DexSwapError> for ProgramError {
    fn from(value: DexSwapError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
