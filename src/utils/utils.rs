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

pub fn read_token_amount(account_info: &AccountInfo) -> Result<u64, ProgramError> {
    let data = account_info.try_borrow_data()?;

    let amount_bytes = data.get(64..72).ok_or(ProgramError::InvalidAccountData)?;

    let amount = u64::from_le_bytes(
        amount_bytes
            .try_into()
            .map_err(|_| ProgramError::InvalidAccountData)?,
    );

    Ok(amount)
}
