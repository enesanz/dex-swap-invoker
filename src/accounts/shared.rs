use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub struct SharedAccounts<'a, 'info> {
    pub user: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub token_program_2022: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub memo_v2_program: &'a AccountInfo<'info>,
}

impl<'a, 'info> SharedAccounts<'a, 'info> {
    pub const ACCOUNT_COUNT: usize = 6;

    // pub fn parse() -> Result<Self, ProgramError> {

    // }
}
