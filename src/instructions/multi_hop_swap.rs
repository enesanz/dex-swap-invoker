use crate::instruction::MultiHopSwapArgs;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn multi_hop_swap(
    args: &MultiHopSwapArgs,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    Ok(())
}
