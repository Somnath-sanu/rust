use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize)]

/**
 * #[derive(BorshSerialize,BorshDeserialize)]
 * This line add some code (imp) below InstructionType like try_from_slice
 */

enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8], // 1 0 0 1 23 21
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    // "?" means if account return account if not return error, dont panic ; the `?` operator can only be used in a function that returns `Result` or `Option`

    //borrow means read the data
    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?;
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            counter_data.count -= value;
        }
    }

    // borrow_mut means I can read and modify as well
    counter_data.serialize(&mut *acc.data.borrow_mut())?; // "*" -> dereference

    msg!("Counter updated to {}", counter_data.count);

    Ok(())
}
