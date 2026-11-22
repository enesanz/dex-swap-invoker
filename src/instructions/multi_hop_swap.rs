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
        SwapDirection,
    },
    protocols::{
        pumpfun::{self, accounts::PumpFunAccounts},
        pumpswap::{self, accounts::PumpSwapAccounts},
        whirlpool::{self, accounts::WhirlpoolAccounts, invoke},
    },
    utils::utils::read_token_amount,
};
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn multi_hop_swap(
    args: &MultiHopSwapArgs,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    if args.hops.len() < 2 {
        return Err(DexSwapError::NotEnoughHops.into());
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

        let amount_in = if index == 0 && args.amount_in > 0 {
            args.amount_in
        } else {
            read_token_amount(swap_ctx.input_token_account)?
        };

        let is_last_hop = index == args.hops.len() - 1;

        let min_out = if is_last_hop { args.min_amount_out } else { 1 };

        match *protocol {
            Pumpfun => {
                let pumpfun = PumpFunAccounts::parse(&mut cursor)?;

                match args.direction {
                    SwapDirection::Buy => pumpfun::invoke::buy_exact_in_v3(
                        amount_in, min_out, &shared, &swap_ctx, &pumpfun,
                    )?,
                    SwapDirection::Sell => {
                        pumpfun::invoke::sell_v3(amount_in, min_out, &shared, &swap_ctx, &pumpfun)?
                    }
                }
            }
            Pumpswap => {
                let pumpswap = PumpSwapAccounts::parse(&mut cursor)?;

                match args.direction {
                    SwapDirection::Buy => pumpswap::invoke::buy_exact_in_v2(
                        amount_in, min_out, &shared, &swap_ctx, &pumpswap,
                    )?,
                    SwapDirection::Sell => pumpswap::invoke::sell_v2(
                        amount_in, min_out, &shared, &swap_ctx, &pumpswap,
                    )?,
                }
            }
            Whirlpool => {
                let whirlpool = WhirlpoolAccounts::parse(&mut cursor)?;

                match args.direction {
                    SwapDirection::Buy => whirlpool::invoke::buy_swap_v2(
                        amount_in, min_out, &shared, &swap_ctx, &whirlpool,
                    )?,
                    SwapDirection::Sell => whirlpool::invoke::sell_swap_v2(
                        amount_in, min_out, &shared, &swap_ctx, &whirlpool,
                    )?,
                }
            }
        }
    }

    Ok(())
}
