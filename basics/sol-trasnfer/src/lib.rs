#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use processor::process_instruction;

pub mod instructions;
pub mod processor;

pinocchio_pubkey::declare_id!("QBDA4wAjJpX1rmpW7g6eSdize5Dq4mHbnRxkfNQCWya");

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();


//    * Program Entry Point: This is the main entry point for your Solana program.
//    * #![no_std]: Indicates that the program does not link the standard Rust library, which is typical for Solana programs to keep them small and efficient.
//    * pinocchio_pubkey::declare_id!: Defines the unique program ID for this Solana program.
//    * program_entrypoint!(process_instruction): This macro sets up the process_instruction function from processor.rs as the entry point for all incoming transactions to this program.
//    * no_allocator!() and nostd_panic_handler!(): These macros are part of the pinocchio framework, providing necessary implementations for a no_std environment, specifically for memory allocation and
//      panic handling.