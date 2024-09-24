use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::AccountInfo,
};

pub mod instruction;
use instruction::{IntroInstruction};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = IntroInstruction::unpack(instruction_data)?;
    match instruction {
        IntroInstruction::InitUserInput {name}  => {
              add_student_intro(program_id, accounts, name)
        }
    } 
}

pub fn add_student_intro(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  name: String,
) -> ProgramResult {
    msg!("Adding student name...");
    msg!("Name: {}", name);
    
    Ok(())

}