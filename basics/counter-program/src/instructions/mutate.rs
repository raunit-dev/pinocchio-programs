use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};

use crate::{
    constants::COUNTER_SEED,
    error::CounterError,
    state::{Counter, MutationType},
};

pub struct MutateCounterInstructionData {
    pub mutation: MutationType,
}

impl MutateCounterInstructionData {
    pub const LEN: usize = core::mem::size_of::<MutateCounterInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for MutateCounterInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let mutation =
            MutationType::try_from(data).map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(Self { mutation })
    }
}

pub struct MutateCounterIxsAccounts<'info> {
    pub maker: &'info AccountInfo,
    pub counter: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for MutateCounterIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [maker, counter, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !maker.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !counter.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if !counter.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { maker, counter })
    }
}

pub struct Mutate<'info> {
    pub accounts: MutateCounterIxsAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountInfo]> for Mutate<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = MutateCounterIxsAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'info> Mutate<'info> {
    pub fn handler(&mut self, action: MutationType) -> ProgramResult {
        let counter = unsafe {
            bytemuck::try_from_bytes_mut::<Counter>(
                self.accounts.counter.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        let seeds = &[COUNTER_SEED];
        let (counter_pubkey, _) = pubkey::find_program_address(seeds, &crate::ID);

        if self.accounts.counter.key().ne(&counter_pubkey) {
            return Err(ProgramError::InvalidAccountData);
        }

        match action {
            MutationType::INCREASE => self.increment(counter)?,
            MutationType::DECREASE => self.decrement(counter)?,
        }

        Ok(())
    }

    fn increment(&mut self, counter: &mut Counter) -> ProgramResult {
        let mutated_state = u64::from_le_bytes(counter.count)
            .checked_add(1)
            .ok_or(CounterError::Overflow)?;

        counter.count = mutated_state.to_le_bytes();
        Ok(())
    }

    fn decrement(&mut self, counter: &mut Counter) -> ProgramResult {
        let mutated_state = u64::from_le_bytes(counter.count)
            .checked_sub(1)
            .ok_or(CounterError::Overflow)?;

        counter.count = mutated_state.to_le_bytes();
        Ok(())
    }
}
