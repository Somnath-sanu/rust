use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("This is a middle Contract. Which also take data account");

    let mut itr = accounts.iter();
    let data_account = next_account_info(&mut itr)?;
    let double_contract_address = next_account_info(&mut itr)?;

    if !double_contract_address.executable {
        msg!("Provided program is not executable");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("double_contract_address {:?}", double_contract_address);

    let instruction = Instruction {
        program_id: *double_contract_address.key,
        accounts: vec![AccountMeta {
            is_signer: false,
            is_writable: true,
            pubkey: *data_account.key, // deference means I passed the ownership
        }],
        data: vec![],
    };

    // so here I again cann't use &[*data_account] "*" -> deference so we have to clone it
    invoke(
        &instruction,
        &[data_account.clone(), double_contract_address.clone()],
    )?;

    Ok(())
}
