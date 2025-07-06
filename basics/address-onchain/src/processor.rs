use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::{Create, Instruction};
use pinocchio_log::log;

#[inline(always)]
pub fn process_instruction( // These function "process_instruction" is the instruction router 
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_datas: &[u8],
) -> ProgramResult {
    
    //Validate Program ID
    if _program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    //deserializes the instruction data to determine which instruction is being called (the rest part of the code not these line)
    let (discriminator, data) = instruction_datas
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match Instruction::try_from(discriminator)? {
        Instruction::Create => {
            log!("Instruction: Create");
            Create::try_from((accounts, data))?.handler()
        }
    }
}