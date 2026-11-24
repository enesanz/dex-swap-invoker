use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{
    error::DexSwapError, instructions::multi_hop_swap::multi_hop_swap, ix_data::MultiHopSwapArgs,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = MultiHopSwapArgs::try_from_slice(instruction_data)
        .map_err(|_| DexSwapError::InvalidInstructionData)?;

    multi_hop_swap(&args, accounts)?;

    Ok(())
}
