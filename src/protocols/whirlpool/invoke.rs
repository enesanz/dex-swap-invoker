use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program_error::ProgramError,
};

use crate::{
    accounts::{route::SwapContext, shared::SharedAccounts},
    protocols::whirlpool::accounts::WhirlpoolAccounts,
    utils::utils::get_token_program,
};

const SWAP_V2_DISCR: [u8; 8] = [43, 4, 237, 11, 26, 201, 30, 98];

pub fn buy_swap_v2<'a, 'info>(
    amount_in: u64,
    min_amount_out: u64,
    shared: &SharedAccounts<'a, 'info>,
    swap_ctx: &SwapContext<'a, 'info>,
    orca: &WhirlpoolAccounts<'a, 'info>,
) -> Result<(), ProgramError> {
    let accounts = vec![
        AccountMeta::new_readonly(*swap_ctx.output_mint.owner, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.owner, false),
        AccountMeta::new_readonly(*shared.memo_v2_program.key, false),
        AccountMeta::new(*shared.user.key, true),
        AccountMeta::new(*orca.whirlpool.key, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.key, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.key, false),
        AccountMeta::new(*swap_ctx.output_token_account.key, false),
        AccountMeta::new(*orca.vault_a.key, false),
        AccountMeta::new(*swap_ctx.input_token_account.key, false),
        AccountMeta::new(*orca.vault_b.key, false),
        AccountMeta::new(*orca.tick_array_0.key, false),
        AccountMeta::new(*orca.tick_array_1.key, false),
        AccountMeta::new(*orca.tick_array_2.key, false),
        AccountMeta::new(*orca.oracle.key, false),
    ];

    let ix = Instruction {
        program_id: *orca.program.key,
        accounts,
        data: serialize_whirlpool_swap_data(amount_in, min_amount_out, 0, true, true),
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
        output_token_program,
        input_token_program,
        &shared.memo_v2_program,
        &shared.user,
        &orca.whirlpool,
        &swap_ctx.output_mint,
        &swap_ctx.input_mint,
        &swap_ctx.output_token_account,
        &orca.vault_a,
        &swap_ctx.input_token_account,
        &orca.vault_b,
        &orca.tick_array_0,
        &orca.tick_array_1,
        &orca.tick_array_2,
        &orca.oracle,
    ]
    .iter()
    .map(|account| (*account).clone())
    .collect();

    invoke(&ix, &account_infos)?;

    Ok(())
}

pub fn sell_swap_v2<'a, 'info>(
    amount_in: u64,
    min_amount_out: u64,
    shared: &SharedAccounts<'a, 'info>,
    swap_ctx: &SwapContext<'a, 'info>,
    orca: &WhirlpoolAccounts<'a, 'info>,
) -> Result<(), ProgramError> {
    let accounts = vec![
        AccountMeta::new_readonly(*swap_ctx.input_mint.owner, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.owner, false),
        AccountMeta::new_readonly(*shared.memo_v2_program.key, false),
        AccountMeta::new(*shared.user.key, true),
        AccountMeta::new(*orca.whirlpool.key, false),
        AccountMeta::new_readonly(*swap_ctx.input_mint.key, false),
        AccountMeta::new_readonly(*swap_ctx.output_mint.key, false),
        AccountMeta::new(*swap_ctx.input_token_account.key, false),
        AccountMeta::new(*orca.vault_a.key, false),
        AccountMeta::new(*swap_ctx.output_token_account.key, false),
        AccountMeta::new(*orca.vault_b.key, false),
        AccountMeta::new(*orca.tick_array_0.key, false),
        AccountMeta::new(*orca.tick_array_1.key, false),
        AccountMeta::new(*orca.tick_array_2.key, false),
        AccountMeta::new(*orca.oracle.key, false),
    ];

    let ix = Instruction {
        program_id: *orca.program.key,
        accounts,
        data: serialize_whirlpool_swap_data(amount_in, min_amount_out, 0, true, true),
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
        input_token_program,
        output_token_program,
        &shared.memo_v2_program,
        &shared.user,
        &orca.whirlpool,
        &swap_ctx.input_mint,
        &swap_ctx.output_mint,
        &swap_ctx.input_token_account,
        &orca.vault_a,
        &swap_ctx.output_token_account,
        &orca.vault_b,
        &orca.tick_array_0,
        &orca.tick_array_1,
        &orca.tick_array_2,
        &orca.oracle,
    ]
    .iter()
    .map(|account| (*account).clone())
    .collect();

    invoke(&ix, &account_infos)?;

    Ok(())
}

fn serialize_whirlpool_swap_data(
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
    amount_specified_is_input: bool,
    a_to_b: bool,
) -> Vec<u8> {
    let mut data = Vec::with_capacity(43);

    data.extend_from_slice(&SWAP_V2_DISCR);
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&other_amount_threshold.to_le_bytes());
    data.extend_from_slice(&sqrt_price_limit.to_le_bytes());
    data.push(amount_specified_is_input as u8);
    data.push(a_to_b as u8);
    data.push(0);

    data
}
