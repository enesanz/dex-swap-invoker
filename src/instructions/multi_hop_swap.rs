use crate::{
    accounts::{
        cursor::AccountCursor,
        route::{RouteAccounts, SwapContext},
        shared::SharedAccounts,
    },
    error::DexSwapError,
    ix_data::{
        MultiHopSwapArgs,
        Protocol::{Pumpfun, Pumpswap, Whirlpool},
    },
    protocols::{
        pumpfun::{accounts::PumpFunAccounts, invoke},
        pumpswap::accounts::PumpSwapAccounts,
        whirlpool::accounts::WhirlpoolAccounts,
    },
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
        let swap_ctx = SwapContext {
            input_mint: route.mint(index)?,
            output_mint: route.mint(index + 1)?,
            input_token_account: route.user_token_account(index)?,
            output_token_account: route.user_token_account(index + 1)?,
        };

        match *protocol {
            Pumpfun => {
                let pumpfun = PumpFunAccounts::parse(&mut cursor)?;

                invoke::buy_exact_in_v3(0, 0, &shared, &swap_ctx, &pumpfun)?;
            }
            Pumpswap => {
                let pumpswap = PumpSwapAccounts::parse(&mut cursor)?;
            }
            Whirlpool => {
                let whirlpool = WhirlpoolAccounts::parse(&mut cursor)?;
            }
        }
    }

    Ok(())
}
