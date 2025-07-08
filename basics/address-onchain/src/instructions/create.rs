use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use bytemuck::{Pod, Zeroable};
use crate::state::AddressInfo;

// A struct to hold the accounts required by the `CreateAddressInfo` instruction.
// This provides a layer of validation and abstraction over the raw `accounts` slice.
pub struct CreateAddressInfoAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub address_info: &'info AccountInfo,
}

// This implementation allows us to safely convert the raw `accounts` slice
// into our structured `CreateAddressInfoAccounts` type.
impl<'info> TryFrom<&'info [AccountInfo]> for CreateAddressInfoAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        // This is a destructuring assignment. It checks if `accounts` has at least 3 elements.
        // If not, it returns `NotEnoughAccountKeys`, ensuring we have the accounts we expect.
        let [payer, address_info, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // --- Account Validation ---
        // 1. The `payer` must sign the transaction to authorize account creation and rent payment.
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // 2. The `address_info` account must also be a signer, as we are creating it.
        if !address_info.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // 3. The `address_info` account must be writable so we can store data in it.
        if !address_info.is_writable() {
            return Err(ProgramError::InvalidAccountData)
        }
        // 4. We check that the account is not already initialized by ensuring its data length is 0.
        if address_info.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // If all checks pass, return the structured accounts.
        Ok(Self {
            payer,
            address_info,
        })
    }
}

// This struct defines the expected layout of the instruction's data buffer.
// `#[repr(C)]` and `#[derive(Pod, Zeroable)]` are crucial for safe, zero-copy deserialization.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateAddressInfoInstructionData {
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

// This implementation handles the deserialization of the raw instruction byte slice (`data`)
// into our structured `CreateAddressInfoInstructionData` type.
impl<'info> TryFrom<&'info [u8]> for CreateAddressInfoInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        // `bytemuck::try_from_bytes` attempts to safely interpret the byte slice as our struct.
        // It succeeds only if the slice's length exactly matches the struct's size.
        // This is a zero-cost abstraction; no data is actually copied here.
        let result = bytemuck::try_from_bytes::<Self>(&data)
            // If `try_from_bytes` fails, we map its error to our standard instruction data error.
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        // The `?` operator propagates the error if the above failed.
        // If it succeeded, `result` is a `&CreateAddressInfoInstructionData`.
        // We dereference (`*`) it to create a full copy, since the struct implements `Copy`.
        Ok(*result)
    }
}


// This is the main instruction struct, combining the validated accounts and deserialized data.
pub struct Create<'info> {
    pub accounts: CreateAddressInfoAccounts<'info>,
    pub instruction_datas: CreateAddressInfoInstructionData
}

// This final `TryFrom` brings everything together. It takes the raw inputs from the
// Solana runtime and orchestrates the validation and deserialization.
impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])>  for Create<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        // 1. First, validate the accounts. The `?` will propagate any errors.
        let accounts = CreateAddressInfoAccounts::try_from(accounts)?;
        // 2. Second, deserialize the instruction data. The `?` will propagate any errors.
        let instruction_datas = CreateAddressInfoInstructionData::try_from(data)?;

        // If both steps succeed, create the final `Create` instruction object.
        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> Create<'info> {
    // The `handler` contains the core business logic of the instruction.
    // It is only called if all validation and deserialization has succeeded.
    pub fn handler(&mut self) -> ProgramResult {
        // Use the `pinocchio_system` crate to create the new account on-chain.
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.address_info,
            space: AddressInfo::LEN as u64, // Allocate space based on our state struct's size.
            lamports: Rent::get()?.minimum_balance(AddressInfo::LEN), // Calculate rent.
            owner: &crate::ID, // Set the program itself as the owner of the new account.
        }
        .invoke()?; // Invoke the cross-program instruction.

        // Now, write the data into the newly created account.
        // `borrow_mut_data_unchecked` gives us a mutable reference to the account's raw byte buffer.
        // `unsafe` is required here because we are responsible for ensuring correct byte manipulation.
        let address_info_state = unsafe {
            // `bytemuck::try_from_bytes_mut` safely casts the raw byte slice into a mutable reference
            // to our `AddressInfo` state struct. This is another zero-cost abstraction.
            bytemuck::try_from_bytes_mut::<AddressInfo>(
                self.accounts.address_info.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        // With a mutable reference to the on-chain state, we can now update its fields.
        address_info_state.set_inner(AddressInfo {
            name: self.instruction_datas.name,
            house_number: self.instruction_datas.house_number,
            street: self.instruction_datas.street,
            city: self.instruction_datas.city,
        });

        Ok(())
    }
}
