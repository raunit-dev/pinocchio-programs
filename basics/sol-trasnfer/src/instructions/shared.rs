use core::mem::transmute;

use pinocchio::{account_info::AccountInfo, program_error::ProgramError};


//    * Common Data Structures: This file defines data structures that are shared between different instruction handlers.
//    * TransferSolAccounts<'info>: A struct to hold references to the payer and recipient AccountInfo objects.
//        * impl TryFrom<&'info [AccountInfo]> for TransferSolAccounts<'info>: This implementation handles the parsing and validation of the accounts passed into the instruction. It ensures:
//            * There are enough accounts provided.
//            * The payer account is a signer.
//            * The recipient account is writable.
//            * The recipient account is owned by the System Program (pinocchio_system::ID), which is necessary for SOL transfers.
//    * TransferSolInstructionData: A struct to hold the amount of lamports to transfer.
//        * #[repr(C)]: Ensures a C-compatible memory layout, which is important for safely transmuting raw bytes into this struct.
//        * impl TryFrom<&'info [u8]> for TransferSolInstructionData: Implements conversion from a byte slice to this struct, allowing the program to easily extract the transfer amount from the
//          instruction data.

pub struct TransferSolAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub recipient: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for TransferSolAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, recipient, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !recipient.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if !recipient.is_owned_by(&pinocchio_system::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { payer, recipient })
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TransferSolInstructionData {
    pub amount: u64,
}

impl TransferSolInstructionData {
    pub const LEN: usize = core::mem::size_of::<TransferSolInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for TransferSolInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        Ok(unsafe {
            transmute(
                TryInto::<[u8; size_of::<TransferSolInstructionData>()]>::try_into(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?,
            )
        })
    }
}
