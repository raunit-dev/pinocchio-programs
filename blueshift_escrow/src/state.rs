use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::Ref, program_error::ProgramError, pubkey::Pubkey};
use core::mem::size_of;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Escrow {
    pub seed: u64,        // Random seed for PDA derivation
    pub maker: Pubkey,    // Creator of the escrow
    pub mint_a: Pubkey,   // Token being deposited
    pub mint_b: Pubkey,   // Token being requested
    pub receive: u64,     // Amount of token B wanted
    pub bump: u8,         // PDA bump seed
    pub _padding: [u8; 7],
}

impl Escrow {
    pub const LEN: usize = size_of::<Self>();

    pub fn load<'a>(data: &'a Ref<[u8]>) -> Result<&'a Self, ProgramError> {
        let escrow: &Self = bytemuck::from_bytes(data);
        Ok(escrow)
    }

    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        let escrow: &mut Self = bytemuck::from_bytes_mut(data);
        Ok(escrow)
    }

    pub fn set_inner(&mut self, seed: u64, maker: Pubkey, mint_a: Pubkey, mint_b: Pubkey, receive: u64, bump: u8) {
        self.seed = seed;
        self.maker = maker;
        self.mint_a = mint_a;
        self.mint_b = mint_b;
        self.receive = receive;
        self.bump = bump;
    }
}