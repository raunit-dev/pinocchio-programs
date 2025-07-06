use crate::state::Escrow;
use core::mem::size_of;
use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
    error::PinocchioError,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    system_instruction,
};
use pinocchio_associated_token_account::{create as create_associated_token_account, get_associated_token_address};
use pinocchio_token::{
    self,
    state::{Account as TokenAccount, Mint},
};

pub struct MakeAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> MakeAccounts<'a> {
    pub fn new(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        let [maker, escrow, mint_a, mint_b, maker_ata_a, vault, system_program, token_program, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !maker.is_signer() {
            return Err(PinocchioError::NotSigner.into());
        }

        Ok(Self {
            maker,
            escrow,
            mint_a,
            mint_b,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}

pub struct MakeInstructionData {
    pub seed: u64,
    pub receive: u64,
    pub amount: u64,
}

impl MakeInstructionData {
    pub fn new(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() != size_of::<u64>() * 3 {
            return Err(ProgramError::InvalidInstructionData);
        }

        let seed = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let receive = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let amount = u64::from_le_bytes(data[16..24].try_into().unwrap());

        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self {
            seed,
            receive,
            amount,
        })
    }
}

pub struct Make<'a> {
    pub accounts: MakeAccounts<'a>,
    pub instruction_data: MakeInstructionData,
    pub bump: u8,
}

impl<'a> Make<'a> {
    pub const DISCRIMINATOR: u8 = 0;

    pub fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, ProgramError> {
        let accounts = MakeAccounts::new(accounts)?;
        let instruction_data = MakeInstructionData::new(data)?;

        let (escrow_key, bump) = find_program_address(
            &[
                b"escrow",
                accounts.maker.key.as_ref(),
                &instruction_data.seed.to_le_bytes(),
            ],
            &crate::id(),
        );

        if escrow_key.ne(accounts.escrow.key) {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(Self {
            accounts,
            instruction_data,
            bump,
        })
    }

    pub fn process(&mut self) -> ProgramResult {
        let seed_binding = self.instruction_data.seed.to_le_bytes();
        let bump_binding = [self.bump];
        let escrow_seeds = [
            b"escrow".as_ref(),
            self.accounts.maker.key.as_ref(),
            seed_binding.as_ref(),
            bump_binding.as_ref(),
        ];

        invoke_signed(
            &system_instruction::create_account(
                self.accounts.maker.key,
                self.accounts.escrow.key,
                1, // lamports
                Escrow::LEN as u64,
                &crate::id(),
            ),
            &[self.accounts.maker.clone(), self.accounts.escrow.clone()],
            &[&escrow_seeds],
        )?;

        create_associated_token_account(
            self.accounts.maker.clone(),
            self.accounts.escrow.clone(),
            self.accounts.mint_a.clone(),
        )?;

        let mut data = self.accounts.escrow.try_borrow_mut_data()?;
        let escrow = Escrow::load_mut(&mut data)?;

        escrow.set_inner(
            self.instruction_data.seed,
            *self.accounts.maker.key,
            *self.accounts.mint_a.key,
            *self.accounts.mint_b.key,
            self.instruction_data.receive,
            self.bump,
        );

        pinocchio_token::instruction::transfer(
            self.accounts.token_program.key,
            self.accounts.maker_ata_a.key,
            self.accounts.vault.key,
            self.accounts.maker.key,
            &[],
            self.instruction_data.amount,
        )?;

        Ok(())
    }
}

