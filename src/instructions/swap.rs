use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::instruction::{
    Protocol::{Pumpfun, Pumpswap, RaydiumCpmm},
    SwapArgs,
};

pub fn swap(args: &SwapArgs, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    match args.protocol {
        Pumpfun => {}
        Pumpswap => {}
        RaydiumCpmm => {}
    }

    Ok(())
}
