use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::accounts::cursor::AccountCursor;

pub struct WhirlpoolAccounts<'a, 'info> {
    pub whirlpool: &'a AccountInfo<'info>,
    pub vault_a: &'a AccountInfo<'info>,
    pub vault_b: &'a AccountInfo<'info>,
    pub tick_array_0: &'a AccountInfo<'info>,
    pub tick_array_1: &'a AccountInfo<'info>,
    pub tick_array_2: &'a AccountInfo<'info>,
    pub oracle: &'a AccountInfo<'info>,
}

impl<'a, 'info> WhirlpoolAccounts<'a, 'info> {
    pub const ACCOUNT_COUNT: usize = 7;

    pub fn parse(cursor: &mut AccountCursor<'a, 'info>) -> Result<Self, ProgramError> {
        Ok(Self {
            whirlpool: cursor.next()?,
            vault_a: cursor.next()?,
            vault_b: cursor.next()?,
            tick_array_0: cursor.next()?,
            tick_array_1: cursor.next()?,
            tick_array_2: cursor.next()?,
            oracle: cursor.next()?,
        })
    }
}
