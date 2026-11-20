use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::accounts::cursor::AccountCursor;

pub struct PumpFunAccounts<'a, 'info> {
    pub pool: &'a AccountInfo<'info>,
    pub base_vault: &'a AccountInfo<'info>,
    pub quote_vault: &'a AccountInfo<'info>,
    pub user_volume_accumulator: &'a AccountInfo<'info>,
    pub fee_config: &'a AccountInfo<'info>,
    pub fee_recipient: &'a AccountInfo<'info>,
    pub event_authority: &'a AccountInfo<'info>,
    pub program: &'a AccountInfo<'info>,
}

impl<'a, 'info> PumpFunAccounts<'a, 'info> {
    pub const ACCOUNT_COUNT: usize = 8;

    pub fn parse(cursor: &mut AccountCursor<'a, 'info>) -> Result<Self, ProgramError> {
        Ok(Self {
            pool: cursor.next()?,
            base_vault: cursor.next()?,
            quote_vault: cursor.next()?,
            user_volume_accumulator: cursor.next()?,
            fee_config: cursor.next()?,
            fee_recipient: cursor.next()?,
            event_authority: cursor.next()?,
            program: cursor.next()?,
        })
    }
}
