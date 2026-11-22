use solana_program::account_info::AccountInfo;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::program::invoke;
use solana_program::program_error::ProgramError;

use crate::accounts::route::SwapContext;
use crate::protocols::pumpswap::accounts::PumpSwapAccounts;
use crate::utils::utils::get_token_program;
use crate::{accounts::shared::SharedAccounts, protocols::pumpfun::accounts::PumpFunAccounts};

const BUY_EXACT_IN_V2_DISCR: [u8; 8] = [194, 171, 28, 70, 104, 77, 91, 47];
const SELL_V2_DISCR: [u8; 8] = [93, 246, 130, 60, 231, 233, 64, 178];

pub fn buy_exact_in_v3<'a, 'info>(
    amount_in: u64,
    min_amount_out: u64,
    shared: &SharedAccounts<'a, 'info>,
    swap_ctx: &SwapContext<'a, 'info>,
    pump: &PumpSwapAccounts<'a, 'info>,
) -> Result<(), ProgramError> {
    let accounts = vec![
        AccountMeta::new(*pump.pool.key, false),
        AccountMeta::new(*shared.user.key, true),
        AccountMeta::new_readonly(*pump.global.key, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.key, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.key, false),
        AccountMeta::new(*swap_ctx.output_token_account.key, false),
        AccountMeta::new(*swap_ctx.input_token_account.key, false),
        AccountMeta::new(*pump.base_vault.key, false),
        AccountMeta::new(*pump.quote_vault.key, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.owner, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.owner, false),
        AccountMeta::new_readonly(*shared.system_program.key, false),
        AccountMeta::new(*pump.user_volume_accumulator.key, false),
        AccountMeta::new_readonly(*pump.fee_config.key, false),
        AccountMeta::new(*pump.fee_recipient.key, false),
        AccountMeta::new_readonly(*pump.event_authority.key, false),
        AccountMeta::new_readonly(*pump.program.key, false),
    ];

    let ix = Instruction {
        program_id: *pump.program.key,
        accounts,
        data: serialize_trade_data(amount_in, min_amount_out, &BUY_EXACT_IN_V2_DISCR),
    };

    let input_token_program = get_token_program(
        swap_ctx.input_mint,
        &shared.token_program,
        &shared.token_2022_program,
    )?;

    let output_token_program = get_token_program(
        swap_ctx.output_mint,
        &shared.token_program,
        &shared.token_2022_program,
    )?;

    let account_infos: Vec<AccountInfo<'info>> = [
        &pump.pool,
        &shared.user,
        &pump.global,
        &swap_ctx.output_mint,
        &swap_ctx.input_mint,
        &swap_ctx.output_token_account,
        &swap_ctx.input_token_account,
        &pump.base_vault,
        &pump.quote_vault,
        output_token_program,
        input_token_program,
        &shared.system_program,
        &pump.user_volume_accumulator,
        &pump.fee_config,
        &pump.fee_recipient,
        &pump.event_authority,
        &pump.program,
    ]
    .iter()
    .map(|account| (*account).clone())
    .collect();

    invoke(&ix, &account_infos)?;

    Ok(())
}

pub fn sell_v3<'a, 'info>(
    amount_in: u64,
    min_amount_out: u64,
    shared: &SharedAccounts<'a, 'info>,
    swap_ctx: &SwapContext<'a, 'info>,
    pump: &PumpFunAccounts<'a, 'info>,
) -> Result<(), ProgramError> {
    let accounts = vec![
        AccountMeta::new(*pump.pool.key, false),
        AccountMeta::new(*shared.user.key, true),
        AccountMeta::new_readonly(*pump.global.key, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.key, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.key, false),
        AccountMeta::new(*swap_ctx.input_token_account.key, false),
        AccountMeta::new(*swap_ctx.output_token_account.key, false),
        AccountMeta::new(*pump.base_vault.key, false),
        AccountMeta::new(*pump.quote_vault.key, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.owner, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.owner, false),
        AccountMeta::new_readonly(*shared.system_program.key, false),
        AccountMeta::new(*pump.user_volume_accumulator.key, false),
        AccountMeta::new_readonly(*pump.fee_config.key, false),
        AccountMeta::new(*pump.fee_recipient.key, false),
        AccountMeta::new_readonly(*pump.event_authority.key, false),
        AccountMeta::new_readonly(*pump.program.key, false),
    ];

    let ix = Instruction {
        program_id: *pump.program.key,
        accounts,
        data: serialize_trade_data(amount_in, min_amount_out, &SELL_V2_DISCR),
    };

    let input_token_program = get_token_program(
        swap_ctx.input_mint,
        &shared.token_program,
        &shared.token_2022_program,
    )?;

    let output_token_program = get_token_program(
        swap_ctx.output_mint,
        &shared.token_program,
        &shared.token_2022_program,
    )?;

    let account_infos: Vec<AccountInfo<'info>> = [
        &pump.global,
        &swap_ctx.input_mint,
        &swap_ctx.output_mint,
        input_token_program,
        output_token_program,
        &pump.pool,
        &pump.base_vault,
        &pump.quote_vault,
        &shared.user,
        &swap_ctx.input_token_account,
        &swap_ctx.output_token_account,
        &pump.user_volume_accumulator,
        &pump.fee_config,
        &pump.fee_recipient,
        &shared.system_program,
        &pump.event_authority,
        &pump.program,
    ]
    .iter()
    .map(|account| (*account).clone())
    .collect();

    invoke(&ix, &account_infos)?;

    Ok(())
}

fn serialize_trade_data(amount_in: u64, min_amount_out: u64, discriminator: &[u8; 8]) -> Vec<u8> {
    let mut data = Vec::with_capacity(24);

    data.extend_from_slice(discriminator);
    data.extend_from_slice(&amount_in.to_le_bytes());
    data.extend_from_slice(&min_amount_out.to_le_bytes());

    data
}
