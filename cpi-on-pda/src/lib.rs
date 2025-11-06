use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::create_account,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // create a new PDA onchain
    // pda, userAcc, systemProgram

    let mut itr = accounts.iter();

    let pda = next_account_info(&mut itr)?;
    let user_acc = next_account_info(&mut itr)?;
    let system_program = next_account_info(&mut itr)?;

    let ix = create_account(user_acc.key, pda.key, 1000000000, 8, program_id);

    let seeds = &[user_acc.key.as_ref(), b"user"];

    let (pda_pubkey, bump) = Pubkey::find_program_address(seeds, program_id);

    invoke_signed(&ix, accounts, &[&[user_acc.key.as_ref(), b"user", &[bump]]])?;

    Ok(())
}
