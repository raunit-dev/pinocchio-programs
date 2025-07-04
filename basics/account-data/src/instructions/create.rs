use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use bytemuck::{Pod, Zeroable};
use crate::state::AddressInfo;


pub struct CreateAddressInfoAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub address_info: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateAddressInfoAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, address_info, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // check payer is signer
        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        // check address_info is writable
        if !address_info.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !address_info.is_writable() {
            return Err(ProgramError::InvalidAccountData)
        }
        // check address_info is not already initialized
        if address_info.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        Ok(Self {
            payer,
            address_info,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateAddressInfoInstructionData { 
    pub name: [u8; 50],
    pub house_number: u8,
    pub street: [u8; 50],
    pub city: [u8; 50],
}

// impl CreateAddressInfoInstructionData {
//     pub const LEN: usize = core::mem::size_of::<CreateAddressInfoInstructionData>();
// }

impl<'info> TryFrom<&'info [u8]> for CreateAddressInfoInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}



pub struct Create<'info> {
    pub accounts: CreateAddressInfoAccounts<'info>,
    pub instruction_datas: CreateAddressInfoInstructionData
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])>  for Create<'info> {
    type Error = ProgramError;


    //Account Validation Starts first
    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateAddressInfoAccounts::try_from(accounts)?;
        let instruction_datas = CreateAddressInfoInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_datas,
        })
    }
}

impl<'info> Create<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        //Create address info account
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.address_info,
            space: AddressInfo::LEN as u64,
            lamports: Rent::get()?.minimum_balance(AddressInfo::LEN),
            owner: &crate::ID,
        }
        .invoke()?;

    //write data to address info account 
    let address_info_state = unsafe {
        bytemuck::try_from_bytes_mut::<AddressInfo>(
            self.accounts.address_info.borrow_mut_data_unchecked(),
        )
        .map_err(|_| ProgramError::InvalidAccountData)?
    };
    //No, you can’t just use set_inner without first getting a mutable reference 
    //to the deserialized data in the account’s raw byte buffer. 
    //The let address_info_state = ... line is getting that reference.

    address_info_state.set_inner(AddressInfo {
        name: self.instruction_datas.name,
        house_number: self.instruction_datas.house_number,
        street: self.instruction_datas.street,
        city: self.instruction_datas.city,
    });
    Ok(())

    }
}