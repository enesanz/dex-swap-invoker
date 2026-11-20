use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::accounts::cursor::AccountCursor;

pub struct SharedAccounts<'a, 'info> {
    pub user: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub token_2022_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub memo_v2_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> SharedAccounts<'a, 'info> {
    pub const ACCOUNT_COUNT: usize = 5;

    pub fn parse(cursor: &mut AccountCursor<'a, 'info>) -> Result<Self, ProgramError> {
        Ok(Self {
            user: cursor.next()?,
            token_program: cursor.next()?,
            token_2022_program: cursor.next()?,
            system_program: cursor.next()?,
            memo_v2_program: cursor.next()?,
        })
    }
}
