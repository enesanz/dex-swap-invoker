use solana_program::program_error::ProgramError;

pub enum DexSwapError {
    InvalidInstructionData = 1,
    InvalidAccountLayout = 2,
    NotEnoughtHops = 3,
    RouteAccountOutOfBound = 4,
}

impl From<DexSwapError> for ProgramError {
    fn from(value: DexSwapError) -> Self {
        ProgramError::Custom(value as u32)
    }
}
