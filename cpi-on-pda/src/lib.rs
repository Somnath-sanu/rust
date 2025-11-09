use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
};

use solana_program::system_program;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // create a new PDA onchain
    // pda, userAcc, systemProgram

    let mut itr = accounts.iter();
    // order matters

    let pda = next_account_info(&mut itr)?;
    let user_acc = next_account_info(&mut itr)?;
    let system_program = next_account_info(&mut itr)?;

    // Basic checks
    if !user_acc.is_signer {
        msg!("Payer must sign"); // msg!() -> On-chain message
        return Err(ProgramError::MissingRequiredSignature);
    }
    if !user_acc.is_writable || !pda.is_writable {
        msg!("Payer and PDA must be writable");
        return Err(ProgramError::InvalidAccountData);
    }
    if *system_program.key != system_program::ID {
        msg!("Incorrect System Program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let seeds = &[user_acc.key.as_ref(), b"user"];

    let (pda_pubkey, bump) = Pubkey::find_program_address(seeds, program_id);

    if pda_pubkey != *pda.key {
        msg!("PDA mismatch");
        return Err(ProgramError::InvalidSeeds);
    }

    if !pda.data_is_empty() {
        // pda account already exists
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // let space: usize = 8;
    // let rent_lamports = Rent::get()?.minimum_balance(space);

    // Build system ix
    let ix = create_account(user_acc.key, pda.key, 1000000000, 8, program_id);

    invoke_signed(&ix, accounts, &[&[user_acc.key.as_ref(), b"user", &[bump]]])?;

    Ok(())
}

/*
*invoke_signed is the key to let PDAs (which can’t hold a private key) “sign” using seeds+bump.

*create_account builds the instruction that the System Program will execute to create & assign the PDA account to this program.

You expect exactly three accounts in this order:

1. pda – the to-be-created PDA account (must be writable, not signer )

2. user_acc – the payer (must be writable and a signer)

3. system_program – must be the well-known System Program id 11111111111111111111111111111111


*Q: Why does create_account need the PDA to “sign”? It doesn’t exist yet.

*The System Program requires the new account to sign the creation instruction. For a PDA, there’s no keypair, so you provide seeds+bump through invoke_signed. The runtime validates they produce the target PDA and treats it as signed.
*/
