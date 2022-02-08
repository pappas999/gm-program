use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    pub name: String,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the GM program was loaded into
    accounts: &[AccountInfo], // The account to say GM to
    input: &[u8], // String input data, contains the name to say GM to
) -> ProgramResult {
    msg!("GM program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say GM to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Deserialize the input data, and store it in a GreetingAccout struct
    let input_data = GreetingAccount::try_from_slice(&input).unwrap();

    //Say GM in the Program output
    msg!("GM {}", input_data.name);

    //Serialize the name, and store it in the passed in account
    input_data.serialize(&mut &mut account.try_borrow_mut_data()?[..])?;

    Ok(())
}

