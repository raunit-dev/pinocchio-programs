use crate::state::Escrow;
use pinocchio::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    error::PinocchioError,
    instruction::{Seed, Signer},
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::{create_program_address, find_program_address, Pubkey},
};
use pinocchio_token::{
    self,
    instruction::close_account,
    state::{Account as TokenAccount, Mint},
    ID as TOKEN_PROGRAM_ID,
};

// NOTE: This is a placeholder for the actual Token-2022 program ID.
// The official one should be used when it's available in the target environment.
const TOKEN_2022_PROGRAM_ID: Pubkey = Pubkey::new_from_array([
    0x06, 0x5a, 0x52, 0x2c, 0x2b, 0x24, 0x67, 0xef, 0x17, 0x78, 0x75, 0x68, 0x4c, 0x48, 0x82, 0x53,
    0x4a, 0xdd, 0x47, 0xb3, 0xe1, 0x39, 0xac, 0x7c, 0x69, 0x83, 0x44, 0x4d, 0x84, 0x84, 0x86, 0x58,
]);
const TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET: usize = 0;
const TOKEN_2022_MINT_DISCRIMINATOR: u8 = 2;

pub struct RefundAccounts<'a> {
    pub maker: &'a AccountInfo,
    pub escrow: &'a AccountInfo,
    pub mint_a: &'a AccountInfo,
    pub maker_ata_a: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}

trait AccountCheck {
    fn check(&self) -> ProgramResult;
}

impl<'a> TryFrom<&'a [AccountInfo]> for RefundAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, escrow, mint_a, maker_ata_a, vault, system_program, token_program, ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        struct SignerAccount<'a>(&'a AccountInfo);

        impl<'a> AccountCheck for SignerAccount<'a> {
            fn check(&self) -> ProgramResult {
                if !self.0.is_signer() {
                    return Err(PinocchioError::NotSigner.into());
                }
                Ok(())
            }
        }

        struct MintInterface<'a>(&'a AccountInfo);

        impl<'a> AccountCheck for MintInterface<'a> {
            fn check(&self) -> ProgramResult {
                if !self.0.is_owned_by(&TOKEN_2022_PROGRAM_ID) {
                    if !self.0.is_owned_by(&pinocchio_token::ID) {
                        return Err(PinocchioError::InvalidOwner.into());
                    } else {
                        if self.0.data_len() != Mint::LEN {
                            return Err(PinocchioError::InvalidAccountData.into());
                        }
                    }
                } else {
                    let data = self.0.try_borrow_data()?;

                    if data.len() != Mint::LEN {
                        if data[TOKEN_2022_ACCOUNT_DISCRIMINATOR_OFFSET]
                            != TOKEN_2022_MINT_DISCRIMINATOR
                        {
                            return Err(PinocchioError::InvalidAccountData.into());
                        }
                    }
                }

                Ok(())
            }
        }

        // Basic Accounts Checks
        SignerAccount(maker).check()?;
        MintInterface(mint_a).check()?;

        // Return the accounts
        Ok(Self {
            maker,
            escrow,
            mint_a,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })
    }
}

pub struct Refund<'a> {
    pub accounts: RefundAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Refund<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = RefundAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}

impl<'a> Refund<'a> {
    pub const DISCRIMINATOR: &'a u8 = &2;

    pub fn process(&mut self) -> ProgramResult {
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;

        // Check if the escrow is valid
        let escrow_key = create_program_address(
            &[
                b"escrow",
                self.accounts.maker.key.as_ref(),
                &escrow.seed.to_le_bytes(),
                &[escrow.bump],
            ],
            &crate::ID,
        )?;
        if &escrow_key != self.accounts.escrow.key {
            return Err(ProgramError::InvalidAccountOwner);
        }

        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = [escrow.bump];
        let escrow_seeds = [
            b"escrow".as_ref(),
            self.accounts.maker.key.as_ref(),
            seed_binding.as_ref(),
            bump_binding.as_ref(),
        ];

        let vault_data = self.accounts.vault.try_borrow_data()?;
        let vault = TokenAccount::unpack(&vault_data)?;

        // Transfer tokens from vault
        pinocchio_token::instruction::transfer(
            self.accounts.token_program.key,
            self.accounts.vault.key,
            self.accounts.maker_ata_a.key,
            self.accounts.escrow.key,
            &[],
            vault.amount,
        )?;

        // Close the vault
        close_account(
            self.accounts.token_program.key,
            self.accounts.vault.key,
            self.accounts.maker.key,
            self.accounts.escrow.key,
            &[],
        )?;

        // Close the escrow
        self.accounts.escrow.realloc(0, false)?;
        self.accounts
            .escrow
            .assign(&pinocchio_system::ID);

        Ok(())
    }
}
