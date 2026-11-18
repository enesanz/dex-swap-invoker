use crate::{
    error::DexSwapError,
    instruction::{
        MultiHopSwapArgs,
        Protocol::{Pumpfun, Pumpswap, RaydiumCpmm},
    },
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn multi_hop_swap(
    args: &MultiHopSwapArgs,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    if args.hops.len() != 2 {
        return Err(DexSwapError::InvalidHopAmount.into());
    }

    for protocol in args.hops.iter() {
        match *protocol {
            Pumpfun => {}
            Pumpswap => {}
            RaydiumCpmm => {}
        }
    }

    Ok(())
}
