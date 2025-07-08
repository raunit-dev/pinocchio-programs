use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::{Create, Instruction};
use pinocchio_log::log;

#[inline(always)]
pub fn process_instruction(
    // These function "process_instruction" is the instruction router
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_datas: &[u8],
) -> ProgramResult {
    // Validate Program ID
    // We dereference `_program_id` to get the `Pubkey` value.
    // The `Pubkey` type implements the `Copy` trait, so `crate::ID` is copied,
    // not moved. This means we can use `crate::ID` elsewhere (e.g., in tests)
    // without ownership issues.
    if *_program_id != crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    //deserializes the instruction data to determine which instruction is being called (the rest part of the code not these line)
    let (discriminator, data) = instruction_datas
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match Instruction::try_from(discriminator)? {
        Instruction::Create => {
            log!("Instruction: Create");
            //    Below is a chain of operations that does three main things:
            //    1. Parses and Validates the instruction inputs.
            //    2. Handles potential errors during parsing.
            //    3. Executes the instruction's business logic.
            Create::try_from((accounts, data))?.handler()
        }
    }
}
