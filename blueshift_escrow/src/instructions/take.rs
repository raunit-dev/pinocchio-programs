use crate::state::Escrow;
use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
    error::PinocchioError,
    program_error::ProgramError,
    pubkey::{create_program_address, Pubkey},
};
use pinocchio_associated_token_account::create as create_associated_token_account;
use pinocchio_token::{
    self,
    instruction::close_account,
    state::Account as TokenAccount,
};

pub struct TakeAccounts<'a> {
    pub taker: &'a AccountInfo,
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub mint_b: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub taker_ata_a: &'a AccountInfo,
    pub taker_ata_b: &'a AccountInfo,
    pub maker_ata_b: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

impl<'a> TakeAccounts<'a> {
    pub fn new(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        let [taker, maker, escrow, mint_a, mint_b, vault, taker_ata_a, taker_ata_b, maker_ata_b, system_program, token_program, ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !taker.is_signer() {
            return Err(PinocchioError::NotSigner.into());
        }

        Ok(Self {
            taker,
            maker,
            escrow,
            mint_a,
            mint_b,
            taker_ata_a,
            taker_ata_b,
            maker_ata_b,
            vault,
            system_program,
            token_program,
        })
    }
}

pub struct Take<'a> {
    pub accounts: TakeAccounts<'a>,
}

impl<'a> Take<'a> {
    pub const DISCRIMINATOR: u8 = 1;

    pub fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, ProgramError> {
        let accounts = TakeAccounts::new(accounts)?;

        create_associated_token_account(
            accounts.taker.clone(),
            accounts.taker.clone(),
            accounts.mint_a.clone(),
        )?;

        create_associated_token_account(
            accounts.taker.clone(),
            accounts.maker.clone(),
            accounts.mint_b.clone(),
        )?;

        Ok(Self {
            accounts,
        })
    }

    pub fn process(&mut self) -> ProgramResult {
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;

        let escrow_key = create_program_address(
            &[
                b"escrow",
                self.accounts.maker.key.as_ref(),
                &escrow.seed.to_le_bytes(),
                &[escrow.bump],
            ],
            &crate::id(),
        )?;
        if &escrow_key != self.accounts.escrow.key {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let vault_data = self.accounts.vault.try_borrow_data()?;
        let vault = TokenAccount::unpack(&vault_data)?;

        pinocchio_token::instruction::transfer(
            self.accounts.token_program.key,
            self.accounts.vault.key,
            self.accounts.taker_ata_a.key,
            self.accounts.escrow.key,
            &[],
            vault.amount,
        )?;

        close_account(
            self.accounts.token_program.key,
            self.accounts.vault.key,
            self.accounts.maker.key,
            self.accounts.escrow.key,
            &[],
        )?;

        pinocchio_token::instruction::transfer(
            self.accounts.token_program.key,
            self.accounts.taker_ata_b.key,
            self.accounts.maker_ata_b.key,
            self.accounts.taker.key,
            &[],
            escrow.receive,
        )?;

        self.accounts.escrow.realloc(0, false)?;
        self.accounts
            .escrow
            .assign(&pinocchio_system::ID);

        Ok(())
    }
}
