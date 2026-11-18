use solana_program::{account_info::AccountInfo, program_error::ProgramError};

use crate::error::DexSwapError;

pub struct AccountCursor<'a, 'info> {
    accounts: &'a [AccountInfo<'info>],
    position: usize,
}

impl<'a, 'info> AccountCursor<'a, 'info> {
    pub fn new(accounts: &'a [AccountInfo<'info>]) -> Self {
        Self {
            accounts,
            position: 0,
        }
    }

    pub fn next(&mut self) -> Result<&'a AccountInfo<'info>, ProgramError> {
        let account = self
            .accounts
            .get(self.position)
            .ok_or(ProgramError::NotEnoughAccountKeys)?;

        self.position += 1;

        Ok(account)
    }

    pub fn take(&mut self, count: usize) -> Result<&'a [AccountInfo<'info>], ProgramError> {
        let end = self
            .position
            .checked_add(count)
            .ok_or(DexSwapError::AccountParsingProblem)?;

        let result = self
            .accounts
            .get(self.position..end)
            .ok_or(ProgramError::NotEnoughAccountKeys)?;

        self.position = end;

        Ok(result)
    }

    pub fn is_finished(&self) -> bool {
        self.position == self.accounts.len()
    }
}
