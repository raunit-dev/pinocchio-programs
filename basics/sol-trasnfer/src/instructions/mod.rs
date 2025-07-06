pub mod shared;
pub mod transfer_sol_with_cpi;
pub mod transfer_sol_with_program;
use pinocchio::program_error::ProgramError;

#[repr(u8)]
pub enum Instruction {
    TransferSolWithProgram,
    TransferSolWithCpi,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
           0 => Ok(Instruction::TransferSolWithProgram),
           1 => Ok(Instruction::TransferSolWithCpi),
           _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}


//    * Instruction Enum and Routing: This file defines an enum Instruction that acts as a dispatcher for different types of instructions your program can handle.
//    * #[repr(u8)]: Ensures that the enum variants are represented by a single u8 byte, which is crucial for using the first byte of instruction data as a discriminator.
//    * impl TryFrom<&u8> for Instruction: Implements a conversion from a u8 byte to an Instruction enum variant. This is how the processor.rs determines which instruction was requested based on the
//      incoming instruction data.
//        * 0: Maps to TransferSolWithProgram.
//        * 1: Maps to TransferSolWithCpi.
//        * Any other value results in ProgramError::InvalidInstructionData.
//    * It also declares the modules within the instructions directory: shared, transfer_sol_with_cpi, and transfer_sol_with_program.