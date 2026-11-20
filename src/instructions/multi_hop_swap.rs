use crate::{
    accounts::{cursor::AccountCursor, route::RouteAccounts, shared::SharedAccounts},
    error::DexSwapError,
    instruction::{
        MultiHopSwapArgs,
        Protocol::{Pumpfun, Pumpswap, RaydiumCpmm},
    },
    protocols::{pumpfun::accounts::PumpFunAccounts, pumpswap::accounts::PumpSwapAccounts},
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

    for (index, protocol) in args.hops.iter().enumerate() {
        let input_mint = route.mint(index)?;
        let output_mint = route.mint(index + 1)?;

        let input_token_account = route.user_token_account(index)?;
        let output_token_account = route.user_token_account(index + 1)?;
        match *protocol {
            Pumpfun => {
                let pumpfun = PumpFunAccounts::parse(&mut cursor)?;
            }
            Pumpswap => {
                let pumpswap = PumpSwapAccounts::parse(&mut cursor)?;
            }
            RaydiumCpmm => {}
        }
    }

    Ok(())
}
