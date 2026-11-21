use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::{accounts::cursor::AccountCursor, error::DexSwapError};

pub struct RouteAccounts<'a, 'info> {
    pub mints: &'a [AccountInfo<'info>],
    pub user_token_accounts: &'a [AccountInfo<'info>],
}

impl<'a, 'info> RouteAccounts<'a, 'info> {
    pub fn parse(
        cursor: &mut AccountCursor<'a, 'info>,
        mint_count: usize,
    ) -> Result<Self, ProgramError> {
        Ok(Self {
            mints: cursor.take(mint_count)?,
            user_token_accounts: cursor.take(mint_count)?,
        })
    }

    pub fn mint(&self, index: usize) -> Result<&'a AccountInfo<'info>, ProgramError> {
        self.mints
            .get(index)
            .ok_or(DexSwapError::RouteAccountOutOfBound.into())
    }

    pub fn user_token_account(&self, index: usize) -> Result<&'a AccountInfo<'info>, ProgramError> {
        self.user_token_accounts
            .get(index)
            .ok_or(DexSwapError::RouteAccountOutOfBound.into())
    }
}
