use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::instruction::SwapArgs;

pub fn swap(args: &SwapArgs, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    Ok(())
}
