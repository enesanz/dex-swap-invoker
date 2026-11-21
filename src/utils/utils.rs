use solana_program::{account_info::AccountInfo, program_error::ProgramError};

pub fn get_token_program<'a, 'info>(
    mint: &'a AccountInfo<'info>,
    token_program: &'a AccountInfo<'info>,
    token_2022_program: &'a AccountInfo<'info>,
) -> Result<&'a AccountInfo<'info>, ProgramError> {
    if mint.owner == token_program.key {
        Ok(token_program)
    } else if mint.owner == token_2022_program.key {
        Ok(token_2022_program)
    } else {
        Err(ProgramError::IncorrectProgramId)
    }
}
