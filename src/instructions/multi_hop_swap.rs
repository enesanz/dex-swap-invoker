use crate::{
    accounts::{cursor::AccountCursor, route::RouteAccounts, shared::SharedAccounts},
    error::DexSwapError,
    instruction::{
        MultiHopSwapArgs,
        Protocol::{Pumpfun, Pumpswap, RaydiumCpmm},
    },
    protocols::pumpfun::accounts::PumpFunAccounts,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn multi_hop_swap(
    args: &MultiHopSwapArgs,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    if args.hops.len() < 2 {
        return Err(DexSwapError::NotEnoughtHops.into());
    }

    let mut cursor = AccountCursor::new(accounts);

    let shared = SharedAccounts::parse(&mut cursor)?;

    let mint_count = args.hops.len() + 1;

    let route = RouteAccounts::parse(&mut cursor, mint_count)?;

    for protocol in args.hops.iter() {
        match *protocol {
            Pumpfun => {
                let pump_accounts = PumpFunAccounts::parse(&mut cursor)?;
            }
            Pumpswap => {}
            RaydiumCpmm => {}
        }
    }

    Ok(())
}
