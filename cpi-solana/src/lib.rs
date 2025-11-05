use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError
};

#[derive(BorshDeserialize, BorshSerialize)]
struct OnChainData {
    count: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Contract Started");

    let mut account_itr = accounts.iter();
    let data_account = next_account_info(&mut account_itr)?;

    // The account must be owned by the program in order to modify its data
    let data_account_owner = data_account.owner;

    if data_account_owner != program_id {
      msg!("Account does not have the correct program id");
      return Err(ProgramError::IncorrectProgramId);
    }

    let data_bytes = data_account.data.borrow(); // [u8]

    let mut counter = OnChainData::try_from_slice(&data_bytes)?; 

    if counter.count == 0 {
        counter.count = 1
    } else {
        counter.count = counter.count * 2; // double
    }

    counter.serialize(&mut *data_account.data.borrow_mut())?;

    msg!("Counter updated to {}", counter.count);

    Ok(())
}
