use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::ix_data::{
    Protocol::{Pumpfun, Pumpswap, Whirlpool},
    SwapArgs,
};

pub fn swap(args: &SwapArgs, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    match args.protocol {
        Pumpfun => {}
        Pumpswap => {}
        Whirlpool => {}
    }

    Ok(())
}
