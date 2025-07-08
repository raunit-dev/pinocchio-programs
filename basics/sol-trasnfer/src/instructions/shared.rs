use core::mem::transmute;
// use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};


// What is Zero-Copy?
// Zero-copy deserialization means no new memory allocation or data copying happens when you turn raw bytes into a typed struct.
// Instead, the program interprets the existing bytes in-place as the struct.

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

        //whenever you feel like confused see these https://chatgpt.com/share/686d3d61-81e4-8012-ac1b-3fbff1a9106a

//         Using bytemuck is basically:

// “Let a well-tested crate write the unsafe for you, and make your assumptions explicit via traits like Pod.”

// Whereas transmute is:

// “Trust me, I know this struct is safe. Fingers crossed.”

        //Bytemuck is safe if your type implements Pod + Zeroable correctly (i.e., no pointers, no padding, no non-Copy types).
        // Your unsafe transmute is functionally similar to bytemuck, but you are assuming the same safety manually — any padding, misalignment, or 
        // incorrect size will result in UB (undefined behavior).
        // Official docs recommend bytemuck or similar methods for large, performance-critical types where you want to avoid Serde/Borsh-style allocations.
        // let result = bytemuck::try_from_bytes::<Self>(&data)
        //     .map_err(|_| ProgramError::InvalidInstructionData)?;

        // Ok(*result)
    }
}
