use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_log::log;

use crate::instructions::{
    transfer_sol_with_cpi::TransferSolWithCpi, transfer_sol_with_program::TransferSolWithProgram,
    Instruction,
};

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match Instruction::try_from(discriminator)? {
        Instruction::TransferSolWithProgram => {
            log!("Instruction: TransferSolWithProgram");
            TransferSolWithProgram::try_from((accounts, data))?.handler()
        }
        Instruction::TransferSolWithCpi => {
            log!("Instruction: TransferSolWithCpi");
            TransferSolWithCpi::try_from((accounts, data))?.handler()
        }
    }
}