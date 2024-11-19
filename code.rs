// Solana Program Structure
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

// Accounts and Data Management
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program_error::ProgramError,
};

pub fn handle_accounts(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.is_writable {
        account.data.borrow_mut()[0] = 42; // Example: writing data to account
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

// Transactions and Instructions
use solana_program::{
    instruction::{AccountMeta, Instruction},
    system_program,
};

pub fn create_instruction(
    program_id: Pubkey,
    account_pubkey: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(account_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![0], // Custom instruction data
    }
}

// Custom Error Handling
use solana_program::{
    decode_error::DecodeError,
    program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CustomError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Data Serialization with Borsh
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MyData {
    pub field1: u32,
    pub field2: String,
}

pub fn serialize_example() -> Vec<u8> {
    let data = MyData {
        field1: 42,
        field2: "Hello, Solana!".to_string(),
    };
    data.try_to_vec().unwrap()
}

pub fn deserialize_example(bytes: &[u8]) -> MyData {
    MyData::try_from_slice(bytes).unwrap()
}

// Cross-Program Invocation (CPI)
use solana_program::{
    program::invoke,
    system_instruction,
};

pub fn transfer_sol(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
) -> ProgramResult {
    let ix = system_instruction::transfer(from.key, to.key, lamports);
    invoke(&ix, &[from.clone(), to.clone()])
}
