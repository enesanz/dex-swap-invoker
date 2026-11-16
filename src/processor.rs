use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{error::DexSwapError, instruction::InstructionData};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let data = InstructionData::try_from_slice(&instruction_data)
        .map_err(|_| DexSwapError::InvalidInstructionData)?;

    match data {
        InstructionData::Swap(args) => {
            // TODO invoke Swap function
        }
        InstructionData::MultiHopSwap(args) => {
            // TODO invoke MultiHopSwap function
        }
    };

    Ok(())
}
